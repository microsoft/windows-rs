#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtClose(hdrt: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtClose ( hdrt : *const ::core::ffi::c_void ) -> ( ) );
    DrtClose(hdrt)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtContinueSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtContinueSearch ( hsearchcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DrtContinueSearch(hsearchcontext).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn DrtCreateDerivedKey(plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<DRT_DATA> {
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtCreateDerivedKey ( plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT , pkey : *mut DRT_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<DRT_DATA>();
    DrtCreateDerivedKey(plocalcert, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn DrtCreateDerivedKeySecurityProvider(prootcert: *const super::super::Security::Cryptography::CERT_CONTEXT, plocalcert: ::core::option::Option<*const super::super::Security::Cryptography::CERT_CONTEXT>) -> ::windows::core::Result<*mut DRT_SECURITY_PROVIDER> {
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtCreateDerivedKeySecurityProvider ( prootcert : *const super::super::Security::Cryptography:: CERT_CONTEXT , plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT , ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut DRT_SECURITY_PROVIDER>();
    DrtCreateDerivedKeySecurityProvider(prootcert, ::core::mem::transmute(plocalcert.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtCreateDnsBootstrapResolver<P0>(port: u16, pwszaddress: P0) -> ::windows::core::Result<*mut DRT_BOOTSTRAP_PROVIDER>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtCreateDnsBootstrapResolver ( port : u16 , pwszaddress : :: windows::core::PCWSTR , ppmodule : *mut *mut DRT_BOOTSTRAP_PROVIDER ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut DRT_BOOTSTRAP_PROVIDER>();
    DrtCreateDnsBootstrapResolver(port, pwszaddress.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtCreateIpv6UdpTransport(scope: DRT_SCOPE, dwscopeid: u32, dwlocalitythreshold: u32, pwport: *mut u16, phtransport: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drttransport.dll""system" fn DrtCreateIpv6UdpTransport ( scope : DRT_SCOPE , dwscopeid : u32 , dwlocalitythreshold : u32 , pwport : *mut u16 , phtransport : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DrtCreateIpv6UdpTransport(scope, dwscopeid, dwlocalitythreshold, pwport, phtransport).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtCreateNullSecurityProvider() -> ::windows::core::Result<*mut DRT_SECURITY_PROVIDER> {
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtCreateNullSecurityProvider ( ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut DRT_SECURITY_PROVIDER>();
    DrtCreateNullSecurityProvider(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtCreatePnrpBootstrapResolver<P0, P1, P2, P3>(fpublish: P0, pwzpeername: P1, pwzcloudname: P2, pwzpublishingidentity: P3) -> ::windows::core::Result<*mut DRT_BOOTSTRAP_PROVIDER>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtCreatePnrpBootstrapResolver ( fpublish : super::super::Foundation:: BOOL , pwzpeername : :: windows::core::PCWSTR , pwzcloudname : :: windows::core::PCWSTR , pwzpublishingidentity : :: windows::core::PCWSTR , ppresolver : *mut *mut DRT_BOOTSTRAP_PROVIDER ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut DRT_BOOTSTRAP_PROVIDER>();
    DrtCreatePnrpBootstrapResolver(fpublish.into_param().abi(), pwzpeername.into_param().abi(), pwzcloudname.into_param().abi(), pwzpublishingidentity.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtDeleteDerivedKeySecurityProvider ( psecurityprovider : *const DRT_SECURITY_PROVIDER ) -> ( ) );
    DrtDeleteDerivedKeySecurityProvider(psecurityprovider)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtDeleteDnsBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtDeleteDnsBootstrapResolver ( presolver : *const DRT_BOOTSTRAP_PROVIDER ) -> ( ) );
    DrtDeleteDnsBootstrapResolver(presolver)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtDeleteIpv6UdpTransport(htransport: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drttransport.dll""system" fn DrtDeleteIpv6UdpTransport ( htransport : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DrtDeleteIpv6UdpTransport(htransport).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtDeleteNullSecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtDeleteNullSecurityProvider ( psecurityprovider : *const DRT_SECURITY_PROVIDER ) -> ( ) );
    DrtDeleteNullSecurityProvider(psecurityprovider)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtDeletePnrpBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    ::windows::imp::link ! ( "drtprov.dll""system" fn DrtDeletePnrpBootstrapResolver ( presolver : *const DRT_BOOTSTRAP_PROVIDER ) -> ( ) );
    DrtDeletePnrpBootstrapResolver(presolver)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtEndSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtEndSearch ( hsearchcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DrtEndSearch(hsearchcontext).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DrtGetEventData(hdrt: *const ::core::ffi::c_void, uleventdatalen: u32, peventdata: *mut DRT_EVENT_DATA) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetEventData ( hdrt : *const ::core::ffi::c_void , uleventdatalen : u32 , peventdata : *mut DRT_EVENT_DATA ) -> :: windows::core::HRESULT );
    DrtGetEventData(hdrt, uleventdatalen, peventdata).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtGetEventDataSize(hdrt: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetEventDataSize ( hdrt : *const ::core::ffi::c_void , puleventdatalen : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    DrtGetEventDataSize(hdrt, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtGetInstanceName(hdrt: *const ::core::ffi::c_void, ulcbinstancenamesize: u32, pwzdrtinstancename: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetInstanceName ( hdrt : *const ::core::ffi::c_void , ulcbinstancenamesize : u32 , pwzdrtinstancename : :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    DrtGetInstanceName(hdrt, ulcbinstancenamesize, ::core::mem::transmute(pwzdrtinstancename)).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtGetInstanceNameSize(hdrt: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetInstanceNameSize ( hdrt : *const ::core::ffi::c_void , pulcbinstancenamesize : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    DrtGetInstanceNameSize(hdrt, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DrtGetSearchPath(hsearchcontext: *const ::core::ffi::c_void, ulsearchpathsize: u32, psearchpath: *mut DRT_ADDRESS_LIST) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetSearchPath ( hsearchcontext : *const ::core::ffi::c_void , ulsearchpathsize : u32 , psearchpath : *mut DRT_ADDRESS_LIST ) -> :: windows::core::HRESULT );
    DrtGetSearchPath(hsearchcontext, ulsearchpathsize, psearchpath).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtGetSearchPathSize(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetSearchPathSize ( hsearchcontext : *const ::core::ffi::c_void , pulsearchpathsize : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    DrtGetSearchPathSize(hsearchcontext, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtGetSearchResult(hsearchcontext: *const ::core::ffi::c_void, ulsearchresultsize: u32, psearchresult: *mut DRT_SEARCH_RESULT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetSearchResult ( hsearchcontext : *const ::core::ffi::c_void , ulsearchresultsize : u32 , psearchresult : *mut DRT_SEARCH_RESULT ) -> :: windows::core::HRESULT );
    DrtGetSearchResult(hsearchcontext, ulsearchresultsize, psearchresult).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtGetSearchResultSize(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtGetSearchResultSize ( hsearchcontext : *const ::core::ffi::c_void , pulsearchresultsize : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    DrtGetSearchResultSize(hsearchcontext, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtOpen<P0>(psettings: *const DRT_SETTINGS, hevent: P0, pvcontext: ::core::option::Option<*const ::core::ffi::c_void>, phdrt: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "drt.dll""system" fn DrtOpen ( psettings : *const DRT_SETTINGS , hevent : super::super::Foundation:: HANDLE , pvcontext : *const ::core::ffi::c_void , phdrt : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DrtOpen(psettings, hevent.into_param().abi(), ::core::mem::transmute(pvcontext.unwrap_or(::std::ptr::null())), phdrt).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtRegisterKey(hdrt: *const ::core::ffi::c_void, pregistration: *const DRT_REGISTRATION, pvkeycontext: ::core::option::Option<*const ::core::ffi::c_void>, phkeyregistration: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtRegisterKey ( hdrt : *const ::core::ffi::c_void , pregistration : *const DRT_REGISTRATION , pvkeycontext : *const ::core::ffi::c_void , phkeyregistration : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DrtRegisterKey(hdrt, pregistration, ::core::mem::transmute(pvkeycontext.unwrap_or(::std::ptr::null())), phkeyregistration).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrtStartSearch<P0>(hdrt: *const ::core::ffi::c_void, pkey: *const DRT_DATA, pinfo: ::core::option::Option<*const DRT_SEARCH_INFO>, timeout: u32, hevent: P0, pvcontext: ::core::option::Option<*const ::core::ffi::c_void>, hsearchcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "drt.dll""system" fn DrtStartSearch ( hdrt : *const ::core::ffi::c_void , pkey : *const DRT_DATA , pinfo : *const DRT_SEARCH_INFO , timeout : u32 , hevent : super::super::Foundation:: HANDLE , pvcontext : *const ::core::ffi::c_void , hsearchcontext : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DrtStartSearch(hdrt, pkey, ::core::mem::transmute(pinfo.unwrap_or(::std::ptr::null())), timeout, hevent.into_param().abi(), ::core::mem::transmute(pvcontext.unwrap_or(::std::ptr::null())), hsearchcontext).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtUnregisterKey(hkeyregistration: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtUnregisterKey ( hkeyregistration : *const ::core::ffi::c_void ) -> ( ) );
    DrtUnregisterKey(hkeyregistration)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn DrtUpdateKey(hkeyregistration: *const ::core::ffi::c_void, pappdata: *const DRT_DATA) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "drt.dll""system" fn DrtUpdateKey ( hkeyregistration : *const ::core::ffi::c_void , pappdata : *const DRT_DATA ) -> :: windows::core::HRESULT );
    DrtUpdateKey(hkeyregistration, pappdata).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabAddContact<P0>(pwzcontactdata: P0, ppcontact: ::core::option::Option<*mut *mut PEER_CONTACT>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabAddContact ( pwzcontactdata : :: windows::core::PCWSTR , ppcontact : *mut *mut PEER_CONTACT ) -> :: windows::core::HRESULT );
    PeerCollabAddContact(pwzcontactdata.into_param().abi(), ::core::mem::transmute(ppcontact.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabAsyncInviteContact<P0>(pccontact: ::core::option::Option<*const PEER_CONTACT>, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: P0, phinvitation: ::core::option::Option<*mut super::super::Foundation::HANDLE>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabAsyncInviteContact ( pccontact : *const PEER_CONTACT , pcendpoint : *const PEER_ENDPOINT , pcinvitation : *const PEER_INVITATION , hevent : super::super::Foundation:: HANDLE , phinvitation : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    PeerCollabAsyncInviteContact(::core::mem::transmute(pccontact.unwrap_or(::std::ptr::null())), pcendpoint, pcinvitation, hevent.into_param().abi(), ::core::mem::transmute(phinvitation.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabAsyncInviteEndpoint<P0>(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: P0, phinvitation: ::core::option::Option<*mut super::super::Foundation::HANDLE>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabAsyncInviteEndpoint ( pcendpoint : *const PEER_ENDPOINT , pcinvitation : *const PEER_INVITATION , hevent : super::super::Foundation:: HANDLE , phinvitation : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    PeerCollabAsyncInviteEndpoint(pcendpoint, pcinvitation, hevent.into_param().abi(), ::core::mem::transmute(phinvitation.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabCancelInvitation<P0>(hinvitation: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabCancelInvitation ( hinvitation : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    PeerCollabCancelInvitation(hinvitation.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabCloseHandle<P0>(hinvitation: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabCloseHandle ( hinvitation : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    PeerCollabCloseHandle(hinvitation.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabDeleteContact<P0>(pwzpeername: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabDeleteContact ( pwzpeername : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerCollabDeleteContact(pwzpeername.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabDeleteEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabDeleteEndpointData ( pcendpoint : *const PEER_ENDPOINT ) -> :: windows::core::HRESULT );
    PeerCollabDeleteEndpointData(pcendpoint).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabDeleteObject(pobjectid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabDeleteObject ( pobjectid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    PeerCollabDeleteObject(pobjectid).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabEnumApplicationRegistrationInfo(registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabEnumApplicationRegistrationInfo ( registrationtype : PEER_APPLICATION_REGISTRATION_TYPE , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabEnumApplicationRegistrationInfo(registrationtype, phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabEnumApplications(pcendpoint: ::core::option::Option<*const PEER_ENDPOINT>, papplicationid: ::core::option::Option<*const ::windows::core::GUID>, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabEnumApplications ( pcendpoint : *const PEER_ENDPOINT , papplicationid : *const :: windows::core::GUID , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabEnumApplications(::core::mem::transmute(pcendpoint.unwrap_or(::std::ptr::null())), ::core::mem::transmute(papplicationid.unwrap_or(::std::ptr::null())), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabEnumContacts(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabEnumContacts ( phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabEnumContacts(phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabEnumEndpoints(pccontact: ::core::option::Option<*const PEER_CONTACT>, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabEnumEndpoints ( pccontact : *const PEER_CONTACT , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabEnumEndpoints(::core::mem::transmute(pccontact.unwrap_or(::std::ptr::null())), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabEnumObjects(pcendpoint: ::core::option::Option<*const PEER_ENDPOINT>, pobjectid: ::core::option::Option<*const ::windows::core::GUID>, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabEnumObjects ( pcendpoint : *const PEER_ENDPOINT , pobjectid : *const :: windows::core::GUID , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabEnumObjects(::core::mem::transmute(pcendpoint.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pobjectid.unwrap_or(::std::ptr::null())), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabEnumPeopleNearMe(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabEnumPeopleNearMe ( phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabEnumPeopleNearMe(phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabExportContact<P0>(pwzpeername: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabExportContact ( pwzpeername : :: windows::core::PCWSTR , ppwzcontactdata : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerCollabExportContact(pwzpeername.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabGetAppLaunchInfo() -> ::windows::core::Result<*mut PEER_APP_LAUNCH_INFO> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetAppLaunchInfo ( pplaunchinfo : *mut *mut PEER_APP_LAUNCH_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_APP_LAUNCH_INFO>();
    PeerCollabGetAppLaunchInfo(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabGetApplicationRegistrationInfo(papplicationid: *const ::windows::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::Result<*mut PEER_APPLICATION_REGISTRATION_INFO> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetApplicationRegistrationInfo ( papplicationid : *const :: windows::core::GUID , registrationtype : PEER_APPLICATION_REGISTRATION_TYPE , ppapplication : *mut *mut PEER_APPLICATION_REGISTRATION_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_APPLICATION_REGISTRATION_INFO>();
    PeerCollabGetApplicationRegistrationInfo(papplicationid, registrationtype, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabGetContact<P0>(pwzpeername: P0) -> ::windows::core::Result<*mut PEER_CONTACT>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetContact ( pwzpeername : :: windows::core::PCWSTR , ppcontact : *mut *mut PEER_CONTACT ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_CONTACT>();
    PeerCollabGetContact(pwzpeername.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabGetEndpointName() -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetEndpointName ( ppwzendpointname : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerCollabGetEndpointName(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabGetEventData(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_COLLAB_EVENT_DATA> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetEventData ( hpeerevent : *const ::core::ffi::c_void , ppeventdata : *mut *mut PEER_COLLAB_EVENT_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_COLLAB_EVENT_DATA>();
    PeerCollabGetEventData(hpeerevent, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabGetInvitationResponse<P0>(hinvitation: P0) -> ::windows::core::Result<*mut PEER_INVITATION_RESPONSE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetInvitationResponse ( hinvitation : super::super::Foundation:: HANDLE , ppinvitationresponse : *mut *mut PEER_INVITATION_RESPONSE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_INVITATION_RESPONSE>();
    PeerCollabGetInvitationResponse(hinvitation.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabGetPresenceInfo(pcendpoint: ::core::option::Option<*const PEER_ENDPOINT>) -> ::windows::core::Result<*mut PEER_PRESENCE_INFO> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetPresenceInfo ( pcendpoint : *const PEER_ENDPOINT , pppresenceinfo : *mut *mut PEER_PRESENCE_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_PRESENCE_INFO>();
    PeerCollabGetPresenceInfo(::core::mem::transmute(pcendpoint.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabGetSigninOptions() -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabGetSigninOptions ( pdwsigninoptions : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PeerCollabGetSigninOptions(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn PeerCollabInviteContact(pccontact: ::core::option::Option<*const PEER_CONTACT>, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION) -> ::windows::core::Result<*mut PEER_INVITATION_RESPONSE> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabInviteContact ( pccontact : *const PEER_CONTACT , pcendpoint : *const PEER_ENDPOINT , pcinvitation : *const PEER_INVITATION , ppresponse : *mut *mut PEER_INVITATION_RESPONSE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_INVITATION_RESPONSE>();
    PeerCollabInviteContact(::core::mem::transmute(pccontact.unwrap_or(::std::ptr::null())), pcendpoint, pcinvitation, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION) -> ::windows::core::Result<*mut PEER_INVITATION_RESPONSE> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabInviteEndpoint ( pcendpoint : *const PEER_ENDPOINT , pcinvitation : *const PEER_INVITATION , ppresponse : *mut *mut PEER_INVITATION_RESPONSE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_INVITATION_RESPONSE>();
    PeerCollabInviteEndpoint(pcendpoint, pcinvitation, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabParseContact<P0>(pwzcontactdata: P0) -> ::windows::core::Result<*mut PEER_CONTACT>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabParseContact ( pwzcontactdata : :: windows::core::PCWSTR , ppcontact : *mut *mut PEER_CONTACT ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_CONTACT>();
    PeerCollabParseContact(pwzcontactdata.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabQueryContactData(pcendpoint: ::core::option::Option<*const PEER_ENDPOINT>) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabQueryContactData ( pcendpoint : *const PEER_ENDPOINT , ppwzcontactdata : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerCollabQueryContactData(::core::mem::transmute(pcendpoint.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabRefreshEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabRefreshEndpointData ( pcendpoint : *const PEER_ENDPOINT ) -> :: windows::core::HRESULT );
    PeerCollabRefreshEndpointData(pcendpoint).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabRegisterApplication(pcapplication: *const PEER_APPLICATION_REGISTRATION_INFO, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabRegisterApplication ( pcapplication : *const PEER_APPLICATION_REGISTRATION_INFO , registrationtype : PEER_APPLICATION_REGISTRATION_TYPE ) -> :: windows::core::HRESULT );
    PeerCollabRegisterApplication(pcapplication, registrationtype).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabRegisterEvent<P0>(hevent: P0, peventregistrations: &[PEER_COLLAB_EVENT_REGISTRATION], phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabRegisterEvent ( hevent : super::super::Foundation:: HANDLE , ceventregistration : u32 , peventregistrations : *const PEER_COLLAB_EVENT_REGISTRATION , phpeerevent : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabRegisterEvent(hevent.into_param().abi(), peventregistrations.len() as _, ::core::mem::transmute(peventregistrations.as_ptr()), phpeerevent).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabSetEndpointName<P0>(pwzendpointname: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabSetEndpointName ( pwzendpointname : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerCollabSetEndpointName(pwzendpointname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabSetObject(pcobject: *const PEER_OBJECT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabSetObject ( pcobject : *const PEER_OBJECT ) -> :: windows::core::HRESULT );
    PeerCollabSetObject(pcobject).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabSetPresenceInfo(pcpresenceinfo: *const PEER_PRESENCE_INFO) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabSetPresenceInfo ( pcpresenceinfo : *const PEER_PRESENCE_INFO ) -> :: windows::core::HRESULT );
    PeerCollabSetPresenceInfo(pcpresenceinfo).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabShutdown() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabShutdown ( ) -> :: windows::core::HRESULT );
    PeerCollabShutdown().ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabSignin<P0>(hwndparent: P0, dwsigninoptions: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabSignin ( hwndparent : super::super::Foundation:: HWND , dwsigninoptions : u32 ) -> :: windows::core::HRESULT );
    PeerCollabSignin(hwndparent.into_param().abi(), dwsigninoptions).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabSignout(dwsigninoptions: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabSignout ( dwsigninoptions : u32 ) -> :: windows::core::HRESULT );
    PeerCollabSignout(dwsigninoptions).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabStartup(wversionrequested: u16) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabStartup ( wversionrequested : u16 ) -> :: windows::core::HRESULT );
    PeerCollabStartup(wversionrequested).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabSubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabSubscribeEndpointData ( pcendpoint : *const PEER_ENDPOINT ) -> :: windows::core::HRESULT );
    PeerCollabSubscribeEndpointData(pcendpoint).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabUnregisterApplication(papplicationid: *const ::windows::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabUnregisterApplication ( papplicationid : *const :: windows::core::GUID , registrationtype : PEER_APPLICATION_REGISTRATION_TYPE ) -> :: windows::core::HRESULT );
    PeerCollabUnregisterApplication(papplicationid, registrationtype).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCollabUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabUnregisterEvent ( hpeerevent : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerCollabUnregisterEvent(hpeerevent).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabUnsubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabUnsubscribeEndpointData ( pcendpoint : *const PEER_ENDPOINT ) -> :: windows::core::HRESULT );
    PeerCollabUnsubscribeEndpointData(pcendpoint).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerCollabUpdateContact(pcontact: *const PEER_CONTACT) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCollabUpdateContact ( pcontact : *const PEER_CONTACT ) -> :: windows::core::HRESULT );
    PeerCollabUpdateContact(pcontact).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerCreatePeerName<P0, P1>(pwzidentity: P0, pwzclassifier: P1) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerCreatePeerName ( pwzidentity : :: windows::core::PCWSTR , pwzclassifier : :: windows::core::PCWSTR , ppwzpeername : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerCreatePeerName(pwzidentity.into_param().abi(), pwzclassifier.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientAddContentInformation(hpeerdist: isize, hcontenthandle: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientAddContentInformation ( hpeerdist : isize , hcontenthandle : isize , cbnumberofbytes : u32 , pbuffer : *const u8 , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistClientAddContentInformation(hpeerdist, hcontenthandle, pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientAddData(hpeerdist: isize, hcontenthandle: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientAddData ( hpeerdist : isize , hcontenthandle : isize , cbnumberofbytes : u32 , pbuffer : *const u8 , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistClientAddData(hpeerdist, hcontenthandle, pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientBlockRead(hpeerdist: isize, hcontenthandle: isize, pbuffer: ::core::option::Option<&mut [u8]>, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientBlockRead ( hpeerdist : isize , hcontenthandle : isize , cbmaxnumberofbytes : u32 , pbuffer : *mut u8 , dwtimeoutinmilliseconds : u32 , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistClientBlockRead(hpeerdist, hcontenthandle, pbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dwtimeoutinmilliseconds, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientCancelAsyncOperation(hpeerdist: isize, hcontenthandle: isize, poverlapped: ::core::option::Option<*const super::super::System::IO::OVERLAPPED>) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientCancelAsyncOperation ( hpeerdist : isize , hcontenthandle : isize , poverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistClientCancelAsyncOperation(hpeerdist, hcontenthandle, ::core::mem::transmute(poverlapped.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistClientCloseContent(hpeerdist: isize, hcontenthandle: isize) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientCloseContent ( hpeerdist : isize , hcontenthandle : isize ) -> u32 );
    PeerDistClientCloseContent(hpeerdist, hcontenthandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientCompleteContentInformation(hpeerdist: isize, hcontenthandle: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientCompleteContentInformation ( hpeerdist : isize , hcontenthandle : isize , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistClientCompleteContentInformation(hpeerdist, hcontenthandle, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientFlushContent<P0>(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: P0, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientFlushContent ( hpeerdist : isize , pcontenttag : *const PEERDIST_CONTENT_TAG , hcompletionport : super::super::Foundation:: HANDLE , ulcompletionkey : usize , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistClientFlushContent(hpeerdist, pcontenttag, hcompletionport.into_param().abi(), ulcompletionkey, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistClientGetInformationByHandle(hpeerdist: isize, hcontenthandle: isize, peerdistclientinfoclass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize: u32, lpinformation: *mut ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientGetInformationByHandle ( hpeerdist : isize , hcontenthandle : isize , peerdistclientinfoclass : PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS , dwbuffersize : u32 , lpinformation : *mut ::core::ffi::c_void ) -> u32 );
    PeerDistClientGetInformationByHandle(hpeerdist, hcontenthandle, peerdistclientinfoclass, dwbuffersize, lpinformation)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistClientOpenContent<P0>(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: P0, ulcompletionkey: usize, phcontenthandle: *mut isize) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientOpenContent ( hpeerdist : isize , pcontenttag : *const PEERDIST_CONTENT_TAG , hcompletionport : super::super::Foundation:: HANDLE , ulcompletionkey : usize , phcontenthandle : *mut isize ) -> u32 );
    PeerDistClientOpenContent(hpeerdist, pcontenttag, hcompletionport.into_param().abi(), ulcompletionkey, phcontenthandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistClientStreamRead(hpeerdist: isize, hcontenthandle: isize, pbuffer: ::core::option::Option<&mut [u8]>, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistClientStreamRead ( hpeerdist : isize , hcontenthandle : isize , cbmaxnumberofbytes : u32 , pbuffer : *mut u8 , dwtimeoutinmilliseconds : u32 , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistClientStreamRead(hpeerdist, hcontenthandle, pbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dwtimeoutinmilliseconds, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistGetOverlappedResult<P0>(lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistGetOverlappedResult ( lpoverlapped : *const super::super::System::IO:: OVERLAPPED , lpnumberofbytestransferred : *mut u32 , bwait : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    PeerDistGetOverlappedResult(lpoverlapped, lpnumberofbytestransferred, bwait.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistGetStatus(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistGetStatus ( hpeerdist : isize , ppeerdiststatus : *mut PEERDIST_STATUS ) -> u32 );
    PeerDistGetStatus(hpeerdist, ppeerdiststatus)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistGetStatusEx(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistGetStatusEx ( hpeerdist : isize , ppeerdiststatus : *mut PEERDIST_STATUS_INFO ) -> u32 );
    PeerDistGetStatusEx(hpeerdist, ppeerdiststatus)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotification<P0>(hpeerdist: isize, hcompletionport: P0, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistRegisterForStatusChangeNotification ( hpeerdist : isize , hcompletionport : super::super::Foundation:: HANDLE , ulcompletionkey : usize , lpoverlapped : *const super::super::System::IO:: OVERLAPPED , ppeerdiststatus : *mut PEERDIST_STATUS ) -> u32 );
    PeerDistRegisterForStatusChangeNotification(hpeerdist, hcompletionport.into_param().abi(), ulcompletionkey, lpoverlapped, ppeerdiststatus)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotificationEx<P0>(hpeerdist: isize, hcompletionport: P0, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistRegisterForStatusChangeNotificationEx ( hpeerdist : isize , hcompletionport : super::super::Foundation:: HANDLE , ulcompletionkey : usize , lpoverlapped : *const super::super::System::IO:: OVERLAPPED , ppeerdiststatus : *mut PEERDIST_STATUS_INFO ) -> u32 );
    PeerDistRegisterForStatusChangeNotificationEx(hpeerdist, hcompletionport.into_param().abi(), ulcompletionkey, lpoverlapped, ppeerdiststatus)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerCancelAsyncOperation(hpeerdist: isize, pcontentidentifier: &[u8], poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerCancelAsyncOperation ( hpeerdist : isize , cbcontentidentifier : u32 , pcontentidentifier : *const u8 , poverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistServerCancelAsyncOperation(hpeerdist, pcontentidentifier.len() as _, ::core::mem::transmute(pcontentidentifier.as_ptr()), poverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistServerCloseContentInformation(hpeerdist: isize, hcontentinfo: isize) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerCloseContentInformation ( hpeerdist : isize , hcontentinfo : isize ) -> u32 );
    PeerDistServerCloseContentInformation(hpeerdist, hcontentinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistServerCloseStreamHandle(hpeerdist: isize, hstream: isize) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerCloseStreamHandle ( hpeerdist : isize , hstream : isize ) -> u32 );
    PeerDistServerCloseStreamHandle(hpeerdist, hstream)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistServerOpenContentInformation<P0>(hpeerdist: isize, pcontentidentifier: &[u8], ullcontentoffset: u64, cbcontentlength: u64, hcompletionport: P0, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerOpenContentInformation ( hpeerdist : isize , cbcontentidentifier : u32 , pcontentidentifier : *const u8 , ullcontentoffset : u64 , cbcontentlength : u64 , hcompletionport : super::super::Foundation:: HANDLE , ulcompletionkey : usize , phcontentinfo : *mut isize ) -> u32 );
    PeerDistServerOpenContentInformation(hpeerdist, pcontentidentifier.len() as _, ::core::mem::transmute(pcontentidentifier.as_ptr()), ullcontentoffset, cbcontentlength, hcompletionport.into_param().abi(), ulcompletionkey, phcontentinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistServerOpenContentInformationEx<P0>(hpeerdist: isize, pcontentidentifier: &[u8], ullcontentoffset: u64, cbcontentlength: u64, pretrievaloptions: *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport: P0, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerOpenContentInformationEx ( hpeerdist : isize , cbcontentidentifier : u32 , pcontentidentifier : *const u8 , ullcontentoffset : u64 , cbcontentlength : u64 , pretrievaloptions : *const PEERDIST_RETRIEVAL_OPTIONS , hcompletionport : super::super::Foundation:: HANDLE , ulcompletionkey : usize , phcontentinfo : *mut isize ) -> u32 );
    PeerDistServerOpenContentInformationEx(hpeerdist, pcontentidentifier.len() as _, ::core::mem::transmute(pcontentidentifier.as_ptr()), ullcontentoffset, cbcontentlength, pretrievaloptions, hcompletionport.into_param().abi(), ulcompletionkey, phcontentinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerPublishAddToStream(hpeerdist: isize, hstream: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerPublishAddToStream ( hpeerdist : isize , hstream : isize , cbnumberofbytes : u32 , pbuffer : *const u8 , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistServerPublishAddToStream(hpeerdist, hstream, pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerPublishCompleteStream(hpeerdist: isize, hstream: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerPublishCompleteStream ( hpeerdist : isize , hstream : isize , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistServerPublishCompleteStream(hpeerdist, hstream, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerDistServerPublishStream<P0>(hpeerdist: isize, pcontentidentifier: &[u8], cbcontentlength: u64, ppublishoptions: ::core::option::Option<*const PEERDIST_PUBLICATION_OPTIONS>, hcompletionport: P0, ulcompletionkey: usize, phstream: *mut isize) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerPublishStream ( hpeerdist : isize , cbcontentidentifier : u32 , pcontentidentifier : *const u8 , cbcontentlength : u64 , ppublishoptions : *const PEERDIST_PUBLICATION_OPTIONS , hcompletionport : super::super::Foundation:: HANDLE , ulcompletionkey : usize , phstream : *mut isize ) -> u32 );
    PeerDistServerPublishStream(hpeerdist, pcontentidentifier.len() as _, ::core::mem::transmute(pcontentidentifier.as_ptr()), cbcontentlength, ::core::mem::transmute(ppublishoptions.unwrap_or(::std::ptr::null())), hcompletionport.into_param().abi(), ulcompletionkey, phstream)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn PeerDistServerRetrieveContentInformation(hpeerdist: isize, hcontentinfo: isize, pbuffer: &mut [u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerRetrieveContentInformation ( hpeerdist : isize , hcontentinfo : isize , cbmaxnumberofbytes : u32 , pbuffer : *mut u8 , lpoverlapped : *const super::super::System::IO:: OVERLAPPED ) -> u32 );
    PeerDistServerRetrieveContentInformation(hpeerdist, hcontentinfo, pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistServerUnpublish(hpeerdist: isize, pcontentidentifier: &[u8]) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistServerUnpublish ( hpeerdist : isize , cbcontentidentifier : u32 , pcontentidentifier : *const u8 ) -> u32 );
    PeerDistServerUnpublish(hpeerdist, pcontentidentifier.len() as _, ::core::mem::transmute(pcontentidentifier.as_ptr()))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistShutdown(hpeerdist: isize) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistShutdown ( hpeerdist : isize ) -> u32 );
    PeerDistShutdown(hpeerdist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistStartup(dwversionrequested: u32, phpeerdist: *mut isize, pdwsupportedversion: ::core::option::Option<*mut u32>) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistStartup ( dwversionrequested : u32 , phpeerdist : *mut isize , pdwsupportedversion : *mut u32 ) -> u32 );
    PeerDistStartup(dwversionrequested, phpeerdist, ::core::mem::transmute(pdwsupportedversion.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerDistUnregisterForStatusChangeNotification(hpeerdist: isize) -> u32 {
    ::windows::imp::link ! ( "peerdist.dll""system" fn PeerDistUnregisterForStatusChangeNotification ( hpeerdist : isize ) -> u32 );
    PeerDistUnregisterForStatusChangeNotification(hpeerdist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerEndEnumeration ( hpeerenum : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerEndEnumeration(hpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerEnumGroups<P0>(pwzidentity: P0, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerEnumGroups ( pwzidentity : :: windows::core::PCWSTR , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerEnumGroups(pwzidentity.into_param().abi(), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerEnumIdentities(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerEnumIdentities ( phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerEnumIdentities(phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerFreeData(pvdata: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerFreeData ( pvdata : *const ::core::ffi::c_void ) -> ( ) );
    PeerFreeData(::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGetItemCount(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGetItemCount ( hpeerenum : *const ::core::ffi::c_void , pcount : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PeerGetItemCount(hpeerenum, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGetNextItem ( hpeerenum : *const ::core::ffi::c_void , pcount : *mut u32 , pppvitems : *mut *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGetNextItem(hpeerenum, pcount, pppvitems).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphAddRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphAddRecord ( hgraph : *const ::core::ffi::c_void , precord : *const PEER_RECORD , precordid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    PeerGraphAddRecord(hgraph, precord, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphClose(hgraph: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphClose ( hgraph : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphClose(hgraph).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphCloseDirectConnection(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphCloseDirectConnection ( hgraph : *const ::core::ffi::c_void , ullconnectionid : u64 ) -> :: windows::core::HRESULT );
    PeerGraphCloseDirectConnection(hgraph, ullconnectionid).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGraphConnect<P0>(hgraph: *const ::core::ffi::c_void, pwzpeerid: P0, paddress: *const PEER_ADDRESS) -> ::windows::core::Result<u64>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphConnect ( hgraph : *const ::core::ffi::c_void , pwzpeerid : :: windows::core::PCWSTR , paddress : *const PEER_ADDRESS , pullconnectionid : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    PeerGraphConnect(hgraph, pwzpeerid.into_param().abi(), paddress, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphCreate<P0>(pgraphproperties: *const PEER_GRAPH_PROPERTIES, pwzdatabasename: P0, psecurityinterface: ::core::option::Option<*const PEER_SECURITY_INTERFACE>, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphCreate ( pgraphproperties : *const PEER_GRAPH_PROPERTIES , pwzdatabasename : :: windows::core::PCWSTR , psecurityinterface : *const PEER_SECURITY_INTERFACE , phgraph : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphCreate(pgraphproperties, pwzdatabasename.into_param().abi(), ::core::mem::transmute(psecurityinterface.unwrap_or(::std::ptr::null())), phgraph).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphDelete<P0, P1, P2>(pwzgraphid: P0, pwzpeerid: P1, pwzdatabasename: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphDelete ( pwzgraphid : :: windows::core::PCWSTR , pwzpeerid : :: windows::core::PCWSTR , pwzdatabasename : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerGraphDelete(pwzgraphid.into_param().abi(), pwzpeerid.into_param().abi(), pwzdatabasename.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphDeleteRecord<P0>(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID, flocal: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphDeleteRecord ( hgraph : *const ::core::ffi::c_void , precordid : *const :: windows::core::GUID , flocal : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    PeerGraphDeleteRecord(hgraph, precordid, flocal.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphEndEnumeration ( hpeerenum : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphEndEnumeration(hpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphEnumConnections(hgraph: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphEnumConnections ( hgraph : *const ::core::ffi::c_void , dwflags : u32 , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphEnumConnections(hgraph, dwflags, phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphEnumNodes<P0>(hgraph: *const ::core::ffi::c_void, pwzpeerid: P0, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphEnumNodes ( hgraph : *const ::core::ffi::c_void , pwzpeerid : :: windows::core::PCWSTR , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphEnumNodes(hgraph, pwzpeerid.into_param().abi(), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphEnumRecords<P0>(hgraph: *const ::core::ffi::c_void, precordtype: ::core::option::Option<*const ::windows::core::GUID>, pwzpeerid: P0, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphEnumRecords ( hgraph : *const ::core::ffi::c_void , precordtype : *const :: windows::core::GUID , pwzpeerid : :: windows::core::PCWSTR , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphEnumRecords(hgraph, ::core::mem::transmute(precordtype.unwrap_or(::std::ptr::null())), pwzpeerid.into_param().abi(), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphExportDatabase<P0>(hgraph: *const ::core::ffi::c_void, pwzfilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphExportDatabase ( hgraph : *const ::core::ffi::c_void , pwzfilepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerGraphExportDatabase(hgraph, pwzfilepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphFreeData(pvdata: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphFreeData ( pvdata : *const ::core::ffi::c_void ) -> ( ) );
    PeerGraphFreeData(pvdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphGetEventData(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GRAPH_EVENT_DATA> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphGetEventData ( hpeerevent : *const ::core::ffi::c_void , ppeventdata : *mut *mut PEER_GRAPH_EVENT_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_GRAPH_EVENT_DATA>();
    PeerGraphGetEventData(hpeerevent, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphGetItemCount(hpeerenum: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphGetItemCount ( hpeerenum : *const ::core::ffi::c_void , pcount : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PeerGraphGetItemCount(hpeerenum, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphGetNextItem ( hpeerenum : *const ::core::ffi::c_void , pcount : *mut u32 , pppvitems : *mut *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphGetNextItem(hpeerenum, pcount, pppvitems).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGraphGetNodeInfo(hgraph: *const ::core::ffi::c_void, ullnodeid: u64) -> ::windows::core::Result<*mut PEER_NODE_INFO> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphGetNodeInfo ( hgraph : *const ::core::ffi::c_void , ullnodeid : u64 , ppnodeinfo : *mut *mut PEER_NODE_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_NODE_INFO>();
    PeerGraphGetNodeInfo(hgraph, ullnodeid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphGetProperties(hgraph: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GRAPH_PROPERTIES> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphGetProperties ( hgraph : *const ::core::ffi::c_void , ppgraphproperties : *mut *mut PEER_GRAPH_PROPERTIES ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_GRAPH_PROPERTIES>();
    PeerGraphGetProperties(hgraph, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphGetRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut PEER_RECORD> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphGetRecord ( hgraph : *const ::core::ffi::c_void , precordid : *const :: windows::core::GUID , pprecord : *mut *mut PEER_RECORD ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_RECORD>();
    PeerGraphGetRecord(hgraph, precordid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphGetStatus(hgraph: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphGetStatus ( hgraph : *const ::core::ffi::c_void , pdwstatus : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PeerGraphGetStatus(hgraph, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphImportDatabase<P0>(hgraph: *const ::core::ffi::c_void, pwzfilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphImportDatabase ( hgraph : *const ::core::ffi::c_void , pwzfilepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerGraphImportDatabase(hgraph, pwzfilepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphListen(hgraph: *const ::core::ffi::c_void, dwscope: u32, dwscopeid: u32, wport: u16) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphListen ( hgraph : *const ::core::ffi::c_void , dwscope : u32 , dwscopeid : u32 , wport : u16 ) -> :: windows::core::HRESULT );
    PeerGraphListen(hgraph, dwscope, dwscopeid, wport).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphOpen<P0, P1, P2>(pwzgraphid: P0, pwzpeerid: P1, pwzdatabasename: P2, psecurityinterface: ::core::option::Option<*const PEER_SECURITY_INTERFACE>, precordtypesyncprecedence: ::core::option::Option<&[::windows::core::GUID]>, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphOpen ( pwzgraphid : :: windows::core::PCWSTR , pwzpeerid : :: windows::core::PCWSTR , pwzdatabasename : :: windows::core::PCWSTR , psecurityinterface : *const PEER_SECURITY_INTERFACE , crecordtypesyncprecedence : u32 , precordtypesyncprecedence : *const :: windows::core::GUID , phgraph : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphOpen(pwzgraphid.into_param().abi(), pwzpeerid.into_param().abi(), pwzdatabasename.into_param().abi(), ::core::mem::transmute(psecurityinterface.unwrap_or(::std::ptr::null())), precordtypesyncprecedence.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(precordtypesyncprecedence.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), phgraph).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGraphOpenDirectConnection<P0>(hgraph: *const ::core::ffi::c_void, pwzpeerid: P0, paddress: *const PEER_ADDRESS) -> ::windows::core::Result<u64>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphOpenDirectConnection ( hgraph : *const ::core::ffi::c_void , pwzpeerid : :: windows::core::PCWSTR , paddress : *const PEER_ADDRESS , pullconnectionid : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    PeerGraphOpenDirectConnection(hgraph, pwzpeerid.into_param().abi(), paddress, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphPeerTimeToUniversalTime(hgraph: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphPeerTimeToUniversalTime ( hgraph : *const ::core::ffi::c_void , pftpeertime : *const super::super::Foundation:: FILETIME , pftuniversaltime : *mut super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
    PeerGraphPeerTimeToUniversalTime(hgraph, pftpeertime, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphRegisterEvent<P0>(hgraph: *const ::core::ffi::c_void, hevent: P0, peventregistrations: &[PEER_GRAPH_EVENT_REGISTRATION], phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphRegisterEvent ( hgraph : *const ::core::ffi::c_void , hevent : super::super::Foundation:: HANDLE , ceventregistrations : u32 , peventregistrations : *const PEER_GRAPH_EVENT_REGISTRATION , phpeerevent : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphRegisterEvent(hgraph, hevent.into_param().abi(), peventregistrations.len() as _, ::core::mem::transmute(peventregistrations.as_ptr()), phpeerevent).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphSearchRecords<P0>(hgraph: *const ::core::ffi::c_void, pwzcriteria: P0, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphSearchRecords ( hgraph : *const ::core::ffi::c_void , pwzcriteria : :: windows::core::PCWSTR , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphSearchRecords(hgraph, pwzcriteria.into_param().abi(), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphSendData(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphSendData ( hgraph : *const ::core::ffi::c_void , ullconnectionid : u64 , ptype : *const :: windows::core::GUID , cbdata : u32 , pvdata : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphSendData(hgraph, ullconnectionid, ptype, cbdata, pvdata).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphSetNodeAttributes<P0>(hgraph: *const ::core::ffi::c_void, pwzattributes: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphSetNodeAttributes ( hgraph : *const ::core::ffi::c_void , pwzattributes : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerGraphSetNodeAttributes(hgraph, pwzattributes.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphSetPresence<P0>(hgraph: *const ::core::ffi::c_void, fpresent: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphSetPresence ( hgraph : *const ::core::ffi::c_void , fpresent : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    PeerGraphSetPresence(hgraph, fpresent.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphSetProperties(hgraph: *const ::core::ffi::c_void, pgraphproperties: *const PEER_GRAPH_PROPERTIES) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphSetProperties ( hgraph : *const ::core::ffi::c_void , pgraphproperties : *const PEER_GRAPH_PROPERTIES ) -> :: windows::core::HRESULT );
    PeerGraphSetProperties(hgraph, pgraphproperties).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphShutdown() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphShutdown ( ) -> :: windows::core::HRESULT );
    PeerGraphShutdown().ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphStartup(wversionrequested: u16) -> ::windows::core::Result<PEER_VERSION_DATA> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphStartup ( wversionrequested : u16 , pversiondata : *mut PEER_VERSION_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<PEER_VERSION_DATA>();
    PeerGraphStartup(wversionrequested, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphUniversalTimeToPeerTime(hgraph: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphUniversalTimeToPeerTime ( hgraph : *const ::core::ffi::c_void , pftuniversaltime : *const super::super::Foundation:: FILETIME , pftpeertime : *mut super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
    PeerGraphUniversalTimeToPeerTime(hgraph, pftuniversaltime, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphUnregisterEvent ( hpeerevent : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGraphUnregisterEvent(hpeerevent).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGraphUpdateRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphUpdateRecord ( hgraph : *const ::core::ffi::c_void , precord : *const PEER_RECORD ) -> :: windows::core::HRESULT );
    PeerGraphUpdateRecord(hgraph, precord).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGraphValidateDeferredRecords(hgraph: *const ::core::ffi::c_void, precordids: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2pgraph.dll""system" fn PeerGraphValidateDeferredRecords ( hgraph : *const ::core::ffi::c_void , crecordids : u32 , precordids : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    PeerGraphValidateDeferredRecords(hgraph, precordids.len() as _, ::core::mem::transmute(precordids.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupAddRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupAddRecord ( hgroup : *const ::core::ffi::c_void , precord : *const PEER_RECORD , precordid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    PeerGroupAddRecord(hgroup, precord, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupClose(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupClose ( hgroup : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupClose(hgroup).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupCloseDirectConnection(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupCloseDirectConnection ( hgroup : *const ::core::ffi::c_void , ullconnectionid : u64 ) -> :: windows::core::HRESULT );
    PeerGroupCloseDirectConnection(hgroup, ullconnectionid).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupConnect(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupConnect ( hgroup : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupConnect(hgroup).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGroupConnectByAddress(hgroup: *const ::core::ffi::c_void, paddresses: &[PEER_ADDRESS]) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupConnectByAddress ( hgroup : *const ::core::ffi::c_void , caddresses : u32 , paddresses : *const PEER_ADDRESS ) -> :: windows::core::HRESULT );
    PeerGroupConnectByAddress(hgroup, paddresses.len() as _, ::core::mem::transmute(paddresses.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupCreate(pproperties: *const PEER_GROUP_PROPERTIES, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupCreate ( pproperties : *const PEER_GROUP_PROPERTIES , phgroup : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupCreate(pproperties, phgroup).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupCreateInvitation<P0>(hgroup: *const ::core::ffi::c_void, pwzidentityinfo: P0, pftexpiration: ::core::option::Option<*const super::super::Foundation::FILETIME>, proles: ::core::option::Option<&[::windows::core::GUID]>) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupCreateInvitation ( hgroup : *const ::core::ffi::c_void , pwzidentityinfo : :: windows::core::PCWSTR , pftexpiration : *const super::super::Foundation:: FILETIME , croles : u32 , proles : *const :: windows::core::GUID , ppwzinvitation : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerGroupCreateInvitation(hgroup, pwzidentityinfo.into_param().abi(), ::core::mem::transmute(pftexpiration.unwrap_or(::std::ptr::null())), proles.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(proles.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupCreatePasswordInvitation(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupCreatePasswordInvitation ( hgroup : *const ::core::ffi::c_void , ppwzinvitation : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerGroupCreatePasswordInvitation(hgroup, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupDelete<P0, P1>(pwzidentity: P0, pwzgrouppeername: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupDelete ( pwzidentity : :: windows::core::PCWSTR , pwzgrouppeername : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerGroupDelete(pwzidentity.into_param().abi(), pwzgrouppeername.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupDeleteRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupDeleteRecord ( hgroup : *const ::core::ffi::c_void , precordid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    PeerGroupDeleteRecord(hgroup, precordid).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupEnumConnections(hgroup: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupEnumConnections ( hgroup : *const ::core::ffi::c_void , dwflags : u32 , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupEnumConnections(hgroup, dwflags, phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupEnumMembers<P0>(hgroup: *const ::core::ffi::c_void, dwflags: u32, pwzidentity: P0, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupEnumMembers ( hgroup : *const ::core::ffi::c_void , dwflags : u32 , pwzidentity : :: windows::core::PCWSTR , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupEnumMembers(hgroup, dwflags, pwzidentity.into_param().abi(), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupEnumRecords(hgroup: *const ::core::ffi::c_void, precordtype: ::core::option::Option<*const ::windows::core::GUID>, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupEnumRecords ( hgroup : *const ::core::ffi::c_void , precordtype : *const :: windows::core::GUID , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupEnumRecords(hgroup, ::core::mem::transmute(precordtype.unwrap_or(::std::ptr::null())), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupExportConfig<P0>(hgroup: *const ::core::ffi::c_void, pwzpassword: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupExportConfig ( hgroup : *const ::core::ffi::c_void , pwzpassword : :: windows::core::PCWSTR , ppwzxml : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerGroupExportConfig(hgroup, pwzpassword.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupExportDatabase<P0>(hgroup: *const ::core::ffi::c_void, pwzfilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupExportDatabase ( hgroup : *const ::core::ffi::c_void , pwzfilepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerGroupExportDatabase(hgroup, pwzfilepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupGetEventData(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GROUP_EVENT_DATA> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupGetEventData ( hpeerevent : *const ::core::ffi::c_void , ppeventdata : *mut *mut PEER_GROUP_EVENT_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_GROUP_EVENT_DATA>();
    PeerGroupGetEventData(hpeerevent, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupGetProperties(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_GROUP_PROPERTIES> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupGetProperties ( hgroup : *const ::core::ffi::c_void , ppproperties : *mut *mut PEER_GROUP_PROPERTIES ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_GROUP_PROPERTIES>();
    PeerGroupGetProperties(hgroup, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupGetRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut PEER_RECORD> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupGetRecord ( hgroup : *const ::core::ffi::c_void , precordid : *const :: windows::core::GUID , pprecord : *mut *mut PEER_RECORD ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_RECORD>();
    PeerGroupGetRecord(hgroup, precordid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupGetStatus(hgroup: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupGetStatus ( hgroup : *const ::core::ffi::c_void , pdwstatus : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PeerGroupGetStatus(hgroup, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupImportConfig<P0, P1, P2>(pwzxml: P0, pwzpassword: P1, foverwrite: P2, ppwzidentity: *mut ::windows::core::PWSTR, ppwzgroup: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupImportConfig ( pwzxml : :: windows::core::PCWSTR , pwzpassword : :: windows::core::PCWSTR , foverwrite : super::super::Foundation:: BOOL , ppwzidentity : *mut :: windows::core::PWSTR , ppwzgroup : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    PeerGroupImportConfig(pwzxml.into_param().abi(), pwzpassword.into_param().abi(), foverwrite.into_param().abi(), ppwzidentity, ppwzgroup).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupImportDatabase<P0>(hgroup: *const ::core::ffi::c_void, pwzfilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupImportDatabase ( hgroup : *const ::core::ffi::c_void , pwzfilepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerGroupImportDatabase(hgroup, pwzfilepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn PeerGroupIssueCredentials<P0>(hgroup: *const ::core::ffi::c_void, pwzsubjectidentity: P0, pcredentialinfo: ::core::option::Option<*const PEER_CREDENTIAL_INFO>, dwflags: u32, ppwzinvitation: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupIssueCredentials ( hgroup : *const ::core::ffi::c_void , pwzsubjectidentity : :: windows::core::PCWSTR , pcredentialinfo : *const PEER_CREDENTIAL_INFO , dwflags : u32 , ppwzinvitation : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    PeerGroupIssueCredentials(hgroup, pwzsubjectidentity.into_param().abi(), ::core::mem::transmute(pcredentialinfo.unwrap_or(::std::ptr::null())), dwflags, ::core::mem::transmute(ppwzinvitation.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupJoin<P0, P1, P2>(pwzidentity: P0, pwzinvitation: P1, pwzcloud: P2, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupJoin ( pwzidentity : :: windows::core::PCWSTR , pwzinvitation : :: windows::core::PCWSTR , pwzcloud : :: windows::core::PCWSTR , phgroup : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupJoin(pwzidentity.into_param().abi(), pwzinvitation.into_param().abi(), pwzcloud.into_param().abi(), phgroup).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupOpen<P0, P1, P2>(pwzidentity: P0, pwzgrouppeername: P1, pwzcloud: P2, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupOpen ( pwzidentity : :: windows::core::PCWSTR , pwzgrouppeername : :: windows::core::PCWSTR , pwzcloud : :: windows::core::PCWSTR , phgroup : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupOpen(pwzidentity.into_param().abi(), pwzgrouppeername.into_param().abi(), pwzcloud.into_param().abi(), phgroup).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGroupOpenDirectConnection<P0>(hgroup: *const ::core::ffi::c_void, pwzidentity: P0, paddress: *const PEER_ADDRESS) -> ::windows::core::Result<u64>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupOpenDirectConnection ( hgroup : *const ::core::ffi::c_void , pwzidentity : :: windows::core::PCWSTR , paddress : *const PEER_ADDRESS , pullconnectionid : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    PeerGroupOpenDirectConnection(hgroup, pwzidentity.into_param().abi(), paddress, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn PeerGroupParseInvitation<P0>(pwzinvitation: P0) -> ::windows::core::Result<*mut PEER_INVITATION_INFO>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupParseInvitation ( pwzinvitation : :: windows::core::PCWSTR , ppinvitationinfo : *mut *mut PEER_INVITATION_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_INVITATION_INFO>();
    PeerGroupParseInvitation(pwzinvitation.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupPasswordJoin<P0, P1, P2, P3>(pwzidentity: P0, pwzinvitation: P1, pwzpassword: P2, pwzcloud: P3, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupPasswordJoin ( pwzidentity : :: windows::core::PCWSTR , pwzinvitation : :: windows::core::PCWSTR , pwzpassword : :: windows::core::PCWSTR , pwzcloud : :: windows::core::PCWSTR , phgroup : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupPasswordJoin(pwzidentity.into_param().abi(), pwzinvitation.into_param().abi(), pwzpassword.into_param().abi(), pwzcloud.into_param().abi(), phgroup).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupPeerTimeToUniversalTime(hgroup: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupPeerTimeToUniversalTime ( hgroup : *const ::core::ffi::c_void , pftpeertime : *const super::super::Foundation:: FILETIME , pftuniversaltime : *mut super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
    PeerGroupPeerTimeToUniversalTime(hgroup, pftpeertime, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupRegisterEvent<P0>(hgroup: *const ::core::ffi::c_void, hevent: P0, peventregistrations: &[PEER_GROUP_EVENT_REGISTRATION], phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupRegisterEvent ( hgroup : *const ::core::ffi::c_void , hevent : super::super::Foundation:: HANDLE , ceventregistration : u32 , peventregistrations : *const PEER_GROUP_EVENT_REGISTRATION , phpeerevent : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupRegisterEvent(hgroup, hevent.into_param().abi(), peventregistrations.len() as _, ::core::mem::transmute(peventregistrations.as_ptr()), phpeerevent).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupResumePasswordAuthentication(hgroup: *const ::core::ffi::c_void, hpeereventhandle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupResumePasswordAuthentication ( hgroup : *const ::core::ffi::c_void , hpeereventhandle : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupResumePasswordAuthentication(hgroup, hpeereventhandle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupSearchRecords<P0>(hgroup: *const ::core::ffi::c_void, pwzcriteria: P0, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupSearchRecords ( hgroup : *const ::core::ffi::c_void , pwzcriteria : :: windows::core::PCWSTR , phpeerenum : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupSearchRecords(hgroup, pwzcriteria.into_param().abi(), phpeerenum).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupSendData(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupSendData ( hgroup : *const ::core::ffi::c_void , ullconnectionid : u64 , ptype : *const :: windows::core::GUID , cbdata : u32 , pvdata : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupSendData(hgroup, ullconnectionid, ptype, cbdata, pvdata).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupSetProperties(hgroup: *const ::core::ffi::c_void, pproperties: *const PEER_GROUP_PROPERTIES) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupSetProperties ( hgroup : *const ::core::ffi::c_void , pproperties : *const PEER_GROUP_PROPERTIES ) -> :: windows::core::HRESULT );
    PeerGroupSetProperties(hgroup, pproperties).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupShutdown() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupShutdown ( ) -> :: windows::core::HRESULT );
    PeerGroupShutdown().ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupStartup(wversionrequested: u16) -> ::windows::core::Result<PEER_VERSION_DATA> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupStartup ( wversionrequested : u16 , pversiondata : *mut PEER_VERSION_DATA ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<PEER_VERSION_DATA>();
    PeerGroupStartup(wversionrequested, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupUniversalTimeToPeerTime(hgroup: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupUniversalTimeToPeerTime ( hgroup : *const ::core::ffi::c_void , pftuniversaltime : *const super::super::Foundation:: FILETIME , pftpeertime : *mut super::super::Foundation:: FILETIME ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
    PeerGroupUniversalTimeToPeerTime(hgroup, pftuniversaltime, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerGroupUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupUnregisterEvent ( hpeerevent : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerGroupUnregisterEvent(hpeerevent).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerGroupUpdateRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerGroupUpdateRecord ( hgroup : *const ::core::ffi::c_void , precord : *const PEER_RECORD ) -> :: windows::core::HRESULT );
    PeerGroupUpdateRecord(hgroup, precord).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerHostNameToPeerName<P0>(pwzhostname: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerHostNameToPeerName ( pwzhostname : :: windows::core::PCWSTR , ppwzpeername : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerHostNameToPeerName(pwzhostname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityCreate<P0, P1>(pwzclassifier: P0, pwzfriendlyname: P1, hcryptprov: usize) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityCreate ( pwzclassifier : :: windows::core::PCWSTR , pwzfriendlyname : :: windows::core::PCWSTR , hcryptprov : usize , ppwzidentity : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerIdentityCreate(pwzclassifier.into_param().abi(), pwzfriendlyname.into_param().abi(), hcryptprov, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityDelete<P0>(pwzidentity: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityDelete ( pwzidentity : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerIdentityDelete(pwzidentity.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityExport<P0, P1>(pwzidentity: P0, pwzpassword: P1) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityExport ( pwzidentity : :: windows::core::PCWSTR , pwzpassword : :: windows::core::PCWSTR , ppwzexportxml : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerIdentityExport(pwzidentity.into_param().abi(), pwzpassword.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityGetCryptKey<P0>(pwzidentity: P0) -> ::windows::core::Result<usize>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityGetCryptKey ( pwzidentity : :: windows::core::PCWSTR , phcryptprov : *mut usize ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<usize>();
    PeerIdentityGetCryptKey(pwzidentity.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityGetDefault() -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityGetDefault ( ppwzpeername : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerIdentityGetDefault(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityGetFriendlyName<P0>(pwzidentity: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityGetFriendlyName ( pwzidentity : :: windows::core::PCWSTR , ppwzfriendlyname : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerIdentityGetFriendlyName(pwzidentity.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityGetXML<P0>(pwzidentity: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityGetXML ( pwzidentity : :: windows::core::PCWSTR , ppwzidentityxml : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerIdentityGetXML(pwzidentity.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentityImport<P0, P1>(pwzimportxml: P0, pwzpassword: P1) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentityImport ( pwzimportxml : :: windows::core::PCWSTR , pwzpassword : :: windows::core::PCWSTR , ppwzidentity : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerIdentityImport(pwzimportxml.into_param().abi(), pwzpassword.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerIdentitySetFriendlyName<P0, P1>(pwzidentity: P0, pwzfriendlyname: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerIdentitySetFriendlyName ( pwzidentity : :: windows::core::PCWSTR , pwzfriendlyname : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    PeerIdentitySetFriendlyName(pwzidentity.into_param().abi(), pwzfriendlyname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerNameToPeerHostName<P0>(pwzpeername: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerNameToPeerHostName ( pwzpeername : :: windows::core::PCWSTR , ppwzhostname : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PeerNameToPeerHostName(pwzpeername.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerPnrpEndResolve(hresolve: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpEndResolve ( hresolve : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerPnrpEndResolve(hresolve).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerPnrpGetCloudInfo(pcnumclouds: *mut u32, ppcloudinfo: *mut *mut PEER_PNRP_CLOUD_INFO) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpGetCloudInfo ( pcnumclouds : *mut u32 , ppcloudinfo : *mut *mut PEER_PNRP_CLOUD_INFO ) -> :: windows::core::HRESULT );
    PeerPnrpGetCloudInfo(pcnumclouds, ppcloudinfo).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpGetEndpoint(hresolve: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut PEER_PNRP_ENDPOINT_INFO> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpGetEndpoint ( hresolve : *const ::core::ffi::c_void , ppendpoint : *mut *mut PEER_PNRP_ENDPOINT_INFO ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut PEER_PNRP_ENDPOINT_INFO>();
    PeerPnrpGetEndpoint(hresolve, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpRegister<P0>(pcwzpeername: P0, pregistrationinfo: ::core::option::Option<*const PEER_PNRP_REGISTRATION_INFO>, phregistration: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpRegister ( pcwzpeername : :: windows::core::PCWSTR , pregistrationinfo : *const PEER_PNRP_REGISTRATION_INFO , phregistration : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerPnrpRegister(pcwzpeername.into_param().abi(), ::core::mem::transmute(pregistrationinfo.unwrap_or(::std::ptr::null())), phregistration).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpResolve<P0, P1>(pcwzpeername: P0, pcwzcloudname: P1, pcendpoints: *mut u32, ppendpoints: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpResolve ( pcwzpeername : :: windows::core::PCWSTR , pcwzcloudname : :: windows::core::PCWSTR , pcendpoints : *mut u32 , ppendpoints : *mut *mut PEER_PNRP_ENDPOINT_INFO ) -> :: windows::core::HRESULT );
    PeerPnrpResolve(pcwzpeername.into_param().abi(), pcwzcloudname.into_param().abi(), pcendpoints, ppendpoints).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerPnrpShutdown() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpShutdown ( ) -> :: windows::core::HRESULT );
    PeerPnrpShutdown().ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeerPnrpStartResolve<P0, P1, P2>(pcwzpeername: P0, pcwzcloudname: P1, cmaxendpoints: u32, hevent: P2, phresolve: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpStartResolve ( pcwzpeername : :: windows::core::PCWSTR , pcwzcloudname : :: windows::core::PCWSTR , cmaxendpoints : u32 , hevent : super::super::Foundation:: HANDLE , phresolve : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerPnrpStartResolve(pcwzpeername.into_param().abi(), pcwzcloudname.into_param().abi(), cmaxendpoints, hevent.into_param().abi(), phresolve).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerPnrpStartup(wversionrequested: u16) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpStartup ( wversionrequested : u16 ) -> :: windows::core::HRESULT );
    PeerPnrpStartup(wversionrequested).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[inline]
pub unsafe fn PeerPnrpUnregister(hregistration: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpUnregister ( hregistration : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    PeerPnrpUnregister(hregistration).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpUpdateRegistration(hregistration: *const ::core::ffi::c_void, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "p2p.dll""system" fn PeerPnrpUpdateRegistration ( hregistration : *const ::core::ffi::c_void , pregistrationinfo : *const PEER_PNRP_REGISTRATION_INFO ) -> :: windows::core::HRESULT );
    PeerPnrpUpdateRegistration(hregistration, pregistrationinfo).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_BOOTSTRAPPROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052914i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052913i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_CAPABILITY_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052657i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_DUPLICATE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052919i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_FAULTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052662i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INSUFFICIENT_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052660i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052923i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_BOOTSTRAP_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052924i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_CERT_CHAIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057020i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_INSTANCE_PREFIX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052659i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057015i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_KEY_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057022i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_MAX_ADDRESSES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057017i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_MAX_ENDPOINTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057007i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_MESSAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057019i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_PORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052928i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_SCOPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052922i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_SEARCH_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052663i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_SEARCH_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057006i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_SECURITY_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052658i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_SECURITY_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052926i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052664i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_INVALID_TRANSPORT_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052927i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_NO_ADDRESSES_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052920i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_NO_MORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057018i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_SEARCH_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057016i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_SECURITYPROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052916i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_SECURITYPROVIDER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052915i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_STILL_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052925i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141057023i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORTPROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052918i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052917i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_ALREADY_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052671i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052665i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_EXECUTING_CALLBACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052666i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_INVALID_ARGUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052668i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_NOT_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052670i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_NO_DEST_ADDRESSES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052667i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_SHUTTING_DOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052921i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_STILL_BOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052661i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_E_TRANSPORT_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2141052669i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_LINK_LOCAL_ISATAP_SCOPEID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_MAX_INSTANCE_PREFIX_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_MAX_PAYLOAD_SIZE: u32 = 5120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_MAX_ROUTING_ADDRESSES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_MIN_ROUTING_ADDRESSES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_PAYLOAD_REVOKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_S_RETRY: ::windows::core::HRESULT = ::windows::core::HRESULT(6426640i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const FACILITY_DRT: u32 = 98u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const NS_PNRPCLOUD: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const NS_PNRPNAME: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const NS_PROVIDER_PNRPCLOUD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03fe89ce_766d_4976_b9c1_bb9bc42c7b4d);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const NS_PROVIDER_PNRPNAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03fe89cd_766d_4976_b9c1_bb9bc42c7b4d);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_READ_TIMEOUT_DEFAULT: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_READ_TIMEOUT_LOCAL_CACHE_ONLY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_COLLAB_OBJECTID_USER_PICTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd15f41f_fc4e_4922_b035_4c06a754d01d);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024713i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_CLIENT_INVALID_COMPARTMENT_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013390i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_CLOUD_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013394i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_CLOUD_IS_DEAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013387i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_CLOUD_IS_SEARCH_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013391i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_CLOUD_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013395i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_DISK_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024784i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_DUPLICATE_PEER_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013388i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_INVALID_IDENTITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013393i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147023728i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_E_TOO_MUCH_LOAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147013392i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_ROLE_ADMIN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04387127_aa56_450a_8ce5_4f565c6790f4);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_ROLE_INVITING_MEMBER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4370fd89_dc18_4cfb_8dbf_9853a8a9f905);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_ROLE_MEMBER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf12dc4c7_0857_4ca0_93fc_b1bb19a3d8c2);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PNRP_ALL_LINK_CLOUDS: ::windows::core::PCWSTR = ::windows::w!("PEER_PNRP_ALL_LINKS");
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRPINFO_HINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_MAX_ENDPOINT_ADDRESSES: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_MAX_EXTENDED_PAYLOAD_BYTES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const SVCID_PNRPCLOUD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2239ce6_00c0_4fbf_bad6_18139385a49a);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const SVCID_PNRPNAME_V1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2239ce5_00c0_4fbf_bad6_18139385a49a);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const SVCID_PNRPNAME_V2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2239ce7_00c0_4fbf_bad6_18139385a49a);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_CLIENT_INVALID_COMPARTMENT_ID: u32 = 11506u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_CLOUD_DISABLED: u32 = 11502u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_CLOUD_IS_DEAD: u32 = 11509u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_CLOUD_IS_SEARCH_ONLY: u32 = 11505u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_CLOUD_NOT_FOUND: u32 = 11501u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_DUPLICATE_PEER_NAME: u32 = 11508u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_ERROR_BASE: u32 = 11500u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_INVALID_IDENTITY: u32 = 11503u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSA_PNRP_TOO_MUCH_LOAD: u32 = 11504u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSZ_SCOPE_GLOBAL: ::windows::core::PCWSTR = ::windows::w!("GLOBAL");
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSZ_SCOPE_LINKLOCAL: ::windows::core::PCWSTR = ::windows::w!("LINKLOCAL");
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const WSZ_SCOPE_SITELOCAL: ::windows::core::PCWSTR = ::windows::w!("SITELOCAL");
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_ADDRESS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_ACCEPTED: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_REJECTED: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_UNREACHABLE: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_LOOP: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_TOO_BUSY: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_BAD_VALIDATE_ID: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_SUSPECT_UNREGISTERED_ID: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ADDRESS_FLAG_INQUIRE: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(128i32);
impl ::core::marker::Copy for DRT_ADDRESS_FLAGS {}
impl ::core::clone::Clone for DRT_ADDRESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_ADDRESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_ADDRESS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_ADDRESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_ADDRESS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_EVENT_STATUS_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_EVENT_LEAFSET_KEY_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_EVENT_REGISTRATION_STATE_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(2i32);
impl ::core::marker::Copy for DRT_EVENT_TYPE {}
impl ::core::clone::Clone for DRT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_LEAFSET_KEY_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_LEAFSET_KEY_ADDED: DRT_LEAFSET_KEY_CHANGE_TYPE = DRT_LEAFSET_KEY_CHANGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_LEAFSET_KEY_DELETED: DRT_LEAFSET_KEY_CHANGE_TYPE = DRT_LEAFSET_KEY_CHANGE_TYPE(1i32);
impl ::core::marker::Copy for DRT_LEAFSET_KEY_CHANGE_TYPE {}
impl ::core::clone::Clone for DRT_LEAFSET_KEY_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_LEAFSET_KEY_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_LEAFSET_KEY_CHANGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_LEAFSET_KEY_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_LEAFSET_KEY_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_MATCH_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_MATCH_EXACT: DRT_MATCH_TYPE = DRT_MATCH_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_MATCH_NEAR: DRT_MATCH_TYPE = DRT_MATCH_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_MATCH_INTERMEDIATE: DRT_MATCH_TYPE = DRT_MATCH_TYPE(2i32);
impl ::core::marker::Copy for DRT_MATCH_TYPE {}
impl ::core::clone::Clone for DRT_MATCH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_MATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_MATCH_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_MATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_MATCH_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_REGISTRATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_REGISTRATION_STATE_UNRESOLVEABLE: DRT_REGISTRATION_STATE = DRT_REGISTRATION_STATE(1i32);
impl ::core::marker::Copy for DRT_REGISTRATION_STATE {}
impl ::core::clone::Clone for DRT_REGISTRATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_REGISTRATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_REGISTRATION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_REGISTRATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_REGISTRATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_GLOBAL_SCOPE: DRT_SCOPE = DRT_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_SITE_LOCAL_SCOPE: DRT_SCOPE = DRT_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_LINK_LOCAL_SCOPE: DRT_SCOPE = DRT_SCOPE(3i32);
impl ::core::marker::Copy for DRT_SCOPE {}
impl ::core::clone::Clone for DRT_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_SECURITY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_SECURE_RESOLVE: DRT_SECURITY_MODE = DRT_SECURITY_MODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_SECURE_MEMBERSHIP: DRT_SECURITY_MODE = DRT_SECURITY_MODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_SECURE_CONFIDENTIALPAYLOAD: DRT_SECURITY_MODE = DRT_SECURITY_MODE(2i32);
impl ::core::marker::Copy for DRT_SECURITY_MODE {}
impl ::core::clone::Clone for DRT_SECURITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_SECURITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_SECURITY_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_SECURITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_SECURITY_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ACTIVE: DRT_STATUS = DRT_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_ALONE: DRT_STATUS = DRT_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_NO_NETWORK: DRT_STATUS = DRT_STATUS(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const DRT_FAULTED: DRT_STATUS = DRT_STATUS(20i32);
impl ::core::marker::Copy for DRT_STATUS {}
impl ::core::clone::Clone for DRT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRT_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PeerDistClientBasicInfo: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const MaximumPeerDistClientInfoByHandlesClass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(1i32);
impl ::core::marker::Copy for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {}
impl ::core::clone::Clone for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_1: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_2: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(2u32);
impl ::core::marker::Copy for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {}
impl ::core::clone::Clone for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEERDIST_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_STATUS_DISABLED: PEERDIST_STATUS = PEERDIST_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_STATUS_UNAVAILABLE: PEERDIST_STATUS = PEERDIST_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEERDIST_STATUS_AVAILABLE: PEERDIST_STATUS = PEERDIST_STATUS(2i32);
impl ::core::marker::Copy for PEERDIST_STATUS {}
impl ::core::clone::Clone for PEERDIST_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEERDIST_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEERDIST_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEERDIST_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEERDIST_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_APPLICATION_REGISTRATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_APPLICATION_CURRENT_USER: PEER_APPLICATION_REGISTRATION_TYPE = PEER_APPLICATION_REGISTRATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_APPLICATION_ALL_USERS: PEER_APPLICATION_REGISTRATION_TYPE = PEER_APPLICATION_REGISTRATION_TYPE(1i32);
impl ::core::marker::Copy for PEER_APPLICATION_REGISTRATION_TYPE {}
impl ::core::clone::Clone for PEER_APPLICATION_REGISTRATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_APPLICATION_REGISTRATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_APPLICATION_REGISTRATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_APPLICATION_REGISTRATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_APPLICATION_REGISTRATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_CHANGE_ADDED: PEER_CHANGE_TYPE = PEER_CHANGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_CHANGE_DELETED: PEER_CHANGE_TYPE = PEER_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_CHANGE_UPDATED: PEER_CHANGE_TYPE = PEER_CHANGE_TYPE(2i32);
impl ::core::marker::Copy for PEER_CHANGE_TYPE {}
impl ::core::clone::Clone for PEER_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_CHANGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_COLLAB_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_WATCHLIST_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_ENDPOINT_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_ENDPOINT_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_ENDPOINT_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_MY_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_MY_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_MY_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_MY_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_PEOPLE_NEAR_ME_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_EVENT_REQUEST_STATUS_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(11i32);
impl ::core::marker::Copy for PEER_COLLAB_EVENT_TYPE {}
impl ::core::clone::Clone for PEER_COLLAB_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_COLLAB_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_COLLAB_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_COLLAB_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_COLLAB_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_CONNECTION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_CONNECTION_NEIGHBOR: PEER_CONNECTION_FLAGS = PEER_CONNECTION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_CONNECTION_DIRECT: PEER_CONNECTION_FLAGS = PEER_CONNECTION_FLAGS(2i32);
impl ::core::marker::Copy for PEER_CONNECTION_FLAGS {}
impl ::core::clone::Clone for PEER_CONNECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_CONNECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_CONNECTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_CONNECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_CONNECTION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_CONNECTION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_CONNECTED: PEER_CONNECTION_STATUS = PEER_CONNECTION_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_DISCONNECTED: PEER_CONNECTION_STATUS = PEER_CONNECTION_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_CONNECTION_FAILED: PEER_CONNECTION_STATUS = PEER_CONNECTION_STATUS(3i32);
impl ::core::marker::Copy for PEER_CONNECTION_STATUS {}
impl ::core::clone::Clone for PEER_CONNECTION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_CONNECTION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_CONNECTION_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_CONNECTION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_CONNECTION_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GRAPH_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_STATUS_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_PROPERTY_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_RECORD_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_DIRECT_CONNECTION: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_NEIGHBOR_CONNECTION: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_INCOMING_DATA: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_CONNECTION_REQUIRED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_NODE_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_EVENT_SYNCHRONIZED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(9i32);
impl ::core::marker::Copy for PEER_GRAPH_EVENT_TYPE {}
impl ::core::clone::Clone for PEER_GRAPH_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GRAPH_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GRAPH_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GRAPH_PROPERTY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_PROPERTY_HEARTBEATS: PEER_GRAPH_PROPERTY_FLAGS = PEER_GRAPH_PROPERTY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_PROPERTY_DEFER_EXPIRATION: PEER_GRAPH_PROPERTY_FLAGS = PEER_GRAPH_PROPERTY_FLAGS(2i32);
impl ::core::marker::Copy for PEER_GRAPH_PROPERTY_FLAGS {}
impl ::core::clone::Clone for PEER_GRAPH_PROPERTY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GRAPH_PROPERTY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_PROPERTY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GRAPH_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_PROPERTY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GRAPH_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_SCOPE_ANY: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_SCOPE_GLOBAL: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_SCOPE_SITELOCAL: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_SCOPE_LINKLOCAL: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_SCOPE_LOOPBACK: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(4i32);
impl ::core::marker::Copy for PEER_GRAPH_SCOPE {}
impl ::core::clone::Clone for PEER_GRAPH_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GRAPH_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GRAPH_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GRAPH_STATUS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_STATUS_LISTENING: PEER_GRAPH_STATUS_FLAGS = PEER_GRAPH_STATUS_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_STATUS_HAS_CONNECTIONS: PEER_GRAPH_STATUS_FLAGS = PEER_GRAPH_STATUS_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GRAPH_STATUS_SYNCHRONIZED: PEER_GRAPH_STATUS_FLAGS = PEER_GRAPH_STATUS_FLAGS(4i32);
impl ::core::marker::Copy for PEER_GRAPH_STATUS_FLAGS {}
impl ::core::clone::Clone for PEER_GRAPH_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GRAPH_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_STATUS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GRAPH_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GROUP_AUTHENTICATION_SCHEME(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_GMC_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = PEER_GROUP_AUTHENTICATION_SCHEME(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_PASSWORD_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = PEER_GROUP_AUTHENTICATION_SCHEME(2i32);
impl ::core::marker::Copy for PEER_GROUP_AUTHENTICATION_SCHEME {}
impl ::core::clone::Clone for PEER_GROUP_AUTHENTICATION_SCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GROUP_AUTHENTICATION_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_AUTHENTICATION_SCHEME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GROUP_AUTHENTICATION_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_AUTHENTICATION_SCHEME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GROUP_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_STATUS_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_PROPERTY_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_RECORD_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_DIRECT_CONNECTION: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_NEIGHBOR_CONNECTION: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_INCOMING_DATA: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_MEMBER_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_CONNECTION_FAILED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_EVENT_AUTHENTICATION_FAILED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(11i32);
impl ::core::marker::Copy for PEER_GROUP_EVENT_TYPE {}
impl ::core::clone::Clone for PEER_GROUP_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GROUP_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GROUP_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GROUP_ISSUE_CREDENTIAL_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_STORE_CREDENTIALS: PEER_GROUP_ISSUE_CREDENTIAL_FLAGS = PEER_GROUP_ISSUE_CREDENTIAL_FLAGS(1i32);
impl ::core::marker::Copy for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {}
impl ::core::clone::Clone for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_ISSUE_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GROUP_PROPERTY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_MEMBER_DATA_OPTIONAL: PEER_GROUP_PROPERTY_FLAGS = PEER_GROUP_PROPERTY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_DISABLE_PRESENCE: PEER_GROUP_PROPERTY_FLAGS = PEER_GROUP_PROPERTY_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_DEFER_EXPIRATION: PEER_GROUP_PROPERTY_FLAGS = PEER_GROUP_PROPERTY_FLAGS(4i32);
impl ::core::marker::Copy for PEER_GROUP_PROPERTY_FLAGS {}
impl ::core::clone::Clone for PEER_GROUP_PROPERTY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GROUP_PROPERTY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_PROPERTY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GROUP_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_PROPERTY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_GROUP_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_STATUS_LISTENING: PEER_GROUP_STATUS = PEER_GROUP_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_GROUP_STATUS_HAS_CONNECTIONS: PEER_GROUP_STATUS = PEER_GROUP_STATUS(2i32);
impl ::core::marker::Copy for PEER_GROUP_STATUS {}
impl ::core::clone::Clone for PEER_GROUP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_GROUP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_GROUP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_INVITATION_RESPONSE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_INVITATION_RESPONSE_DECLINED: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_INVITATION_RESPONSE_ACCEPTED: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_INVITATION_RESPONSE_EXPIRED: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_INVITATION_RESPONSE_ERROR: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(3i32);
impl ::core::marker::Copy for PEER_INVITATION_RESPONSE_TYPE {}
impl ::core::clone::Clone for PEER_INVITATION_RESPONSE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_INVITATION_RESPONSE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_INVITATION_RESPONSE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_INVITATION_RESPONSE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_INVITATION_RESPONSE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_MEMBER_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_MEMBER_CONNECTED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_MEMBER_DISCONNECTED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_MEMBER_UPDATED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_MEMBER_JOINED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_MEMBER_LEFT: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(5i32);
impl ::core::marker::Copy for PEER_MEMBER_CHANGE_TYPE {}
impl ::core::clone::Clone for PEER_MEMBER_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_MEMBER_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_MEMBER_CHANGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_MEMBER_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_MEMBER_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_MEMBER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_MEMBER_PRESENT: PEER_MEMBER_FLAGS = PEER_MEMBER_FLAGS(1i32);
impl ::core::marker::Copy for PEER_MEMBER_FLAGS {}
impl ::core::clone::Clone for PEER_MEMBER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_MEMBER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_MEMBER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_MEMBER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_MEMBER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_NODE_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_NODE_CHANGE_CONNECTED: PEER_NODE_CHANGE_TYPE = PEER_NODE_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_NODE_CHANGE_DISCONNECTED: PEER_NODE_CHANGE_TYPE = PEER_NODE_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_NODE_CHANGE_UPDATED: PEER_NODE_CHANGE_TYPE = PEER_NODE_CHANGE_TYPE(3i32);
impl ::core::marker::Copy for PEER_NODE_CHANGE_TYPE {}
impl ::core::clone::Clone for PEER_NODE_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_NODE_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_NODE_CHANGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_NODE_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_NODE_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_PRESENCE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_OFFLINE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_OUT_TO_LUNCH: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_AWAY: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_BE_RIGHT_BACK: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_IDLE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_BUSY: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_ON_THE_PHONE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PRESENCE_ONLINE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(7i32);
impl ::core::marker::Copy for PEER_PRESENCE_STATUS {}
impl ::core::clone::Clone for PEER_PRESENCE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_PRESENCE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_PRESENCE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_PRESENCE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_PRESENCE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_PUBLICATION_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PUBLICATION_SCOPE_NONE: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PUBLICATION_SCOPE_NEAR_ME: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PUBLICATION_SCOPE_INTERNET: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_PUBLICATION_SCOPE_ALL: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(3i32);
impl ::core::marker::Copy for PEER_PUBLICATION_SCOPE {}
impl ::core::clone::Clone for PEER_PUBLICATION_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_PUBLICATION_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_PUBLICATION_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_PUBLICATION_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_PUBLICATION_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_RECORD_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_RECORD_ADDED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_RECORD_UPDATED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_RECORD_DELETED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_RECORD_EXPIRED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(4i32);
impl ::core::marker::Copy for PEER_RECORD_CHANGE_TYPE {}
impl ::core::clone::Clone for PEER_RECORD_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_RECORD_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_RECORD_CHANGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_RECORD_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_RECORD_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_RECORD_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_RECORD_FLAG_AUTOREFRESH: PEER_RECORD_FLAGS = PEER_RECORD_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_RECORD_FLAG_DELETED: PEER_RECORD_FLAGS = PEER_RECORD_FLAGS(2i32);
impl ::core::marker::Copy for PEER_RECORD_FLAGS {}
impl ::core::clone::Clone for PEER_RECORD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_RECORD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_RECORD_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_RECORD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_RECORD_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_SIGNIN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_SIGNIN_NONE: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_SIGNIN_NEAR_ME: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_SIGNIN_INTERNET: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_SIGNIN_ALL: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(3i32);
impl ::core::marker::Copy for PEER_SIGNIN_FLAGS {}
impl ::core::clone::Clone for PEER_SIGNIN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_SIGNIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_SIGNIN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_SIGNIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_SIGNIN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PEER_WATCH_PERMISSION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_WATCH_BLOCKED: PEER_WATCH_PERMISSION = PEER_WATCH_PERMISSION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PEER_WATCH_ALLOWED: PEER_WATCH_PERMISSION = PEER_WATCH_PERMISSION(1i32);
impl ::core::marker::Copy for PEER_WATCH_PERMISSION {}
impl ::core::clone::Clone for PEER_WATCH_PERMISSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PEER_WATCH_PERMISSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PEER_WATCH_PERMISSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PEER_WATCH_PERMISSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_WATCH_PERMISSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PNRP_CLOUD_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_NO_FLAGS: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_NAME_LOCAL: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_RESOLVE_ONLY: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_FULL_PARTICIPANT: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(4i32);
impl ::core::marker::Copy for PNRP_CLOUD_FLAGS {}
impl ::core::clone::Clone for PNRP_CLOUD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PNRP_CLOUD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PNRP_CLOUD_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PNRP_CLOUD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_CLOUD_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PNRP_CLOUD_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_STATE_VIRTUAL: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_STATE_SYNCHRONISING: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_STATE_ACTIVE: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_STATE_DEAD: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_STATE_DISABLED: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_STATE_NO_NET: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_CLOUD_STATE_ALONE: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(6i32);
impl ::core::marker::Copy for PNRP_CLOUD_STATE {}
impl ::core::clone::Clone for PNRP_CLOUD_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PNRP_CLOUD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PNRP_CLOUD_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PNRP_CLOUD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_CLOUD_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PNRP_EXTENDED_PAYLOAD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_EXTENDED_PAYLOAD_TYPE_NONE: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_EXTENDED_PAYLOAD_TYPE_BINARY: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_EXTENDED_PAYLOAD_TYPE_STRING: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(2i32);
impl ::core::marker::Copy for PNRP_EXTENDED_PAYLOAD_TYPE {}
impl ::core::clone::Clone for PNRP_EXTENDED_PAYLOAD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PNRP_EXTENDED_PAYLOAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PNRP_EXTENDED_PAYLOAD_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PNRP_EXTENDED_PAYLOAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_EXTENDED_PAYLOAD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PNRP_REGISTERED_ID_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_REGISTERED_ID_STATE_OK: PNRP_REGISTERED_ID_STATE = PNRP_REGISTERED_ID_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_REGISTERED_ID_STATE_PROBLEM: PNRP_REGISTERED_ID_STATE = PNRP_REGISTERED_ID_STATE(2i32);
impl ::core::marker::Copy for PNRP_REGISTERED_ID_STATE {}
impl ::core::clone::Clone for PNRP_REGISTERED_ID_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PNRP_REGISTERED_ID_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PNRP_REGISTERED_ID_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PNRP_REGISTERED_ID_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_REGISTERED_ID_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PNRP_RESOLVE_CRITERIA(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_RESOLVE_CRITERIA_DEFAULT: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_RESOLVE_CRITERIA_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_RESOLVE_CRITERIA_NEAREST_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_RESOLVE_CRITERIA_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_RESOLVE_CRITERIA_NEAREST_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_RESOLVE_CRITERIA_ANY_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_RESOLVE_CRITERIA_NEAREST_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(6i32);
impl ::core::marker::Copy for PNRP_RESOLVE_CRITERIA {}
impl ::core::clone::Clone for PNRP_RESOLVE_CRITERIA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PNRP_RESOLVE_CRITERIA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PNRP_RESOLVE_CRITERIA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PNRP_RESOLVE_CRITERIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_RESOLVE_CRITERIA").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PNRP_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_SCOPE_ANY: PNRP_SCOPE = PNRP_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_GLOBAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_SITE_LOCAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub const PNRP_LINK_LOCAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(3i32);
impl ::core::marker::Copy for PNRP_SCOPE {}
impl ::core::clone::Clone for PNRP_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PNRP_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PNRP_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PNRP_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_SCOPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DRT_ADDRESS {
    pub socketAddress: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub flags: u32,
    pub nearness: i32,
    pub latency: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for DRT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_ADDRESS").field("socketAddress", &self.socketAddress).field("flags", &self.flags).field("nearness", &self.nearness).field("latency", &self.latency).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DRT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.socketAddress == other.socketAddress && self.flags == other.flags && self.nearness == other.nearness && self.latency == other.latency
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DRT_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DRT_ADDRESS_LIST {
    pub AddressCount: u32,
    pub AddressList: [DRT_ADDRESS; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_ADDRESS_LIST {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for DRT_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_ADDRESS_LIST").field("AddressCount", &self.AddressCount).field("AddressList", &self.AddressList).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_ADDRESS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DRT_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AddressCount == other.AddressCount && self.AddressList == other.AddressList
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DRT_ADDRESS_LIST {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct DRT_BOOTSTRAP_PROVIDER {
    pub pvContext: *mut ::core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub InitResolve: isize,
    pub IssueResolve: isize,
    pub EndResolve: isize,
    pub Register: isize,
    pub Unregister: isize,
}
impl ::core::marker::Copy for DRT_BOOTSTRAP_PROVIDER {}
impl ::core::clone::Clone for DRT_BOOTSTRAP_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRT_BOOTSTRAP_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_BOOTSTRAP_PROVIDER").field("pvContext", &self.pvContext).field("Attach", &self.Attach).field("Detach", &self.Detach).field("InitResolve", &self.InitResolve).field("IssueResolve", &self.IssueResolve).field("EndResolve", &self.EndResolve).field("Register", &self.Register).field("Unregister", &self.Unregister).finish()
    }
}
impl ::windows::core::TypeKind for DRT_BOOTSTRAP_PROVIDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRT_BOOTSTRAP_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.pvContext == other.pvContext && self.Attach == other.Attach && self.Detach == other.Detach && self.InitResolve == other.InitResolve && self.IssueResolve == other.IssueResolve && self.EndResolve == other.EndResolve && self.Register == other.Register && self.Unregister == other.Unregister
    }
}
impl ::core::cmp::Eq for DRT_BOOTSTRAP_PROVIDER {}
impl ::core::default::Default for DRT_BOOTSTRAP_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct DRT_DATA {
    pub cb: u32,
    pub pb: *mut u8,
}
impl ::core::marker::Copy for DRT_DATA {}
impl ::core::clone::Clone for DRT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_DATA").field("cb", &self.cb).field("pb", &self.pb).finish()
    }
}
impl ::windows::core::TypeKind for DRT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.pb == other.pb
    }
}
impl ::core::cmp::Eq for DRT_DATA {}
impl ::core::default::Default for DRT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DRT_EVENT_DATA {
    pub r#type: DRT_EVENT_TYPE,
    pub hr: ::windows::core::HRESULT,
    pub pvContext: *mut ::core::ffi::c_void,
    pub Anonymous: DRT_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_EVENT_DATA {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_EVENT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union DRT_EVENT_DATA_0 {
    pub leafsetKeyChange: DRT_EVENT_DATA_0_0,
    pub registrationStateChange: DRT_EVENT_DATA_0_1,
    pub statusChange: DRT_EVENT_DATA_0_2,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_EVENT_DATA_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_EVENT_DATA_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DRT_EVENT_DATA_0_0 {
    pub change: DRT_LEAFSET_KEY_CHANGE_TYPE,
    pub localKey: DRT_DATA,
    pub remoteKey: DRT_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for DRT_EVENT_DATA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_EVENT_DATA_0_0").field("change", &self.change).field("localKey", &self.localKey).field("remoteKey", &self.remoteKey).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_EVENT_DATA_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.change == other.change && self.localKey == other.localKey && self.remoteKey == other.remoteKey
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DRT_EVENT_DATA_0_1 {
    pub state: DRT_REGISTRATION_STATE,
    pub localKey: DRT_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for DRT_EVENT_DATA_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_EVENT_DATA_0_1").field("state", &self.state).field("localKey", &self.localKey).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_EVENT_DATA_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.localKey == other.localKey
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DRT_EVENT_DATA_0_2 {
    pub status: DRT_STATUS,
    pub bootstrapAddresses: DRT_EVENT_DATA_0_2_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for DRT_EVENT_DATA_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_EVENT_DATA_0_2").field("status", &self.status).field("bootstrapAddresses", &self.bootstrapAddresses).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_EVENT_DATA_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.bootstrapAddresses == other.bootstrapAddresses
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DRT_EVENT_DATA_0_2_0 {
    pub cntAddress: u32,
    pub pAddresses: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_2_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for DRT_EVENT_DATA_0_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_EVENT_DATA_0_2_0").field("cntAddress", &self.cntAddress).field("pAddresses", &self.pAddresses).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for DRT_EVENT_DATA_0_2_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DRT_EVENT_DATA_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.cntAddress == other.cntAddress && self.pAddresses == other.pAddresses
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DRT_EVENT_DATA_0_2_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DRT_EVENT_DATA_0_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct DRT_REGISTRATION {
    pub key: DRT_DATA,
    pub appData: DRT_DATA,
}
impl ::core::marker::Copy for DRT_REGISTRATION {}
impl ::core::clone::Clone for DRT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_REGISTRATION").field("key", &self.key).field("appData", &self.appData).finish()
    }
}
impl ::windows::core::TypeKind for DRT_REGISTRATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.appData == other.appData
    }
}
impl ::core::cmp::Eq for DRT_REGISTRATION {}
impl ::core::default::Default for DRT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRT_SEARCH_INFO {
    pub dwSize: u32,
    pub fIterative: super::super::Foundation::BOOL,
    pub fAllowCurrentInstanceMatch: super::super::Foundation::BOOL,
    pub fAnyMatchInRange: super::super::Foundation::BOOL,
    pub cMaxEndpoints: u32,
    pub pMaximumKey: *mut DRT_DATA,
    pub pMinimumKey: *mut DRT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRT_SEARCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRT_SEARCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRT_SEARCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SEARCH_INFO").field("dwSize", &self.dwSize).field("fIterative", &self.fIterative).field("fAllowCurrentInstanceMatch", &self.fAllowCurrentInstanceMatch).field("fAnyMatchInRange", &self.fAnyMatchInRange).field("cMaxEndpoints", &self.cMaxEndpoints).field("pMaximumKey", &self.pMaximumKey).field("pMinimumKey", &self.pMinimumKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DRT_SEARCH_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRT_SEARCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fIterative == other.fIterative && self.fAllowCurrentInstanceMatch == other.fAllowCurrentInstanceMatch && self.fAnyMatchInRange == other.fAnyMatchInRange && self.cMaxEndpoints == other.cMaxEndpoints && self.pMaximumKey == other.pMaximumKey && self.pMinimumKey == other.pMinimumKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRT_SEARCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRT_SEARCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct DRT_SEARCH_RESULT {
    pub dwSize: u32,
    pub r#type: DRT_MATCH_TYPE,
    pub pvContext: *mut ::core::ffi::c_void,
    pub registration: DRT_REGISTRATION,
}
impl ::core::marker::Copy for DRT_SEARCH_RESULT {}
impl ::core::clone::Clone for DRT_SEARCH_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRT_SEARCH_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SEARCH_RESULT").field("dwSize", &self.dwSize).field("type", &self.r#type).field("pvContext", &self.pvContext).field("registration", &self.registration).finish()
    }
}
impl ::windows::core::TypeKind for DRT_SEARCH_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRT_SEARCH_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.r#type == other.r#type && self.pvContext == other.pvContext && self.registration == other.registration
    }
}
impl ::core::cmp::Eq for DRT_SEARCH_RESULT {}
impl ::core::default::Default for DRT_SEARCH_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct DRT_SECURITY_PROVIDER {
    pub pvContext: *mut ::core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub RegisterKey: isize,
    pub UnregisterKey: isize,
    pub ValidateAndUnpackPayload: isize,
    pub SecureAndPackPayload: isize,
    pub FreeData: isize,
    pub EncryptData: isize,
    pub DecryptData: isize,
    pub GetSerializedCredential: isize,
    pub ValidateRemoteCredential: isize,
    pub SignData: isize,
    pub VerifyData: isize,
}
impl ::core::marker::Copy for DRT_SECURITY_PROVIDER {}
impl ::core::clone::Clone for DRT_SECURITY_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRT_SECURITY_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SECURITY_PROVIDER")
            .field("pvContext", &self.pvContext)
            .field("Attach", &self.Attach)
            .field("Detach", &self.Detach)
            .field("RegisterKey", &self.RegisterKey)
            .field("UnregisterKey", &self.UnregisterKey)
            .field("ValidateAndUnpackPayload", &self.ValidateAndUnpackPayload)
            .field("SecureAndPackPayload", &self.SecureAndPackPayload)
            .field("FreeData", &self.FreeData)
            .field("EncryptData", &self.EncryptData)
            .field("DecryptData", &self.DecryptData)
            .field("GetSerializedCredential", &self.GetSerializedCredential)
            .field("ValidateRemoteCredential", &self.ValidateRemoteCredential)
            .field("SignData", &self.SignData)
            .field("VerifyData", &self.VerifyData)
            .finish()
    }
}
impl ::windows::core::TypeKind for DRT_SECURITY_PROVIDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRT_SECURITY_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.pvContext == other.pvContext && self.Attach == other.Attach && self.Detach == other.Detach && self.RegisterKey == other.RegisterKey && self.UnregisterKey == other.UnregisterKey && self.ValidateAndUnpackPayload == other.ValidateAndUnpackPayload && self.SecureAndPackPayload == other.SecureAndPackPayload && self.FreeData == other.FreeData && self.EncryptData == other.EncryptData && self.DecryptData == other.DecryptData && self.GetSerializedCredential == other.GetSerializedCredential && self.ValidateRemoteCredential == other.ValidateRemoteCredential && self.SignData == other.SignData && self.VerifyData == other.VerifyData
    }
}
impl ::core::cmp::Eq for DRT_SECURITY_PROVIDER {}
impl ::core::default::Default for DRT_SECURITY_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct DRT_SETTINGS {
    pub dwSize: u32,
    pub cbKey: u32,
    pub bProtocolMajorVersion: u8,
    pub bProtocolMinorVersion: u8,
    pub ulMaxRoutingAddresses: u32,
    pub pwzDrtInstancePrefix: ::windows::core::PWSTR,
    pub hTransport: *mut ::core::ffi::c_void,
    pub pSecurityProvider: *mut DRT_SECURITY_PROVIDER,
    pub pBootstrapProvider: *mut DRT_BOOTSTRAP_PROVIDER,
    pub eSecurityMode: DRT_SECURITY_MODE,
}
impl ::core::marker::Copy for DRT_SETTINGS {}
impl ::core::clone::Clone for DRT_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRT_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SETTINGS")
            .field("dwSize", &self.dwSize)
            .field("cbKey", &self.cbKey)
            .field("bProtocolMajorVersion", &self.bProtocolMajorVersion)
            .field("bProtocolMinorVersion", &self.bProtocolMinorVersion)
            .field("ulMaxRoutingAddresses", &self.ulMaxRoutingAddresses)
            .field("pwzDrtInstancePrefix", &self.pwzDrtInstancePrefix)
            .field("hTransport", &self.hTransport)
            .field("pSecurityProvider", &self.pSecurityProvider)
            .field("pBootstrapProvider", &self.pBootstrapProvider)
            .field("eSecurityMode", &self.eSecurityMode)
            .finish()
    }
}
impl ::windows::core::TypeKind for DRT_SETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DRT_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cbKey == other.cbKey && self.bProtocolMajorVersion == other.bProtocolMajorVersion && self.bProtocolMinorVersion == other.bProtocolMinorVersion && self.ulMaxRoutingAddresses == other.ulMaxRoutingAddresses && self.pwzDrtInstancePrefix == other.pwzDrtInstancePrefix && self.hTransport == other.hTransport && self.pSecurityProvider == other.pSecurityProvider && self.pBootstrapProvider == other.pBootstrapProvider && self.eSecurityMode == other.eSecurityMode
    }
}
impl ::core::cmp::Eq for DRT_SETTINGS {}
impl ::core::default::Default for DRT_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEERDIST_CLIENT_BASIC_INFO {
    pub fFlashCrowd: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEERDIST_CLIENT_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEERDIST_CLIENT_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEERDIST_CLIENT_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_CLIENT_BASIC_INFO").field("fFlashCrowd", &self.fFlashCrowd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PEERDIST_CLIENT_BASIC_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEERDIST_CLIENT_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fFlashCrowd == other.fFlashCrowd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEERDIST_CLIENT_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEERDIST_CLIENT_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEERDIST_CONTENT_TAG {
    pub Data: [u8; 16],
}
impl ::core::marker::Copy for PEERDIST_CONTENT_TAG {}
impl ::core::clone::Clone for PEERDIST_CONTENT_TAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEERDIST_CONTENT_TAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_CONTENT_TAG").field("Data", &self.Data).finish()
    }
}
impl ::windows::core::TypeKind for PEERDIST_CONTENT_TAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEERDIST_CONTENT_TAG {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for PEERDIST_CONTENT_TAG {}
impl ::core::default::Default for PEERDIST_CONTENT_TAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEERDIST_PUBLICATION_OPTIONS {
    pub dwVersion: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for PEERDIST_PUBLICATION_OPTIONS {}
impl ::core::clone::Clone for PEERDIST_PUBLICATION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEERDIST_PUBLICATION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_PUBLICATION_OPTIONS").field("dwVersion", &self.dwVersion).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for PEERDIST_PUBLICATION_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEERDIST_PUBLICATION_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for PEERDIST_PUBLICATION_OPTIONS {}
impl ::core::default::Default for PEERDIST_PUBLICATION_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEERDIST_RETRIEVAL_OPTIONS {
    pub cbSize: u32,
    pub dwContentInfoMinVersion: u32,
    pub dwContentInfoMaxVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for PEERDIST_RETRIEVAL_OPTIONS {}
impl ::core::clone::Clone for PEERDIST_RETRIEVAL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEERDIST_RETRIEVAL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_RETRIEVAL_OPTIONS").field("cbSize", &self.cbSize).field("dwContentInfoMinVersion", &self.dwContentInfoMinVersion).field("dwContentInfoMaxVersion", &self.dwContentInfoMaxVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::windows::core::TypeKind for PEERDIST_RETRIEVAL_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEERDIST_RETRIEVAL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwContentInfoMinVersion == other.dwContentInfoMinVersion && self.dwContentInfoMaxVersion == other.dwContentInfoMaxVersion && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for PEERDIST_RETRIEVAL_OPTIONS {}
impl ::core::default::Default for PEERDIST_RETRIEVAL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEERDIST_STATUS_INFO {
    pub cbSize: u32,
    pub status: PEERDIST_STATUS,
    pub dwMinVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
    pub dwMaxVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
}
impl ::core::marker::Copy for PEERDIST_STATUS_INFO {}
impl ::core::clone::Clone for PEERDIST_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEERDIST_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_STATUS_INFO").field("cbSize", &self.cbSize).field("status", &self.status).field("dwMinVer", &self.dwMinVer).field("dwMaxVer", &self.dwMaxVer).finish()
    }
}
impl ::windows::core::TypeKind for PEERDIST_STATUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEERDIST_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.status == other.status && self.dwMinVer == other.dwMinVer && self.dwMaxVer == other.dwMaxVer
    }
}
impl ::core::cmp::Eq for PEERDIST_STATUS_INFO {}
impl ::core::default::Default for PEERDIST_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_ADDRESS {
    pub dwSize: u32,
    pub sin6: super::super::Networking::WinSock::SOCKADDR_IN6,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_APPLICATION {
    pub id: ::windows::core::GUID,
    pub data: PEER_DATA,
    pub pwzDescription: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PEER_APPLICATION {}
impl ::core::clone::Clone for PEER_APPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_APPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_APPLICATION").field("id", &self.id).field("data", &self.data).field("pwzDescription", &self.pwzDescription).finish()
    }
}
impl ::windows::core::TypeKind for PEER_APPLICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_APPLICATION {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data && self.pwzDescription == other.pwzDescription
    }
}
impl ::core::cmp::Eq for PEER_APPLICATION {}
impl ::core::default::Default for PEER_APPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_APPLICATION_REGISTRATION_INFO {
    pub application: PEER_APPLICATION,
    pub pwzApplicationToLaunch: ::windows::core::PWSTR,
    pub pwzApplicationArguments: ::windows::core::PWSTR,
    pub dwPublicationScope: u32,
}
impl ::core::marker::Copy for PEER_APPLICATION_REGISTRATION_INFO {}
impl ::core::clone::Clone for PEER_APPLICATION_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_APPLICATION_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_APPLICATION_REGISTRATION_INFO").field("application", &self.application).field("pwzApplicationToLaunch", &self.pwzApplicationToLaunch).field("pwzApplicationArguments", &self.pwzApplicationArguments).field("dwPublicationScope", &self.dwPublicationScope).finish()
    }
}
impl ::windows::core::TypeKind for PEER_APPLICATION_REGISTRATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_APPLICATION_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.application == other.application && self.pwzApplicationToLaunch == other.pwzApplicationToLaunch && self.pwzApplicationArguments == other.pwzApplicationArguments && self.dwPublicationScope == other.dwPublicationScope
    }
}
impl ::core::cmp::Eq for PEER_APPLICATION_REGISTRATION_INFO {}
impl ::core::default::Default for PEER_APPLICATION_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_APP_LAUNCH_INFO {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub pInvitation: *mut PEER_INVITATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_APP_LAUNCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_APP_LAUNCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_APP_LAUNCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_APP_LAUNCH_INFO").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("pInvitation", &self.pInvitation).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows::core::TypeKind for PEER_APP_LAUNCH_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_APP_LAUNCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.pInvitation == other.pInvitation
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_APP_LAUNCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_APP_LAUNCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_COLLAB_EVENT_DATA {
    pub eventType: PEER_COLLAB_EVENT_TYPE,
    pub Anonymous: PEER_COLLAB_EVENT_DATA_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_COLLAB_EVENT_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_COLLAB_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows::core::TypeKind for PEER_COLLAB_EVENT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_COLLAB_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union PEER_COLLAB_EVENT_DATA_0 {
    pub watchListChangedData: PEER_EVENT_WATCHLIST_CHANGED_DATA,
    pub presenceChangedData: PEER_EVENT_PRESENCE_CHANGED_DATA,
    pub applicationChangedData: PEER_EVENT_APPLICATION_CHANGED_DATA,
    pub objectChangedData: PEER_EVENT_OBJECT_CHANGED_DATA,
    pub endpointChangedData: PEER_EVENT_ENDPOINT_CHANGED_DATA,
    pub peopleNearMeChangedData: PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA,
    pub requestStatusChangedData: PEER_EVENT_REQUEST_STATUS_CHANGED_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_COLLAB_EVENT_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_COLLAB_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows::core::TypeKind for PEER_COLLAB_EVENT_DATA_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_COLLAB_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_COLLAB_EVENT_REGISTRATION {
    pub eventType: PEER_COLLAB_EVENT_TYPE,
    pub pInstance: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_COLLAB_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_COLLAB_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_COLLAB_EVENT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_COLLAB_EVENT_REGISTRATION").field("eventType", &self.eventType).field("pInstance", &self.pInstance).finish()
    }
}
impl ::windows::core::TypeKind for PEER_COLLAB_EVENT_REGISTRATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_COLLAB_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.eventType == other.eventType && self.pInstance == other.pInstance
    }
}
impl ::core::cmp::Eq for PEER_COLLAB_EVENT_REGISTRATION {}
impl ::core::default::Default for PEER_COLLAB_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_CONNECTION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub pwzPeerId: ::windows::core::PWSTR,
    pub address: PEER_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_CONNECTION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_CONNECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_CONNECTION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_CONTACT {
    pub pwzPeerName: ::windows::core::PWSTR,
    pub pwzNickName: ::windows::core::PWSTR,
    pub pwzDisplayName: ::windows::core::PWSTR,
    pub pwzEmailAddress: ::windows::core::PWSTR,
    pub fWatch: super::super::Foundation::BOOL,
    pub WatcherPermissions: PEER_WATCH_PERMISSION,
    pub credentials: PEER_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_CONTACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_CONTACT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEER_CONTACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_CONTACT").field("pwzPeerName", &self.pwzPeerName).field("pwzNickName", &self.pwzNickName).field("pwzDisplayName", &self.pwzDisplayName).field("pwzEmailAddress", &self.pwzEmailAddress).field("fWatch", &self.fWatch).field("WatcherPermissions", &self.WatcherPermissions).field("credentials", &self.credentials).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PEER_CONTACT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_CONTACT {
    fn eq(&self, other: &Self) -> bool {
        self.pwzPeerName == other.pwzPeerName && self.pwzNickName == other.pwzNickName && self.pwzDisplayName == other.pwzDisplayName && self.pwzEmailAddress == other.pwzEmailAddress && self.fWatch == other.fWatch && self.WatcherPermissions == other.WatcherPermissions && self.credentials == other.credentials
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_CONTACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_CONTACT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct PEER_CREDENTIAL_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzFriendlyName: ::windows::core::PWSTR,
    pub pPublicKey: *mut super::super::Security::Cryptography::CERT_PUBLIC_KEY_INFO,
    pub pwzIssuerPeerName: ::windows::core::PWSTR,
    pub pwzIssuerFriendlyName: ::windows::core::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PEER_CREDENTIAL_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PEER_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for PEER_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_CREDENTIAL_INFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("pwzFriendlyName", &self.pwzFriendlyName).field("pPublicKey", &self.pPublicKey).field("pwzIssuerPeerName", &self.pwzIssuerPeerName).field("pwzIssuerFriendlyName", &self.pwzIssuerFriendlyName).field("ftValidityStart", &self.ftValidityStart).field("ftValidityEnd", &self.ftValidityEnd).field("cRoles", &self.cRoles).field("pRoles", &self.pRoles).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for PEER_CREDENTIAL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.pwzFriendlyName == other.pwzFriendlyName && self.pPublicKey == other.pPublicKey && self.pwzIssuerPeerName == other.pwzIssuerPeerName && self.pwzIssuerFriendlyName == other.pwzIssuerFriendlyName && self.ftValidityStart == other.ftValidityStart && self.ftValidityEnd == other.ftValidityEnd && self.cRoles == other.cRoles && self.pRoles == other.pRoles
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_CREDENTIAL_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_DATA {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for PEER_DATA {}
impl ::core::clone::Clone for PEER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_DATA").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::windows::core::TypeKind for PEER_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for PEER_DATA {}
impl ::core::default::Default for PEER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_ENDPOINT {
    pub address: PEER_ADDRESS,
    pub pwzEndpointName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_ENDPOINT {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_ENDPOINT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_APPLICATION_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pApplication: *mut PEER_APPLICATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_APPLICATION_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_APPLICATION_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("changeType", &self.changeType).field("pApplication", &self.pApplication).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows::core::TypeKind for PEER_EVENT_APPLICATION_CHANGED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.changeType == other.changeType && self.pApplication == other.pApplication
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_APPLICATION_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_EVENT_CONNECTION_CHANGE_DATA {
    pub dwSize: u32,
    pub status: PEER_CONNECTION_STATUS,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub ullNextConnectionId: u64,
    pub hrConnectionFailedReason: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for PEER_EVENT_CONNECTION_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_CONNECTION_CHANGE_DATA").field("dwSize", &self.dwSize).field("status", &self.status).field("ullConnectionId", &self.ullConnectionId).field("ullNodeId", &self.ullNodeId).field("ullNextConnectionId", &self.ullNextConnectionId).field("hrConnectionFailedReason", &self.hrConnectionFailedReason).finish()
    }
}
impl ::windows::core::TypeKind for PEER_EVENT_CONNECTION_CHANGE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.status == other.status && self.ullConnectionId == other.ullConnectionId && self.ullNodeId == other.ullNodeId && self.ullNextConnectionId == other.ullNextConnectionId && self.hrConnectionFailedReason == other.hrConnectionFailedReason
    }
}
impl ::core::cmp::Eq for PEER_EVENT_CONNECTION_CHANGE_DATA {}
impl ::core::default::Default for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_ENDPOINT_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_ENDPOINT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_ENDPOINT_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows::core::TypeKind for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_ENDPOINT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_EVENT_INCOMING_DATA {
    pub dwSize: u32,
    pub ullConnectionId: u64,
    pub r#type: ::windows::core::GUID,
    pub data: PEER_DATA,
}
impl ::core::marker::Copy for PEER_EVENT_INCOMING_DATA {}
impl ::core::clone::Clone for PEER_EVENT_INCOMING_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_EVENT_INCOMING_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_INCOMING_DATA").field("dwSize", &self.dwSize).field("ullConnectionId", &self.ullConnectionId).field("type", &self.r#type).field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for PEER_EVENT_INCOMING_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_EVENT_INCOMING_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ullConnectionId == other.ullConnectionId && self.r#type == other.r#type && self.data == other.data
    }
}
impl ::core::cmp::Eq for PEER_EVENT_INCOMING_DATA {}
impl ::core::default::Default for PEER_EVENT_INCOMING_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_EVENT_MEMBER_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_MEMBER_CHANGE_TYPE,
    pub pwzIdentity: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PEER_EVENT_MEMBER_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_MEMBER_CHANGE_DATA").field("dwSize", &self.dwSize).field("changeType", &self.changeType).field("pwzIdentity", &self.pwzIdentity).finish()
    }
}
impl ::windows::core::TypeKind for PEER_EVENT_MEMBER_CHANGE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.changeType == other.changeType && self.pwzIdentity == other.pwzIdentity
    }
}
impl ::core::cmp::Eq for PEER_EVENT_MEMBER_CHANGE_DATA {}
impl ::core::default::Default for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_EVENT_NODE_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_NODE_CHANGE_TYPE,
    pub ullNodeId: u64,
    pub pwzPeerId: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PEER_EVENT_NODE_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_NODE_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_EVENT_NODE_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_NODE_CHANGE_DATA").field("dwSize", &self.dwSize).field("changeType", &self.changeType).field("ullNodeId", &self.ullNodeId).field("pwzPeerId", &self.pwzPeerId).finish()
    }
}
impl ::windows::core::TypeKind for PEER_EVENT_NODE_CHANGE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_EVENT_NODE_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.changeType == other.changeType && self.ullNodeId == other.ullNodeId && self.pwzPeerId == other.pwzPeerId
    }
}
impl ::core::cmp::Eq for PEER_EVENT_NODE_CHANGE_DATA {}
impl ::core::default::Default for PEER_EVENT_NODE_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_OBJECT_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pObject: *mut PEER_OBJECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_OBJECT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_OBJECT_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("changeType", &self.changeType).field("pObject", &self.pObject).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows::core::TypeKind for PEER_EVENT_OBJECT_CHANGED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.changeType == other.changeType && self.pObject == other.pObject
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_OBJECT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    pub changeType: PEER_CHANGE_TYPE,
    pub pPeopleNearMe: *mut PEER_PEOPLE_NEAR_ME,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA").field("changeType", &self.changeType).field("pPeopleNearMe", &self.pPeopleNearMe).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.pPeopleNearMe == other.pPeopleNearMe
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_PRESENCE_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pPresenceInfo: *mut PEER_PRESENCE_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_PRESENCE_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_PRESENCE_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("changeType", &self.changeType).field("pPresenceInfo", &self.pPresenceInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows::core::TypeKind for PEER_EVENT_PRESENCE_CHANGED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.changeType == other.changeType && self.pPresenceInfo == other.pPresenceInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_PRESENCE_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_EVENT_RECORD_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_RECORD_CHANGE_TYPE,
    pub recordId: ::windows::core::GUID,
    pub recordType: ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_EVENT_RECORD_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_RECORD_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_EVENT_RECORD_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_RECORD_CHANGE_DATA").field("dwSize", &self.dwSize).field("changeType", &self.changeType).field("recordId", &self.recordId).field("recordType", &self.recordType).finish()
    }
}
impl ::windows::core::TypeKind for PEER_EVENT_RECORD_CHANGE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_EVENT_RECORD_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.changeType == other.changeType && self.recordId == other.recordId && self.recordType == other.recordType
    }
}
impl ::core::cmp::Eq for PEER_EVENT_RECORD_CHANGE_DATA {}
impl ::core::default::Default for PEER_EVENT_RECORD_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub hrChange: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_REQUEST_STATUS_CHANGED_DATA").field("pEndpoint", &self.pEndpoint).field("hrChange", &self.hrChange).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pEndpoint == other.pEndpoint && self.hrChange == other.hrChange
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_EVENT_SYNCHRONIZED_DATA {
    pub dwSize: u32,
    pub recordType: ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_EVENT_SYNCHRONIZED_DATA {}
impl ::core::clone::Clone for PEER_EVENT_SYNCHRONIZED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_EVENT_SYNCHRONIZED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_SYNCHRONIZED_DATA").field("dwSize", &self.dwSize).field("recordType", &self.recordType).finish()
    }
}
impl ::windows::core::TypeKind for PEER_EVENT_SYNCHRONIZED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_EVENT_SYNCHRONIZED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.recordType == other.recordType
    }
}
impl ::core::cmp::Eq for PEER_EVENT_SYNCHRONIZED_DATA {}
impl ::core::default::Default for PEER_EVENT_SYNCHRONIZED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_EVENT_WATCHLIST_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub changeType: PEER_CHANGE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_EVENT_WATCHLIST_CHANGED_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_WATCHLIST_CHANGED_DATA").field("pContact", &self.pContact).field("changeType", &self.changeType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.changeType == other.changeType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_EVENT_WATCHLIST_CHANGED_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_GRAPH_EVENT_DATA {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub Anonymous: PEER_GRAPH_EVENT_DATA_0,
}
impl ::core::marker::Copy for PEER_GRAPH_EVENT_DATA {}
impl ::core::clone::Clone for PEER_GRAPH_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_EVENT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PEER_GRAPH_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub union PEER_GRAPH_EVENT_DATA_0 {
    pub dwStatus: PEER_GRAPH_STATUS_FLAGS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub nodeChangeData: PEER_EVENT_NODE_CHANGE_DATA,
    pub synchronizedData: PEER_EVENT_SYNCHRONIZED_DATA,
}
impl ::core::marker::Copy for PEER_GRAPH_EVENT_DATA_0 {}
impl ::core::clone::Clone for PEER_GRAPH_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_EVENT_DATA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PEER_GRAPH_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_GRAPH_EVENT_REGISTRATION {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub pType: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_GRAPH_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_GRAPH_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_GRAPH_EVENT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GRAPH_EVENT_REGISTRATION").field("eventType", &self.eventType).field("pType", &self.pType).finish()
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_EVENT_REGISTRATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_GRAPH_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.eventType == other.eventType && self.pType == other.pType
    }
}
impl ::core::cmp::Eq for PEER_GRAPH_EVENT_REGISTRATION {}
impl ::core::default::Default for PEER_GRAPH_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_GRAPH_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwScope: u32,
    pub dwMaxRecordSize: u32,
    pub pwzGraphId: ::windows::core::PWSTR,
    pub pwzCreatorId: ::windows::core::PWSTR,
    pub pwzFriendlyName: ::windows::core::PWSTR,
    pub pwzComment: ::windows::core::PWSTR,
    pub ulPresenceLifetime: u32,
    pub cPresenceMax: u32,
}
impl ::core::marker::Copy for PEER_GRAPH_PROPERTIES {}
impl ::core::clone::Clone for PEER_GRAPH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_GRAPH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GRAPH_PROPERTIES").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwScope", &self.dwScope).field("dwMaxRecordSize", &self.dwMaxRecordSize).field("pwzGraphId", &self.pwzGraphId).field("pwzCreatorId", &self.pwzCreatorId).field("pwzFriendlyName", &self.pwzFriendlyName).field("pwzComment", &self.pwzComment).field("ulPresenceLifetime", &self.ulPresenceLifetime).field("cPresenceMax", &self.cPresenceMax).finish()
    }
}
impl ::windows::core::TypeKind for PEER_GRAPH_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_GRAPH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwScope == other.dwScope && self.dwMaxRecordSize == other.dwMaxRecordSize && self.pwzGraphId == other.pwzGraphId && self.pwzCreatorId == other.pwzCreatorId && self.pwzFriendlyName == other.pwzFriendlyName && self.pwzComment == other.pwzComment && self.ulPresenceLifetime == other.ulPresenceLifetime && self.cPresenceMax == other.cPresenceMax
    }
}
impl ::core::cmp::Eq for PEER_GRAPH_PROPERTIES {}
impl ::core::default::Default for PEER_GRAPH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_GROUP_EVENT_DATA {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub Anonymous: PEER_GROUP_EVENT_DATA_0,
}
impl ::core::marker::Copy for PEER_GROUP_EVENT_DATA {}
impl ::core::clone::Clone for PEER_GROUP_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_EVENT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PEER_GROUP_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub union PEER_GROUP_EVENT_DATA_0 {
    pub dwStatus: PEER_GROUP_STATUS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub memberChangeData: PEER_EVENT_MEMBER_CHANGE_DATA,
    pub hrConnectionFailedReason: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for PEER_GROUP_EVENT_DATA_0 {}
impl ::core::clone::Clone for PEER_GROUP_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_EVENT_DATA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PEER_GROUP_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_GROUP_EVENT_REGISTRATION {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub pType: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_GROUP_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_GROUP_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_GROUP_EVENT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GROUP_EVENT_REGISTRATION").field("eventType", &self.eventType).field("pType", &self.pType).finish()
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_EVENT_REGISTRATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_GROUP_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.eventType == other.eventType && self.pType == other.pType
    }
}
impl ::core::cmp::Eq for PEER_GROUP_EVENT_REGISTRATION {}
impl ::core::default::Default for PEER_GROUP_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_GROUP_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloud: ::windows::core::PWSTR,
    pub pwzClassifier: ::windows::core::PWSTR,
    pub pwzGroupPeerName: ::windows::core::PWSTR,
    pub pwzCreatorPeerName: ::windows::core::PWSTR,
    pub pwzFriendlyName: ::windows::core::PWSTR,
    pub pwzComment: ::windows::core::PWSTR,
    pub ulMemberDataLifetime: u32,
    pub ulPresenceLifetime: u32,
    pub dwAuthenticationSchemes: u32,
    pub pwzGroupPassword: ::windows::core::PWSTR,
    pub groupPasswordRole: ::windows::core::GUID,
}
impl ::core::marker::Copy for PEER_GROUP_PROPERTIES {}
impl ::core::clone::Clone for PEER_GROUP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_GROUP_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GROUP_PROPERTIES")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("pwzCloud", &self.pwzCloud)
            .field("pwzClassifier", &self.pwzClassifier)
            .field("pwzGroupPeerName", &self.pwzGroupPeerName)
            .field("pwzCreatorPeerName", &self.pwzCreatorPeerName)
            .field("pwzFriendlyName", &self.pwzFriendlyName)
            .field("pwzComment", &self.pwzComment)
            .field("ulMemberDataLifetime", &self.ulMemberDataLifetime)
            .field("ulPresenceLifetime", &self.ulPresenceLifetime)
            .field("dwAuthenticationSchemes", &self.dwAuthenticationSchemes)
            .field("pwzGroupPassword", &self.pwzGroupPassword)
            .field("groupPasswordRole", &self.groupPasswordRole)
            .finish()
    }
}
impl ::windows::core::TypeKind for PEER_GROUP_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_GROUP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.pwzCloud == other.pwzCloud && self.pwzClassifier == other.pwzClassifier && self.pwzGroupPeerName == other.pwzGroupPeerName && self.pwzCreatorPeerName == other.pwzCreatorPeerName && self.pwzFriendlyName == other.pwzFriendlyName && self.pwzComment == other.pwzComment && self.ulMemberDataLifetime == other.ulMemberDataLifetime && self.ulPresenceLifetime == other.ulPresenceLifetime && self.dwAuthenticationSchemes == other.dwAuthenticationSchemes && self.pwzGroupPassword == other.pwzGroupPassword && self.groupPasswordRole == other.groupPasswordRole
    }
}
impl ::core::cmp::Eq for PEER_GROUP_PROPERTIES {}
impl ::core::default::Default for PEER_GROUP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_INVITATION {
    pub applicationId: ::windows::core::GUID,
    pub applicationData: PEER_DATA,
    pub pwzMessage: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PEER_INVITATION {}
impl ::core::clone::Clone for PEER_INVITATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_INVITATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_INVITATION").field("applicationId", &self.applicationId).field("applicationData", &self.applicationData).field("pwzMessage", &self.pwzMessage).finish()
    }
}
impl ::windows::core::TypeKind for PEER_INVITATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_INVITATION {
    fn eq(&self, other: &Self) -> bool {
        self.applicationId == other.applicationId && self.applicationData == other.applicationData && self.pwzMessage == other.pwzMessage
    }
}
impl ::core::cmp::Eq for PEER_INVITATION {}
impl ::core::default::Default for PEER_INVITATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct PEER_INVITATION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloudName: ::windows::core::PWSTR,
    pub dwScope: u32,
    pub dwCloudFlags: u32,
    pub pwzGroupPeerName: ::windows::core::PWSTR,
    pub pwzIssuerPeerName: ::windows::core::PWSTR,
    pub pwzSubjectPeerName: ::windows::core::PWSTR,
    pub pwzGroupFriendlyName: ::windows::core::PWSTR,
    pub pwzIssuerFriendlyName: ::windows::core::PWSTR,
    pub pwzSubjectFriendlyName: ::windows::core::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut ::windows::core::GUID,
    pub cClassifiers: u32,
    pub ppwzClassifiers: *mut ::windows::core::PWSTR,
    pub pSubjectPublicKey: *mut super::super::Security::Cryptography::CERT_PUBLIC_KEY_INFO,
    pub authScheme: PEER_GROUP_AUTHENTICATION_SCHEME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PEER_INVITATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PEER_INVITATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for PEER_INVITATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_INVITATION_INFO")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("pwzCloudName", &self.pwzCloudName)
            .field("dwScope", &self.dwScope)
            .field("dwCloudFlags", &self.dwCloudFlags)
            .field("pwzGroupPeerName", &self.pwzGroupPeerName)
            .field("pwzIssuerPeerName", &self.pwzIssuerPeerName)
            .field("pwzSubjectPeerName", &self.pwzSubjectPeerName)
            .field("pwzGroupFriendlyName", &self.pwzGroupFriendlyName)
            .field("pwzIssuerFriendlyName", &self.pwzIssuerFriendlyName)
            .field("pwzSubjectFriendlyName", &self.pwzSubjectFriendlyName)
            .field("ftValidityStart", &self.ftValidityStart)
            .field("ftValidityEnd", &self.ftValidityEnd)
            .field("cRoles", &self.cRoles)
            .field("pRoles", &self.pRoles)
            .field("cClassifiers", &self.cClassifiers)
            .field("ppwzClassifiers", &self.ppwzClassifiers)
            .field("pSubjectPublicKey", &self.pSubjectPublicKey)
            .field("authScheme", &self.authScheme)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for PEER_INVITATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_INVITATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.pwzCloudName == other.pwzCloudName
            && self.dwScope == other.dwScope
            && self.dwCloudFlags == other.dwCloudFlags
            && self.pwzGroupPeerName == other.pwzGroupPeerName
            && self.pwzIssuerPeerName == other.pwzIssuerPeerName
            && self.pwzSubjectPeerName == other.pwzSubjectPeerName
            && self.pwzGroupFriendlyName == other.pwzGroupFriendlyName
            && self.pwzIssuerFriendlyName == other.pwzIssuerFriendlyName
            && self.pwzSubjectFriendlyName == other.pwzSubjectFriendlyName
            && self.ftValidityStart == other.ftValidityStart
            && self.ftValidityEnd == other.ftValidityEnd
            && self.cRoles == other.cRoles
            && self.pRoles == other.pRoles
            && self.cClassifiers == other.cClassifiers
            && self.ppwzClassifiers == other.ppwzClassifiers
            && self.pSubjectPublicKey == other.pSubjectPublicKey
            && self.authScheme == other.authScheme
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_INVITATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_INVITATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_INVITATION_RESPONSE {
    pub action: PEER_INVITATION_RESPONSE_TYPE,
    pub pwzMessage: ::windows::core::PWSTR,
    pub hrExtendedInfo: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for PEER_INVITATION_RESPONSE {}
impl ::core::clone::Clone for PEER_INVITATION_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_INVITATION_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_INVITATION_RESPONSE").field("action", &self.action).field("pwzMessage", &self.pwzMessage).field("hrExtendedInfo", &self.hrExtendedInfo).finish()
    }
}
impl ::windows::core::TypeKind for PEER_INVITATION_RESPONSE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_INVITATION_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.pwzMessage == other.pwzMessage && self.hrExtendedInfo == other.hrExtendedInfo
    }
}
impl ::core::cmp::Eq for PEER_INVITATION_RESPONSE {}
impl ::core::default::Default for PEER_INVITATION_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
pub struct PEER_MEMBER {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzIdentity: ::windows::core::PWSTR,
    pub pwzAttributes: ::windows::core::PWSTR,
    pub ullNodeId: u64,
    pub cAddresses: u32,
    pub pAddresses: *mut PEER_ADDRESS,
    pub pCredentialInfo: *mut PEER_CREDENTIAL_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PEER_MEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PEER_MEMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for PEER_MEMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_MEMBER").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("pwzIdentity", &self.pwzIdentity).field("pwzAttributes", &self.pwzAttributes).field("ullNodeId", &self.ullNodeId).field("cAddresses", &self.cAddresses).field("pAddresses", &self.pAddresses).field("pCredentialInfo", &self.pCredentialInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::windows::core::TypeKind for PEER_MEMBER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_MEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.pwzIdentity == other.pwzIdentity && self.pwzAttributes == other.pwzAttributes && self.ullNodeId == other.ullNodeId && self.cAddresses == other.cAddresses && self.pAddresses == other.pAddresses && self.pCredentialInfo == other.pCredentialInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_MEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_MEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_NAME_PAIR {
    pub dwSize: u32,
    pub pwzPeerName: ::windows::core::PWSTR,
    pub pwzFriendlyName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PEER_NAME_PAIR {}
impl ::core::clone::Clone for PEER_NAME_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_NAME_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_NAME_PAIR").field("dwSize", &self.dwSize).field("pwzPeerName", &self.pwzPeerName).field("pwzFriendlyName", &self.pwzFriendlyName).finish()
    }
}
impl ::windows::core::TypeKind for PEER_NAME_PAIR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_NAME_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pwzPeerName == other.pwzPeerName && self.pwzFriendlyName == other.pwzFriendlyName
    }
}
impl ::core::cmp::Eq for PEER_NAME_PAIR {}
impl ::core::default::Default for PEER_NAME_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_NODE_INFO {
    pub dwSize: u32,
    pub ullNodeId: u64,
    pub pwzPeerId: ::windows::core::PWSTR,
    pub cAddresses: u32,
    pub pAddresses: *mut PEER_ADDRESS,
    pub pwzAttributes: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_NODE_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_NODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_NODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_NODE_INFO").field("dwSize", &self.dwSize).field("ullNodeId", &self.ullNodeId).field("pwzPeerId", &self.pwzPeerId).field("cAddresses", &self.cAddresses).field("pAddresses", &self.pAddresses).field("pwzAttributes", &self.pwzAttributes).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_NODE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_NODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ullNodeId == other.ullNodeId && self.pwzPeerId == other.pwzPeerId && self.cAddresses == other.cAddresses && self.pAddresses == other.pAddresses && self.pwzAttributes == other.pwzAttributes
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_NODE_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_NODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_OBJECT {
    pub id: ::windows::core::GUID,
    pub data: PEER_DATA,
    pub dwPublicationScope: u32,
}
impl ::core::marker::Copy for PEER_OBJECT {}
impl ::core::clone::Clone for PEER_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_OBJECT").field("id", &self.id).field("data", &self.data).field("dwPublicationScope", &self.dwPublicationScope).finish()
    }
}
impl ::windows::core::TypeKind for PEER_OBJECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data && self.dwPublicationScope == other.dwPublicationScope
    }
}
impl ::core::cmp::Eq for PEER_OBJECT {}
impl ::core::default::Default for PEER_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_PEOPLE_NEAR_ME {
    pub pwzNickName: ::windows::core::PWSTR,
    pub endpoint: PEER_ENDPOINT,
    pub id: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_PEOPLE_NEAR_ME {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_PEOPLE_NEAR_ME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_PEOPLE_NEAR_ME {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_PEOPLE_NEAR_ME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_PNRP_CLOUD_INFO {
    pub pwzCloudName: ::windows::core::PWSTR,
    pub dwScope: PNRP_SCOPE,
    pub dwScopeId: u32,
}
impl ::core::marker::Copy for PEER_PNRP_CLOUD_INFO {}
impl ::core::clone::Clone for PEER_PNRP_CLOUD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_PNRP_CLOUD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PNRP_CLOUD_INFO").field("pwzCloudName", &self.pwzCloudName).field("dwScope", &self.dwScope).field("dwScopeId", &self.dwScopeId).finish()
    }
}
impl ::windows::core::TypeKind for PEER_PNRP_CLOUD_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_PNRP_CLOUD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwzCloudName == other.pwzCloudName && self.dwScope == other.dwScope && self.dwScopeId == other.dwScopeId
    }
}
impl ::core::cmp::Eq for PEER_PNRP_CLOUD_INFO {}
impl ::core::default::Default for PEER_PNRP_CLOUD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_PNRP_ENDPOINT_INFO {
    pub pwzPeerName: ::windows::core::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub pwzComment: ::windows::core::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_PNRP_ENDPOINT_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_PNRP_ENDPOINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_PNRP_ENDPOINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PNRP_ENDPOINT_INFO").field("pwzPeerName", &self.pwzPeerName).field("cAddresses", &self.cAddresses).field("ppAddresses", &self.ppAddresses).field("pwzComment", &self.pwzComment).field("payload", &self.payload).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_PNRP_ENDPOINT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_PNRP_ENDPOINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwzPeerName == other.pwzPeerName && self.cAddresses == other.cAddresses && self.ppAddresses == other.ppAddresses && self.pwzComment == other.pwzComment && self.payload == other.payload
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_PNRP_ENDPOINT_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_PNRP_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PEER_PNRP_REGISTRATION_INFO {
    pub pwzCloudName: ::windows::core::PWSTR,
    pub pwzPublishingIdentity: ::windows::core::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub wPort: u16,
    pub pwzComment: ::windows::core::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PEER_PNRP_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PEER_PNRP_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_PNRP_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PNRP_REGISTRATION_INFO").field("pwzCloudName", &self.pwzCloudName).field("pwzPublishingIdentity", &self.pwzPublishingIdentity).field("cAddresses", &self.cAddresses).field("ppAddresses", &self.ppAddresses).field("wPort", &self.wPort).field("pwzComment", &self.pwzComment).field("payload", &self.payload).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PEER_PNRP_REGISTRATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_PNRP_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwzCloudName == other.pwzCloudName && self.pwzPublishingIdentity == other.pwzPublishingIdentity && self.cAddresses == other.cAddresses && self.ppAddresses == other.ppAddresses && self.wPort == other.wPort && self.pwzComment == other.pwzComment && self.payload == other.payload
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_PNRP_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_PNRP_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_PRESENCE_INFO {
    pub status: PEER_PRESENCE_STATUS,
    pub pwzDescriptiveText: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PEER_PRESENCE_INFO {}
impl ::core::clone::Clone for PEER_PRESENCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_PRESENCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PRESENCE_INFO").field("status", &self.status).field("pwzDescriptiveText", &self.pwzDescriptiveText).finish()
    }
}
impl ::windows::core::TypeKind for PEER_PRESENCE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_PRESENCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.pwzDescriptiveText == other.pwzDescriptiveText
    }
}
impl ::core::cmp::Eq for PEER_PRESENCE_INFO {}
impl ::core::default::Default for PEER_PRESENCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_RECORD {
    pub dwSize: u32,
    pub r#type: ::windows::core::GUID,
    pub id: ::windows::core::GUID,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pwzCreatorId: ::windows::core::PWSTR,
    pub pwzModifiedById: ::windows::core::PWSTR,
    pub pwzAttributes: ::windows::core::PWSTR,
    pub ftCreation: super::super::Foundation::FILETIME,
    pub ftExpiration: super::super::Foundation::FILETIME,
    pub ftLastModified: super::super::Foundation::FILETIME,
    pub securityData: PEER_DATA,
    pub data: PEER_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEER_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_RECORD")
            .field("dwSize", &self.dwSize)
            .field("type", &self.r#type)
            .field("id", &self.id)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("pwzCreatorId", &self.pwzCreatorId)
            .field("pwzModifiedById", &self.pwzModifiedById)
            .field("pwzAttributes", &self.pwzAttributes)
            .field("ftCreation", &self.ftCreation)
            .field("ftExpiration", &self.ftExpiration)
            .field("ftLastModified", &self.ftLastModified)
            .field("securityData", &self.securityData)
            .field("data", &self.data)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PEER_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.r#type == other.r#type && self.id == other.id && self.dwVersion == other.dwVersion && self.dwFlags == other.dwFlags && self.pwzCreatorId == other.pwzCreatorId && self.pwzModifiedById == other.pwzModifiedById && self.pwzAttributes == other.pwzAttributes && self.ftCreation == other.ftCreation && self.ftExpiration == other.ftExpiration && self.ftLastModified == other.ftLastModified && self.securityData == other.securityData && self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_SECURITY_INTERFACE {
    pub dwSize: u32,
    pub pwzSspFilename: ::windows::core::PWSTR,
    pub pwzPackageName: ::windows::core::PWSTR,
    pub cbSecurityInfo: u32,
    pub pbSecurityInfo: *mut u8,
    pub pvContext: *mut ::core::ffi::c_void,
    pub pfnValidateRecord: PFNPEER_VALIDATE_RECORD,
    pub pfnSecureRecord: PFNPEER_SECURE_RECORD,
    pub pfnFreeSecurityData: PFNPEER_FREE_SECURITY_DATA,
    pub pfnAuthFailed: PFNPEER_ON_PASSWORD_AUTH_FAILED,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_SECURITY_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_SECURITY_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEER_SECURITY_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_SECURITY_INTERFACE").field("dwSize", &self.dwSize).field("pwzSspFilename", &self.pwzSspFilename).field("pwzPackageName", &self.pwzPackageName).field("cbSecurityInfo", &self.cbSecurityInfo).field("pbSecurityInfo", &self.pbSecurityInfo).field("pvContext", &self.pvContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PEER_SECURITY_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_SECURITY_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PEER_VERSION_DATA {
    pub wVersion: u16,
    pub wHighestVersion: u16,
}
impl ::core::marker::Copy for PEER_VERSION_DATA {}
impl ::core::clone::Clone for PEER_VERSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PEER_VERSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_VERSION_DATA").field("wVersion", &self.wVersion).field("wHighestVersion", &self.wHighestVersion).finish()
    }
}
impl ::windows::core::TypeKind for PEER_VERSION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PEER_VERSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighestVersion == other.wHighestVersion
    }
}
impl ::core::cmp::Eq for PEER_VERSION_DATA {}
impl ::core::default::Default for PEER_VERSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PNRPCLOUDINFO {
    pub dwSize: u32,
    pub Cloud: PNRP_CLOUD_ID,
    pub enCloudState: PNRP_CLOUD_STATE,
    pub enCloudFlags: PNRP_CLOUD_FLAGS,
}
impl ::core::marker::Copy for PNRPCLOUDINFO {}
impl ::core::clone::Clone for PNRPCLOUDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PNRPCLOUDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PNRPCLOUDINFO").field("dwSize", &self.dwSize).field("Cloud", &self.Cloud).field("enCloudState", &self.enCloudState).field("enCloudFlags", &self.enCloudFlags).finish()
    }
}
impl ::windows::core::TypeKind for PNRPCLOUDINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PNRPCLOUDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.Cloud == other.Cloud && self.enCloudState == other.enCloudState && self.enCloudFlags == other.enCloudFlags
    }
}
impl ::core::cmp::Eq for PNRPCLOUDINFO {}
impl ::core::default::Default for PNRPCLOUDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct PNRPINFO_V1 {
    pub dwSize: u32,
    pub lpwszIdentity: ::windows::core::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for PNRPINFO_V1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for PNRPINFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PNRPINFO_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PNRPINFO_V1").field("dwSize", &self.dwSize).field("lpwszIdentity", &self.lpwszIdentity).field("nMaxResolve", &self.nMaxResolve).field("dwTimeout", &self.dwTimeout).field("dwLifetime", &self.dwLifetime).field("enResolveCriteria", &self.enResolveCriteria).field("dwFlags", &self.dwFlags).field("saHint", &self.saHint).field("enNameState", &self.enNameState).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for PNRPINFO_V1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PNRPINFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpwszIdentity == other.lpwszIdentity && self.nMaxResolve == other.nMaxResolve && self.dwTimeout == other.dwTimeout && self.dwLifetime == other.dwLifetime && self.enResolveCriteria == other.enResolveCriteria && self.dwFlags == other.dwFlags && self.saHint == other.saHint && self.enNameState == other.enNameState
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PNRPINFO_V1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PNRPINFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
pub struct PNRPINFO_V2 {
    pub dwSize: u32,
    pub lpwszIdentity: ::windows::core::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
    pub enExtendedPayloadType: PNRP_EXTENDED_PAYLOAD_TYPE,
    pub Anonymous: PNRPINFO_V2_0,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PNRPINFO_V2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PNRPINFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::windows::core::TypeKind for PNRPINFO_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::default::Default for PNRPINFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
pub union PNRPINFO_V2_0 {
    pub blobPayload: super::super::System::Com::BLOB,
    pub pwszPayload: ::windows::core::PWSTR,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PNRPINFO_V2_0 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PNRPINFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::windows::core::TypeKind for PNRPINFO_V2_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::default::Default for PNRPINFO_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub struct PNRP_CLOUD_ID {
    pub AddressFamily: i32,
    pub Scope: PNRP_SCOPE,
    pub ScopeId: u32,
}
impl ::core::marker::Copy for PNRP_CLOUD_ID {}
impl ::core::clone::Clone for PNRP_CLOUD_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PNRP_CLOUD_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PNRP_CLOUD_ID").field("AddressFamily", &self.AddressFamily).field("Scope", &self.Scope).field("ScopeId", &self.ScopeId).finish()
    }
}
impl ::windows::core::TypeKind for PNRP_CLOUD_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PNRP_CLOUD_ID {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Scope == other.Scope && self.ScopeId == other.ScopeId
    }
}
impl ::core::cmp::Eq for PNRP_CLOUD_ID {}
impl ::core::default::Default for PNRP_CLOUD_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type DRT_BOOTSTRAP_RESOLVE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hr: ::windows::core::HRESULT, pvcontext: *mut ::core::ffi::c_void, paddresses: *mut super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, ffatalerror: super::super::Foundation::BOOL) -> ()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub type PFNPEER_FREE_SECURITY_DATA = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, psecuritydata: *const PEER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`*"]
pub type PFNPEER_ON_PASSWORD_AUTH_FAILED = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNPEER_SECURE_RECORD = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE, ppsecuritydata: *mut *mut PEER_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_NetworkManagement_P2P\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNPEER_VALIDATE_RECORD = ::core::option::Option<unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
