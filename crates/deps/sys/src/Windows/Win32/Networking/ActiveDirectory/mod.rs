#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsBuildEnumerator(padscontainer: IADsContainer, ppenumvariant: *mut super::super::System::Ole::IEnumVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayInt(lpdwobjecttypes: *mut u32, dwobjecttypes: u32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayStr(lpppathnames: *const super::super::Foundation::PWSTR, dwpathnames: u32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsDecodeBinaryData(szsrcdata: super::super::Foundation::PWSTR, ppbdestdata: *mut *mut u8, pdwdestlen: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsEncodeBinaryData(pbsrcdata: *mut u8, dwsrclen: u32, ppszdestdata: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsEnumerateNext(penumvariant: super::super::System::Ole::IEnumVARIANT, celements: u32, pvar: *mut super::super::System::Com::VARIANT, pcelementsfetched: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsFreeEnumerator(penumvariant: super::super::System::Ole::IEnumVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetLastError(lperror: *mut u32, lperrorbuf: super::super::Foundation::PWSTR, dwerrorbuflen: u32, lpnamebuf: super::super::Foundation::PWSTR, dwnamebuflen: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetObject(lpszpathname: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsOpenObject(lpszpathname: super::super::Foundation::PWSTR, lpszusername: super::super::Foundation::PWSTR, lpszpassword: super::super::Foundation::PWSTR, dwreserved: ADS_AUTHENTICATION_ENUM, riid: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropCheckIfWritable(pwzattr: super::super::Foundation::PWSTR, pwritableattrs: *const ADS_ATTR_INFO) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ADsPropCreateNotifyObj(pappthddataobj: super::super::System::Com::IDataObject, pwzadsobjname: super::super::Foundation::PWSTR, phnotifyobj: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropGetInitInfo(hnotifyobj: super::super::Foundation::HWND, pinitparams: *mut ADSPROPINITPARAMS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSendErrorMessage(hnotifyobj: super::super::Foundation::HWND, perror: *mut ADSPROPERROR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSetHwnd(hnotifyobj: super::super::Foundation::HWND, hpage: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSetHwndWithTitle(hnotifyobj: super::super::Foundation::HWND, hpage: super::super::Foundation::HWND, ptztitle: *const i8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropShowErrorDialog(hnotifyobj: super::super::Foundation::HWND, hpage: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsSetLastError(dwerr: u32, pszerror: super::super::Foundation::PWSTR, pszprovider: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdsFreeAdsValues(padsvalues: *mut ADSVALUE, dwnumvalues: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AdsTypeToPropVariant(padsvalues: *mut ADSVALUE, dwnumvalues: u32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    pub fn AllocADsMem(cb: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocADsStr(pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn BinarySDToSecurityDescriptor(psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, pvarsec: *mut super::super::System::Com::VARIANT, pszservername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsAddSidHistoryA(hds: super::super::Foundation::HANDLE, flags: u32, srcdomain: super::super::Foundation::PSTR, srcprincipal: super::super::Foundation::PSTR, srcdomaincontroller: super::super::Foundation::PSTR, srcdomaincreds: *const ::core::ffi::c_void, dstdomain: super::super::Foundation::PSTR, dstprincipal: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsAddSidHistoryW(hds: super::super::Foundation::HANDLE, flags: u32, srcdomain: super::super::Foundation::PWSTR, srcprincipal: super::super::Foundation::PWSTR, srcdomaincontroller: super::super::Foundation::PWSTR, srcdomaincreds: *const ::core::ffi::c_void, dstdomain: super::super::Foundation::PWSTR, dstprincipal: super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesA(computername: super::super::Foundation::PSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesExA(computername: super::super::Foundation::PSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PSTR, subnetnames: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesExW(computername: super::super::Foundation::PWSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PWSTR, subnetnames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesW(computername: super::super::Foundation::PWSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindByInstanceA(servername: super::super::Foundation::PSTR, annotation: super::super::Foundation::PSTR, instanceguid: *const ::windows_sys::core::GUID, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindByInstanceW(servername: super::super::Foundation::PWSTR, annotation: super::super::Foundation::PWSTR, instanceguid: *const ::windows_sys::core::GUID, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PWSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindToISTGA(sitename: super::super::Foundation::PSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindToISTGW(sitename: super::super::Foundation::PWSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithCredA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithCredW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnExA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnExW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PWSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PWSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindingSetTimeout(hds: super::super::Foundation::HANDLE, ctimeoutsecs: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn DsBrowseForContainerA(pinfo: *mut DSBROWSEINFOA) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn DsBrowseForContainerW(pinfo: *mut DSBROWSEINFOW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsClientMakeSpnForTargetServerA(serviceclass: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsClientMakeSpnForTargetServerW(serviceclass: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackNamesA(hds: super::super::Foundation::HANDLE, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, cnames: u32, rpnames: *const super::super::Foundation::PSTR, ppresult: *mut *mut DS_NAME_RESULTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackNamesW(hds: super::super::Foundation::HANDLE, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, cnames: u32, rpnames: *const super::super::Foundation::PWSTR, ppresult: *mut *mut DS_NAME_RESULTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn2A(pszspn: super::super::Foundation::PSTR, cspn: u32, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PSTR, pinstanceport: *mut u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn2W(pszspn: super::super::Foundation::PWSTR, cspn: u32, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PWSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pinstanceport: *mut u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn3W(pszspn: super::super::Foundation::PWSTR, cspn: u32, pchostname: *mut u32, hostname: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pportnumber: *mut u16, pcdomainname: *mut u32, domainname: super::super::Foundation::PWSTR, pcrealmname: *mut u32, realmname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn4W(pszspn: super::super::Foundation::PWSTR, cspn: u32, pchostname: *mut u32, hostname: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pcportname: *mut u32, portname: super::super::Foundation::PWSTR, pcdomainname: *mut u32, domainname: super::super::Foundation::PWSTR, pcrealmname: *mut u32, realmname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpnA(pszspn: super::super::Foundation::PSTR, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PSTR, pinstanceport: *mut u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpnW(pszspn: super::super::Foundation::PWSTR, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PWSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pinstanceport: *mut u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackUnquotedMangledRdnA(pszrdn: super::super::Foundation::PSTR, cchrdn: u32, pguid: *mut ::windows_sys::core::GUID, pedsmanglefor: *mut DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackUnquotedMangledRdnW(pszrdn: super::super::Foundation::PWSTR, cchrdn: u32, pguid: *mut ::windows_sys::core::GUID, pedsmanglefor: *mut DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsA(servername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, domainguid: *const ::windows_sys::core::GUID, dsaguid: *const ::windows_sys::core::GUID, dnshostname: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsW(servername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, domainguid: *const ::windows_sys::core::GUID, dsaguid: *const ::windows_sys::core::GUID, dnshostname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsEnumerateDomainTrustsA(servername: super::super::Foundation::PSTR, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSA, domaincount: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsEnumerateDomainTrustsW(servername: super::super::Foundation::PWSTR, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSW, domaincount: *mut u32) -> u32;
    pub fn DsFreeDomainControllerInfoA(infolevel: u32, cinfo: u32, pinfo: *const ::core::ffi::c_void);
    pub fn DsFreeDomainControllerInfoW(infolevel: u32, cinfo: u32, pinfo: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeNameResultA(presult: *const DS_NAME_RESULTA);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeNameResultW(presult: *const DS_NAME_RESULTW);
    pub fn DsFreePasswordCredentials(authidentity: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSchemaGuidMapA(pguidmap: *const DS_SCHEMA_GUID_MAPA);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSchemaGuidMapW(pguidmap: *const DS_SCHEMA_GUID_MAPW);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSpnArrayA(cspn: u32, rpszspn: *mut super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSpnArrayW(cspn: u32, rpszspn: *mut super::super::Foundation::PWSTR);
    pub fn DsGetDcCloseW(getdccontexthandle: GetDcContextHandle);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcNameA(computername: super::super::Foundation::PSTR, domainname: super::super::Foundation::PSTR, domainguid: *const ::windows_sys::core::GUID, sitename: super::super::Foundation::PSTR, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcNameW(computername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, domainguid: *const ::windows_sys::core::GUID, sitename: super::super::Foundation::PWSTR, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextA(getdccontexthandle: super::super::Foundation::HANDLE, sockaddresscount: *mut u32, sockaddresses: *mut *mut super::WinSock::SOCKET_ADDRESS, dnshostname: *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextW(getdccontexthandle: super::super::Foundation::HANDLE, sockaddresscount: *mut u32, sockaddresses: *mut *mut super::WinSock::SOCKET_ADDRESS, dnshostname: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenA(dnsname: super::super::Foundation::PSTR, optionflags: u32, sitename: super::super::Foundation::PSTR, domainguid: *const ::windows_sys::core::GUID, dnsforestname: super::super::Foundation::PSTR, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenW(dnsname: super::super::Foundation::PWSTR, optionflags: u32, sitename: super::super::Foundation::PWSTR, domainguid: *const ::windows_sys::core::GUID, dnsforestname: super::super::Foundation::PWSTR, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcSiteCoverageA(servername: super::super::Foundation::PSTR, entrycount: *mut u32, sitenames: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcSiteCoverageW(servername: super::super::Foundation::PWSTR, entrycount: *mut u32, sitenames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDomainControllerInfoA(hds: super::super::Foundation::HANDLE, domainname: super::super::Foundation::PSTR, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDomainControllerInfoW(hds: super::super::Foundation::HANDLE, domainname: super::super::Foundation::PWSTR, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn DsGetForestTrustInformationW(servername: super::super::Foundation::PWSTR, trusteddomainname: super::super::Foundation::PWSTR, flags: u32, foresttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetFriendlyClassName(pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn DsGetIcon(dwflags: u32, pszobjectclass: super::super::Foundation::PWSTR, cximage: i32, cyimage: i32) -> super::super::UI::WindowsAndMessaging::HICON;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetRdnW(ppdn: *mut super::super::Foundation::PWSTR, pcdn: *mut u32, ppkey: *mut super::super::Foundation::PWSTR, pckey: *mut u32, ppval: *mut super::super::Foundation::PWSTR, pcval: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSiteNameA(computername: super::super::Foundation::PSTR, sitename: *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSiteNameW(computername: super::super::Foundation::PWSTR, sitename: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSpnA(servicetype: DS_SPN_NAME_TYPE, serviceclass: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, instanceport: u16, cinstancenames: u16, pinstancenames: *const super::super::Foundation::PSTR, pinstanceports: *const u16, pcspn: *mut u32, prpszspn: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSpnW(servicetype: DS_SPN_NAME_TYPE, serviceclass: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, instanceport: u16, cinstancenames: u16, pinstancenames: *const super::super::Foundation::PWSTR, pinstanceports: *const u16, pcspn: *mut u32, prpszspn: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsInheritSecurityIdentityA(hds: super::super::Foundation::HANDLE, flags: u32, srcprincipal: super::super::Foundation::PSTR, dstprincipal: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsInheritSecurityIdentityW(hds: super::super::Foundation::HANDLE, flags: u32, srcprincipal: super::super::Foundation::PWSTR, dstprincipal: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledDnA(pszdn: super::super::Foundation::PSTR, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledDnW(pszdn: super::super::Foundation::PWSTR, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledRdnValueA(pszrdn: super::super::Foundation::PSTR, crdn: u32, edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledRdnValueW(pszrdn: super::super::Foundation::PWSTR, crdn: u32, edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListDomainsInSiteA(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PSTR, ppdomains: *mut *mut DS_NAME_RESULTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListDomainsInSiteW(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PWSTR, ppdomains: *mut *mut DS_NAME_RESULTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListInfoForServerA(hds: super::super::Foundation::HANDLE, server: super::super::Foundation::PSTR, ppinfo: *mut *mut DS_NAME_RESULTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListInfoForServerW(hds: super::super::Foundation::HANDLE, server: super::super::Foundation::PWSTR, ppinfo: *mut *mut DS_NAME_RESULTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListRolesA(hds: super::super::Foundation::HANDLE, pproles: *mut *mut DS_NAME_RESULTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListRolesW(hds: super::super::Foundation::HANDLE, pproles: *mut *mut DS_NAME_RESULTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersForDomainInSiteA(hds: super::super::Foundation::HANDLE, domain: super::super::Foundation::PSTR, site: super::super::Foundation::PSTR, ppservers: *mut *mut DS_NAME_RESULTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersForDomainInSiteW(hds: super::super::Foundation::HANDLE, domain: super::super::Foundation::PWSTR, site: super::super::Foundation::PWSTR, ppservers: *mut *mut DS_NAME_RESULTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersInSiteA(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PSTR, ppservers: *mut *mut DS_NAME_RESULTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersInSiteW(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PWSTR, ppservers: *mut *mut DS_NAME_RESULTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListSitesA(hds: super::super::Foundation::HANDLE, ppsites: *mut *mut DS_NAME_RESULTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListSitesW(hds: super::super::Foundation::HANDLE, ppsites: *mut *mut DS_NAME_RESULTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakePasswordCredentialsA(user: super::super::Foundation::PSTR, domain: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakePasswordCredentialsW(user: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakeSpnA(serviceclass: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, instancename: super::super::Foundation::PSTR, instanceport: u16, referrer: super::super::Foundation::PSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakeSpnW(serviceclass: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, instancename: super::super::Foundation::PWSTR, instanceport: u16, referrer: super::super::Foundation::PWSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMapSchemaGuidsA(hds: super::super::Foundation::HANDLE, cguids: u32, rguids: *const ::windows_sys::core::GUID, ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMapSchemaGuidsW(hds: super::super::Foundation::HANDLE, cguids: u32, rguids: *const ::windows_sys::core::GUID, ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPW) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn DsMergeForestTrustInformationW(domainname: super::super::Foundation::PWSTR, newforesttrustinfo: *const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo: *const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION, mergedforesttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuerySitesByCostA(hds: super::super::Foundation::HANDLE, pszfromsite: super::super::Foundation::PSTR, rgsztosites: *const super::super::Foundation::PSTR, ctosites: u32, dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuerySitesByCostW(hds: super::super::Foundation::HANDLE, pwszfromsite: super::super::Foundation::PWSTR, rgwsztosites: *const super::super::Foundation::PWSTR, ctosites: u32, dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32;
    pub fn DsQuerySitesFree(rgsiteinfo: *const DS_SITE_COST_INFO);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuoteRdnValueA(cunquotedrdnvaluelength: u32, psunquotedrdnvalue: super::super::Foundation::PSTR, pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuoteRdnValueW(cunquotedrdnvaluelength: u32, psunquotedrdnvalue: super::super::Foundation::PWSTR, pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsDomainA(hds: super::super::Foundation::HANDLE, domaindn: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsDomainW(hds: super::super::Foundation::HANDLE, domaindn: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsServerA(hds: super::super::Foundation::HANDLE, serverdn: super::super::Foundation::PSTR, domaindn: super::super::Foundation::PSTR, flastdcindomain: *mut super::super::Foundation::BOOL, fcommit: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsServerW(hds: super::super::Foundation::HANDLE, serverdn: super::super::Foundation::PWSTR, domaindn: super::super::Foundation::PWSTR, flastdcindomain: *mut super::super::Foundation::BOOL, fcommit: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaAddA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, sourcedsadn: super::super::Foundation::PSTR, transportdn: super::super::Foundation::PSTR, sourcedsaaddress: super::super::Foundation::PSTR, pschedule: *const SCHEDULE, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaAddW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, sourcedsadn: super::super::Foundation::PWSTR, transportdn: super::super::Foundation::PWSTR, sourcedsaaddress: super::super::Foundation::PWSTR, pschedule: *const SCHEDULE, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaConsistencyCheck(hds: super::super::Foundation::HANDLE, taskid: DS_KCC_TASKID, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaDelA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, dsasrc: super::super::Foundation::PSTR, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaDelW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, dsasrc: super::super::Foundation::PWSTR, options: u32) -> u32;
    pub fn DsReplicaFreeInfo(infotype: DS_REPL_INFO_TYPE, pinfo: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaGetInfo2W(hds: super::super::Foundation::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: super::super::Foundation::PWSTR, puuidforsourcedsaobjguid: *const ::windows_sys::core::GUID, pszattributename: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, dwflags: u32, dwenumerationcontext: u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaGetInfoW(hds: super::super::Foundation::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: super::super::Foundation::PWSTR, puuidforsourcedsaobjguid: *const ::windows_sys::core::GUID, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuidsourcedsa: *const ::windows_sys::core::GUID, transportdn: super::super::Foundation::PSTR, sourcedsaaddress: super::super::Foundation::PSTR, pschedule: *const SCHEDULE, replicaflags: u32, modifyfields: u32, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuidsourcedsa: *const ::windows_sys::core::GUID, transportdn: super::super::Foundation::PWSTR, sourcedsaaddress: super::super::Foundation::PWSTR, pschedule: *const SCHEDULE, replicaflags: u32, modifyfields: u32, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuiddsasrc: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllA(hds: super::super::Foundation::HANDLE, psznamecontext: super::super::Foundation::PSTR, ulflags: u32, pfncallback: isize, pcallbackdata: *const ::core::ffi::c_void, perrors: *mut *mut *mut DS_REPSYNCALL_ERRINFOA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllW(hds: super::super::Foundation::HANDLE, psznamecontext: super::super::Foundation::PWSTR, ulflags: u32, pfncallback: isize, pcallbackdata: *const ::core::ffi::c_void, perrors: *mut *mut *mut DS_REPSYNCALL_ERRINFOW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuiddsasrc: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, dsadest: super::super::Foundation::PSTR, puuiddsadest: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, dsadest: super::super::Foundation::PWSTR, puuiddsadest: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuiddsasrc: *const ::windows_sys::core::GUID, uloptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuiddsasrc: *const ::windows_sys::core::GUID, uloptions: u32) -> u32;
    pub fn DsRoleFreeMemory(buffer: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRoleGetPrimaryDomainInformation(lpserver: super::super::Foundation::PWSTR, infolevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsServerRegisterSpnA(operation: DS_SPN_WRITE_OP, serviceclass: super::super::Foundation::PSTR, userobjectdn: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsServerRegisterSpnW(operation: DS_SPN_WRITE_OP, serviceclass: super::super::Foundation::PWSTR, userobjectdn: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnBindA(phds: *const super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnBindW(phds: *const super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnquoteRdnValueA(cquotedrdnvaluelength: u32, psquotedrdnvalue: super::super::Foundation::PSTR, pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnquoteRdnValueW(cquotedrdnvaluelength: u32, psquotedrdnvalue: super::super::Foundation::PWSTR, pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsValidateSubnetNameA(subnetname: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsValidateSubnetNameW(subnetname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsWriteAccountSpnA(hds: super::super::Foundation::HANDLE, operation: DS_SPN_WRITE_OP, pszaccount: super::super::Foundation::PSTR, cspn: u32, rpszspn: *const super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsWriteAccountSpnW(hds: super::super::Foundation::HANDLE, operation: DS_SPN_WRITE_OP, pszaccount: super::super::Foundation::PWSTR, cspn: u32, rpszspn: *const super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeADsMem(pmem: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeADsStr(pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn PropVariantToAdsType(pvariant: *mut super::super::System::Com::VARIANT, dwnumvariant: u32, ppadsvalues: *mut *mut ADSVALUE, pdwnumvalues: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn ReallocADsMem(poldmem: *mut ::core::ffi::c_void, cbold: u32, cbnew: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReallocADsStr(ppstr: *mut super::super::Foundation::PWSTR, pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn SecurityDescriptorToBinarySD(vvarsecdes: super::super::System::Com::VARIANT, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, pdwsdlength: *mut u32, pszservername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
}
pub const ACTRL_DS_CONTROL_ACCESS: u32 = 256u32;
pub const ACTRL_DS_CREATE_CHILD: u32 = 1u32;
pub const ACTRL_DS_DELETE_CHILD: u32 = 2u32;
pub const ACTRL_DS_DELETE_TREE: u32 = 64u32;
pub const ACTRL_DS_LIST: u32 = 4u32;
pub const ACTRL_DS_LIST_OBJECT: u32 = 128u32;
pub const ACTRL_DS_OPEN: u32 = 0u32;
pub const ACTRL_DS_READ_PROP: u32 = 16u32;
pub const ACTRL_DS_SELF: u32 = 8u32;
pub const ACTRL_DS_WRITE_PROP: u32 = 32u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED: u32 = 2u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE: u32 = 1u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH: u32 = 0u32;
#[repr(transparent)]
pub struct ADSI_DIALECT_ENUM(pub i32);
pub const ADSI_DIALECT_LDAP: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(0i32);
pub const ADSI_DIALECT_SQL: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADSPROPERROR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADSPROPINITPARAMS(i32);
#[repr(transparent)]
pub struct ADSTYPEENUM(pub i32);
pub const ADSTYPE_INVALID: ADSTYPEENUM = ADSTYPEENUM(0i32);
pub const ADSTYPE_DN_STRING: ADSTYPEENUM = ADSTYPEENUM(1i32);
pub const ADSTYPE_CASE_EXACT_STRING: ADSTYPEENUM = ADSTYPEENUM(2i32);
pub const ADSTYPE_CASE_IGNORE_STRING: ADSTYPEENUM = ADSTYPEENUM(3i32);
pub const ADSTYPE_PRINTABLE_STRING: ADSTYPEENUM = ADSTYPEENUM(4i32);
pub const ADSTYPE_NUMERIC_STRING: ADSTYPEENUM = ADSTYPEENUM(5i32);
pub const ADSTYPE_BOOLEAN: ADSTYPEENUM = ADSTYPEENUM(6i32);
pub const ADSTYPE_INTEGER: ADSTYPEENUM = ADSTYPEENUM(7i32);
pub const ADSTYPE_OCTET_STRING: ADSTYPEENUM = ADSTYPEENUM(8i32);
pub const ADSTYPE_UTC_TIME: ADSTYPEENUM = ADSTYPEENUM(9i32);
pub const ADSTYPE_LARGE_INTEGER: ADSTYPEENUM = ADSTYPEENUM(10i32);
pub const ADSTYPE_PROV_SPECIFIC: ADSTYPEENUM = ADSTYPEENUM(11i32);
pub const ADSTYPE_OBJECT_CLASS: ADSTYPEENUM = ADSTYPEENUM(12i32);
pub const ADSTYPE_CASEIGNORE_LIST: ADSTYPEENUM = ADSTYPEENUM(13i32);
pub const ADSTYPE_OCTET_LIST: ADSTYPEENUM = ADSTYPEENUM(14i32);
pub const ADSTYPE_PATH: ADSTYPEENUM = ADSTYPEENUM(15i32);
pub const ADSTYPE_POSTALADDRESS: ADSTYPEENUM = ADSTYPEENUM(16i32);
pub const ADSTYPE_TIMESTAMP: ADSTYPEENUM = ADSTYPEENUM(17i32);
pub const ADSTYPE_BACKLINK: ADSTYPEENUM = ADSTYPEENUM(18i32);
pub const ADSTYPE_TYPEDNAME: ADSTYPEENUM = ADSTYPEENUM(19i32);
pub const ADSTYPE_HOLD: ADSTYPEENUM = ADSTYPEENUM(20i32);
pub const ADSTYPE_NETADDRESS: ADSTYPEENUM = ADSTYPEENUM(21i32);
pub const ADSTYPE_REPLICAPOINTER: ADSTYPEENUM = ADSTYPEENUM(22i32);
pub const ADSTYPE_FAXNUMBER: ADSTYPEENUM = ADSTYPEENUM(23i32);
pub const ADSTYPE_EMAIL: ADSTYPEENUM = ADSTYPEENUM(24i32);
pub const ADSTYPE_NT_SECURITY_DESCRIPTOR: ADSTYPEENUM = ADSTYPEENUM(25i32);
pub const ADSTYPE_UNKNOWN: ADSTYPEENUM = ADSTYPEENUM(26i32);
pub const ADSTYPE_DN_WITH_BINARY: ADSTYPEENUM = ADSTYPEENUM(27i32);
pub const ADSTYPE_DN_WITH_STRING: ADSTYPEENUM = ADSTYPEENUM(28i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADSVALUE(i32);
#[repr(transparent)]
pub struct ADS_ACEFLAG_ENUM(pub i32);
pub const ADS_ACEFLAG_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(2i32);
pub const ADS_ACEFLAG_NO_PROPAGATE_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(4i32);
pub const ADS_ACEFLAG_INHERIT_ONLY_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(8i32);
pub const ADS_ACEFLAG_INHERITED_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(16i32);
pub const ADS_ACEFLAG_VALID_INHERIT_FLAGS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(31i32);
pub const ADS_ACEFLAG_SUCCESSFUL_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(64i32);
pub const ADS_ACEFLAG_FAILED_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(128i32);
#[repr(transparent)]
pub struct ADS_ACETYPE_ENUM(pub i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(0i32);
pub const ADS_ACETYPE_ACCESS_DENIED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(1i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(2i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(5i32);
pub const ADS_ACETYPE_ACCESS_DENIED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(6i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(7i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(8i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(9i32);
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(10i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(11i32);
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(12i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(13i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(14i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(15i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(16i32);
pub const ADS_ATTR_APPEND: u32 = 3u32;
pub const ADS_ATTR_CLEAR: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_ATTR_DEF(i32);
pub const ADS_ATTR_DELETE: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_ATTR_INFO(i32);
pub const ADS_ATTR_UPDATE: u32 = 2u32;
#[repr(transparent)]
pub struct ADS_AUTHENTICATION_ENUM(pub u32);
pub const ADS_SECURE_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1u32);
pub const ADS_USE_ENCRYPTION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
pub const ADS_USE_SSL: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
pub const ADS_READONLY_SERVER: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(4u32);
pub const ADS_PROMPT_CREDENTIALS: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(8u32);
pub const ADS_NO_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(16u32);
pub const ADS_FAST_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(32u32);
pub const ADS_USE_SIGNING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(64u32);
pub const ADS_USE_SEALING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(128u32);
pub const ADS_USE_DELEGATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(256u32);
pub const ADS_SERVER_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(512u32);
pub const ADS_NO_REFERRAL_CHASING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1024u32);
pub const ADS_AUTH_RESERVED: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2147483648u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_BACKLINK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_CASEIGNORE_LIST(i32);
#[repr(transparent)]
pub struct ADS_CHASE_REFERRALS_ENUM(pub i32);
pub const ADS_CHASE_REFERRALS_NEVER: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(0i32);
pub const ADS_CHASE_REFERRALS_SUBORDINATE: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(32i32);
pub const ADS_CHASE_REFERRALS_EXTERNAL: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(64i32);
pub const ADS_CHASE_REFERRALS_ALWAYS: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(96i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_CLASS_DEF(i32);
#[repr(transparent)]
pub struct ADS_DEREFENUM(pub i32);
pub const ADS_DEREF_NEVER: ADS_DEREFENUM = ADS_DEREFENUM(0i32);
pub const ADS_DEREF_SEARCHING: ADS_DEREFENUM = ADS_DEREFENUM(1i32);
pub const ADS_DEREF_FINDING: ADS_DEREFENUM = ADS_DEREFENUM(2i32);
pub const ADS_DEREF_ALWAYS: ADS_DEREFENUM = ADS_DEREFENUM(3i32);
#[repr(transparent)]
pub struct ADS_DISPLAY_ENUM(pub i32);
pub const ADS_DISPLAY_FULL: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(1i32);
pub const ADS_DISPLAY_VALUE_ONLY: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_DN_WITH_BINARY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_DN_WITH_STRING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_EMAIL(i32);
#[repr(transparent)]
pub struct ADS_ESCAPE_MODE_ENUM(pub i32);
pub const ADS_ESCAPEDMODE_DEFAULT: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(1i32);
pub const ADS_ESCAPEDMODE_ON: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(2i32);
pub const ADS_ESCAPEDMODE_OFF: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(3i32);
pub const ADS_ESCAPEDMODE_OFF_EX: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(4i32);
pub const ADS_EXT_INITCREDENTIALS: u32 = 1u32;
pub const ADS_EXT_INITIALIZE_COMPLETE: u32 = 2u32;
pub const ADS_EXT_MAXEXTDISPID: u32 = 16777215u32;
pub const ADS_EXT_MINEXTDISPID: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_FAXNUMBER(i32);
#[repr(transparent)]
pub struct ADS_FLAGTYPE_ENUM(pub i32);
pub const ADS_FLAG_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(1i32);
pub const ADS_FLAG_INHERITED_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(2i32);
#[repr(transparent)]
pub struct ADS_FORMAT_ENUM(pub i32);
pub const ADS_FORMAT_WINDOWS: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(1i32);
pub const ADS_FORMAT_WINDOWS_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(2i32);
pub const ADS_FORMAT_WINDOWS_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(3i32);
pub const ADS_FORMAT_WINDOWS_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(4i32);
pub const ADS_FORMAT_X500: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(5i32);
pub const ADS_FORMAT_X500_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(6i32);
pub const ADS_FORMAT_X500_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(7i32);
pub const ADS_FORMAT_X500_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(8i32);
pub const ADS_FORMAT_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(9i32);
pub const ADS_FORMAT_PROVIDER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(10i32);
pub const ADS_FORMAT_LEAF: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(11i32);
#[repr(transparent)]
pub struct ADS_GROUP_TYPE_ENUM(pub i32);
pub const ADS_GROUP_TYPE_GLOBAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(2i32);
pub const ADS_GROUP_TYPE_DOMAIN_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
pub const ADS_GROUP_TYPE_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
pub const ADS_GROUP_TYPE_UNIVERSAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(8i32);
pub const ADS_GROUP_TYPE_SECURITY_ENABLED: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(-2147483648i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_HOLD(i32);
#[repr(transparent)]
pub struct ADS_NAME_INITTYPE_ENUM(pub i32);
pub const ADS_NAME_INITTYPE_DOMAIN: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(1i32);
pub const ADS_NAME_INITTYPE_SERVER: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(2i32);
pub const ADS_NAME_INITTYPE_GC: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(3i32);
#[repr(transparent)]
pub struct ADS_NAME_TYPE_ENUM(pub i32);
pub const ADS_NAME_TYPE_1779: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(1i32);
pub const ADS_NAME_TYPE_CANONICAL: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(2i32);
pub const ADS_NAME_TYPE_NT4: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(3i32);
pub const ADS_NAME_TYPE_DISPLAY: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(4i32);
pub const ADS_NAME_TYPE_DOMAIN_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(5i32);
pub const ADS_NAME_TYPE_ENTERPRISE_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(6i32);
pub const ADS_NAME_TYPE_GUID: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(7i32);
pub const ADS_NAME_TYPE_UNKNOWN: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(8i32);
pub const ADS_NAME_TYPE_USER_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(9i32);
pub const ADS_NAME_TYPE_CANONICAL_EX: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(10i32);
pub const ADS_NAME_TYPE_SERVICE_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(11i32);
pub const ADS_NAME_TYPE_SID_OR_SID_HISTORY_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(12i32);
#[repr(C)]
pub struct ADS_NETADDRESS(i32);
#[repr(C)]
pub struct ADS_NT_SECURITY_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_OBJECT_INFO(i32);
#[repr(C)]
pub struct ADS_OCTET_LIST(i32);
#[repr(C)]
pub struct ADS_OCTET_STRING(i32);
#[repr(transparent)]
pub struct ADS_OPTION_ENUM(pub i32);
pub const ADS_OPTION_SERVERNAME: ADS_OPTION_ENUM = ADS_OPTION_ENUM(0i32);
pub const ADS_OPTION_REFERRALS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(1i32);
pub const ADS_OPTION_PAGE_SIZE: ADS_OPTION_ENUM = ADS_OPTION_ENUM(2i32);
pub const ADS_OPTION_SECURITY_MASK: ADS_OPTION_ENUM = ADS_OPTION_ENUM(3i32);
pub const ADS_OPTION_MUTUAL_AUTH_STATUS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(4i32);
pub const ADS_OPTION_QUOTA: ADS_OPTION_ENUM = ADS_OPTION_ENUM(5i32);
pub const ADS_OPTION_PASSWORD_PORTNUMBER: ADS_OPTION_ENUM = ADS_OPTION_ENUM(6i32);
pub const ADS_OPTION_PASSWORD_METHOD: ADS_OPTION_ENUM = ADS_OPTION_ENUM(7i32);
pub const ADS_OPTION_ACCUMULATIVE_MODIFICATION: ADS_OPTION_ENUM = ADS_OPTION_ENUM(8i32);
pub const ADS_OPTION_SKIP_SID_LOOKUP: ADS_OPTION_ENUM = ADS_OPTION_ENUM(9i32);
#[repr(transparent)]
pub struct ADS_PASSWORD_ENCODING_ENUM(pub i32);
pub const ADS_PASSWORD_ENCODE_REQUIRE_SSL: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(0i32);
pub const ADS_PASSWORD_ENCODE_CLEAR: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_PATH(i32);
#[repr(transparent)]
pub struct ADS_PATHTYPE_ENUM(pub i32);
pub const ADS_PATH_FILE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(1i32);
pub const ADS_PATH_FILESHARE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(2i32);
pub const ADS_PATH_REGISTRY: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_POSTALADDRESS(i32);
#[repr(transparent)]
pub struct ADS_PREFERENCES_ENUM(pub i32);
pub const ADSIPROP_ASYNCHRONOUS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(0i32);
pub const ADSIPROP_DEREF_ALIASES: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(1i32);
pub const ADSIPROP_SIZE_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(2i32);
pub const ADSIPROP_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(3i32);
pub const ADSIPROP_ATTRIBTYPES_ONLY: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(4i32);
pub const ADSIPROP_SEARCH_SCOPE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(5i32);
pub const ADSIPROP_TIMEOUT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(6i32);
pub const ADSIPROP_PAGESIZE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(7i32);
pub const ADSIPROP_PAGED_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(8i32);
pub const ADSIPROP_CHASE_REFERRALS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(9i32);
pub const ADSIPROP_SORT_ON: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(10i32);
pub const ADSIPROP_CACHE_RESULTS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(11i32);
pub const ADSIPROP_ADSIFLAG: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(12i32);
#[repr(transparent)]
pub struct ADS_PROPERTY_OPERATION_ENUM(pub i32);
pub const ADS_PROPERTY_CLEAR: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(1i32);
pub const ADS_PROPERTY_UPDATE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(2i32);
pub const ADS_PROPERTY_APPEND: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(3i32);
pub const ADS_PROPERTY_DELETE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(4i32);
#[repr(C)]
pub struct ADS_PROV_SPECIFIC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_REPLICAPOINTER(i32);
#[repr(transparent)]
pub struct ADS_RIGHTS_ENUM(pub i32);
pub const ADS_RIGHT_DELETE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(65536i32);
pub const ADS_RIGHT_READ_CONTROL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(131072i32);
pub const ADS_RIGHT_WRITE_DAC: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(262144i32);
pub const ADS_RIGHT_WRITE_OWNER: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(524288i32);
pub const ADS_RIGHT_SYNCHRONIZE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1048576i32);
pub const ADS_RIGHT_ACCESS_SYSTEM_SECURITY: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16777216i32);
pub const ADS_RIGHT_GENERIC_READ: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(-2147483648i32);
pub const ADS_RIGHT_GENERIC_WRITE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1073741824i32);
pub const ADS_RIGHT_GENERIC_EXECUTE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(536870912i32);
pub const ADS_RIGHT_GENERIC_ALL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(268435456i32);
pub const ADS_RIGHT_DS_CREATE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1i32);
pub const ADS_RIGHT_DS_DELETE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(2i32);
pub const ADS_RIGHT_ACTRL_DS_LIST: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(4i32);
pub const ADS_RIGHT_DS_SELF: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(8i32);
pub const ADS_RIGHT_DS_READ_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16i32);
pub const ADS_RIGHT_DS_WRITE_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(32i32);
pub const ADS_RIGHT_DS_DELETE_TREE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(64i32);
pub const ADS_RIGHT_DS_LIST_OBJECT: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(128i32);
pub const ADS_RIGHT_DS_CONTROL_ACCESS: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(256i32);
#[repr(transparent)]
pub struct ADS_SCOPEENUM(pub i32);
pub const ADS_SCOPE_BASE: ADS_SCOPEENUM = ADS_SCOPEENUM(0i32);
pub const ADS_SCOPE_ONELEVEL: ADS_SCOPEENUM = ADS_SCOPEENUM(1i32);
pub const ADS_SCOPE_SUBTREE: ADS_SCOPEENUM = ADS_SCOPEENUM(2i32);
#[repr(transparent)]
pub struct ADS_SD_CONTROL_ENUM(pub i32);
pub const ADS_SD_CONTROL_SE_OWNER_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1i32);
pub const ADS_SD_CONTROL_SE_GROUP_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2i32);
pub const ADS_SD_CONTROL_SE_DACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4i32);
pub const ADS_SD_CONTROL_SE_DACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8i32);
pub const ADS_SD_CONTROL_SE_SACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(16i32);
pub const ADS_SD_CONTROL_SE_SACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32i32);
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(256i32);
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(512i32);
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1024i32);
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2048i32);
pub const ADS_SD_CONTROL_SE_DACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4096i32);
pub const ADS_SD_CONTROL_SE_SACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8192i32);
pub const ADS_SD_CONTROL_SE_SELF_RELATIVE: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32768i32);
#[repr(transparent)]
pub struct ADS_SD_FORMAT_ENUM(pub i32);
pub const ADS_SD_FORMAT_IID: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(1i32);
pub const ADS_SD_FORMAT_RAW: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(2i32);
pub const ADS_SD_FORMAT_HEXSTRING: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(3i32);
#[repr(transparent)]
pub struct ADS_SD_REVISION_ENUM(pub i32);
pub const ADS_SD_REVISION_DS: ADS_SD_REVISION_ENUM = ADS_SD_REVISION_ENUM(4i32);
#[repr(transparent)]
pub struct ADS_SEARCHPREF_ENUM(pub i32);
pub const ADS_SEARCHPREF_ASYNCHRONOUS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(0i32);
pub const ADS_SEARCHPREF_DEREF_ALIASES: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(1i32);
pub const ADS_SEARCHPREF_SIZE_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(2i32);
pub const ADS_SEARCHPREF_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(3i32);
pub const ADS_SEARCHPREF_ATTRIBTYPES_ONLY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(4i32);
pub const ADS_SEARCHPREF_SEARCH_SCOPE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(5i32);
pub const ADS_SEARCHPREF_TIMEOUT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(6i32);
pub const ADS_SEARCHPREF_PAGESIZE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(7i32);
pub const ADS_SEARCHPREF_PAGED_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(8i32);
pub const ADS_SEARCHPREF_CHASE_REFERRALS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(9i32);
pub const ADS_SEARCHPREF_SORT_ON: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(10i32);
pub const ADS_SEARCHPREF_CACHE_RESULTS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(11i32);
pub const ADS_SEARCHPREF_DIRSYNC: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(12i32);
pub const ADS_SEARCHPREF_TOMBSTONE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(13i32);
pub const ADS_SEARCHPREF_VLV: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(14i32);
pub const ADS_SEARCHPREF_ATTRIBUTE_QUERY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(15i32);
pub const ADS_SEARCHPREF_SECURITY_MASK: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(16i32);
pub const ADS_SEARCHPREF_DIRSYNC_FLAG: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(17i32);
pub const ADS_SEARCHPREF_EXTENDED_DN: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(18i32);
#[repr(transparent)]
pub struct ADS_SECURITY_INFO_ENUM(pub i32);
pub const ADS_SECURITY_INFO_OWNER: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(1i32);
pub const ADS_SECURITY_INFO_GROUP: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(2i32);
pub const ADS_SECURITY_INFO_DACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(4i32);
pub const ADS_SECURITY_INFO_SACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(8i32);
#[repr(transparent)]
pub struct ADS_SETTYPE_ENUM(pub i32);
pub const ADS_SETTYPE_FULL: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(1i32);
pub const ADS_SETTYPE_PROVIDER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(2i32);
pub const ADS_SETTYPE_SERVER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(3i32);
pub const ADS_SETTYPE_DN: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_SORTKEY(i32);
#[repr(transparent)]
pub struct ADS_STATUSENUM(pub i32);
pub const ADS_STATUS_S_OK: ADS_STATUSENUM = ADS_STATUSENUM(0i32);
pub const ADS_STATUS_INVALID_SEARCHPREF: ADS_STATUSENUM = ADS_STATUSENUM(1i32);
pub const ADS_STATUS_INVALID_SEARCHPREFVALUE: ADS_STATUSENUM = ADS_STATUSENUM(2i32);
#[repr(transparent)]
pub struct ADS_SYSTEMFLAG_ENUM(pub i32);
pub const ADS_SYSTEMFLAG_DISALLOW_DELETE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(-2147483648i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1073741824i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(536870912i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_LIMITED_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(268435456i32);
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(134217728i32);
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(67108864i32);
pub const ADS_SYSTEMFLAG_CR_NTDS_NC: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
pub const ADS_SYSTEMFLAG_CR_NTDS_DOMAIN: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(2i32);
pub const ADS_SYSTEMFLAG_ATTR_NOT_REPLICATED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
pub const ADS_SYSTEMFLAG_ATTR_IS_CONSTRUCTED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(4i32);
#[repr(C)]
pub struct ADS_TIMESTAMP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_TYPEDNAME(i32);
#[repr(transparent)]
pub struct ADS_USER_FLAG_ENUM(pub i32);
pub const ADS_UF_SCRIPT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1i32);
pub const ADS_UF_ACCOUNTDISABLE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2i32);
pub const ADS_UF_HOMEDIR_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8i32);
pub const ADS_UF_LOCKOUT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16i32);
pub const ADS_UF_PASSWD_NOTREQD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(32i32);
pub const ADS_UF_PASSWD_CANT_CHANGE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(64i32);
pub const ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(128i32);
pub const ADS_UF_TEMP_DUPLICATE_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(256i32);
pub const ADS_UF_NORMAL_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(512i32);
pub const ADS_UF_INTERDOMAIN_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2048i32);
pub const ADS_UF_WORKSTATION_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4096i32);
pub const ADS_UF_SERVER_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8192i32);
pub const ADS_UF_DONT_EXPIRE_PASSWD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(65536i32);
pub const ADS_UF_MNS_LOGON_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(131072i32);
pub const ADS_UF_SMARTCARD_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(262144i32);
pub const ADS_UF_TRUSTED_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(524288i32);
pub const ADS_UF_NOT_DELEGATED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1048576i32);
pub const ADS_UF_USE_DES_KEY_ONLY: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2097152i32);
pub const ADS_UF_DONT_REQUIRE_PREAUTH: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4194304i32);
pub const ADS_UF_PASSWORD_EXPIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8388608i32);
pub const ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16777216i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADS_VLV(i32);
#[repr(C)]
pub struct ADSystemInfo(i32);
#[repr(C)]
pub struct ADsSecurityUtility(i32);
#[repr(C)]
pub struct AccessControlEntry(i32);
#[repr(C)]
pub struct AccessControlList(i32);
#[repr(C)]
pub struct BackLink(i32);
pub const CLSID_CommonQuery: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2210160320, data2: 28458, data3: 4560, data4: [161, 196, 0, 170, 0, 193, 110, 101] };
pub const CLSID_DsAdminCreateObj: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3808534537, data2: 63745, data3: 4562, data4: [130, 185, 0, 192, 79, 104, 146, 139] };
pub const CLSID_DsDisplaySpecifier: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 448047296, data2: 27147, data3: 4562, data4: [173, 73, 0, 192, 79, 163, 26, 134] };
pub const CLSID_DsDomainTreeBrowser: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 379091210, data2: 58036, data3: 4560, data4: [176, 177, 0, 192, 79, 216, 220, 166] };
pub const CLSID_DsFindAdvanced: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2213429219, data2: 22489, data3: 4560, data4: [185, 50, 0, 160, 36, 171, 45, 187] };
pub const CLSID_DsFindComputer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 369125120, data2: 34733, data3: 4560, data4: [145, 64, 0, 170, 0, 193, 110, 101] };
pub const CLSID_DsFindContainer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3249785842, data2: 34922, data3: 4560, data4: [145, 64, 0, 170, 0, 193, 110, 101] };
pub const CLSID_DsFindDomainController: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1401715582, data2: 53854, data3: 4560, data4: [151, 66, 0, 160, 201, 6, 175, 69] };
pub const CLSID_DsFindFrsMembers: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2496547608, data2: 46035, data3: 4561, data4: [185, 180, 0, 192, 79, 216, 213, 176] };
pub const CLSID_DsFindObjects: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2213429217, data2: 22489, data3: 4560, data4: [185, 50, 0, 160, 36, 171, 45, 187] };
pub const CLSID_DsFindPeople: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2213429218, data2: 22489, data3: 4560, data4: [185, 50, 0, 160, 36, 171, 45, 187] };
pub const CLSID_DsFindPrinter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3044536432, data2: 32482, data3: 4560, data4: [145, 63, 0, 170, 0, 193, 110, 101] };
pub const CLSID_DsFindVolume: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3249785841, data2: 34922, data3: 4560, data4: [145, 64, 0, 170, 0, 193, 110, 101] };
pub const CLSID_DsFindWriteableDomainController: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2092888185,
    data2: 43652,
    data3: 17483,
    data4: [188, 112, 104, 228, 18, 131, 234, 188],
};
pub const CLSID_DsFolderProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2656166096, data2: 28175, data3: 4562, data4: [150, 1, 0, 192, 79, 163, 26, 134] };
pub const CLSID_DsObjectPicker: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 399953112, data2: 15227, data3: 4562, data4: [185, 224, 0, 192, 79, 216, 219, 247] };
pub const CLSID_DsPropertyPages: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 222680368, data2: 30283, data3: 4560, data4: [161, 202, 0, 170, 0, 193, 110, 101] };
pub const CLSID_DsQuery: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2317608542, data2: 12738, data3: 4560, data4: [137, 28, 0, 160, 36, 171, 45, 187] };
pub const CLSID_MicrosoftDS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4262629616, data2: 53181, data3: 4559, data4: [163, 48, 0, 170, 0, 193, 110, 101] };
pub const CQFF_ISOPTIONAL: u32 = 2u32;
pub const CQFF_NOGLOBALPAGES: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct CQFORM(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct CQPAGE(i32);
pub const CQPM_CLEARFORM: u32 = 6u32;
pub const CQPM_ENABLE: u32 = 3u32;
pub const CQPM_GETPARAMETERS: u32 = 5u32;
pub const CQPM_HANDLERSPECIFIC: u32 = 268435456u32;
pub const CQPM_HELP: u32 = 8u32;
pub const CQPM_INITIALIZE: u32 = 1u32;
pub const CQPM_PERSIST: u32 = 7u32;
pub const CQPM_RELEASE: u32 = 2u32;
pub const CQPM_SETDEFAULTPARAMETERS: u32 = 9u32;
#[repr(C)]
pub struct CaseIgnoreList(i32);
pub const DBDTF_RETURNEXTERNAL: u32 = 4u32;
pub const DBDTF_RETURNFQDN: u32 = 1u32;
pub const DBDTF_RETURNINBOUND: u32 = 8u32;
pub const DBDTF_RETURNINOUTBOUND: u32 = 16u32;
pub const DBDTF_RETURNMIXEDDOMAINS: u32 = 2u32;
#[repr(C)]
pub struct DNWithBinary(i32);
#[repr(C)]
pub struct DNWithString(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOMAINDESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOMAIN_CONTROLLER_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOMAIN_CONTROLLER_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOMAIN_TREE(i32);
pub const DSA_NEWOBJ_CTX_CLEANUP: u32 = 4u32;
pub const DSA_NEWOBJ_CTX_COMMIT: u32 = 2u32;
pub const DSA_NEWOBJ_CTX_POSTCOMMIT: u32 = 3u32;
pub const DSA_NEWOBJ_CTX_PRECOMMIT: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct DSA_NEWOBJ_DISPINFO(i32);
pub const DSA_NOTIFY_DEL: u32 = 1u32;
pub const DSA_NOTIFY_FLAG_ADDITIONAL_DATA: u32 = 2u32;
pub const DSA_NOTIFY_FLAG_FORCE_ADDITIONAL_DATA: u32 = 1u32;
pub const DSA_NOTIFY_MOV: u32 = 4u32;
pub const DSA_NOTIFY_PROP: u32 = 8u32;
pub const DSA_NOTIFY_REN: u32 = 2u32;
pub const DSBF_DISPLAYNAME: u32 = 4u32;
pub const DSBF_ICONLOCATION: u32 = 2u32;
pub const DSBF_STATE: u32 = 1u32;
pub const DSBID_BANNER: u32 = 256u32;
pub const DSBID_CONTAINERLIST: u32 = 257u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSBITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSBITEMW(i32);
pub const DSBI_CHECKBOXES: u32 = 256u32;
pub const DSBI_DONTSIGNSEAL: u32 = 33554432u32;
pub const DSBI_ENTIREDIRECTORY: u32 = 589824u32;
pub const DSBI_EXPANDONOPEN: u32 = 262144u32;
pub const DSBI_HASCREDENTIALS: u32 = 2097152u32;
pub const DSBI_IGNORETREATASLEAF: u32 = 4194304u32;
pub const DSBI_INCLUDEHIDDEN: u32 = 131072u32;
pub const DSBI_NOBUTTONS: u32 = 1u32;
pub const DSBI_NOLINES: u32 = 2u32;
pub const DSBI_NOLINESATROOT: u32 = 4u32;
pub const DSBI_NOROOT: u32 = 65536u32;
pub const DSBI_RETURNOBJECTCLASS: u32 = 16777216u32;
pub const DSBI_RETURN_FORMAT: u32 = 1048576u32;
pub const DSBI_SIMPLEAUTHENTICATE: u32 = 8388608u32;
pub const DSBM_CHANGEIMAGESTATE: u32 = 102u32;
pub const DSBM_CONTEXTMENU: u32 = 104u32;
pub const DSBM_HELP: u32 = 103u32;
pub const DSBM_QUERYINSERT: u32 = 100u32;
pub const DSBM_QUERYINSERTA: u32 = 101u32;
pub const DSBM_QUERYINSERTW: u32 = 100u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[repr(C)]
pub struct DSBROWSEINFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[repr(C)]
pub struct DSBROWSEINFOW(i32);
pub const DSBS_CHECKED: u32 = 1u32;
pub const DSBS_HIDDEN: u32 = 2u32;
pub const DSBS_ROOT: u32 = 4u32;
pub const DSB_MAX_DISPLAYNAME_CHARS: u32 = 64u32;
pub const DSCCIF_HASWIZARDDIALOG: u32 = 1u32;
pub const DSCCIF_HASWIZARDPRIMARYPAGE: u32 = 2u32;
#[repr(C)]
pub struct DSCLASSCREATIONINFO(i32);
#[repr(C)]
pub struct DSCOLUMN(i32);
#[repr(C)]
pub struct DSDISPLAYSPECOPTIONS(i32);
pub const DSDSOF_DONTSIGNSEAL: u32 = 4u32;
pub const DSDSOF_DSAVAILABLE: u32 = 1073741824u32;
pub const DSDSOF_HASUSERANDSERVERINFO: u32 = 1u32;
pub const DSDSOF_SIMPLEAUTHENTICATE: u32 = 2u32;
pub const DSECAF_NOTLISTED: u32 = 1u32;
pub const DSGIF_DEFAULTISCONTAINER: u32 = 32u32;
pub const DSGIF_GETDEFAULTICON: u32 = 16u32;
pub const DSGIF_ISDISABLED: u32 = 2u32;
pub const DSGIF_ISMASK: u32 = 15u32;
pub const DSGIF_ISNORMAL: u32 = 0u32;
pub const DSGIF_ISOPEN: u32 = 1u32;
pub const DSICCF_IGNORETREATASLEAF: u32 = 1u32;
#[repr(C)]
pub struct DSOBJECT(i32);
#[repr(C)]
pub struct DSOBJECTNAMES(i32);
pub const DSOBJECT_ISCONTAINER: u32 = 1u32;
pub const DSOBJECT_READONLYPAGES: u32 = 2147483648u32;
pub const DSOP_DOWNLEVEL_FILTER_ALL_APP_PACKAGES: u32 = 2281701376u32;
pub const DSOP_DOWNLEVEL_FILTER_ALL_WELLKNOWN_SIDS: u32 = 2147614720u32;
pub const DSOP_DOWNLEVEL_FILTER_ANONYMOUS: u32 = 2147483712u32;
pub const DSOP_DOWNLEVEL_FILTER_AUTHENTICATED_USER: u32 = 2147483680u32;
pub const DSOP_DOWNLEVEL_FILTER_BATCH: u32 = 2147483776u32;
pub const DSOP_DOWNLEVEL_FILTER_COMPUTERS: u32 = 2147483656u32;
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_GROUP: u32 = 2147484160u32;
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_OWNER: u32 = 2147483904u32;
pub const DSOP_DOWNLEVEL_FILTER_DIALUP: u32 = 2147484672u32;
pub const DSOP_DOWNLEVEL_FILTER_EXCLUDE_BUILTIN_GROUPS: u32 = 2147516416u32;
pub const DSOP_DOWNLEVEL_FILTER_GLOBAL_GROUPS: u32 = 2147483652u32;
pub const DSOP_DOWNLEVEL_FILTER_IIS_APP_POOL: u32 = 2214592512u32;
pub const DSOP_DOWNLEVEL_FILTER_INTERACTIVE: u32 = 2147485696u32;
pub const DSOP_DOWNLEVEL_FILTER_INTERNET_USER: u32 = 2149580800u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_ACCOUNTS: u32 = 2415919104u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_GROUPS: u32 = 2147483650u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_LOGON: u32 = 2164260864u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_SERVICE: u32 = 2147745792u32;
pub const DSOP_DOWNLEVEL_FILTER_NETWORK: u32 = 2147487744u32;
pub const DSOP_DOWNLEVEL_FILTER_NETWORK_SERVICE: u32 = 2148007936u32;
pub const DSOP_DOWNLEVEL_FILTER_OWNER_RIGHTS: u32 = 2151677952u32;
pub const DSOP_DOWNLEVEL_FILTER_REMOTE_LOGON: u32 = 2148532224u32;
pub const DSOP_DOWNLEVEL_FILTER_SERVICE: u32 = 2147491840u32;
pub const DSOP_DOWNLEVEL_FILTER_SERVICES: u32 = 2155872256u32;
pub const DSOP_DOWNLEVEL_FILTER_SYSTEM: u32 = 2147500032u32;
pub const DSOP_DOWNLEVEL_FILTER_TERMINAL_SERVER: u32 = 2147549184u32;
pub const DSOP_DOWNLEVEL_FILTER_THIS_ORG_CERT: u32 = 2181038080u32;
pub const DSOP_DOWNLEVEL_FILTER_USERS: u32 = 2147483649u32;
pub const DSOP_DOWNLEVEL_FILTER_WORLD: u32 = 2147483664u32;
pub const DSOP_FILTER_BUILTIN_GROUPS: u32 = 4u32;
pub const DSOP_FILTER_COMPUTERS: u32 = 2048u32;
pub const DSOP_FILTER_CONTACTS: u32 = 1024u32;
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_DL: u32 = 256u32;
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_SE: u32 = 512u32;
#[repr(C)]
pub struct DSOP_FILTER_FLAGS(i32);
pub const DSOP_FILTER_GLOBAL_GROUPS_DL: u32 = 64u32;
pub const DSOP_FILTER_GLOBAL_GROUPS_SE: u32 = 128u32;
pub const DSOP_FILTER_INCLUDE_ADVANCED_VIEW: u32 = 1u32;
pub const DSOP_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 8192u32;
pub const DSOP_FILTER_SERVICE_ACCOUNTS: u32 = 4096u32;
pub const DSOP_FILTER_UNIVERSAL_GROUPS_DL: u32 = 16u32;
pub const DSOP_FILTER_UNIVERSAL_GROUPS_SE: u32 = 32u32;
pub const DSOP_FILTER_USERS: u32 = 2u32;
pub const DSOP_FILTER_WELL_KNOWN_PRINCIPALS: u32 = 8u32;
pub const DSOP_FLAG_MULTISELECT: u32 = 1u32;
pub const DSOP_FLAG_SKIP_TARGET_COMPUTER_DC_CHECK: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSOP_INIT_INFO(i32);
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_COMPUTERS: u32 = 256u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_CONTACTS: u32 = 512u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_GROUPS: u32 = 128u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 2048u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_SERVICE_ACCOUNTS: u32 = 1024u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_USERS: u32 = 64u32;
pub const DSOP_SCOPE_FLAG_STARTING_SCOPE: u32 = 1u32;
pub const DSOP_SCOPE_FLAG_WANT_DOWNLEVEL_BUILTIN_PATH: u32 = 32u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_GC: u32 = 8u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_LDAP: u32 = 4u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_WINNT: u32 = 2u32;
pub const DSOP_SCOPE_FLAG_WANT_SID_PATH: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSOP_SCOPE_INIT_INFO(i32);
pub const DSOP_SCOPE_TYPE_DOWNLEVEL_JOINED_DOMAIN: u32 = 4u32;
pub const DSOP_SCOPE_TYPE_ENTERPRISE_DOMAIN: u32 = 8u32;
pub const DSOP_SCOPE_TYPE_EXTERNAL_DOWNLEVEL_DOMAIN: u32 = 64u32;
pub const DSOP_SCOPE_TYPE_EXTERNAL_UPLEVEL_DOMAIN: u32 = 32u32;
pub const DSOP_SCOPE_TYPE_GLOBAL_CATALOG: u32 = 16u32;
pub const DSOP_SCOPE_TYPE_TARGET_COMPUTER: u32 = 1u32;
pub const DSOP_SCOPE_TYPE_UPLEVEL_JOINED_DOMAIN: u32 = 2u32;
pub const DSOP_SCOPE_TYPE_USER_ENTERED_DOWNLEVEL_SCOPE: u32 = 512u32;
pub const DSOP_SCOPE_TYPE_USER_ENTERED_UPLEVEL_SCOPE: u32 = 256u32;
pub const DSOP_SCOPE_TYPE_WORKGROUP: u32 = 128u32;
#[repr(C)]
pub struct DSOP_UPLEVEL_FILTER_FLAGS(i32);
#[repr(C)]
pub struct DSPROPERTYPAGEINFO(i32);
pub const DSPROVIDER_ADVANCED: u32 = 16u32;
pub const DSPROVIDER_AD_LDS: u32 = 32u32;
pub const DSPROVIDER_UNUSED_0: u32 = 1u32;
pub const DSPROVIDER_UNUSED_1: u32 = 2u32;
pub const DSPROVIDER_UNUSED_2: u32 = 4u32;
pub const DSPROVIDER_UNUSED_3: u32 = 8u32;
pub const DSQPF_ENABLEADMINFEATURES: u32 = 8u32;
pub const DSQPF_ENABLEADVANCEDFEATURES: u32 = 16u32;
pub const DSQPF_HASCREDENTIALS: u32 = 32u32;
pub const DSQPF_NOCHOOSECOLUMNS: u32 = 64u32;
pub const DSQPF_NOSAVE: u32 = 1u32;
pub const DSQPF_SAVELOCATION: u32 = 2u32;
pub const DSQPF_SHOWHIDDENOBJECTS: u32 = 4u32;
pub const DSQPM_GETCLASSLIST: u32 = 268435456u32;
pub const DSQPM_HELPTOPICS: u32 = 268435457u32;
#[repr(C)]
pub struct DSQUERYCLASSLIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSQUERYINITPARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSQUERYPARAMS(i32);
#[repr(transparent)]
pub struct DSROLE_MACHINE_ROLE(pub i32);
pub const DsRole_RoleStandaloneWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(0i32);
pub const DsRole_RoleMemberWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(1i32);
pub const DsRole_RoleStandaloneServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(2i32);
pub const DsRole_RoleMemberServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(3i32);
pub const DsRole_RoleBackupDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(4i32);
pub const DsRole_RolePrimaryDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(5i32);
#[repr(transparent)]
pub struct DSROLE_OPERATION_STATE(pub i32);
pub const DsRoleOperationIdle: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(0i32);
pub const DsRoleOperationActive: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(1i32);
pub const DsRoleOperationNeedReboot: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(2i32);
#[repr(C)]
pub struct DSROLE_OPERATION_STATE_INFO(i32);
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: u32 = 16777216u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC(i32);
#[repr(transparent)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(pub i32);
pub const DsRolePrimaryDomainInfoBasic: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(1i32);
pub const DsRoleUpgradeStatus: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(2i32);
pub const DsRoleOperationState: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(3i32);
pub const DSROLE_PRIMARY_DS_MIXED_MODE: u32 = 2u32;
pub const DSROLE_PRIMARY_DS_READONLY: u32 = 8u32;
pub const DSROLE_PRIMARY_DS_RUNNING: u32 = 1u32;
#[repr(transparent)]
pub struct DSROLE_SERVER_STATE(pub i32);
pub const DsRoleServerUnknown: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(0i32);
pub const DsRoleServerPrimary: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(1i32);
pub const DsRoleServerBackup: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(2i32);
pub const DSROLE_UPGRADE_IN_PROGRESS: u32 = 4u32;
#[repr(C)]
pub struct DSROLE_UPGRADE_STATUS_INFO(i32);
pub const DSSSF_DONTSIGNSEAL: u32 = 2u32;
pub const DSSSF_DSAVAILABLE: u32 = 2147483648u32;
pub const DSSSF_SIMPLEAUTHENTICATE: u32 = 1u32;
pub const DS_AVOID_SELF: u32 = 16384u32;
pub const DS_BACKGROUND_ONLY: u32 = 256u32;
pub const DS_BEHAVIOR_LONGHORN: u32 = 3u32;
pub const DS_BEHAVIOR_WIN2000: u32 = 0u32;
pub const DS_BEHAVIOR_WIN2003: u32 = 2u32;
pub const DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS: u32 = 1u32;
pub const DS_BEHAVIOR_WIN2008: u32 = 3u32;
pub const DS_BEHAVIOR_WIN2008R2: u32 = 4u32;
pub const DS_BEHAVIOR_WIN2012: u32 = 5u32;
pub const DS_BEHAVIOR_WIN2012R2: u32 = 6u32;
pub const DS_BEHAVIOR_WIN2016: u32 = 7u32;
pub const DS_BEHAVIOR_WIN7: u32 = 4u32;
pub const DS_BEHAVIOR_WIN8: u32 = 5u32;
pub const DS_BEHAVIOR_WINBLUE: u32 = 6u32;
pub const DS_BEHAVIOR_WINTHRESHOLD: u32 = 7u32;
pub const DS_CLOSEST_FLAG: u32 = 128u32;
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: u32 = 8388608u32;
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: u32 = 524288u32;
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: u32 = 2097152u32;
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: u32 = 4194304u32;
pub const DS_DIRECTORY_SERVICE_PREFERRED: u32 = 32u32;
pub const DS_DIRECTORY_SERVICE_REQUIRED: u32 = 16u32;
pub const DS_DNS_CONTROLLER_FLAG: u32 = 536870912u32;
pub const DS_DNS_DOMAIN_FLAG: u32 = 1073741824u32;
pub const DS_DNS_FOREST_FLAG: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1A(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2A(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3A(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3W(i32);
pub const DS_DOMAIN_DIRECT_INBOUND: u32 = 32u32;
pub const DS_DOMAIN_DIRECT_OUTBOUND: u32 = 2u32;
pub const DS_DOMAIN_IN_FOREST: u32 = 1u32;
pub const DS_DOMAIN_NATIVE_MODE: u32 = 16u32;
pub const DS_DOMAIN_PRIMARY: u32 = 8u32;
pub const DS_DOMAIN_TREE_ROOT: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_TRUSTSA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_DOMAIN_TRUSTSW(i32);
pub const DS_DS_10_FLAG: u32 = 65536u32;
pub const DS_DS_8_FLAG: u32 = 16384u32;
pub const DS_DS_9_FLAG: u32 = 32768u32;
pub const DS_DS_FLAG: u32 = 16u32;
pub const DS_EXIST_ADVISORY_MODE: u32 = 1u32;
pub const DS_FORCE_REDISCOVERY: u32 = 1u32;
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: u32 = 4096u32;
pub const DS_GC_FLAG: u32 = 4u32;
pub const DS_GC_SERVER_REQUIRED: u32 = 64u32;
pub const DS_GFTI_UPDATE_TDO: u32 = 1u32;
pub const DS_GFTI_VALID_FLAGS: u32 = 1u32;
pub const DS_GOOD_TIMESERV_FLAG: u32 = 512u32;
pub const DS_GOOD_TIMESERV_PREFERRED: u32 = 8192u32;
pub const DS_INSTANCETYPE_IS_NC_HEAD: u32 = 1u32;
pub const DS_INSTANCETYPE_NC_COMING: u32 = 16u32;
pub const DS_INSTANCETYPE_NC_GOING: u32 = 32u32;
pub const DS_INSTANCETYPE_NC_IS_WRITEABLE: u32 = 4u32;
pub const DS_IP_REQUIRED: u32 = 512u32;
pub const DS_IS_DNS_NAME: u32 = 131072u32;
pub const DS_IS_FLAT_NAME: u32 = 65536u32;
pub const DS_KCC_FLAG_ASYNC_OP: u32 = 1u32;
pub const DS_KCC_FLAG_DAMPED: u32 = 2u32;
#[repr(transparent)]
pub struct DS_KCC_TASKID(pub i32);
pub const DS_KCC_TASKID_UPDATE_TOPOLOGY: DS_KCC_TASKID = DS_KCC_TASKID(0i32);
pub const DS_KDC_FLAG: u32 = 32u32;
pub const DS_KDC_REQUIRED: u32 = 1024u32;
pub const DS_KEY_LIST_FLAG: u32 = 131072u32;
pub const DS_KEY_LIST_SUPPORT_REQUIRED: u32 = 16777216u32;
pub const DS_LDAP_FLAG: u32 = 8u32;
pub const DS_LIST_ACCOUNT_OBJECT_FOR_SERVER: u32 = 2u32;
pub const DS_LIST_DNS_HOST_NAME_FOR_SERVER: u32 = 1u32;
pub const DS_LIST_DSA_OBJECT_FOR_SERVER: u32 = 0u32;
#[repr(transparent)]
pub struct DS_MANGLE_FOR(pub i32);
pub const DS_MANGLE_UNKNOWN: DS_MANGLE_FOR = DS_MANGLE_FOR(0i32);
pub const DS_MANGLE_OBJECT_RDN_FOR_DELETION: DS_MANGLE_FOR = DS_MANGLE_FOR(1i32);
pub const DS_MANGLE_OBJECT_RDN_FOR_NAME_CONFLICT: DS_MANGLE_FOR = DS_MANGLE_FOR(2i32);
#[repr(transparent)]
pub struct DS_NAME_ERROR(pub i32);
pub const DS_NAME_NO_ERROR: DS_NAME_ERROR = DS_NAME_ERROR(0i32);
pub const DS_NAME_ERROR_RESOLVING: DS_NAME_ERROR = DS_NAME_ERROR(1i32);
pub const DS_NAME_ERROR_NOT_FOUND: DS_NAME_ERROR = DS_NAME_ERROR(2i32);
pub const DS_NAME_ERROR_NOT_UNIQUE: DS_NAME_ERROR = DS_NAME_ERROR(3i32);
pub const DS_NAME_ERROR_NO_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(4i32);
pub const DS_NAME_ERROR_DOMAIN_ONLY: DS_NAME_ERROR = DS_NAME_ERROR(5i32);
pub const DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(6i32);
pub const DS_NAME_ERROR_TRUST_REFERRAL: DS_NAME_ERROR = DS_NAME_ERROR(7i32);
#[repr(transparent)]
pub struct DS_NAME_FLAGS(pub i32);
pub const DS_NAME_NO_FLAGS: DS_NAME_FLAGS = DS_NAME_FLAGS(0i32);
pub const DS_NAME_FLAG_SYNTACTICAL_ONLY: DS_NAME_FLAGS = DS_NAME_FLAGS(1i32);
pub const DS_NAME_FLAG_EVAL_AT_DC: DS_NAME_FLAGS = DS_NAME_FLAGS(2i32);
pub const DS_NAME_FLAG_GCVERIFY: DS_NAME_FLAGS = DS_NAME_FLAGS(4i32);
pub const DS_NAME_FLAG_TRUST_REFERRAL: DS_NAME_FLAGS = DS_NAME_FLAGS(8i32);
#[repr(transparent)]
pub struct DS_NAME_FORMAT(pub i32);
pub const DS_UNKNOWN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(0i32);
pub const DS_FQDN_1779_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(1i32);
pub const DS_NT4_ACCOUNT_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(2i32);
pub const DS_DISPLAY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(3i32);
pub const DS_UNIQUE_ID_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(6i32);
pub const DS_CANONICAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(7i32);
pub const DS_USER_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(8i32);
pub const DS_CANONICAL_NAME_EX: DS_NAME_FORMAT = DS_NAME_FORMAT(9i32);
pub const DS_SERVICE_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(10i32);
pub const DS_SID_OR_SID_HISTORY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(11i32);
pub const DS_DNS_DOMAIN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(12i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_NAME_RESULTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_NAME_RESULTW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_NAME_RESULT_ITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_NAME_RESULT_ITEMW(i32);
pub const DS_NDNC_FLAG: u32 = 1024u32;
pub const DS_NOTIFY_AFTER_SITE_RECORDS: u32 = 2u32;
pub const DS_ONLY_DO_SITE_NAME: u32 = 1u32;
pub const DS_ONLY_LDAP_NEEDED: u32 = 32768u32;
pub const DS_PDC_FLAG: u32 = 1u32;
pub const DS_PDC_REQUIRED: u32 = 128u32;
pub const DS_PING_FLAGS: u32 = 1048575u32;
pub const DS_REPADD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPADD_ASYNCHRONOUS_REPLICA: u32 = 32u32;
pub const DS_REPADD_CRITICAL: u32 = 2048u32;
pub const DS_REPADD_DISABLE_NOTIFICATION: u32 = 64u32;
pub const DS_REPADD_DISABLE_PERIODIC: u32 = 128u32;
pub const DS_REPADD_INITIAL: u32 = 4u32;
pub const DS_REPADD_INTERSITE_MESSAGING: u32 = 16u32;
pub const DS_REPADD_NEVER_NOTIFY: u32 = 512u32;
pub const DS_REPADD_NONGC_RO_REPLICA: u32 = 16777216u32;
pub const DS_REPADD_PERIODIC: u32 = 8u32;
pub const DS_REPADD_SELECT_SECRETS: u32 = 4096u32;
pub const DS_REPADD_TWO_WAY: u32 = 1024u32;
pub const DS_REPADD_USE_COMPRESSION: u32 = 256u32;
pub const DS_REPADD_WRITEABLE: u32 = 2u32;
pub const DS_REPDEL_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPDEL_IGNORE_ERRORS: u32 = 8u32;
pub const DS_REPDEL_INTERSITE_MESSAGING: u32 = 4u32;
pub const DS_REPDEL_LOCAL_ONLY: u32 = 16u32;
pub const DS_REPDEL_NO_SOURCE: u32 = 32u32;
pub const DS_REPDEL_REF_OK: u32 = 64u32;
pub const DS_REPDEL_WRITEABLE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_ATTR_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_ATTR_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_ATTR_META_DATA_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_ATTR_VALUE_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_EXT(i32);
#[repr(C)]
pub struct DS_REPL_CURSOR(i32);
#[repr(C)]
pub struct DS_REPL_CURSORS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_CURSORS_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_CURSORS_3W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_CURSOR_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_CURSOR_3W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_CURSOR_BLOB(i32);
pub const DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS: u32 = 1u32;
#[repr(transparent)]
pub struct DS_REPL_INFO_TYPE(pub i32);
pub const DS_REPL_INFO_NEIGHBORS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(0i32);
pub const DS_REPL_INFO_CURSORS_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(1i32);
pub const DS_REPL_INFO_METADATA_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(2i32);
pub const DS_REPL_INFO_KCC_DSA_CONNECT_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(3i32);
pub const DS_REPL_INFO_KCC_DSA_LINK_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(4i32);
pub const DS_REPL_INFO_PENDING_OPS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(5i32);
pub const DS_REPL_INFO_METADATA_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(6i32);
pub const DS_REPL_INFO_CURSORS_2_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(7i32);
pub const DS_REPL_INFO_CURSORS_3_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(8i32);
pub const DS_REPL_INFO_METADATA_2_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(9i32);
pub const DS_REPL_INFO_METADATA_2_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(10i32);
pub const DS_REPL_INFO_METADATA_EXT_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(11i32);
pub const DS_REPL_INFO_TYPE_MAX: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(12i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_KCC_DSA_FAILURESW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_KCC_DSA_FAILUREW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_KCC_DSA_FAILUREW_BLOB(i32);
pub const DS_REPL_NBR_COMPRESS_CHANGES: u32 = 268435456u32;
pub const DS_REPL_NBR_DISABLE_SCHEDULED_SYNC: u32 = 134217728u32;
pub const DS_REPL_NBR_DO_SCHEDULED_SYNCS: u32 = 64u32;
pub const DS_REPL_NBR_FULL_SYNC_IN_PROGRESS: u32 = 65536u32;
pub const DS_REPL_NBR_FULL_SYNC_NEXT_PACKET: u32 = 131072u32;
pub const DS_REPL_NBR_GCSPN: u32 = 1048576u32;
pub const DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS: u32 = 67108864u32;
pub const DS_REPL_NBR_NEVER_SYNCED: u32 = 2097152u32;
pub const DS_REPL_NBR_NONGC_RO_REPLICA: u32 = 1024u32;
pub const DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS: u32 = 536870912u32;
pub const DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET: u32 = 1073741824u32;
pub const DS_REPL_NBR_PREEMPTED: u32 = 16777216u32;
pub const DS_REPL_NBR_RETURN_OBJECT_PARENTS: u32 = 2048u32;
pub const DS_REPL_NBR_SELECT_SECRETS: u32 = 4096u32;
pub const DS_REPL_NBR_SYNC_ON_STARTUP: u32 = 32u32;
pub const DS_REPL_NBR_TWO_WAY_SYNC: u32 = 512u32;
pub const DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT: u32 = 128u32;
pub const DS_REPL_NBR_WRITEABLE: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_NEIGHBORSW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_NEIGHBORW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_NEIGHBORW_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_OBJ_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_OBJ_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_OPW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_OPW_BLOB(i32);
#[repr(transparent)]
pub struct DS_REPL_OP_TYPE(pub i32);
pub const DS_REPL_OP_TYPE_SYNC: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(0i32);
pub const DS_REPL_OP_TYPE_ADD: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(1i32);
pub const DS_REPL_OP_TYPE_DELETE: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(2i32);
pub const DS_REPL_OP_TYPE_MODIFY: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(3i32);
pub const DS_REPL_OP_TYPE_UPDATE_REFS: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_PENDING_OPSW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_QUEUE_STATISTICSW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_VALUE_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_VALUE_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_VALUE_META_DATA_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_VALUE_META_DATA_BLOB_EXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPL_VALUE_META_DATA_EXT(i32);
pub const DS_REPMOD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPMOD_UPDATE_ADDRESS: u32 = 2u32;
pub const DS_REPMOD_UPDATE_FLAGS: u32 = 1u32;
pub const DS_REPMOD_UPDATE_INSTANCE: u32 = 2u32;
pub const DS_REPMOD_UPDATE_RESULT: u32 = 8u32;
pub const DS_REPMOD_UPDATE_SCHEDULE: u32 = 4u32;
pub const DS_REPMOD_UPDATE_TRANSPORT: u32 = 16u32;
pub const DS_REPMOD_WRITEABLE: u32 = 2u32;
pub const DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE: u32 = 1u32;
pub const DS_REPSYNCALL_CROSS_SITE_BOUNDARIES: u32 = 64u32;
pub const DS_REPSYNCALL_DO_NOT_SYNC: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPSYNCALL_ERRINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPSYNCALL_ERRINFOW(i32);
#[repr(transparent)]
pub struct DS_REPSYNCALL_ERROR(pub i32);
pub const DS_REPSYNCALL_WIN32_ERROR_CONTACTING_SERVER: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(0i32);
pub const DS_REPSYNCALL_WIN32_ERROR_REPLICATING: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(1i32);
pub const DS_REPSYNCALL_SERVER_UNREACHABLE: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(2i32);
#[repr(transparent)]
pub struct DS_REPSYNCALL_EVENT(pub i32);
pub const DS_REPSYNCALL_EVENT_ERROR: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(0i32);
pub const DS_REPSYNCALL_EVENT_SYNC_STARTED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(1i32);
pub const DS_REPSYNCALL_EVENT_SYNC_COMPLETED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(2i32);
pub const DS_REPSYNCALL_EVENT_FINISHED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(3i32);
pub const DS_REPSYNCALL_ID_SERVERS_BY_DN: u32 = 4u32;
pub const DS_REPSYNCALL_NO_OPTIONS: u32 = 0u32;
pub const DS_REPSYNCALL_PUSH_CHANGES_OUTWARD: u32 = 32u32;
pub const DS_REPSYNCALL_SKIP_INITIAL_CHECK: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPSYNCALL_SYNCA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPSYNCALL_SYNCW(i32);
pub const DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPSYNCALL_UPDATEA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_REPSYNCALL_UPDATEW(i32);
pub const DS_REPSYNC_ABANDONED: u32 = 32768u32;
pub const DS_REPSYNC_ADD_REFERENCE: u32 = 512u32;
pub const DS_REPSYNC_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPSYNC_ASYNCHRONOUS_REPLICA: u32 = 1048576u32;
pub const DS_REPSYNC_CRITICAL: u32 = 2097152u32;
pub const DS_REPSYNC_FORCE: u32 = 256u32;
pub const DS_REPSYNC_FULL: u32 = 32u32;
pub const DS_REPSYNC_FULL_IN_PROGRESS: u32 = 4194304u32;
pub const DS_REPSYNC_INITIAL: u32 = 8192u32;
pub const DS_REPSYNC_INITIAL_IN_PROGRESS: u32 = 65536u32;
pub const DS_REPSYNC_INTERSITE_MESSAGING: u32 = 8u32;
pub const DS_REPSYNC_NEVER_COMPLETED: u32 = 1024u32;
pub const DS_REPSYNC_NEVER_NOTIFY: u32 = 4096u32;
pub const DS_REPSYNC_NONGC_RO_REPLICA: u32 = 16777216u32;
pub const DS_REPSYNC_NOTIFICATION: u32 = 524288u32;
pub const DS_REPSYNC_NO_DISCARD: u32 = 128u32;
pub const DS_REPSYNC_PARTIAL_ATTRIBUTE_SET: u32 = 131072u32;
pub const DS_REPSYNC_PERIODIC: u32 = 4u32;
pub const DS_REPSYNC_PREEMPTED: u32 = 8388608u32;
pub const DS_REPSYNC_REQUEUE: u32 = 262144u32;
pub const DS_REPSYNC_SELECT_SECRETS: u32 = 32768u32;
pub const DS_REPSYNC_TWO_WAY: u32 = 2048u32;
pub const DS_REPSYNC_URGENT: u32 = 64u32;
pub const DS_REPSYNC_USE_COMPRESSION: u32 = 16384u32;
pub const DS_REPSYNC_WRITEABLE: u32 = 2u32;
pub const DS_REPUPD_ADD_REFERENCE: u32 = 4u32;
pub const DS_REPUPD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPUPD_DELETE_REFERENCE: u32 = 8u32;
pub const DS_REPUPD_REFERENCE_GCSPN: u32 = 16u32;
pub const DS_REPUPD_WRITEABLE: u32 = 2u32;
pub const DS_RETURN_DNS_NAME: u32 = 1073741824u32;
pub const DS_RETURN_FLAT_NAME: u32 = 2147483648u32;
pub const DS_ROLE_DOMAIN_OWNER: u32 = 1u32;
pub const DS_ROLE_INFRASTRUCTURE_OWNER: u32 = 4u32;
pub const DS_ROLE_PDC_OWNER: u32 = 2u32;
pub const DS_ROLE_RID_OWNER: u32 = 3u32;
pub const DS_ROLE_SCHEMA_OWNER: u32 = 0u32;
pub const DS_SCHEMA_GUID_ATTR: u32 = 1u32;
pub const DS_SCHEMA_GUID_ATTR_SET: u32 = 2u32;
pub const DS_SCHEMA_GUID_CLASS: u32 = 3u32;
pub const DS_SCHEMA_GUID_CONTROL_RIGHT: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_SCHEMA_GUID_MAPA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DS_SCHEMA_GUID_MAPW(i32);
pub const DS_SCHEMA_GUID_NOT_FOUND: u32 = 0u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DS_SELECTION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DS_SELECTION_LIST(i32);
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: u32 = 2048u32;
#[repr(C)]
pub struct DS_SITE_COST_INFO(i32);
#[repr(transparent)]
pub struct DS_SPN_NAME_TYPE(pub i32);
pub const DS_SPN_DNS_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(0i32);
pub const DS_SPN_DN_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(1i32);
pub const DS_SPN_NB_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(2i32);
pub const DS_SPN_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(3i32);
pub const DS_SPN_NB_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(4i32);
pub const DS_SPN_SERVICE: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(5i32);
#[repr(transparent)]
pub struct DS_SPN_WRITE_OP(pub i32);
pub const DS_SPN_ADD_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(0i32);
pub const DS_SPN_REPLACE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(1i32);
pub const DS_SPN_DELETE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(2i32);
pub const DS_TIMESERV_FLAG: u32 = 64u32;
pub const DS_TIMESERV_REQUIRED: u32 = 2048u32;
pub const DS_TRY_NEXTCLOSEST_SITE: u32 = 262144u32;
pub const DS_WEB_SERVICE_REQUIRED: u32 = 1048576u32;
pub const DS_WRITABLE_FLAG: u32 = 256u32;
pub const DS_WRITABLE_REQUIRED: u32 = 4096u32;
pub const DS_WS_FLAG: u32 = 8192u32;
#[repr(C)]
pub struct Email(i32);
pub const FACILITY_BACKUP: u32 = 2047u32;
pub const FACILITY_NTDSB: u32 = 2048u32;
pub const FACILITY_SYSTEM: u32 = 0u32;
pub const FLAG_DISABLABLE_OPTIONAL_FEATURE: u32 = 4u32;
pub const FLAG_DOMAIN_OPTIONAL_FEATURE: u32 = 2u32;
pub const FLAG_FOREST_OPTIONAL_FEATURE: u32 = 1u32;
pub const FLAG_SERVER_OPTIONAL_FEATURE: u32 = 8u32;
pub const FRSCONN_MAX_PRIORITY: u32 = 8u32;
pub const FRSCONN_PRIORITY_MASK: u32 = 1879048192u32;
#[repr(C)]
pub struct FaxNumber(i32);
#[repr(C)]
pub struct GetDcContextHandle(i32);
#[repr(C)]
pub struct Hold(i32);
#[repr(transparent)]
pub struct IADs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsADSystemInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsAccessControlEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsAccessControlList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsAcl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsAggregatee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsAggregator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsBackLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsCaseIgnoreList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsComputer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsComputerOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsDNWithBinary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsDNWithString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsDeleteOps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsDomain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsEmail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsFaxNumber(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsFileService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsFileServiceOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsFileShare(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsHold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsLargeInteger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsLocality(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsMembers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsNameTranslate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsNamespaces(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsNetAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsO(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsOU(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsObjectOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsOctetList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsOpenDSObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPathname(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPostalAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPrintJobOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPrintQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPrintQueueOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPropertyEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPropertyList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPropertyValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsPropertyValue2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsReplicaPointer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsSecurityDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsSecurityUtility(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsServiceOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsSyntax(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsTimestamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsTypedName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IADsWinNTSystemInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommonQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectoryObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectorySchemaMgmt(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectorySearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsAdminCreateObj(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsAdminNewObj(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsAdminNewObjExt(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsAdminNewObjPrimarySite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsAdminNotifyHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsBrowseDomainTree(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsDisplaySpecifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsObjectPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDsObjectPickerCredentials(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrivateDispatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrivateUnknown(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryForm(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LPCQADDFORMSPROC(i32);
#[repr(C)]
pub struct LPCQADDPAGESPROC(i32);
#[repr(C)]
pub struct LPCQPAGEPROC(i32);
#[repr(C)]
pub struct LPDSENUMATTRIBUTES(i32);
#[repr(C)]
pub struct LargeInteger(i32);
pub const NTDSAPI_BIND_ALLOW_DELEGATION: u32 = 1u32;
pub const NTDSAPI_BIND_FIND_BINDING: u32 = 2u32;
pub const NTDSAPI_BIND_FORCE_KERBEROS: u32 = 4u32;
pub const NTDSCONN_KCC_GC_TOPOLOGY: u32 = 1u32;
pub const NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY: u32 = 32u32;
pub const NTDSCONN_KCC_INTERSITE_TOPOLOGY: u32 = 64u32;
pub const NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY: u32 = 4u32;
pub const NTDSCONN_KCC_NO_REASON: u32 = 0u32;
pub const NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY: u32 = 16u32;
pub const NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY: u32 = 512u32;
pub const NTDSCONN_KCC_RING_TOPOLOGY: u32 = 2u32;
pub const NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY: u32 = 128u32;
pub const NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY: u32 = 256u32;
pub const NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY: u32 = 8u32;
pub const NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION: u32 = 16u32;
pub const NTDSCONN_OPT_IGNORE_SCHEDULE_MASK: u32 = 2147483648u32;
pub const NTDSCONN_OPT_IS_GENERATED: u32 = 1u32;
pub const NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT: u32 = 4u32;
pub const NTDSCONN_OPT_RODC_TOPOLOGY: u32 = 64u32;
pub const NTDSCONN_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSCONN_OPT_USER_OWNED_SCHEDULE: u32 = 32u32;
pub const NTDSCONN_OPT_USE_NOTIFY: u32 = 8u32;
pub const NTDSDSA_OPT_BLOCK_RPC: u32 = 64u32;
pub const NTDSDSA_OPT_DISABLE_INBOUND_REPL: u32 = 2u32;
pub const NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE: u32 = 8u32;
pub const NTDSDSA_OPT_DISABLE_OUTBOUND_REPL: u32 = 4u32;
pub const NTDSDSA_OPT_DISABLE_SPN_REGISTRATION: u32 = 16u32;
pub const NTDSDSA_OPT_GENERATE_OWN_TOPO: u32 = 32u32;
pub const NTDSDSA_OPT_IS_GC: u32 = 1u32;
pub const NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY: u32 = 2u32;
pub const NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION: u32 = 128u32;
pub const NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR: u32 = 64u32;
pub const NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED: u32 = 1u32;
pub const NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED: u32 = 32u32;
pub const NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED: u32 = 16u32;
pub const NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED: u32 = 256u32;
pub const NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED: u32 = 1024u32;
pub const NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED: u32 = 512u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED: u32 = 2u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED: u32 = 8u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED: u32 = 4u32;
pub const NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED: u32 = 4096u32;
pub const NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES: u32 = 2048u32;
pub const NTDSSITECONN_OPT_DISABLE_COMPRESSION: u32 = 4u32;
pub const NTDSSITECONN_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSSITECONN_OPT_USE_NOTIFY: u32 = 1u32;
pub const NTDSSITELINK_OPT_DISABLE_COMPRESSION: u32 = 4u32;
pub const NTDSSITELINK_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSSITELINK_OPT_USE_NOTIFY: u32 = 1u32;
pub const NTDSTRANSPORT_OPT_BRIDGES_REQUIRED: u32 = 2u32;
pub const NTDSTRANSPORT_OPT_IGNORE_SCHEDULES: u32 = 1u32;
#[repr(C)]
pub struct NameTranslate(i32);
#[repr(C)]
pub struct NetAddress(i32);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct OPENQUERYWINDOW(i32);
pub const OQWF_DEFAULTFORM: u32 = 2u32;
pub const OQWF_HIDEMENUS: u32 = 1024u32;
pub const OQWF_HIDESEARCHUI: u32 = 2048u32;
pub const OQWF_ISSUEONOPEN: u32 = 64u32;
pub const OQWF_LOADQUERY: u32 = 8u32;
pub const OQWF_OKCANCEL: u32 = 1u32;
pub const OQWF_PARAMISPROPERTYBAG: u32 = 2147483648u32;
pub const OQWF_REMOVEFORMS: u32 = 32u32;
pub const OQWF_REMOVESCOPES: u32 = 16u32;
pub const OQWF_SAVEQUERYONOK: u32 = 512u32;
pub const OQWF_SHOWOPTIONAL: u32 = 128u32;
pub const OQWF_SINGLESELECT: u32 = 4u32;
#[repr(C)]
pub struct OctetList(i32);
#[repr(C)]
pub struct Path(i32);
#[repr(C)]
pub struct Pathname(i32);
#[repr(C)]
pub struct PostalAddress(i32);
#[repr(C)]
pub struct PropertyEntry(i32);
#[repr(C)]
pub struct PropertyValue(i32);
pub const QUERYFORM_CHANGESFORMLIST: u64 = 1u64;
pub const QUERYFORM_CHANGESOPTFORMLIST: u64 = 2u64;
#[repr(C)]
pub struct ReplicaPointer(i32);
#[repr(C)]
pub struct SCHEDULE(i32);
pub const SCHEDULE_BANDWIDTH: u32 = 1u32;
#[repr(C)]
pub struct SCHEDULE_HEADER(i32);
pub const SCHEDULE_INTERVAL: u32 = 0u32;
pub const SCHEDULE_PRIORITY: u32 = 2u32;
pub const STATUS_SEVERITY_ERROR: u32 = 3u32;
pub const STATUS_SEVERITY_INFORMATIONAL: u32 = 1u32;
pub const STATUS_SEVERITY_SUCCESS: u32 = 0u32;
pub const STATUS_SEVERITY_WARNING: u32 = 2u32;
#[repr(C)]
pub struct SecurityDescriptor(i32);
#[repr(C)]
pub struct Timestamp(i32);
#[repr(C)]
pub struct TypedName(i32);
pub const WM_ADSPROP_NOTIFY_APPLY: u32 = 2128u32;
pub const WM_ADSPROP_NOTIFY_CHANGE: u32 = 2127u32;
pub const WM_ADSPROP_NOTIFY_ERROR: u32 = 2134u32;
pub const WM_ADSPROP_NOTIFY_EXIT: u32 = 2131u32;
pub const WM_ADSPROP_NOTIFY_FOREGROUND: u32 = 2130u32;
pub const WM_ADSPROP_NOTIFY_PAGEHWND: u32 = 2126u32;
pub const WM_ADSPROP_NOTIFY_PAGEINIT: u32 = 2125u32;
pub const WM_ADSPROP_NOTIFY_SETFOCUS: u32 = 2129u32;
#[repr(C)]
pub struct WinNTSystemInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ads_search_column(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ads_searchpref_info(i32);
