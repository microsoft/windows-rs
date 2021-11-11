#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_System_Ole`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsBuildEnumerator(padscontainer: ::windows::runtime::RawPtr, ppenumvariant: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayInt(lpdwobjecttypes: *mut u32, dwobjecttypes: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayStr(lpppathnames: *const super::super::Foundation::PWSTR, dwpathnames: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsDecodeBinaryData(szsrcdata: super::super::Foundation::PWSTR, ppbdestdata: *mut *mut u8, pdwdestlen: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsEncodeBinaryData(pbsrcdata: *mut u8, dwsrclen: u32, ppszdestdata: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsEnumerateNext(penumvariant: ::windows::runtime::RawPtr, celements: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pcelementsfetched: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_System_Ole`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsFreeEnumerator(penumvariant: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetLastError(lperror: *mut u32, lperrorbuf: super::super::Foundation::PWSTR, dwerrorbuflen: u32, lpnamebuf: super::super::Foundation::PWSTR, dwnamebuflen: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetObject(lpszpathname: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsOpenObject(lpszpathname: super::super::Foundation::PWSTR, lpszusername: super::super::Foundation::PWSTR, lpszpassword: super::super::Foundation::PWSTR, dwreserved: ADS_AUTHENTICATION_ENUM, riid: *const ::windows::runtime::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropCheckIfWritable(pwzattr: super::super::Foundation::PWSTR, pwritableattrs: *const ADS_ATTR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ADsPropCreateNotifyObj(pappthddataobj: ::windows::runtime::RawPtr, pwzadsobjname: super::super::Foundation::PWSTR, phnotifyobj: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropGetInitInfo(hnotifyobj: super::super::Foundation::HWND, pinitparams: *mut ::core::mem::ManuallyDrop<ADSPROPINITPARAMS>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSendErrorMessage(hnotifyobj: super::super::Foundation::HWND, perror: *mut ADSPROPERROR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSetHwnd(hnotifyobj: super::super::Foundation::HWND, hpage: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropSetHwndWithTitle(hnotifyobj: super::super::Foundation::HWND, hpage: super::super::Foundation::HWND, ptztitle: *const i8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropShowErrorDialog(hnotifyobj: super::super::Foundation::HWND, hpage: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsSetLastError(dwerr: u32, pszerror: super::super::Foundation::PWSTR, pszprovider: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdsFreeAdsValues(padsvalues: *mut ADSVALUE, dwnumvalues: u32);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AdsTypeToPropVariant(padsvalues: *mut ADSVALUE, dwnumvalues: u32, pvariant: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn AllocADsMem(cb: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocADsStr(pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn BinarySDToSecurityDescriptor(psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, pvarsec: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszservername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsAddSidHistoryA(hds: super::super::Foundation::HANDLE, flags: u32, srcdomain: super::super::Foundation::PSTR, srcprincipal: super::super::Foundation::PSTR, srcdomaincontroller: super::super::Foundation::PSTR, srcdomaincreds: *const ::core::ffi::c_void, dstdomain: super::super::Foundation::PSTR, dstprincipal: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsAddSidHistoryW(hds: super::super::Foundation::HANDLE, flags: u32, srcdomain: super::super::Foundation::PWSTR, srcprincipal: super::super::Foundation::PWSTR, srcdomaincontroller: super::super::Foundation::PWSTR, srcdomaincreds: *const ::core::ffi::c_void, dstdomain: super::super::Foundation::PWSTR, dstprincipal: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesA(computername: super::super::Foundation::PSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesExA(computername: super::super::Foundation::PSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PSTR, subnetnames: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesExW(computername: super::super::Foundation::PWSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PWSTR, subnetnames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsAddressToSiteNamesW(computername: super::super::Foundation::PWSTR, entrycount: u32, socketaddresses: *const super::WinSock::SOCKET_ADDRESS, sitenames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindByInstanceA(servername: super::super::Foundation::PSTR, annotation: super::super::Foundation::PSTR, instanceguid: *const ::windows::runtime::GUID, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindByInstanceW(servername: super::super::Foundation::PWSTR, annotation: super::super::Foundation::PWSTR, instanceguid: *const ::windows::runtime::GUID, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PWSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindToISTGA(sitename: super::super::Foundation::PSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindToISTGW(sitename: super::super::Foundation::PWSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithCredA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithCredW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnExA(domaincontrollername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnExW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PWSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindWithSpnW(domaincontrollername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PWSTR, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindingSetTimeout(hds: super::super::Foundation::HANDLE, ctimeoutsecs: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn DsBrowseForContainerA(pinfo: *mut ::core::mem::ManuallyDrop<DSBROWSEINFOA>) -> i32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn DsBrowseForContainerW(pinfo: *mut ::core::mem::ManuallyDrop<DSBROWSEINFOW>) -> i32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsClientMakeSpnForTargetServerA(serviceclass: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsClientMakeSpnForTargetServerW(serviceclass: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackNamesA(hds: super::super::Foundation::HANDLE, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, cnames: u32, rpnames: *const super::super::Foundation::PSTR, ppresult: *mut *mut DS_NAME_RESULTA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackNamesW(hds: super::super::Foundation::HANDLE, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, cnames: u32, rpnames: *const super::super::Foundation::PWSTR, ppresult: *mut *mut DS_NAME_RESULTW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn2A(pszspn: super::super::Foundation::PSTR, cspn: u32, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PSTR, pinstanceport: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn2W(pszspn: super::super::Foundation::PWSTR, cspn: u32, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PWSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pinstanceport: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn3W(pszspn: super::super::Foundation::PWSTR, cspn: u32, pchostname: *mut u32, hostname: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pportnumber: *mut u16, pcdomainname: *mut u32, domainname: super::super::Foundation::PWSTR, pcrealmname: *mut u32, realmname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpn4W(pszspn: super::super::Foundation::PWSTR, cspn: u32, pchostname: *mut u32, hostname: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pcportname: *mut u32, portname: super::super::Foundation::PWSTR, pcdomainname: *mut u32, domainname: super::super::Foundation::PWSTR, pcrealmname: *mut u32, realmname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpnA(pszspn: super::super::Foundation::PSTR, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PSTR, pinstanceport: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackSpnW(pszspn: super::super::Foundation::PWSTR, pcserviceclass: *mut u32, serviceclass: super::super::Foundation::PWSTR, pcservicename: *mut u32, servicename: super::super::Foundation::PWSTR, pcinstancename: *mut u32, instancename: super::super::Foundation::PWSTR, pinstanceport: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackUnquotedMangledRdnA(pszrdn: super::super::Foundation::PSTR, cchrdn: u32, pguid: *mut ::windows::runtime::GUID, pedsmanglefor: *mut DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackUnquotedMangledRdnW(pszrdn: super::super::Foundation::PWSTR, cchrdn: u32, pguid: *mut ::windows::runtime::GUID, pedsmanglefor: *mut DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsA(servername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, domainguid: *const ::windows::runtime::GUID, dsaguid: *const ::windows::runtime::GUID, dnshostname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsW(servername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, domainguid: *const ::windows::runtime::GUID, dsaguid: *const ::windows::runtime::GUID, dnshostname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsEnumerateDomainTrustsA(servername: super::super::Foundation::PSTR, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSA, domaincount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsEnumerateDomainTrustsW(servername: super::super::Foundation::PWSTR, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSW, domaincount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsFreeDomainControllerInfoA(infolevel: u32, cinfo: u32, pinfo: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsFreeDomainControllerInfoW(infolevel: u32, cinfo: u32, pinfo: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeNameResultA(presult: *const DS_NAME_RESULTA);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeNameResultW(presult: *const DS_NAME_RESULTW);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsFreePasswordCredentials(authidentity: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSchemaGuidMapA(pguidmap: *const DS_SCHEMA_GUID_MAPA);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSchemaGuidMapW(pguidmap: *const DS_SCHEMA_GUID_MAPW);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSpnArrayA(cspn: u32, rpszspn: *mut super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsFreeSpnArrayW(cspn: u32, rpszspn: *mut super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsGetDcCloseW(getdccontexthandle: GetDcContextHandle);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcNameA(computername: super::super::Foundation::PSTR, domainname: super::super::Foundation::PSTR, domainguid: *const ::windows::runtime::GUID, sitename: super::super::Foundation::PSTR, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcNameW(computername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, domainguid: *const ::windows::runtime::GUID, sitename: super::super::Foundation::PWSTR, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextA(getdccontexthandle: super::super::Foundation::HANDLE, sockaddresscount: *mut u32, sockaddresses: *mut *mut super::WinSock::SOCKET_ADDRESS, dnshostname: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextW(getdccontexthandle: super::super::Foundation::HANDLE, sockaddresscount: *mut u32, sockaddresses: *mut *mut super::WinSock::SOCKET_ADDRESS, dnshostname: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenA(dnsname: super::super::Foundation::PSTR, optionflags: u32, sitename: super::super::Foundation::PSTR, domainguid: *const ::windows::runtime::GUID, dnsforestname: super::super::Foundation::PSTR, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenW(dnsname: super::super::Foundation::PWSTR, optionflags: u32, sitename: super::super::Foundation::PWSTR, domainguid: *const ::windows::runtime::GUID, dnsforestname: super::super::Foundation::PWSTR, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcSiteCoverageA(servername: super::super::Foundation::PSTR, entrycount: *mut u32, sitenames: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcSiteCoverageW(servername: super::super::Foundation::PWSTR, entrycount: *mut u32, sitenames: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDomainControllerInfoA(hds: super::super::Foundation::HANDLE, domainname: super::super::Foundation::PSTR, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDomainControllerInfoW(hds: super::super::Foundation::HANDLE, domainname: super::super::Foundation::PWSTR, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn DsGetForestTrustInformationW(servername: super::super::Foundation::PWSTR, trusteddomainname: super::super::Foundation::PWSTR, flags: u32, foresttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetFriendlyClassName(pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn DsGetIcon(dwflags: u32, pszobjectclass: super::super::Foundation::PWSTR, cximage: i32, cyimage: i32) -> super::super::UI::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetRdnW(ppdn: *mut super::super::Foundation::PWSTR, pcdn: *mut u32, ppkey: *mut super::super::Foundation::PWSTR, pckey: *mut u32, ppval: *mut super::super::Foundation::PWSTR, pcval: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSiteNameA(computername: super::super::Foundation::PSTR, sitename: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSiteNameW(computername: super::super::Foundation::PWSTR, sitename: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSpnA(servicetype: DS_SPN_NAME_TYPE, serviceclass: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, instanceport: u16, cinstancenames: u16, pinstancenames: *const super::super::Foundation::PSTR, pinstanceports: *const u16, pcspn: *mut u32, prpszspn: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetSpnW(servicetype: DS_SPN_NAME_TYPE, serviceclass: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, instanceport: u16, cinstancenames: u16, pinstancenames: *const super::super::Foundation::PWSTR, pinstanceports: *const u16, pcspn: *mut u32, prpszspn: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsInheritSecurityIdentityA(hds: super::super::Foundation::HANDLE, flags: u32, srcprincipal: super::super::Foundation::PSTR, dstprincipal: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsInheritSecurityIdentityW(hds: super::super::Foundation::HANDLE, flags: u32, srcprincipal: super::super::Foundation::PWSTR, dstprincipal: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledDnA(pszdn: super::super::Foundation::PSTR, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledDnW(pszdn: super::super::Foundation::PWSTR, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledRdnValueA(pszrdn: super::super::Foundation::PSTR, crdn: u32, edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsIsMangledRdnValueW(pszrdn: super::super::Foundation::PWSTR, crdn: u32, edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListDomainsInSiteA(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PSTR, ppdomains: *mut *mut DS_NAME_RESULTA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListDomainsInSiteW(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PWSTR, ppdomains: *mut *mut DS_NAME_RESULTW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListInfoForServerA(hds: super::super::Foundation::HANDLE, server: super::super::Foundation::PSTR, ppinfo: *mut *mut DS_NAME_RESULTA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListInfoForServerW(hds: super::super::Foundation::HANDLE, server: super::super::Foundation::PWSTR, ppinfo: *mut *mut DS_NAME_RESULTW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListRolesA(hds: super::super::Foundation::HANDLE, pproles: *mut *mut DS_NAME_RESULTA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListRolesW(hds: super::super::Foundation::HANDLE, pproles: *mut *mut DS_NAME_RESULTW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersForDomainInSiteA(hds: super::super::Foundation::HANDLE, domain: super::super::Foundation::PSTR, site: super::super::Foundation::PSTR, ppservers: *mut *mut DS_NAME_RESULTA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersForDomainInSiteW(hds: super::super::Foundation::HANDLE, domain: super::super::Foundation::PWSTR, site: super::super::Foundation::PWSTR, ppservers: *mut *mut DS_NAME_RESULTW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersInSiteA(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PSTR, ppservers: *mut *mut DS_NAME_RESULTA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListServersInSiteW(hds: super::super::Foundation::HANDLE, site: super::super::Foundation::PWSTR, ppservers: *mut *mut DS_NAME_RESULTW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListSitesA(hds: super::super::Foundation::HANDLE, ppsites: *mut *mut DS_NAME_RESULTA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsListSitesW(hds: super::super::Foundation::HANDLE, ppsites: *mut *mut DS_NAME_RESULTW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakePasswordCredentialsA(user: super::super::Foundation::PSTR, domain: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakePasswordCredentialsW(user: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakeSpnA(serviceclass: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, instancename: super::super::Foundation::PSTR, instanceport: u16, referrer: super::super::Foundation::PSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMakeSpnW(serviceclass: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, instancename: super::super::Foundation::PWSTR, instanceport: u16, referrer: super::super::Foundation::PWSTR, pcspnlength: *mut u32, pszspn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMapSchemaGuidsA(hds: super::super::Foundation::HANDLE, cguids: u32, rguids: *const ::windows::runtime::GUID, ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMapSchemaGuidsW(hds: super::super::Foundation::HANDLE, cguids: u32, rguids: *const ::windows::runtime::GUID, ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn DsMergeForestTrustInformationW(domainname: super::super::Foundation::PWSTR, newforesttrustinfo: *const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo: *const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION, mergedforesttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuerySitesByCostA(hds: super::super::Foundation::HANDLE, pszfromsite: super::super::Foundation::PSTR, rgsztosites: *const super::super::Foundation::PSTR, ctosites: u32, dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuerySitesByCostW(hds: super::super::Foundation::HANDLE, pwszfromsite: super::super::Foundation::PWSTR, rgwsztosites: *const super::super::Foundation::PWSTR, ctosites: u32, dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsQuerySitesFree(rgsiteinfo: *const DS_SITE_COST_INFO);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuoteRdnValueA(cunquotedrdnvaluelength: u32, psunquotedrdnvalue: super::super::Foundation::PSTR, pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsQuoteRdnValueW(cunquotedrdnvaluelength: u32, psunquotedrdnvalue: super::super::Foundation::PWSTR, pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsDomainA(hds: super::super::Foundation::HANDLE, domaindn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsDomainW(hds: super::super::Foundation::HANDLE, domaindn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsServerA(hds: super::super::Foundation::HANDLE, serverdn: super::super::Foundation::PSTR, domaindn: super::super::Foundation::PSTR, flastdcindomain: *mut super::super::Foundation::BOOL, fcommit: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRemoveDsServerW(hds: super::super::Foundation::HANDLE, serverdn: super::super::Foundation::PWSTR, domaindn: super::super::Foundation::PWSTR, flastdcindomain: *mut super::super::Foundation::BOOL, fcommit: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaAddA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, sourcedsadn: super::super::Foundation::PSTR, transportdn: super::super::Foundation::PSTR, sourcedsaaddress: super::super::Foundation::PSTR, pschedule: *const SCHEDULE, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaAddW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, sourcedsadn: super::super::Foundation::PWSTR, transportdn: super::super::Foundation::PWSTR, sourcedsaaddress: super::super::Foundation::PWSTR, pschedule: *const SCHEDULE, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaConsistencyCheck(hds: super::super::Foundation::HANDLE, taskid: DS_KCC_TASKID, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaDelA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, dsasrc: super::super::Foundation::PSTR, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaDelW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, dsasrc: super::super::Foundation::PWSTR, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsReplicaFreeInfo(infotype: DS_REPL_INFO_TYPE, pinfo: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaGetInfo2W(hds: super::super::Foundation::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: super::super::Foundation::PWSTR, puuidforsourcedsaobjguid: *const ::windows::runtime::GUID, pszattributename: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, dwflags: u32, dwenumerationcontext: u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaGetInfoW(hds: super::super::Foundation::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: super::super::Foundation::PWSTR, puuidforsourcedsaobjguid: *const ::windows::runtime::GUID, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuidsourcedsa: *const ::windows::runtime::GUID, transportdn: super::super::Foundation::PSTR, sourcedsaaddress: super::super::Foundation::PSTR, pschedule: *const SCHEDULE, replicaflags: u32, modifyfields: u32, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuidsourcedsa: *const ::windows::runtime::GUID, transportdn: super::super::Foundation::PWSTR, sourcedsaaddress: super::super::Foundation::PWSTR, pschedule: *const SCHEDULE, replicaflags: u32, modifyfields: u32, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuiddsasrc: *const ::windows::runtime::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllA(hds: super::super::Foundation::HANDLE, psznamecontext: super::super::Foundation::PSTR, ulflags: u32, pfncallback: isize, pcallbackdata: *const ::core::ffi::c_void, perrors: *mut *mut *mut DS_REPSYNCALL_ERRINFOA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllW(hds: super::super::Foundation::HANDLE, psznamecontext: super::super::Foundation::PWSTR, ulflags: u32, pfncallback: isize, pcallbackdata: *const ::core::ffi::c_void, perrors: *mut *mut *mut DS_REPSYNCALL_ERRINFOW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuiddsasrc: *const ::windows::runtime::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, dsadest: super::super::Foundation::PSTR, puuiddsadest: *const ::windows::runtime::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, dsadest: super::super::Foundation::PWSTR, puuiddsadest: *const ::windows::runtime::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuiddsasrc: *const ::windows::runtime::GUID, uloptions: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuiddsasrc: *const ::windows::runtime::GUID, uloptions: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn DsRoleFreeMemory(buffer: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsRoleGetPrimaryDomainInformation(lpserver: super::super::Foundation::PWSTR, infolevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsServerRegisterSpnA(operation: DS_SPN_WRITE_OP, serviceclass: super::super::Foundation::PSTR, userobjectdn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsServerRegisterSpnW(operation: DS_SPN_WRITE_OP, serviceclass: super::super::Foundation::PWSTR, userobjectdn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnBindA(phds: *const super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnBindW(phds: *const super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnquoteRdnValueA(cquotedrdnvaluelength: u32, psquotedrdnvalue: super::super::Foundation::PSTR, pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsUnquoteRdnValueW(cquotedrdnvaluelength: u32, psquotedrdnvalue: super::super::Foundation::PWSTR, pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsValidateSubnetNameA(subnetname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsValidateSubnetNameW(subnetname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsWriteAccountSpnA(hds: super::super::Foundation::HANDLE, operation: DS_SPN_WRITE_OP, pszaccount: super::super::Foundation::PSTR, cspn: u32, rpszspn: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsWriteAccountSpnW(hds: super::super::Foundation::HANDLE, operation: DS_SPN_WRITE_OP, pszaccount: super::super::Foundation::PWSTR, cspn: u32, rpszspn: *const super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeADsMem(pmem: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeADsStr(pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn PropVariantToAdsType(pvariant: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, dwnumvariant: u32, ppadsvalues: *mut *mut ADSVALUE, pdwnumvalues: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn ReallocADsMem(poldmem: *mut ::core::ffi::c_void, cbold: u32, cbnew: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReallocADsStr(ppstr: *mut super::super::Foundation::PWSTR, pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn SecurityDescriptorToBinarySD(vvarsecdes: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, pdwsdlength: *mut u32, pszservername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
}
