#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCN_NOTIFICATIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(3i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(4i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(5i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(6i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(7i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(8i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(9i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(16i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(17i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(18i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(16777216i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(-268435456i32);
impl ::core::marker::Copy for HCN_NOTIFICATIONS {}
impl ::core::clone::Clone for HCN_NOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCN_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCN_NOTIFICATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCN_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCN_NOTIFICATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub type HCN_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows::core::HRESULT, notificationdata: ::windows::core::PCWSTR)>;
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCN_PORT_ACCESS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = HCN_PORT_ACCESS(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = HCN_PORT_ACCESS(2i32);
impl ::core::marker::Copy for HCN_PORT_ACCESS {}
impl ::core::clone::Clone for HCN_PORT_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCN_PORT_ACCESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCN_PORT_ACCESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCN_PORT_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCN_PORT_ACCESS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCN_PORT_PROTOCOL(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(3i32);
impl ::core::marker::Copy for HCN_PORT_PROTOCOL {}
impl ::core::clone::Clone for HCN_PORT_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCN_PORT_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCN_PORT_PROTOCOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCN_PORT_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCN_PORT_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: ::windows::core::GUID,
    pub TargetPartitionId: ::windows::core::GUID,
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
impl ::core::fmt::Debug for HCN_PORT_RANGE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCN_PORT_RANGE_ENTRY").field("OwningPartitionId", &self.OwningPartitionId).field("TargetPartitionId", &self.TargetPartitionId).field("Protocol", &self.Protocol).field("Priority", &self.Priority).field("ReservationType", &self.ReservationType).field("SharingFlags", &self.SharingFlags).field("DeliveryMode", &self.DeliveryMode).field("StartingPort", &self.StartingPort).field("EndingPort", &self.EndingPort).finish()
    }
}
unsafe impl ::windows::core::Abi for HCN_PORT_RANGE_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCN_PORT_RANGE_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for HCN_PORT_RANGE_ENTRY {}
impl ::core::default::Default for HCN_PORT_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
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
impl ::core::fmt::Debug for HCN_PORT_RANGE_RESERVATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCN_PORT_RANGE_RESERVATION").field("startingPort", &self.startingPort).field("endingPort", &self.endingPort).finish()
    }
}
unsafe impl ::windows::core::Abi for HCN_PORT_RANGE_RESERVATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCN_PORT_RANGE_RESERVATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for HCN_PORT_RANGE_RESERVATION {}
impl ::core::default::Default for HCN_PORT_RANGE_RESERVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseEndpoint(endpoint: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCloseEndpoint(endpoint: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnCloseEndpoint(::core::mem::transmute(endpoint)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCloseGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnCloseGuestNetworkService(::core::mem::transmute(guestnetworkservice)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseLoadBalancer(loadbalancer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCloseLoadBalancer(loadbalancer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnCloseLoadBalancer(::core::mem::transmute(loadbalancer)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseNamespace(namespace: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCloseNamespace(namespace: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnCloseNamespace(::core::mem::transmute(namespace)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseNetwork(network: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCloseNetwork(network: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnCloseNetwork(::core::mem::transmute(network)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateEndpoint<'a, P0>(network: *const ::core::ffi::c_void, id: *const ::windows::core::GUID, settings: P0, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCreateEndpoint(network: *const ::core::ffi::c_void, id: *const ::windows::core::GUID, settings: ::windows::core::PCWSTR, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnCreateEndpoint(::core::mem::transmute(network), ::core::mem::transmute(id), settings.into(), ::core::mem::transmute(endpoint), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateGuestNetworkService<'a, P0>(id: *const ::windows::core::GUID, settings: P0, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCreateGuestNetworkService(id: *const ::windows::core::GUID, settings: ::windows::core::PCWSTR, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnCreateGuestNetworkService(::core::mem::transmute(id), settings.into(), ::core::mem::transmute(guestnetworkservice), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateLoadBalancer<'a, P0>(id: *const ::windows::core::GUID, settings: P0, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCreateLoadBalancer(id: *const ::windows::core::GUID, settings: ::windows::core::PCWSTR, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnCreateLoadBalancer(::core::mem::transmute(id), settings.into(), ::core::mem::transmute(loadbalancer), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateNamespace<'a, P0>(id: *const ::windows::core::GUID, settings: P0, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCreateNamespace(id: *const ::windows::core::GUID, settings: ::windows::core::PCWSTR, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnCreateNamespace(::core::mem::transmute(id), settings.into(), ::core::mem::transmute(namespace), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateNetwork<'a, P0>(id: *const ::windows::core::GUID, settings: P0, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnCreateNetwork(id: *const ::windows::core::GUID, settings: ::windows::core::PCWSTR, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnCreateNetwork(::core::mem::transmute(id), settings.into(), ::core::mem::transmute(network), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteEndpoint(id: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnDeleteEndpoint(id: *const ::windows::core::GUID, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnDeleteEndpoint(::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteGuestNetworkService(id: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnDeleteGuestNetworkService(id: *const ::windows::core::GUID, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnDeleteGuestNetworkService(::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteLoadBalancer(id: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnDeleteLoadBalancer(id: *const ::windows::core::GUID, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnDeleteLoadBalancer(::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteNamespace(id: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnDeleteNamespace(id: *const ::windows::core::GUID, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnDeleteNamespace(::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteNetwork(id: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnDeleteNetwork(id: *const ::windows::core::GUID, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnDeleteNetwork(::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateEndpoints<'a, P0>(query: P0, endpoints: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnEnumerateEndpoints(query: ::windows::core::PCWSTR, endpoints: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnEnumerateEndpoints(query.into(), ::core::mem::transmute(endpoints), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows::core::HRESULT;
    }
    HcnEnumerateGuestNetworkPortReservations(::core::mem::transmute(returncount), ::core::mem::transmute(portentries)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateLoadBalancers<'a, P0>(query: P0, loadbalancer: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnEnumerateLoadBalancers(query: ::windows::core::PCWSTR, loadbalancer: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnEnumerateLoadBalancers(query.into(), ::core::mem::transmute(loadbalancer), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateNamespaces<'a, P0>(query: P0, namespaces: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnEnumerateNamespaces(query: ::windows::core::PCWSTR, namespaces: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnEnumerateNamespaces(query.into(), ::core::mem::transmute(namespaces), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateNetworks<'a, P0>(query: P0, networks: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnEnumerateNetworks(query: ::windows::core::PCWSTR, networks: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnEnumerateNetworks(query.into(), ::core::mem::transmute(networks), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnFreeGuestNetworkPortReservations(portentries: *mut HCN_PORT_RANGE_ENTRY);
    }
    HcnFreeGuestNetworkPortReservations(::core::mem::transmute(portentries))
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyEndpoint<'a, P0>(endpoint: *const ::core::ffi::c_void, settings: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnModifyEndpoint(endpoint: *const ::core::ffi::c_void, settings: ::windows::core::PCWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnModifyEndpoint(::core::mem::transmute(endpoint), settings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyGuestNetworkService<'a, P0>(guestnetworkservice: *const ::core::ffi::c_void, settings: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnModifyGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void, settings: ::windows::core::PCWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnModifyGuestNetworkService(::core::mem::transmute(guestnetworkservice), settings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyLoadBalancer<'a, P0>(loadbalancer: *const ::core::ffi::c_void, settings: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnModifyLoadBalancer(loadbalancer: *const ::core::ffi::c_void, settings: ::windows::core::PCWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnModifyLoadBalancer(::core::mem::transmute(loadbalancer), settings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyNamespace<'a, P0>(namespace: *const ::core::ffi::c_void, settings: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnModifyNamespace(namespace: *const ::core::ffi::c_void, settings: ::windows::core::PCWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnModifyNamespace(::core::mem::transmute(namespace), settings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyNetwork<'a, P0>(network: *const ::core::ffi::c_void, settings: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnModifyNetwork(network: *const ::core::ffi::c_void, settings: ::windows::core::PCWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnModifyNetwork(::core::mem::transmute(network), settings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenEndpoint(id: *const ::windows::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnOpenEndpoint(id: *const ::windows::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnOpenEndpoint(::core::mem::transmute(id), ::core::mem::transmute(endpoint), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenLoadBalancer(id: *const ::windows::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnOpenLoadBalancer(id: *const ::windows::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnOpenLoadBalancer(::core::mem::transmute(id), ::core::mem::transmute(loadbalancer), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenNamespace(id: *const ::windows::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnOpenNamespace(id: *const ::windows::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnOpenNamespace(::core::mem::transmute(id), ::core::mem::transmute(namespace), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenNetwork(id: *const ::windows::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnOpenNetwork(id: *const ::windows::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnOpenNetwork(::core::mem::transmute(id), ::core::mem::transmute(network), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryEndpointProperties<'a, P0>(endpoint: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnQueryEndpointProperties(endpoint: *const ::core::ffi::c_void, query: ::windows::core::PCWSTR, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnQueryEndpointProperties(::core::mem::transmute(endpoint), query.into(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryLoadBalancerProperties<'a, P0>(loadbalancer: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnQueryLoadBalancerProperties(loadbalancer: *const ::core::ffi::c_void, query: ::windows::core::PCWSTR, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnQueryLoadBalancerProperties(::core::mem::transmute(loadbalancer), query.into(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryNamespaceProperties<'a, P0>(namespace: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnQueryNamespaceProperties(namespace: *const ::core::ffi::c_void, query: ::windows::core::PCWSTR, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnQueryNamespaceProperties(::core::mem::transmute(namespace), query.into(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryNetworkProperties<'a, P0>(network: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnQueryNetworkProperties(network: *const ::core::ffi::c_void, query: ::windows::core::PCWSTR, properties: *mut ::windows::core::PWSTR, errorrecord: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcnQueryNetworkProperties(::core::mem::transmute(network), query.into(), ::core::mem::transmute(properties), ::core::mem::transmute(errorrecord)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const ::core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const ::core::ffi::c_void, callback: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnRegisterGuestNetworkServiceCallback(::core::mem::transmute(guestnetworkservice), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(callbackhandle)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnRegisterServiceCallback(callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnRegisterServiceCallback(callback: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnRegisterServiceCallback(::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(callbackhandle)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReleaseGuestNetworkServicePortReservationHandle<'a, P0>(portreservationhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcnReserveGuestNetworkServicePort(::core::mem::transmute(guestnetworkservice), protocol, access, port, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    HcnReserveGuestNetworkServicePortRange(::core::mem::transmute(guestnetworkservice), portcount, ::core::mem::transmute(portrangereservation), ::core::mem::transmute(portreservationhandle)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnUnregisterGuestNetworkServiceCallback(::core::mem::transmute(callbackhandle)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnUnregisterServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcnUnregisterServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcnUnregisterServiceCallback(::core::mem::transmute(callbackhandle)).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
