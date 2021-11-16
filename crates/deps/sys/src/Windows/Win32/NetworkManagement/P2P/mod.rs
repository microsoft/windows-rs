#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn DrtClose(hdrt: *const ::core::ffi::c_void);
    pub fn DrtContinueSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn DrtCreateDerivedKey(plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT, pkey: *mut DRT_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn DrtCreateDerivedKeySecurityProvider(prootcert: *const super::super::Security::Cryptography::CERT_CONTEXT, plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT, ppsecurityprovider: *mut *mut DRT_SECURITY_PROVIDER) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtCreateDnsBootstrapResolver(port: u16, pwszaddress: super::super::Foundation::PWSTR, ppmodule: *mut *mut DRT_BOOTSTRAP_PROVIDER) -> ::windows_sys::core::HRESULT;
    pub fn DrtCreateIpv6UdpTransport(scope: DRT_SCOPE, dwscopeid: u32, dwlocalitythreshold: u32, pwport: *mut u16, phtransport: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DrtCreateNullSecurityProvider(ppsecurityprovider: *mut *mut DRT_SECURITY_PROVIDER) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtCreatePnrpBootstrapResolver(fpublish: super::super::Foundation::BOOL, pwzpeername: super::super::Foundation::PWSTR, pwzcloudname: super::super::Foundation::PWSTR, pwzpublishingidentity: super::super::Foundation::PWSTR, ppresolver: *mut *mut DRT_BOOTSTRAP_PROVIDER) -> ::windows_sys::core::HRESULT;
    pub fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER);
    pub fn DrtDeleteDnsBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER);
    pub fn DrtDeleteIpv6UdpTransport(htransport: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DrtDeleteNullSecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER);
    pub fn DrtDeletePnrpBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER);
    pub fn DrtEndSearch(hsearchcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DrtGetEventData(hdrt: *const ::core::ffi::c_void, uleventdatalen: u32, peventdata: *mut DRT_EVENT_DATA) -> ::windows_sys::core::HRESULT;
    pub fn DrtGetEventDataSize(hdrt: *const ::core::ffi::c_void, puleventdatalen: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtGetInstanceName(hdrt: *const ::core::ffi::c_void, ulcbinstancenamesize: u32, pwzdrtinstancename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn DrtGetInstanceNameSize(hdrt: *const ::core::ffi::c_void, pulcbinstancenamesize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DrtGetSearchPath(hsearchcontext: *const ::core::ffi::c_void, ulsearchpathsize: u32, psearchpath: *mut DRT_ADDRESS_LIST) -> ::windows_sys::core::HRESULT;
    pub fn DrtGetSearchPathSize(hsearchcontext: *const ::core::ffi::c_void, pulsearchpathsize: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn DrtGetSearchResult(hsearchcontext: *const ::core::ffi::c_void, ulsearchresultsize: u32, psearchresult: *mut DRT_SEARCH_RESULT) -> ::windows_sys::core::HRESULT;
    pub fn DrtGetSearchResultSize(hsearchcontext: *const ::core::ffi::c_void, pulsearchresultsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtOpen(psettings: *const DRT_SETTINGS, hevent: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, phdrt: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DrtRegisterKey(hdrt: *const ::core::ffi::c_void, pregistration: *const DRT_REGISTRATION, pvkeycontext: *const ::core::ffi::c_void, phkeyregistration: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrtStartSearch(hdrt: *const ::core::ffi::c_void, pkey: *const DRT_DATA, pinfo: *const DRT_SEARCH_INFO, timeout: u32, hevent: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, hsearchcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DrtUnregisterKey(hkeyregistration: *const ::core::ffi::c_void);
    pub fn DrtUpdateKey(hkeyregistration: *const ::core::ffi::c_void, pappdata: *const DRT_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabAddContact(pwzcontactdata: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabAsyncInviteContact(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: super::super::Foundation::HANDLE, phinvitation: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabAsyncInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: super::super::Foundation::HANDLE, phinvitation: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabCancelInvitation(hinvitation: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabCloseHandle(hinvitation: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabDeleteContact(pwzpeername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabDeleteEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabDeleteObject(pobjectid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabEnumApplicationRegistrationInfo(registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabEnumApplications(pcendpoint: *const PEER_ENDPOINT, papplicationid: *const ::windows_sys::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabEnumContacts(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabEnumEndpoints(pccontact: *const PEER_CONTACT, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabEnumObjects(pcendpoint: *const PEER_ENDPOINT, pobjectid: *const ::windows_sys::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabEnumPeopleNearMe(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabExportContact(pwzpeername: super::super::Foundation::PWSTR, ppwzcontactdata: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetAppLaunchInfo(pplaunchinfo: *mut *mut PEER_APP_LAUNCH_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetApplicationRegistrationInfo(papplicationid: *const ::windows_sys::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, ppapplication: *mut *mut PEER_APPLICATION_REGISTRATION_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetContact(pwzpeername: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetEndpointName(ppwzendpointname: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_COLLAB_EVENT_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabGetInvitationResponse(hinvitation: super::super::Foundation::HANDLE, ppinvitationresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabGetPresenceInfo(pcendpoint: *const PEER_ENDPOINT, pppresenceinfo: *mut *mut PEER_PRESENCE_INFO) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabGetSigninOptions(pdwsigninoptions: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabInviteContact(pccontact: *const PEER_CONTACT, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, ppresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, ppresponse: *mut *mut PEER_INVITATION_RESPONSE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabParseContact(pwzcontactdata: super::super::Foundation::PWSTR, ppcontact: *mut *mut PEER_CONTACT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabQueryContactData(pcendpoint: *const PEER_ENDPOINT, ppwzcontactdata: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabRefreshEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabRegisterApplication(pcapplication: *const PEER_APPLICATION_REGISTRATION_INFO, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabRegisterEvent(hevent: super::super::Foundation::HANDLE, ceventregistration: u32, peventregistrations: *const PEER_COLLAB_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSetEndpointName(pwzendpointname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabSetObject(pcobject: *const PEER_OBJECT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSetPresenceInfo(pcpresenceinfo: *const PEER_PRESENCE_INFO) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabShutdown() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabSignin(hwndparent: super::super::Foundation::HWND, dwsigninoptions: u32) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabSignout(dwsigninoptions: u32) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabStartup(wversionrequested: u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabSubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabUnregisterApplication(papplicationid: *const ::windows_sys::core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> ::windows_sys::core::HRESULT;
    pub fn PeerCollabUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerCollabUnsubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCollabUpdateContact(pcontact: *const PEER_CONTACT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerCreatePeerName(pwzidentity: super::super::Foundation::PWSTR, pwzclassifier: super::super::Foundation::PWSTR, ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientAddContentInformation(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientAddData(hpeerdist: isize, hcontenthandle: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientBlockRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientCancelAsyncOperation(hpeerdist: isize, hcontenthandle: isize, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    pub fn PeerDistClientCloseContent(hpeerdist: isize, hcontenthandle: isize) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientCompleteContentInformation(hpeerdist: isize, hcontenthandle: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientFlushContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    pub fn PeerDistClientGetInformationByHandle(hpeerdist: isize, hcontenthandle: isize, peerdistclientinfoclass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize: u32, lpinformation: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistClientOpenContent(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontenthandle: *mut isize) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistClientStreamRead(hpeerdist: isize, hcontenthandle: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistGetOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    pub fn PeerDistGetStatus(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32;
    pub fn PeerDistGetStatusEx(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistRegisterForStatusChangeNotification(hpeerdist: isize, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistRegisterForStatusChangeNotificationEx(hpeerdist: isize, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerCancelAsyncOperation(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    pub fn PeerDistServerCloseContentInformation(hpeerdist: isize, hcontentinfo: isize) -> u32;
    pub fn PeerDistServerCloseStreamHandle(hpeerdist: isize, hstream: isize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerOpenContentInformation(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerOpenContentInformationEx(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, ullcontentoffset: u64, cbcontentlength: u64, pretrievaloptions: *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerPublishAddToStream(hpeerdist: isize, hstream: isize, cbnumberofbytes: u32, pbuffer: *const u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerPublishCompleteStream(hpeerdist: isize, hstream: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerDistServerPublishStream(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8, cbcontentlength: u64, ppublishoptions: *const PEERDIST_PUBLICATION_OPTIONS, hcompletionport: super::super::Foundation::HANDLE, ulcompletionkey: usize, phstream: *mut isize) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn PeerDistServerRetrieveContentInformation(hpeerdist: isize, hcontentinfo: isize, cbmaxnumberofbytes: u32, pbuffer: *mut u8, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    pub fn PeerDistServerUnpublish(hpeerdist: isize, cbcontentidentifier: u32, pcontentidentifier: *const u8) -> u32;
    pub fn PeerDistShutdown(hpeerdist: isize) -> u32;
    pub fn PeerDistStartup(dwversionrequested: u32, phpeerdist: *mut isize, pdwsupportedversion: *mut u32) -> u32;
    pub fn PeerDistUnregisterForStatusChangeNotification(hpeerdist: isize) -> u32;
    pub fn PeerEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerEnumGroups(pwzidentity: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerEnumIdentities(phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerFreeData(pvdata: *const ::core::ffi::c_void);
    pub fn PeerGetItemCount(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn PeerGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphAddRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD, precordid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphClose(hgraph: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphCloseDirectConnection(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphConnect(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphCreate(pgraphproperties: *const PEER_GRAPH_PROPERTIES, pwzdatabasename: super::super::Foundation::PWSTR, psecurityinterface: *const PEER_SECURITY_INTERFACE, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphDelete(pwzgraphid: super::super::Foundation::PWSTR, pwzpeerid: super::super::Foundation::PWSTR, pwzdatabasename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphDeleteRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows_sys::core::GUID, flocal: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphEndEnumeration(hpeerenum: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphEnumConnections(hgraph: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphEnumNodes(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphEnumRecords(hgraph: *const ::core::ffi::c_void, precordtype: *const ::windows_sys::core::GUID, pwzpeerid: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphExportDatabase(hgraph: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphFreeData(pvdata: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_GRAPH_EVENT_DATA) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphGetItemCount(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphGetNextItem(hpeerenum: *const ::core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphGetNodeInfo(hgraph: *const ::core::ffi::c_void, ullnodeid: u64, ppnodeinfo: *mut *mut PEER_NODE_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetProperties(hgraph: *const ::core::ffi::c_void, ppgraphproperties: *mut *mut PEER_GRAPH_PROPERTIES) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphGetRecord(hgraph: *const ::core::ffi::c_void, precordid: *const ::windows_sys::core::GUID, pprecord: *mut *mut PEER_RECORD) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphGetStatus(hgraph: *const ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphImportDatabase(hgraph: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphListen(hgraph: *const ::core::ffi::c_void, dwscope: u32, dwscopeid: u32, wport: u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphOpen(pwzgraphid: super::super::Foundation::PWSTR, pwzpeerid: super::super::Foundation::PWSTR, pwzdatabasename: super::super::Foundation::PWSTR, psecurityinterface: *const PEER_SECURITY_INTERFACE, crecordtypesyncprecedence: u32, precordtypesyncprecedence: *const ::windows_sys::core::GUID, phgraph: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGraphOpenDirectConnection(hgraph: *const ::core::ffi::c_void, pwzpeerid: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphPeerTimeToUniversalTime(hgraph: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME, pftuniversaltime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphRegisterEvent(hgraph: *const ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ceventregistrations: u32, peventregistrations: *const PEER_GRAPH_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSearchRecords(hgraph: *const ::core::ffi::c_void, pwzcriteria: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphSendData(hgraph: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows_sys::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetNodeAttributes(hgraph: *const ::core::ffi::c_void, pwzattributes: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetPresence(hgraph: *const ::core::ffi::c_void, fpresent: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphSetProperties(hgraph: *const ::core::ffi::c_void, pgraphproperties: *const PEER_GRAPH_PROPERTIES) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphShutdown() -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphStartup(wversionrequested: u16, pversiondata: *mut PEER_VERSION_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphUniversalTimeToPeerTime(hgraph: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME, pftpeertime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGraphUpdateRecord(hgraph: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows_sys::core::HRESULT;
    pub fn PeerGraphValidateDeferredRecords(hgraph: *const ::core::ffi::c_void, crecordids: u32, precordids: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupAddRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD, precordid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupClose(hgroup: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupCloseDirectConnection(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupConnect(hgroup: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn PeerGroupConnectByAddress(hgroup: *const ::core::ffi::c_void, caddresses: u32, paddresses: *const PEER_ADDRESS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreate(pproperties: *const PEER_GROUP_PROPERTIES, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreateInvitation(hgroup: *const ::core::ffi::c_void, pwzidentityinfo: super::super::Foundation::PWSTR, pftexpiration: *const super::super::Foundation::FILETIME, croles: u32, proles: *const ::windows_sys::core::GUID, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupCreatePasswordInvitation(hgroup: *const ::core::ffi::c_void, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupDelete(pwzidentity: super::super::Foundation::PWSTR, pwzgrouppeername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupDeleteRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupEnumConnections(hgroup: *const ::core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupEnumMembers(hgroup: *const ::core::ffi::c_void, dwflags: u32, pwzidentity: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupEnumRecords(hgroup: *const ::core::ffi::c_void, precordtype: *const ::windows_sys::core::GUID, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupExportConfig(hgroup: *const ::core::ffi::c_void, pwzpassword: super::super::Foundation::PWSTR, ppwzxml: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupExportDatabase(hgroup: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetEventData(hpeerevent: *const ::core::ffi::c_void, ppeventdata: *mut *mut PEER_GROUP_EVENT_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetProperties(hgroup: *const ::core::ffi::c_void, ppproperties: *mut *mut PEER_GROUP_PROPERTIES) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupGetRecord(hgroup: *const ::core::ffi::c_void, precordid: *const ::windows_sys::core::GUID, pprecord: *mut *mut PEER_RECORD) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupGetStatus(hgroup: *const ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupImportConfig(pwzxml: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, foverwrite: super::super::Foundation::BOOL, ppwzidentity: *mut super::super::Foundation::PWSTR, ppwzgroup: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupImportDatabase(hgroup: *const ::core::ffi::c_void, pwzfilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn PeerGroupIssueCredentials(hgroup: *const ::core::ffi::c_void, pwzsubjectidentity: super::super::Foundation::PWSTR, pcredentialinfo: *const PEER_CREDENTIAL_INFO, dwflags: u32, ppwzinvitation: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupJoin(pwzidentity: super::super::Foundation::PWSTR, pwzinvitation: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupOpen(pwzidentity: super::super::Foundation::PWSTR, pwzgrouppeername: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerGroupOpenDirectConnection(hgroup: *const ::core::ffi::c_void, pwzidentity: super::super::Foundation::PWSTR, paddress: *const PEER_ADDRESS, pullconnectionid: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn PeerGroupParseInvitation(pwzinvitation: super::super::Foundation::PWSTR, ppinvitationinfo: *mut *mut PEER_INVITATION_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupPasswordJoin(pwzidentity: super::super::Foundation::PWSTR, pwzinvitation: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, pwzcloud: super::super::Foundation::PWSTR, phgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupPeerTimeToUniversalTime(hgroup: *const ::core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME, pftuniversaltime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupRegisterEvent(hgroup: *const ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ceventregistration: u32, peventregistrations: *const PEER_GROUP_EVENT_REGISTRATION, phpeerevent: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupResumePasswordAuthentication(hgroup: *const ::core::ffi::c_void, hpeereventhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupSearchRecords(hgroup: *const ::core::ffi::c_void, pwzcriteria: super::super::Foundation::PWSTR, phpeerenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupSendData(hgroup: *const ::core::ffi::c_void, ullconnectionid: u64, ptype: *const ::windows_sys::core::GUID, cbdata: u32, pvdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupSetProperties(hgroup: *const ::core::ffi::c_void, pproperties: *const PEER_GROUP_PROPERTIES) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupShutdown() -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupStartup(wversionrequested: u16, pversiondata: *mut PEER_VERSION_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupUniversalTimeToPeerTime(hgroup: *const ::core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME, pftpeertime: *mut super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    pub fn PeerGroupUnregisterEvent(hpeerevent: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerGroupUpdateRecord(hgroup: *const ::core::ffi::c_void, precord: *const PEER_RECORD) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerHostNameToPeerName(pwzhostname: super::super::Foundation::PWSTR, ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityCreate(pwzclassifier: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, hcryptprov: usize, ppwzidentity: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityDelete(pwzidentity: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityExport(pwzidentity: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, ppwzexportxml: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetCryptKey(pwzidentity: super::super::Foundation::PWSTR, phcryptprov: *mut usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetDefault(ppwzpeername: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetFriendlyName(pwzidentity: super::super::Foundation::PWSTR, ppwzfriendlyname: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityGetXML(pwzidentity: super::super::Foundation::PWSTR, ppwzidentityxml: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentityImport(pwzimportxml: super::super::Foundation::PWSTR, pwzpassword: super::super::Foundation::PWSTR, ppwzidentity: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerIdentitySetFriendlyName(pwzidentity: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerNameToPeerHostName(pwzpeername: super::super::Foundation::PWSTR, ppwzhostname: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn PeerPnrpEndResolve(hresolve: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerPnrpGetCloudInfo(pcnumclouds: *mut u32, ppcloudinfo: *mut *mut PEER_PNRP_CLOUD_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpGetEndpoint(hresolve: *const ::core::ffi::c_void, ppendpoint: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpRegister(pcwzpeername: super::super::Foundation::PWSTR, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO, phregistration: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpResolve(pcwzpeername: super::super::Foundation::PWSTR, pcwzcloudname: super::super::Foundation::PWSTR, pcendpoints: *mut u32, ppendpoints: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> ::windows_sys::core::HRESULT;
    pub fn PeerPnrpShutdown() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeerPnrpStartResolve(pcwzpeername: super::super::Foundation::PWSTR, pcwzcloudname: super::super::Foundation::PWSTR, cmaxendpoints: u32, hevent: super::super::Foundation::HANDLE, phresolve: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PeerPnrpStartup(wversionrequested: u16) -> ::windows_sys::core::HRESULT;
    pub fn PeerPnrpUnregister(hregistration: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn PeerPnrpUpdateRegistration(hregistration: *const ::core::ffi::c_void, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_ADDRESS {
    pub socketAddress: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub flags: u32,
    pub nearness: i32,
    pub latency: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DRT_ADDRESS_FLAGS = i32;
pub const DRT_ADDRESS_FLAG_ACCEPTED: DRT_ADDRESS_FLAGS = 1i32;
pub const DRT_ADDRESS_FLAG_REJECTED: DRT_ADDRESS_FLAGS = 2i32;
pub const DRT_ADDRESS_FLAG_UNREACHABLE: DRT_ADDRESS_FLAGS = 4i32;
pub const DRT_ADDRESS_FLAG_LOOP: DRT_ADDRESS_FLAGS = 8i32;
pub const DRT_ADDRESS_FLAG_TOO_BUSY: DRT_ADDRESS_FLAGS = 16i32;
pub const DRT_ADDRESS_FLAG_BAD_VALIDATE_ID: DRT_ADDRESS_FLAGS = 32i32;
pub const DRT_ADDRESS_FLAG_SUSPECT_UNREGISTERED_ID: DRT_ADDRESS_FLAGS = 64i32;
pub const DRT_ADDRESS_FLAG_INQUIRE: DRT_ADDRESS_FLAGS = 128i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_ADDRESS_LIST {
    pub AddressCount: u32,
    pub AddressList: [DRT_ADDRESS; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_ADDRESS_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type DRT_BOOTSTRAP_RESOLVE_CALLBACK = unsafe extern "system" fn(hr: ::windows_sys::core::HRESULT, pvcontext: *mut ::core::ffi::c_void, paddresses: *mut super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, ffatalerror: super::super::Foundation::BOOL);
#[repr(C)]
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
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA {
    pub r#type: DRT_EVENT_TYPE,
    pub hr: ::windows_sys::core::HRESULT,
    pub pvContext: *mut ::core::ffi::c_void,
    pub Anonymous: DRT_EVENT_DATA_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union DRT_EVENT_DATA_0 {
    pub leafsetKeyChange: DRT_EVENT_DATA_0_0,
    pub registrationStateChange: DRT_EVENT_DATA_0_1,
    pub statusChange: DRT_EVENT_DATA_0_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_0 {
    pub change: DRT_LEAFSET_KEY_CHANGE_TYPE,
    pub localKey: DRT_DATA,
    pub remoteKey: DRT_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_1 {
    pub state: DRT_REGISTRATION_STATE,
    pub localKey: DRT_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_2 {
    pub status: DRT_STATUS,
    pub bootstrapAddresses: DRT_EVENT_DATA_0_2_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct DRT_EVENT_DATA_0_2_0 {
    pub cntAddress: u32,
    pub pAddresses: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for DRT_EVENT_DATA_0_2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for DRT_EVENT_DATA_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DRT_EVENT_TYPE = i32;
pub const DRT_EVENT_STATUS_CHANGED: DRT_EVENT_TYPE = 0i32;
pub const DRT_EVENT_LEAFSET_KEY_CHANGED: DRT_EVENT_TYPE = 1i32;
pub const DRT_EVENT_REGISTRATION_STATE_CHANGED: DRT_EVENT_TYPE = 2i32;
pub const DRT_E_BOOTSTRAPPROVIDER_IN_USE: ::windows_sys::core::HRESULT = -2141052914i32;
pub const DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED: ::windows_sys::core::HRESULT = -2141052913i32;
pub const DRT_E_CAPABILITY_MISMATCH: ::windows_sys::core::HRESULT = -2141052657i32;
pub const DRT_E_DUPLICATE_KEY: ::windows_sys::core::HRESULT = -2141052919i32;
pub const DRT_E_FAULTED: ::windows_sys::core::HRESULT = -2141052662i32;
pub const DRT_E_INSUFFICIENT_BUFFER: ::windows_sys::core::HRESULT = -2141052660i32;
pub const DRT_E_INVALID_ADDRESS: ::windows_sys::core::HRESULT = -2141052923i32;
pub const DRT_E_INVALID_BOOTSTRAP_PROVIDER: ::windows_sys::core::HRESULT = -2141052924i32;
pub const DRT_E_INVALID_CERT_CHAIN: ::windows_sys::core::HRESULT = -2141057020i32;
pub const DRT_E_INVALID_INSTANCE_PREFIX: ::windows_sys::core::HRESULT = -2141052659i32;
pub const DRT_E_INVALID_KEY: ::windows_sys::core::HRESULT = -2141057015i32;
pub const DRT_E_INVALID_KEY_SIZE: ::windows_sys::core::HRESULT = -2141057022i32;
pub const DRT_E_INVALID_MAX_ADDRESSES: ::windows_sys::core::HRESULT = -2141057017i32;
pub const DRT_E_INVALID_MAX_ENDPOINTS: ::windows_sys::core::HRESULT = -2141057007i32;
pub const DRT_E_INVALID_MESSAGE: ::windows_sys::core::HRESULT = -2141057019i32;
pub const DRT_E_INVALID_PORT: ::windows_sys::core::HRESULT = -2141052928i32;
pub const DRT_E_INVALID_SCOPE: ::windows_sys::core::HRESULT = -2141052922i32;
pub const DRT_E_INVALID_SEARCH_INFO: ::windows_sys::core::HRESULT = -2141052663i32;
pub const DRT_E_INVALID_SEARCH_RANGE: ::windows_sys::core::HRESULT = -2141057006i32;
pub const DRT_E_INVALID_SECURITY_MODE: ::windows_sys::core::HRESULT = -2141052658i32;
pub const DRT_E_INVALID_SECURITY_PROVIDER: ::windows_sys::core::HRESULT = -2141052926i32;
pub const DRT_E_INVALID_SETTINGS: ::windows_sys::core::HRESULT = -2141052664i32;
pub const DRT_E_INVALID_TRANSPORT_PROVIDER: ::windows_sys::core::HRESULT = -2141052927i32;
pub const DRT_E_NO_ADDRESSES_AVAILABLE: ::windows_sys::core::HRESULT = -2141052920i32;
pub const DRT_E_NO_MORE: ::windows_sys::core::HRESULT = -2141057018i32;
pub const DRT_E_SEARCH_IN_PROGRESS: ::windows_sys::core::HRESULT = -2141057016i32;
pub const DRT_E_SECURITYPROVIDER_IN_USE: ::windows_sys::core::HRESULT = -2141052916i32;
pub const DRT_E_SECURITYPROVIDER_NOT_ATTACHED: ::windows_sys::core::HRESULT = -2141052915i32;
pub const DRT_E_STILL_IN_USE: ::windows_sys::core::HRESULT = -2141052925i32;
pub const DRT_E_TIMEOUT: ::windows_sys::core::HRESULT = -2141057023i32;
pub const DRT_E_TRANSPORTPROVIDER_IN_USE: ::windows_sys::core::HRESULT = -2141052918i32;
pub const DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED: ::windows_sys::core::HRESULT = -2141052917i32;
pub const DRT_E_TRANSPORT_ALREADY_BOUND: ::windows_sys::core::HRESULT = -2141052671i32;
pub const DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE: ::windows_sys::core::HRESULT = -2141052665i32;
pub const DRT_E_TRANSPORT_EXECUTING_CALLBACK: ::windows_sys::core::HRESULT = -2141052666i32;
pub const DRT_E_TRANSPORT_INVALID_ARGUMENT: ::windows_sys::core::HRESULT = -2141052668i32;
pub const DRT_E_TRANSPORT_NOT_BOUND: ::windows_sys::core::HRESULT = -2141052670i32;
pub const DRT_E_TRANSPORT_NO_DEST_ADDRESSES: ::windows_sys::core::HRESULT = -2141052667i32;
pub const DRT_E_TRANSPORT_SHUTTING_DOWN: ::windows_sys::core::HRESULT = -2141052921i32;
pub const DRT_E_TRANSPORT_STILL_BOUND: ::windows_sys::core::HRESULT = -2141052661i32;
pub const DRT_E_TRANSPORT_UNEXPECTED: ::windows_sys::core::HRESULT = -2141052669i32;
pub type DRT_LEAFSET_KEY_CHANGE_TYPE = i32;
pub const DRT_LEAFSET_KEY_ADDED: DRT_LEAFSET_KEY_CHANGE_TYPE = 0i32;
pub const DRT_LEAFSET_KEY_DELETED: DRT_LEAFSET_KEY_CHANGE_TYPE = 1i32;
pub const DRT_LINK_LOCAL_ISATAP_SCOPEID: u32 = 4294967295u32;
pub type DRT_MATCH_TYPE = i32;
pub const DRT_MATCH_EXACT: DRT_MATCH_TYPE = 0i32;
pub const DRT_MATCH_NEAR: DRT_MATCH_TYPE = 1i32;
pub const DRT_MATCH_INTERMEDIATE: DRT_MATCH_TYPE = 2i32;
pub const DRT_MAX_INSTANCE_PREFIX_LEN: u32 = 128u32;
pub const DRT_MAX_PAYLOAD_SIZE: u32 = 5120u32;
pub const DRT_MAX_ROUTING_ADDRESSES: u32 = 20u32;
pub const DRT_MIN_ROUTING_ADDRESSES: u32 = 1u32;
pub const DRT_PAYLOAD_REVOKED: u32 = 1u32;
#[repr(C)]
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
pub type DRT_REGISTRATION_STATE = i32;
pub const DRT_REGISTRATION_STATE_UNRESOLVEABLE: DRT_REGISTRATION_STATE = 1i32;
pub type DRT_SCOPE = i32;
pub const DRT_GLOBAL_SCOPE: DRT_SCOPE = 1i32;
pub const DRT_SITE_LOCAL_SCOPE: DRT_SCOPE = 2i32;
pub const DRT_LINK_LOCAL_SCOPE: DRT_SCOPE = 3i32;
#[repr(C)]
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
#[repr(C)]
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
pub type DRT_SECURITY_MODE = i32;
pub const DRT_SECURE_RESOLVE: DRT_SECURITY_MODE = 0i32;
pub const DRT_SECURE_MEMBERSHIP: DRT_SECURITY_MODE = 1i32;
pub const DRT_SECURE_CONFIDENTIALPAYLOAD: DRT_SECURITY_MODE = 2i32;
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DRT_SETTINGS {
    pub dwSize: u32,
    pub cbKey: u32,
    pub bProtocolMajorVersion: u8,
    pub bProtocolMinorVersion: u8,
    pub ulMaxRoutingAddresses: u32,
    pub pwzDrtInstancePrefix: super::super::Foundation::PWSTR,
    pub hTransport: *mut ::core::ffi::c_void,
    pub pSecurityProvider: *mut DRT_SECURITY_PROVIDER,
    pub pBootstrapProvider: *mut DRT_BOOTSTRAP_PROVIDER,
    pub eSecurityMode: DRT_SECURITY_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRT_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRT_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DRT_STATUS = i32;
pub const DRT_ACTIVE: DRT_STATUS = 0i32;
pub const DRT_ALONE: DRT_STATUS = 1i32;
pub const DRT_NO_NETWORK: DRT_STATUS = 10i32;
pub const DRT_FAULTED: DRT_STATUS = 20i32;
pub const DRT_S_RETRY: ::windows_sys::core::HRESULT = 6426640i32;
pub const FACILITY_DRT: u32 = 98u32;
pub const NS_PNRPCLOUD: u32 = 39u32;
pub const NS_PNRPNAME: u32 = 38u32;
pub const NS_PROVIDER_PNRPCLOUD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 67013070, data2: 30317, data3: 18806, data4: [185, 193, 187, 155, 196, 44, 123, 77] };
pub const NS_PROVIDER_PNRPNAME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 67013069, data2: 30317, data3: 18806, data4: [185, 193, 187, 155, 196, 44, 123, 77] };
#[repr(C)]
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
pub type PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = i32;
pub const PeerDistClientBasicInfo: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = 0i32;
pub const MaximumPeerDistClientInfoByHandlesClass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = 1i32;
#[repr(C)]
pub struct PEERDIST_CONTENT_TAG {
    pub Data: [u8; 16],
}
impl ::core::marker::Copy for PEERDIST_CONTENT_TAG {}
impl ::core::clone::Clone for PEERDIST_CONTENT_TAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION: i32 = 2i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_1: i32 = 1i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_2: i32 = 2i32;
pub const PEERDIST_READ_TIMEOUT_DEFAULT: u32 = 4294967294u32;
pub const PEERDIST_READ_TIMEOUT_LOCAL_CACHE_ONLY: u32 = 0u32;
#[repr(C)]
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
pub type PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = u32;
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_1: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 1u32;
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_2: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 2u32;
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = 2u32;
pub type PEERDIST_STATUS = i32;
pub const PEERDIST_STATUS_DISABLED: PEERDIST_STATUS = 0i32;
pub const PEERDIST_STATUS_UNAVAILABLE: PEERDIST_STATUS = 1i32;
pub const PEERDIST_STATUS_AVAILABLE: PEERDIST_STATUS = 2i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_APPLICATION {
    pub id: ::windows_sys::core::GUID,
    pub data: PEER_DATA,
    pub pwzDescription: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_APPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_APPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_APPLICATION_REGISTRATION_INFO {
    pub application: PEER_APPLICATION,
    pub pwzApplicationToLaunch: super::super::Foundation::PWSTR,
    pub pwzApplicationArguments: super::super::Foundation::PWSTR,
    pub dwPublicationScope: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_APPLICATION_REGISTRATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_APPLICATION_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_APPLICATION_REGISTRATION_TYPE = i32;
pub const PEER_APPLICATION_CURRENT_USER: PEER_APPLICATION_REGISTRATION_TYPE = 0i32;
pub const PEER_APPLICATION_ALL_USERS: PEER_APPLICATION_REGISTRATION_TYPE = 1i32;
#[repr(C)]
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
pub type PEER_CHANGE_TYPE = i32;
pub const PEER_CHANGE_ADDED: PEER_CHANGE_TYPE = 0i32;
pub const PEER_CHANGE_DELETED: PEER_CHANGE_TYPE = 1i32;
pub const PEER_CHANGE_UPDATED: PEER_CHANGE_TYPE = 2i32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
pub struct PEER_COLLAB_EVENT_REGISTRATION {
    pub eventType: PEER_COLLAB_EVENT_TYPE,
    pub pInstance: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PEER_COLLAB_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_COLLAB_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_COLLAB_EVENT_TYPE = i32;
pub const PEER_EVENT_WATCHLIST_CHANGED: PEER_COLLAB_EVENT_TYPE = 1i32;
pub const PEER_EVENT_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = 2i32;
pub const PEER_EVENT_ENDPOINT_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = 3i32;
pub const PEER_EVENT_ENDPOINT_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = 4i32;
pub const PEER_EVENT_ENDPOINT_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = 5i32;
pub const PEER_EVENT_MY_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = 6i32;
pub const PEER_EVENT_MY_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = 7i32;
pub const PEER_EVENT_MY_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = 8i32;
pub const PEER_EVENT_MY_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = 9i32;
pub const PEER_EVENT_PEOPLE_NEAR_ME_CHANGED: PEER_COLLAB_EVENT_TYPE = 10i32;
pub const PEER_EVENT_REQUEST_STATUS_CHANGED: PEER_COLLAB_EVENT_TYPE = 11i32;
pub const PEER_COLLAB_OBJECTID_USER_PICTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3709203487, data2: 64590, data3: 18722, data4: [176, 53, 76, 6, 167, 84, 208, 29] };
pub type PEER_CONNECTION_FLAGS = i32;
pub const PEER_CONNECTION_NEIGHBOR: PEER_CONNECTION_FLAGS = 1i32;
pub const PEER_CONNECTION_DIRECT: PEER_CONNECTION_FLAGS = 2i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_CONNECTION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub pwzPeerId: super::super::Foundation::PWSTR,
    pub address: PEER_ADDRESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_CONNECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_CONNECTION_STATUS = i32;
pub const PEER_CONNECTED: PEER_CONNECTION_STATUS = 1i32;
pub const PEER_DISCONNECTED: PEER_CONNECTION_STATUS = 2i32;
pub const PEER_CONNECTION_FAILED: PEER_CONNECTION_STATUS = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_CONTACT {
    pub pwzPeerName: super::super::Foundation::PWSTR,
    pub pwzNickName: super::super::Foundation::PWSTR,
    pub pwzDisplayName: super::super::Foundation::PWSTR,
    pub pwzEmailAddress: super::super::Foundation::PWSTR,
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
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct PEER_CREDENTIAL_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
    pub pPublicKey: *mut super::super::Security::Cryptography::CERT_PUBLIC_KEY_INFO,
    pub pwzIssuerPeerName: super::super::Foundation::PWSTR,
    pub pwzIssuerFriendlyName: super::super::Foundation::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut ::windows_sys::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PEER_CREDENTIAL_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PEER_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_ENDPOINT {
    pub address: PEER_ADDRESS,
    pub pwzEndpointName: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_ENDPOINT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
pub struct PEER_EVENT_CONNECTION_CHANGE_DATA {
    pub dwSize: u32,
    pub status: PEER_CONNECTION_STATUS,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub ullNextConnectionId: u64,
    pub hrConnectionFailedReason: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for PEER_EVENT_CONNECTION_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
pub struct PEER_EVENT_INCOMING_DATA {
    pub dwSize: u32,
    pub ullConnectionId: u64,
    pub r#type: ::windows_sys::core::GUID,
    pub data: PEER_DATA,
}
impl ::core::marker::Copy for PEER_EVENT_INCOMING_DATA {}
impl ::core::clone::Clone for PEER_EVENT_INCOMING_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_EVENT_MEMBER_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_MEMBER_CHANGE_TYPE,
    pub pwzIdentity: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_EVENT_MEMBER_CHANGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_EVENT_NODE_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_NODE_CHANGE_TYPE,
    pub ullNodeId: u64,
    pub pwzPeerId: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_EVENT_NODE_CHANGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_EVENT_NODE_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    pub changeType: PEER_CHANGE_TYPE,
    pub pPeopleNearMe: *mut PEER_PEOPLE_NEAR_ME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
pub struct PEER_EVENT_RECORD_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_RECORD_CHANGE_TYPE,
    pub recordId: ::windows_sys::core::GUID,
    pub recordType: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PEER_EVENT_RECORD_CHANGE_DATA {}
impl ::core::clone::Clone for PEER_EVENT_RECORD_CHANGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub hrChange: ::windows_sys::core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PEER_EVENT_SYNCHRONIZED_DATA {
    pub dwSize: u32,
    pub recordType: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PEER_EVENT_SYNCHRONIZED_DATA {}
impl ::core::clone::Clone for PEER_EVENT_SYNCHRONIZED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
pub const PEER_E_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147024713i32;
pub const PEER_E_CLIENT_INVALID_COMPARTMENT_ID: ::windows_sys::core::HRESULT = -2147013390i32;
pub const PEER_E_CLOUD_DISABLED: ::windows_sys::core::HRESULT = -2147013394i32;
pub const PEER_E_CLOUD_IS_DEAD: ::windows_sys::core::HRESULT = -2147013387i32;
pub const PEER_E_CLOUD_IS_SEARCH_ONLY: ::windows_sys::core::HRESULT = -2147013391i32;
pub const PEER_E_CLOUD_NOT_FOUND: ::windows_sys::core::HRESULT = -2147013395i32;
pub const PEER_E_DISK_FULL: ::windows_sys::core::HRESULT = -2147024784i32;
pub const PEER_E_DUPLICATE_PEER_NAME: ::windows_sys::core::HRESULT = -2147013388i32;
pub const PEER_E_INVALID_IDENTITY: ::windows_sys::core::HRESULT = -2147013393i32;
pub const PEER_E_NOT_FOUND: ::windows_sys::core::HRESULT = -2147023728i32;
pub const PEER_E_TOO_MUCH_LOAD: ::windows_sys::core::HRESULT = -2147013392i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GRAPH_EVENT_DATA {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub Anonymous: PEER_GRAPH_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GRAPH_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GRAPH_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PEER_GRAPH_EVENT_DATA_0 {
    pub dwStatus: PEER_GRAPH_STATUS_FLAGS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub nodeChangeData: PEER_EVENT_NODE_CHANGE_DATA,
    pub synchronizedData: PEER_EVENT_SYNCHRONIZED_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GRAPH_EVENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GRAPH_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PEER_GRAPH_EVENT_REGISTRATION {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub pType: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PEER_GRAPH_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_GRAPH_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_GRAPH_EVENT_TYPE = i32;
pub const PEER_GRAPH_EVENT_STATUS_CHANGED: PEER_GRAPH_EVENT_TYPE = 1i32;
pub const PEER_GRAPH_EVENT_PROPERTY_CHANGED: PEER_GRAPH_EVENT_TYPE = 2i32;
pub const PEER_GRAPH_EVENT_RECORD_CHANGED: PEER_GRAPH_EVENT_TYPE = 3i32;
pub const PEER_GRAPH_EVENT_DIRECT_CONNECTION: PEER_GRAPH_EVENT_TYPE = 4i32;
pub const PEER_GRAPH_EVENT_NEIGHBOR_CONNECTION: PEER_GRAPH_EVENT_TYPE = 5i32;
pub const PEER_GRAPH_EVENT_INCOMING_DATA: PEER_GRAPH_EVENT_TYPE = 6i32;
pub const PEER_GRAPH_EVENT_CONNECTION_REQUIRED: PEER_GRAPH_EVENT_TYPE = 7i32;
pub const PEER_GRAPH_EVENT_NODE_CHANGED: PEER_GRAPH_EVENT_TYPE = 8i32;
pub const PEER_GRAPH_EVENT_SYNCHRONIZED: PEER_GRAPH_EVENT_TYPE = 9i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GRAPH_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwScope: u32,
    pub dwMaxRecordSize: u32,
    pub pwzGraphId: super::super::Foundation::PWSTR,
    pub pwzCreatorId: super::super::Foundation::PWSTR,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub ulPresenceLifetime: u32,
    pub cPresenceMax: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GRAPH_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GRAPH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_GRAPH_PROPERTY_FLAGS = i32;
pub const PEER_GRAPH_PROPERTY_HEARTBEATS: PEER_GRAPH_PROPERTY_FLAGS = 1i32;
pub const PEER_GRAPH_PROPERTY_DEFER_EXPIRATION: PEER_GRAPH_PROPERTY_FLAGS = 2i32;
pub type PEER_GRAPH_SCOPE = i32;
pub const PEER_GRAPH_SCOPE_ANY: PEER_GRAPH_SCOPE = 0i32;
pub const PEER_GRAPH_SCOPE_GLOBAL: PEER_GRAPH_SCOPE = 1i32;
pub const PEER_GRAPH_SCOPE_SITELOCAL: PEER_GRAPH_SCOPE = 2i32;
pub const PEER_GRAPH_SCOPE_LINKLOCAL: PEER_GRAPH_SCOPE = 3i32;
pub const PEER_GRAPH_SCOPE_LOOPBACK: PEER_GRAPH_SCOPE = 4i32;
pub type PEER_GRAPH_STATUS_FLAGS = i32;
pub const PEER_GRAPH_STATUS_LISTENING: PEER_GRAPH_STATUS_FLAGS = 1i32;
pub const PEER_GRAPH_STATUS_HAS_CONNECTIONS: PEER_GRAPH_STATUS_FLAGS = 2i32;
pub const PEER_GRAPH_STATUS_SYNCHRONIZED: PEER_GRAPH_STATUS_FLAGS = 4i32;
pub type PEER_GROUP_AUTHENTICATION_SCHEME = i32;
pub const PEER_GROUP_GMC_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = 1i32;
pub const PEER_GROUP_PASSWORD_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GROUP_EVENT_DATA {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub Anonymous: PEER_GROUP_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GROUP_EVENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GROUP_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PEER_GROUP_EVENT_DATA_0 {
    pub dwStatus: PEER_GROUP_STATUS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub memberChangeData: PEER_EVENT_MEMBER_CHANGE_DATA,
    pub hrConnectionFailedReason: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GROUP_EVENT_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GROUP_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PEER_GROUP_EVENT_REGISTRATION {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub pType: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PEER_GROUP_EVENT_REGISTRATION {}
impl ::core::clone::Clone for PEER_GROUP_EVENT_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_GROUP_EVENT_TYPE = i32;
pub const PEER_GROUP_EVENT_STATUS_CHANGED: PEER_GROUP_EVENT_TYPE = 1i32;
pub const PEER_GROUP_EVENT_PROPERTY_CHANGED: PEER_GROUP_EVENT_TYPE = 2i32;
pub const PEER_GROUP_EVENT_RECORD_CHANGED: PEER_GROUP_EVENT_TYPE = 3i32;
pub const PEER_GROUP_EVENT_DIRECT_CONNECTION: PEER_GROUP_EVENT_TYPE = 4i32;
pub const PEER_GROUP_EVENT_NEIGHBOR_CONNECTION: PEER_GROUP_EVENT_TYPE = 5i32;
pub const PEER_GROUP_EVENT_INCOMING_DATA: PEER_GROUP_EVENT_TYPE = 6i32;
pub const PEER_GROUP_EVENT_MEMBER_CHANGED: PEER_GROUP_EVENT_TYPE = 8i32;
pub const PEER_GROUP_EVENT_CONNECTION_FAILED: PEER_GROUP_EVENT_TYPE = 10i32;
pub const PEER_GROUP_EVENT_AUTHENTICATION_FAILED: PEER_GROUP_EVENT_TYPE = 11i32;
pub type PEER_GROUP_ISSUE_CREDENTIAL_FLAGS = i32;
pub const PEER_GROUP_STORE_CREDENTIALS: PEER_GROUP_ISSUE_CREDENTIAL_FLAGS = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_GROUP_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloud: super::super::Foundation::PWSTR,
    pub pwzClassifier: super::super::Foundation::PWSTR,
    pub pwzGroupPeerName: super::super::Foundation::PWSTR,
    pub pwzCreatorPeerName: super::super::Foundation::PWSTR,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub ulMemberDataLifetime: u32,
    pub ulPresenceLifetime: u32,
    pub dwAuthenticationSchemes: u32,
    pub pwzGroupPassword: super::super::Foundation::PWSTR,
    pub groupPasswordRole: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_GROUP_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_GROUP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_GROUP_PROPERTY_FLAGS = i32;
pub const PEER_MEMBER_DATA_OPTIONAL: PEER_GROUP_PROPERTY_FLAGS = 1i32;
pub const PEER_DISABLE_PRESENCE: PEER_GROUP_PROPERTY_FLAGS = 2i32;
pub const PEER_DEFER_EXPIRATION: PEER_GROUP_PROPERTY_FLAGS = 4i32;
pub const PEER_GROUP_ROLE_ADMIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 70807847, data2: 43606, data3: 17674, data4: [140, 229, 79, 86, 92, 103, 144, 244] };
pub const PEER_GROUP_ROLE_INVITING_MEMBER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1131478409,
    data2: 56344,
    data3: 19707,
    data4: [141, 191, 152, 83, 168, 169, 249, 5],
};
pub const PEER_GROUP_ROLE_MEMBER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4046308551,
    data2: 2135,
    data3: 19616,
    data4: [147, 252, 177, 187, 25, 163, 216, 194],
};
pub type PEER_GROUP_STATUS = i32;
pub const PEER_GROUP_STATUS_LISTENING: PEER_GROUP_STATUS = 1i32;
pub const PEER_GROUP_STATUS_HAS_CONNECTIONS: PEER_GROUP_STATUS = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_INVITATION {
    pub applicationId: ::windows_sys::core::GUID,
    pub applicationData: PEER_DATA,
    pub pwzMessage: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_INVITATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_INVITATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct PEER_INVITATION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloudName: super::super::Foundation::PWSTR,
    pub dwScope: u32,
    pub dwCloudFlags: u32,
    pub pwzGroupPeerName: super::super::Foundation::PWSTR,
    pub pwzIssuerPeerName: super::super::Foundation::PWSTR,
    pub pwzSubjectPeerName: super::super::Foundation::PWSTR,
    pub pwzGroupFriendlyName: super::super::Foundation::PWSTR,
    pub pwzIssuerFriendlyName: super::super::Foundation::PWSTR,
    pub pwzSubjectFriendlyName: super::super::Foundation::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut ::windows_sys::core::GUID,
    pub cClassifiers: u32,
    pub ppwzClassifiers: *mut super::super::Foundation::PWSTR,
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_INVITATION_RESPONSE {
    pub action: PEER_INVITATION_RESPONSE_TYPE,
    pub pwzMessage: super::super::Foundation::PWSTR,
    pub hrExtendedInfo: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_INVITATION_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_INVITATION_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_INVITATION_RESPONSE_TYPE = i32;
pub const PEER_INVITATION_RESPONSE_DECLINED: PEER_INVITATION_RESPONSE_TYPE = 0i32;
pub const PEER_INVITATION_RESPONSE_ACCEPTED: PEER_INVITATION_RESPONSE_TYPE = 1i32;
pub const PEER_INVITATION_RESPONSE_EXPIRED: PEER_INVITATION_RESPONSE_TYPE = 2i32;
pub const PEER_INVITATION_RESPONSE_ERROR: PEER_INVITATION_RESPONSE_TYPE = 3i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
pub struct PEER_MEMBER {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzIdentity: super::super::Foundation::PWSTR,
    pub pwzAttributes: super::super::Foundation::PWSTR,
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
pub type PEER_MEMBER_CHANGE_TYPE = i32;
pub const PEER_MEMBER_CONNECTED: PEER_MEMBER_CHANGE_TYPE = 1i32;
pub const PEER_MEMBER_DISCONNECTED: PEER_MEMBER_CHANGE_TYPE = 2i32;
pub const PEER_MEMBER_UPDATED: PEER_MEMBER_CHANGE_TYPE = 3i32;
pub const PEER_MEMBER_JOINED: PEER_MEMBER_CHANGE_TYPE = 4i32;
pub const PEER_MEMBER_LEFT: PEER_MEMBER_CHANGE_TYPE = 5i32;
pub type PEER_MEMBER_FLAGS = i32;
pub const PEER_MEMBER_PRESENT: PEER_MEMBER_FLAGS = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_NAME_PAIR {
    pub dwSize: u32,
    pub pwzPeerName: super::super::Foundation::PWSTR,
    pub pwzFriendlyName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_NAME_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_NAME_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_NODE_CHANGE_TYPE = i32;
pub const PEER_NODE_CHANGE_CONNECTED: PEER_NODE_CHANGE_TYPE = 1i32;
pub const PEER_NODE_CHANGE_DISCONNECTED: PEER_NODE_CHANGE_TYPE = 2i32;
pub const PEER_NODE_CHANGE_UPDATED: PEER_NODE_CHANGE_TYPE = 3i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_NODE_INFO {
    pub dwSize: u32,
    pub ullNodeId: u64,
    pub pwzPeerId: super::super::Foundation::PWSTR,
    pub cAddresses: u32,
    pub pAddresses: *mut PEER_ADDRESS,
    pub pwzAttributes: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_NODE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_NODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PEER_OBJECT {
    pub id: ::windows_sys::core::GUID,
    pub data: PEER_DATA,
    pub dwPublicationScope: u32,
}
impl ::core::marker::Copy for PEER_OBJECT {}
impl ::core::clone::Clone for PEER_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_PEOPLE_NEAR_ME {
    pub pwzNickName: super::super::Foundation::PWSTR,
    pub endpoint: PEER_ENDPOINT,
    pub id: ::windows_sys::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_PEOPLE_NEAR_ME {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_PEOPLE_NEAR_ME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_PNRP_CLOUD_INFO {
    pub pwzCloudName: super::super::Foundation::PWSTR,
    pub dwScope: PNRP_SCOPE,
    pub dwScopeId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_PNRP_CLOUD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_PNRP_CLOUD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_PNRP_ENDPOINT_INFO {
    pub pwzPeerName: super::super::Foundation::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_PNRP_ENDPOINT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_PNRP_ENDPOINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PEER_PNRP_REGISTRATION_INFO {
    pub pwzCloudName: super::super::Foundation::PWSTR,
    pub pwzPublishingIdentity: super::super::Foundation::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub wPort: u16,
    pub pwzComment: super::super::Foundation::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PEER_PNRP_REGISTRATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PEER_PNRP_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_PRESENCE_INFO {
    pub status: PEER_PRESENCE_STATUS,
    pub pwzDescriptiveText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PEER_PRESENCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PEER_PRESENCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PEER_PRESENCE_STATUS = i32;
pub const PEER_PRESENCE_OFFLINE: PEER_PRESENCE_STATUS = 0i32;
pub const PEER_PRESENCE_OUT_TO_LUNCH: PEER_PRESENCE_STATUS = 1i32;
pub const PEER_PRESENCE_AWAY: PEER_PRESENCE_STATUS = 2i32;
pub const PEER_PRESENCE_BE_RIGHT_BACK: PEER_PRESENCE_STATUS = 3i32;
pub const PEER_PRESENCE_IDLE: PEER_PRESENCE_STATUS = 4i32;
pub const PEER_PRESENCE_BUSY: PEER_PRESENCE_STATUS = 5i32;
pub const PEER_PRESENCE_ON_THE_PHONE: PEER_PRESENCE_STATUS = 6i32;
pub const PEER_PRESENCE_ONLINE: PEER_PRESENCE_STATUS = 7i32;
pub type PEER_PUBLICATION_SCOPE = i32;
pub const PEER_PUBLICATION_SCOPE_NONE: PEER_PUBLICATION_SCOPE = 0i32;
pub const PEER_PUBLICATION_SCOPE_NEAR_ME: PEER_PUBLICATION_SCOPE = 1i32;
pub const PEER_PUBLICATION_SCOPE_INTERNET: PEER_PUBLICATION_SCOPE = 2i32;
pub const PEER_PUBLICATION_SCOPE_ALL: PEER_PUBLICATION_SCOPE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_RECORD {
    pub dwSize: u32,
    pub r#type: ::windows_sys::core::GUID,
    pub id: ::windows_sys::core::GUID,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pwzCreatorId: super::super::Foundation::PWSTR,
    pub pwzModifiedById: super::super::Foundation::PWSTR,
    pub pwzAttributes: super::super::Foundation::PWSTR,
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
pub type PEER_RECORD_CHANGE_TYPE = i32;
pub const PEER_RECORD_ADDED: PEER_RECORD_CHANGE_TYPE = 1i32;
pub const PEER_RECORD_UPDATED: PEER_RECORD_CHANGE_TYPE = 2i32;
pub const PEER_RECORD_DELETED: PEER_RECORD_CHANGE_TYPE = 3i32;
pub const PEER_RECORD_EXPIRED: PEER_RECORD_CHANGE_TYPE = 4i32;
pub type PEER_RECORD_FLAGS = i32;
pub const PEER_RECORD_FLAG_AUTOREFRESH: PEER_RECORD_FLAGS = 1i32;
pub const PEER_RECORD_FLAG_DELETED: PEER_RECORD_FLAGS = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PEER_SECURITY_INTERFACE {
    pub dwSize: u32,
    pub pwzSspFilename: super::super::Foundation::PWSTR,
    pub pwzPackageName: super::super::Foundation::PWSTR,
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
pub type PEER_SIGNIN_FLAGS = i32;
pub const PEER_SIGNIN_NONE: PEER_SIGNIN_FLAGS = 0i32;
pub const PEER_SIGNIN_NEAR_ME: PEER_SIGNIN_FLAGS = 1i32;
pub const PEER_SIGNIN_INTERNET: PEER_SIGNIN_FLAGS = 2i32;
pub const PEER_SIGNIN_ALL: PEER_SIGNIN_FLAGS = 3i32;
#[repr(C)]
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
pub type PEER_WATCH_PERMISSION = i32;
pub const PEER_WATCH_BLOCKED: PEER_WATCH_PERMISSION = 0i32;
pub const PEER_WATCH_ALLOWED: PEER_WATCH_PERMISSION = 1i32;
pub type PFNPEER_FREE_SECURITY_DATA = unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, psecuritydata: *const PEER_DATA) -> ::windows_sys::core::HRESULT;
pub type PFNPEER_ON_PASSWORD_AUTH_FAILED = unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNPEER_SECURE_RECORD = unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE, ppsecuritydata: *mut *mut PEER_DATA) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNPEER_VALIDATE_RECORD = unsafe extern "system" fn(hgraph: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE) -> ::windows_sys::core::HRESULT;
#[repr(C)]
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
pub const PNRPINFO_HINT: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct PNRPINFO_V1 {
    pub dwSize: u32,
    pub lpwszIdentity: super::super::Foundation::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for PNRPINFO_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for PNRPINFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
pub struct PNRPINFO_V2 {
    pub dwSize: u32,
    pub lpwszIdentity: super::super::Foundation::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PNRPINFO_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PNRPINFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
pub union PNRPINFO_V2_0 {
    pub blobPayload: super::super::System::Com::BLOB,
    pub pwszPayload: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PNRPINFO_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PNRPINFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PNRP_CLOUD_FLAGS = i32;
pub const PNRP_CLOUD_NO_FLAGS: PNRP_CLOUD_FLAGS = 0i32;
pub const PNRP_CLOUD_NAME_LOCAL: PNRP_CLOUD_FLAGS = 1i32;
pub const PNRP_CLOUD_RESOLVE_ONLY: PNRP_CLOUD_FLAGS = 2i32;
pub const PNRP_CLOUD_FULL_PARTICIPANT: PNRP_CLOUD_FLAGS = 4i32;
#[repr(C)]
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
pub type PNRP_CLOUD_STATE = i32;
pub const PNRP_CLOUD_STATE_VIRTUAL: PNRP_CLOUD_STATE = 0i32;
pub const PNRP_CLOUD_STATE_SYNCHRONISING: PNRP_CLOUD_STATE = 1i32;
pub const PNRP_CLOUD_STATE_ACTIVE: PNRP_CLOUD_STATE = 2i32;
pub const PNRP_CLOUD_STATE_DEAD: PNRP_CLOUD_STATE = 3i32;
pub const PNRP_CLOUD_STATE_DISABLED: PNRP_CLOUD_STATE = 4i32;
pub const PNRP_CLOUD_STATE_NO_NET: PNRP_CLOUD_STATE = 5i32;
pub const PNRP_CLOUD_STATE_ALONE: PNRP_CLOUD_STATE = 6i32;
pub type PNRP_EXTENDED_PAYLOAD_TYPE = i32;
pub const PNRP_EXTENDED_PAYLOAD_TYPE_NONE: PNRP_EXTENDED_PAYLOAD_TYPE = 0i32;
pub const PNRP_EXTENDED_PAYLOAD_TYPE_BINARY: PNRP_EXTENDED_PAYLOAD_TYPE = 1i32;
pub const PNRP_EXTENDED_PAYLOAD_TYPE_STRING: PNRP_EXTENDED_PAYLOAD_TYPE = 2i32;
pub const PNRP_MAX_ENDPOINT_ADDRESSES: u32 = 10u32;
pub const PNRP_MAX_EXTENDED_PAYLOAD_BYTES: u32 = 4096u32;
pub type PNRP_REGISTERED_ID_STATE = i32;
pub const PNRP_REGISTERED_ID_STATE_OK: PNRP_REGISTERED_ID_STATE = 1i32;
pub const PNRP_REGISTERED_ID_STATE_PROBLEM: PNRP_REGISTERED_ID_STATE = 2i32;
pub type PNRP_RESOLVE_CRITERIA = i32;
pub const PNRP_RESOLVE_CRITERIA_DEFAULT: PNRP_RESOLVE_CRITERIA = 0i32;
pub const PNRP_RESOLVE_CRITERIA_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = 1i32;
pub const PNRP_RESOLVE_CRITERIA_NEAREST_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = 2i32;
pub const PNRP_RESOLVE_CRITERIA_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = 3i32;
pub const PNRP_RESOLVE_CRITERIA_NEAREST_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = 4i32;
pub const PNRP_RESOLVE_CRITERIA_ANY_PEER_NAME: PNRP_RESOLVE_CRITERIA = 5i32;
pub const PNRP_RESOLVE_CRITERIA_NEAREST_PEER_NAME: PNRP_RESOLVE_CRITERIA = 6i32;
pub type PNRP_SCOPE = i32;
pub const PNRP_SCOPE_ANY: PNRP_SCOPE = 0i32;
pub const PNRP_GLOBAL_SCOPE: PNRP_SCOPE = 1i32;
pub const PNRP_SITE_LOCAL_SCOPE: PNRP_SCOPE = 2i32;
pub const PNRP_LINK_LOCAL_SCOPE: PNRP_SCOPE = 3i32;
pub const SVCID_PNRPCLOUD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3257113830, data2: 192, data3: 20415, data4: [186, 214, 24, 19, 147, 133, 164, 154] };
pub const SVCID_PNRPNAME_V1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3257113829, data2: 192, data3: 20415, data4: [186, 214, 24, 19, 147, 133, 164, 154] };
pub const SVCID_PNRPNAME_V2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3257113831, data2: 192, data3: 20415, data4: [186, 214, 24, 19, 147, 133, 164, 154] };
pub const WSA_PNRP_CLIENT_INVALID_COMPARTMENT_ID: u32 = 11506u32;
pub const WSA_PNRP_CLOUD_DISABLED: u32 = 11502u32;
pub const WSA_PNRP_CLOUD_IS_DEAD: u32 = 11509u32;
pub const WSA_PNRP_CLOUD_IS_SEARCH_ONLY: u32 = 11505u32;
pub const WSA_PNRP_CLOUD_NOT_FOUND: u32 = 11501u32;
pub const WSA_PNRP_DUPLICATE_PEER_NAME: u32 = 11508u32;
pub const WSA_PNRP_ERROR_BASE: u32 = 11500u32;
pub const WSA_PNRP_INVALID_IDENTITY: u32 = 11503u32;
pub const WSA_PNRP_TOO_MUCH_LOAD: u32 = 11504u32;
