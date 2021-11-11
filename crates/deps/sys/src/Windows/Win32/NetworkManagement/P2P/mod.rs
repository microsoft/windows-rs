#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtClose(hdrt: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtContinueSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn DrtCreateDerivedKey(plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT, pkey: *mut DRT_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn DrtCreateDerivedKeySecurityProvider(prootcert: *const super::super::Security::Cryptography::CERT_CONTEXT, plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT, ppsecurityprovider: *mut *mut DRT_SECURITY_PROVIDER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtCreateDnsBootstrapResolver(port: u16, pwszaddress: super::super::Foundation::PWSTR, ppmodule: *mut *mut DRT_BOOTSTRAP_PROVIDER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtCreateIpv6UdpTransport(scope: DRT_SCOPE, dwscopeid: u32, dwlocalitythreshold: u32, pwport: *mut u16, phtransport: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtCreateNullSecurityProvider(ppsecurityprovider: *mut *mut DRT_SECURITY_PROVIDER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtCreatePnrpBootstrapResolver(fpublish: super::super::Foundation::BOOL, pwzpeername: super::super::Foundation::PWSTR, pwzcloudname: super::super::Foundation::PWSTR, pwzpublishingidentity: super::super::Foundation::PWSTR, ppresolver: *mut *mut DRT_BOOTSTRAP_PROVIDER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteDnsBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteIpv6UdpTransport(htransport: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeleteNullSecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtDeletePnrpBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtEndSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DrtGetEventData(hdrt: *const ::core::ffi::c_void, uleventdatalen: u32, peventdata: *mut DRT_EVENT_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetEventDataSize(hdrt: *const ::core::ffi::c_void, puleventdatalen: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtGetInstanceName(hdrt: *const ::core::ffi::c_void, ulcbinstancenamesize: u32, pwzdrtinstancename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetInstanceNameSize(hdrt: *const ::core::ffi::c_void, pulcbinstancenamesize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DrtGetSearchPath(hsearchcontext: *const ::core::ffi::c_void, ulsearchpathsize: u32, psearchpath: *mut DRT_ADDRESS_LIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetSearchPathSize(hsearchcontext: *const ::core::ffi::c_void, pulsearchpathsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetSearchResult(hsearchcontext: *const ::core::ffi::c_void, ulsearchresultsize: u32, psearchresult: *mut DRT_SEARCH_RESULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtGetSearchResultSize(hsearchcontext: *const ::core::ffi::c_void, pulsearchresultsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtOpen(psettings: *const DRT_SETTINGS, hevent: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, phdrt: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtRegisterKey(hdrt: *const ::core::ffi::c_void, pregistration: *const DRT_REGISTRATION, pvkeycontext: *const ::core::ffi::c_void, phkeyregistration: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtStartSearch(hdrt: *const ::core::ffi::c_void, pkey: *const DRT_DATA, pinfo: *const DRT_SEARCH_INFO, timeout: u32, hevent: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, hsearchcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtUnregisterKey(hkeyregistration: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn DrtUpdateKey(hkeyregistration: *const ::core::ffi::c_void, pappdata: *const DRT_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabAddContact(pwzcontactdata: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabAsyncInviteContact(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: super::super::Foundation::HANDLE, phinvitation: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabAsyncInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: super::super::Foundation::HANDLE, phinvitation: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabCancelInvitation(hinvitation: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabCloseHandle(hinvitation: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabDeleteContact(pwzpeername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabDeleteEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabDeleteObject(pobjectid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabEnumApplicationRegistrationInfo(registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabEnumApplications(pcendpoint: *const PEER_ENDPOINT, papplicationid: *const ::windows::runtime::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabEnumContacts(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabEnumEndpoints(pccontact: *const PEER_CONTACT, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabEnumObjects(pcendpoint: *const PEER_ENDPOINT, pobjectid: *const ::windows::runtime::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabEnumPeopleNearMe(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabExportContact(pwzpeername: super::super::Foundation::PWSTR, ppwzcontactdata: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetAppLaunchInfo(pplaunchinfo: *mut *mut PEER_APP_LAUNCH_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetApplicationRegistrationInfo(papplicationid: *const ::windows::runtime::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, ppapplication: *mut *mut PEER_APPLICATION_REGISTRATION_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetContact(pwzpeername: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetEndpointName(ppwzendpointname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_COLLAB_EVENT_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetInvitationResponse(hinvitation: super::super::Foundation::HANDLE, ppinvitationresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetPresenceInfo(pcendpoint: *const PEER_ENDPOINT, pppresenceinfo: *mut *mut PEER_PRESENCE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabGetSigninOptions(pdwsigninoptions: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabInviteContact(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, ppresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, ppresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabParseContact(pwzcontactdata: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabQueryContactData(pcendpoint: *const PEER_ENDPOINT, ppwzcontactdata: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabRefreshEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabRegisterApplication(pcapplication: *const PEER_APPLICATION_REGISTRATION_INFO, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabRegisterEvent(hevent: super::super::Foundation::HANDLE, ceventregistration: u32, peventregistrations: *const PEER_COLLAB_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSetEndpointName(pwzendpointname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabSetObject(pcobject: *const PEER_OBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSetPresenceInfo(pcpresenceinfo: *const PEER_PRESENCE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabShutdown() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSignin(hwndparent: super::super::Foundation::HWND, dwsigninoptions: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabSignout(dwsigninoptions: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabStartup(wversionrequested: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabSubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabUnregisterApplication(papplicationid: *const ::windows::runtime::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerCollabUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabUnsubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabUpdateContact(pcontact: *const PEER_CONTACT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCreatePeerName(pwzidentity: super::super::Foundation::PWSTR, pwzclassifier: super::super::Foundation::PWSTR, ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientAddContentInformation(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientAddData(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientBlockRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientCancelAsyncOperation(hpeerdist: isize, hcontenthandle: isize, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistClientCloseContent(hpeerdist: isize, hcontenthandle: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientCompleteContentInformation(hpeerdist: isize, hcontenthandle: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientFlushContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistClientGetInformationByHandle(hpeerdist: isize, hcontenthandle: isize, peerdistclientinfoclass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize: u32, lpinformation: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistClientOpenContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontenthandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientStreamRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistGetOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistGetStatus(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistGetStatusEx(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistRegisterForStatusChangeNotification(hpeerdist: isize, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistRegisterForStatusChangeNotificationEx(hpeerdist: isize, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerCancelAsyncOperation(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistServerCloseContentInformation(hpeerdist: isize, hcontentinfo: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistServerCloseStreamHandle(hpeerdist: isize, hstream: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerOpenContentInformation(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerOpenContentInformationEx(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, pretrievaloptions: *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerPublishAddToStream(hpeerdist: isize, hstream: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerPublishCompleteStream(hpeerdist: isize, hstream: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerPublishStream(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, cbcontentlength: u64, ppublishoptions: *const PEERDIST_PUBLICATION_OPTIONS, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phstream: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerRetrieveContentInformation(hpeerdist: isize, hcontentinfo: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistServerUnpublish(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistShutdown(hpeerdist: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistStartup(dwversionrequested: u32, phpeerdist: *mut isize, pdwsupportedversion: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerDistUnregisterForStatusChangeNotification(hpeerdist: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerEnumGroups(pwzidentity: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerEnumIdentities(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerFreeData(pvdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGetItemCount(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphAddRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD, precordid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphClose(hgraph: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphCloseDirectConnection(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphConnect(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphCreate(pgraphproperties: *const PEER_GRAPH_PROPERTIES, pwzdatabasename: super::super::Foundation::PWSTR, psecurityinterface: *const ::core::mem::ManuallyDrop<PEER_SECURITY_INTERFACE>, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphDelete(pwzgraphid: super::super::Foundation::PWSTR, pwzpeerid: super::super::Foundation::PWSTR, pwzdatabasename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphDeleteRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::runtime::GUID, flocal: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphEnumConnections(hgraph: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphEnumNodes(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphEnumRecords(hgraph: *const ::core::ffi::c_void, precordtype: *const ::windows::runtime::GUID, pwzpeerid: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphExportDatabase(hgraph: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphFreeData(pvdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_GRAPH_EVENT_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphGetItemCount(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphGetNodeInfo(hgraph: *const ::core::ffi::c_void, ullnodeid: u64, ppnodeinfo: *mut *mut PEER_NODE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetProperties(hgraph: *const ::core::ffi::c_void, ppgraphproperties: *mut *mut PEER_GRAPH_PROPERTIES) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows::runtime::GUID, pprecord: *mut *mut PEER_RECORD) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphGetStatus(hgraph: *const ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphImportDatabase(hgraph: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphListen(hgraph: *const ::core::ffi::c_void, dwscope: u32, dwscopeid: u32, wport: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphOpen(pwzgraphid: super::super::Foundation::PWSTR, pwzpeerid: super::super::Foundation::PWSTR, pwzdatabasename: super::super::Foundation::PWSTR, psecurityinterface: *const ::core::mem::ManuallyDrop<PEER_SECURITY_INTERFACE>, crecordtypesyncprecedence: u32, precordtypesyncprecedence: *const ::windows::runtime::GUID, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphOpenDirectConnection(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphPeerTimeToUniversalTime(hgraph: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME, pftuniversaltime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphRegisterEvent(hgraph: *const ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ceventregistrations: u32, peventregistrations: *const PEER_GRAPH_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSearchRecords(hgraph: *const ::core::ffi::c_void, pwzcriteria: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphSendData(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::runtime::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetNodeAttributes(hgraph: *const ::core::ffi::c_void, pwzattributes: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetPresence(hgraph: *const ::core::ffi::c_void, fpresent: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetProperties(hgraph: *const ::core::ffi::c_void, pgraphproperties: *const PEER_GRAPH_PROPERTIES) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphShutdown() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphStartup(wversionrequested: u16, pversiondata: *mut PEER_VERSION_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphUniversalTimeToPeerTime(hgraph: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME, pftpeertime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphUpdateRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGraphValidateDeferredRecords(hgraph: *const ::core::ffi::c_void, crecordids: u32, precordids: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupAddRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD, precordid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupClose(hgroup: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupCloseDirectConnection(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupConnect(hgroup: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn PeerGroupConnectByAddress(hgroup: *const ::core::ffi::c_void, caddresses: u32, paddresses: *const PEER_ADDRESS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreate(pproperties: *const PEER_GROUP_PROPERTIES, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreateInvitation(hgroup: *const ::core::ffi::c_void, pwzidentityinfo: super::super::Foundation::PWSTR, pftexpiration: *const super::super::Foundation::FILETIME, croles: u32, proles: *const ::windows::runtime::GUID, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreatePasswordInvitation(hgroup: *const ::core::ffi::c_void, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupDelete(pwzidentity: super::super::Foundation::PWSTR, pwzgrouppeername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupDeleteRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupEnumConnections(hgroup: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupEnumMembers(hgroup: *const ::core::ffi::c_void, dwflags: u32, pwzidentity: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupEnumRecords(hgroup: *const ::core::ffi::c_void, precordtype: *const ::windows::runtime::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupExportConfig(hgroup: *const ::core::ffi::c_void, pwzpassword: super::super::Foundation::PWSTR, ppwzxml: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupExportDatabase(hgroup: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_GROUP_EVENT_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetProperties(hgroup: *const ::core::ffi::c_void, ppproperties: *mut *mut PEER_GROUP_PROPERTIES) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows::runtime::GUID, pprecord: *mut *mut PEER_RECORD) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupGetStatus(hgroup: *const ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupImportConfig(pwzxml: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, foverwrite: super::super::Foundation::BOOL, ppwzidentity: *mut super::super::Foundation::PWSTR, ppwzgroup: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupImportDatabase(hgroup: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn PeerGroupIssueCredentials(hgroup: *const ::core::ffi::c_void, pwzsubjectidentity: super::super::Foundation::PWSTR, pcredentialinfo: *const PEER_CREDENTIAL_INFO, dwflags: u32, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupJoin(pwzidentity: super::super::Foundation::PWSTR, pwzinvitation: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupOpen(pwzidentity: super::super::Foundation::PWSTR, pwzgrouppeername: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGroupOpenDirectConnection(hgroup: *const ::core::ffi::c_void, pwzidentity: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn PeerGroupParseInvitation(pwzinvitation: super::super::Foundation::PWSTR, ppinvitationinfo: *mut *mut PEER_INVITATION_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupPasswordJoin(pwzidentity: super::super::Foundation::PWSTR, pwzinvitation: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupPeerTimeToUniversalTime(hgroup: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME, pftuniversaltime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupRegisterEvent(hgroup: *const ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ceventregistration: u32, peventregistrations: *const PEER_GROUP_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupResumePasswordAuthentication(hgroup: *const ::core::ffi::c_void, hpeereventhandle: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupSearchRecords(hgroup: *const ::core::ffi::c_void, pwzcriteria: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupSendData(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows::runtime::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupSetProperties(hgroup: *const ::core::ffi::c_void, pproperties: *const PEER_GROUP_PROPERTIES) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupShutdown() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupStartup(wversionrequested: u16, pversiondata: *mut PEER_VERSION_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupUniversalTimeToPeerTime(hgroup: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME, pftpeertime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerGroupUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupUpdateRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerHostNameToPeerName(pwzhostname: super::super::Foundation::PWSTR, ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityCreate(pwzclassifier: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, hcryptprov: usize, ppwzidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityDelete(pwzidentity: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityExport(pwzidentity: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, ppwzexportxml: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetCryptKey(pwzidentity: super::super::Foundation::PWSTR, phcryptprov: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetDefault(ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetFriendlyName(pwzidentity: super::super::Foundation::PWSTR, ppwzfriendlyname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetXML(pwzidentity: super::super::Foundation::PWSTR, ppwzidentityxml: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityImport(pwzimportxml: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, ppwzidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentitySetFriendlyName(pwzidentity: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerNameToPeerHostName(pwzpeername: super::super::Foundation::PWSTR, ppwzhostname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpEndResolve(hresolve: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerPnrpGetCloudInfo(pcnumclouds: *mut u32, ppcloudinfo: *mut *mut PEER_PNRP_CLOUD_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpGetEndpoint(hresolve: *const ::core::ffi::c_void, ppendpoint: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpRegister(pcwzpeername: super::super::Foundation::PWSTR, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO, phregistration: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpResolve(pcwzpeername: super::super::Foundation::PWSTR, pcwzcloudname: super::super::Foundation::PWSTR, pcendpoints: *mut u32, ppendpoints: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpShutdown() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerPnrpStartResolve(pcwzpeername: super::super::Foundation::PWSTR, pcwzcloudname: super::super::Foundation::PWSTR, cmaxendpoints: u32, hevent: super::super::Foundation::HANDLE, phresolve: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpStartup(wversionrequested: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`*"]
    pub fn PeerPnrpUnregister(hregistration: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_P2P`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpUpdateRegistration(hregistration: *const ::core::ffi::c_void, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO) -> ::windows::runtime::HRESULT;
}
