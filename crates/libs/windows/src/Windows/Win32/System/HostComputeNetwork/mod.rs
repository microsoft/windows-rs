#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseEndpoint(endpoint: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCloseEndpoint ( endpoint : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnCloseEndpoint(endpoint).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseGuestNetworkService(guestnetworkservice: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCloseGuestNetworkService ( guestnetworkservice : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnCloseGuestNetworkService(guestnetworkservice).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseLoadBalancer(loadbalancer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCloseLoadBalancer ( loadbalancer : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnCloseLoadBalancer(loadbalancer).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseNamespace(namespace: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCloseNamespace ( namespace : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnCloseNamespace(namespace).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCloseNetwork(network: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCloseNetwork ( network : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnCloseNetwork(network).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateEndpoint<P0>(network: *const ::core::ffi::c_void, id: *const ::windows::core::GUID, settings: P0, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCreateEndpoint ( network : *const ::core::ffi::c_void , id : *const :: windows::core::GUID , settings : :: windows::core::PCWSTR , endpoint : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnCreateEndpoint(network, id, settings.into_param().abi(), endpoint, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateGuestNetworkService<P0>(id: *const ::windows::core::GUID, settings: P0, guestnetworkservice: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCreateGuestNetworkService ( id : *const :: windows::core::GUID , settings : :: windows::core::PCWSTR , guestnetworkservice : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnCreateGuestNetworkService(id, settings.into_param().abi(), guestnetworkservice, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateLoadBalancer<P0>(id: *const ::windows::core::GUID, settings: P0, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCreateLoadBalancer ( id : *const :: windows::core::GUID , settings : :: windows::core::PCWSTR , loadbalancer : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnCreateLoadBalancer(id, settings.into_param().abi(), loadbalancer, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateNamespace<P0>(id: *const ::windows::core::GUID, settings: P0, namespace: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCreateNamespace ( id : *const :: windows::core::GUID , settings : :: windows::core::PCWSTR , namespace : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnCreateNamespace(id, settings.into_param().abi(), namespace, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnCreateNetwork<P0>(id: *const ::windows::core::GUID, settings: P0, network: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnCreateNetwork ( id : *const :: windows::core::GUID , settings : :: windows::core::PCWSTR , network : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnCreateNetwork(id, settings.into_param().abi(), network, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteEndpoint(id: *const ::windows::core::GUID, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnDeleteEndpoint ( id : *const :: windows::core::GUID , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnDeleteEndpoint(id, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteGuestNetworkService(id: *const ::windows::core::GUID, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnDeleteGuestNetworkService ( id : *const :: windows::core::GUID , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnDeleteGuestNetworkService(id, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteLoadBalancer(id: *const ::windows::core::GUID, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnDeleteLoadBalancer ( id : *const :: windows::core::GUID , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnDeleteLoadBalancer(id, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteNamespace(id: *const ::windows::core::GUID, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnDeleteNamespace ( id : *const :: windows::core::GUID , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnDeleteNamespace(id, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnDeleteNetwork(id: *const ::windows::core::GUID, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnDeleteNetwork ( id : *const :: windows::core::GUID , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnDeleteNetwork(id, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateEndpoints<P0>(query: P0, endpoints: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnEnumerateEndpoints ( query : :: windows::core::PCWSTR , endpoints : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnEnumerateEndpoints(query.into_param().abi(), endpoints, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnEnumerateGuestNetworkPortReservations ( returncount : *mut u32 , portentries : *mut *mut HCN_PORT_RANGE_ENTRY ) -> :: windows::core::HRESULT );
    HcnEnumerateGuestNetworkPortReservations(returncount, portentries).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateLoadBalancers<P0>(query: P0, loadbalancer: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnEnumerateLoadBalancers ( query : :: windows::core::PCWSTR , loadbalancer : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnEnumerateLoadBalancers(query.into_param().abi(), loadbalancer, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateNamespaces<P0>(query: P0, namespaces: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnEnumerateNamespaces ( query : :: windows::core::PCWSTR , namespaces : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnEnumerateNamespaces(query.into_param().abi(), namespaces, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnEnumerateNetworks<P0>(query: P0, networks: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnEnumerateNetworks ( query : :: windows::core::PCWSTR , networks : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnEnumerateNetworks(query.into_param().abi(), networks, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnFreeGuestNetworkPortReservations(portentries: ::core::option::Option<*mut HCN_PORT_RANGE_ENTRY>) {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnFreeGuestNetworkPortReservations ( portentries : *mut HCN_PORT_RANGE_ENTRY ) -> ( ) );
    HcnFreeGuestNetworkPortReservations(::core::mem::transmute(portentries.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyEndpoint<P0>(endpoint: *const ::core::ffi::c_void, settings: P0, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnModifyEndpoint ( endpoint : *const ::core::ffi::c_void , settings : :: windows::core::PCWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnModifyEndpoint(endpoint, settings.into_param().abi(), ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyGuestNetworkService<P0>(guestnetworkservice: *const ::core::ffi::c_void, settings: P0, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnModifyGuestNetworkService ( guestnetworkservice : *const ::core::ffi::c_void , settings : :: windows::core::PCWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnModifyGuestNetworkService(guestnetworkservice, settings.into_param().abi(), ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyLoadBalancer<P0>(loadbalancer: *const ::core::ffi::c_void, settings: P0, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnModifyLoadBalancer ( loadbalancer : *const ::core::ffi::c_void , settings : :: windows::core::PCWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnModifyLoadBalancer(loadbalancer, settings.into_param().abi(), ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyNamespace<P0>(namespace: *const ::core::ffi::c_void, settings: P0, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnModifyNamespace ( namespace : *const ::core::ffi::c_void , settings : :: windows::core::PCWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnModifyNamespace(namespace, settings.into_param().abi(), ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnModifyNetwork<P0>(network: *const ::core::ffi::c_void, settings: P0, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnModifyNetwork ( network : *const ::core::ffi::c_void , settings : :: windows::core::PCWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnModifyNetwork(network, settings.into_param().abi(), ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenEndpoint(id: *const ::windows::core::GUID, endpoint: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnOpenEndpoint ( id : *const :: windows::core::GUID , endpoint : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnOpenEndpoint(id, endpoint, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenLoadBalancer(id: *const ::windows::core::GUID, loadbalancer: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnOpenLoadBalancer ( id : *const :: windows::core::GUID , loadbalancer : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnOpenLoadBalancer(id, loadbalancer, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenNamespace(id: *const ::windows::core::GUID, namespace: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnOpenNamespace ( id : *const :: windows::core::GUID , namespace : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnOpenNamespace(id, namespace, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnOpenNetwork(id: *const ::windows::core::GUID, network: *mut *mut ::core::ffi::c_void, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnOpenNetwork ( id : *const :: windows::core::GUID , network : *mut *mut ::core::ffi::c_void , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnOpenNetwork(id, network, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryEndpointProperties<P0>(endpoint: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnQueryEndpointProperties ( endpoint : *const ::core::ffi::c_void , query : :: windows::core::PCWSTR , properties : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnQueryEndpointProperties(endpoint, query.into_param().abi(), properties, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryLoadBalancerProperties<P0>(loadbalancer: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnQueryLoadBalancerProperties ( loadbalancer : *const ::core::ffi::c_void , query : :: windows::core::PCWSTR , properties : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnQueryLoadBalancerProperties(loadbalancer, query.into_param().abi(), properties, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryNamespaceProperties<P0>(namespace: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnQueryNamespaceProperties ( namespace : *const ::core::ffi::c_void , query : :: windows::core::PCWSTR , properties : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnQueryNamespaceProperties(namespace, query.into_param().abi(), properties, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnQueryNetworkProperties<P0>(network: *const ::core::ffi::c_void, query: P0, properties: *mut ::windows::core::PWSTR, errorrecord: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnQueryNetworkProperties ( network : *const ::core::ffi::c_void , query : :: windows::core::PCWSTR , properties : *mut :: windows::core::PWSTR , errorrecord : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcnQueryNetworkProperties(network, query.into_param().abi(), properties, ::core::mem::transmute(errorrecord.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const ::core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnRegisterGuestNetworkServiceCallback ( guestnetworkservice : *const ::core::ffi::c_void , callback : HCN_NOTIFICATION_CALLBACK , context : *const ::core::ffi::c_void , callbackhandle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnRegisterGuestNetworkServiceCallback(guestnetworkservice, callback, context, callbackhandle).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnRegisterServiceCallback(callback: HCN_NOTIFICATION_CALLBACK, context: *const ::core::ffi::c_void, callbackhandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnRegisterServiceCallback ( callback : HCN_NOTIFICATION_CALLBACK , context : *const ::core::ffi::c_void , callbackhandle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnRegisterServiceCallback(callback, context, callbackhandle).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReleaseGuestNetworkServicePortReservationHandle<P0>(portreservationhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnReleaseGuestNetworkServicePortReservationHandle ( portreservationhandle : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const ::core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnReserveGuestNetworkServicePort ( guestnetworkservice : *const ::core::ffi::c_void , protocol : HCN_PORT_PROTOCOL , access : HCN_PORT_ACCESS , port : u16 , portreservationhandle : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HANDLE>();
    HcnReserveGuestNetworkServicePort(guestnetworkservice, protocol, access, port, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const ::core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnReserveGuestNetworkServicePortRange ( guestnetworkservice : *const ::core::ffi::c_void , portcount : u16 , portrangereservation : *mut HCN_PORT_RANGE_RESERVATION , portreservationhandle : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    HcnReserveGuestNetworkServicePortRange(guestnetworkservice, portcount, portrangereservation, portreservationhandle).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnUnregisterGuestNetworkServiceCallback ( callbackhandle : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnUnregisterGuestNetworkServiceCallback(callbackhandle).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
#[inline]
pub unsafe fn HcnUnregisterServiceCallback(callbackhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "computenetwork.dll""system" fn HcnUnregisterServiceCallback ( callbackhandle : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcnUnregisterServiceCallback(callbackhandle).ok()
}
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
impl ::windows::core::TypeKind for HCN_NOTIFICATIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HCN_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCN_NOTIFICATIONS").field(&self.0).finish()
    }
}
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
impl ::windows::core::TypeKind for HCN_PORT_ACCESS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for HCN_PORT_PROTOCOL {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for HCN_PORT_RANGE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.OwningPartitionId == other.OwningPartitionId && self.TargetPartitionId == other.TargetPartitionId && self.Protocol == other.Protocol && self.Priority == other.Priority && self.ReservationType == other.ReservationType && self.SharingFlags == other.SharingFlags && self.DeliveryMode == other.DeliveryMode && self.StartingPort == other.StartingPort && self.EndingPort == other.EndingPort
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
impl ::windows::core::TypeKind for HCN_PORT_RANGE_RESERVATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        self.startingPort == other.startingPort && self.endingPort == other.endingPort
    }
}
impl ::core::cmp::Eq for HCN_PORT_RANGE_RESERVATION {}
impl ::core::default::Default for HCN_PORT_RANGE_RESERVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeNetwork\"`*"]
pub type HCN_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows::core::HRESULT, notificationdata: ::windows::core::PCWSTR) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
