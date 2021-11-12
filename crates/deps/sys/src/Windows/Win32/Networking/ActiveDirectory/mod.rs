#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_System_Ole`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsBuildEnumerator(padscontainer: IADsContainer, ppenumvariant: *mut super::super::System::Ole::IEnumVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayInt(lpdwobjecttypes: *mut u32, dwobjecttypes: u32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsBuildVarArrayStr(lpppathnames: *const super::super::Foundation::PWSTR, dwpathnames: u32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsDecodeBinaryData(szsrcdata: super::super::Foundation::PWSTR, ppbdestdata: *mut *mut u8, pdwdestlen: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsEncodeBinaryData(pbsrcdata: *mut u8, dwsrclen: u32, ppszdestdata: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ADsEnumerateNext(penumvariant: super::super::System::Ole::IEnumVARIANT, celements: u32, pvar: *mut super::super::System::Com::VARIANT, pcelementsfetched: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_System_Ole`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub fn ADsFreeEnumerator(penumvariant: super::super::System::Ole::IEnumVARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetLastError(lperror: *mut u32, lperrorbuf: super::super::Foundation::PWSTR, dwerrorbuflen: u32, lpnamebuf: super::super::Foundation::PWSTR, dwnamebuflen: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsGetObject(lpszpathname: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsOpenObject(lpszpathname: super::super::Foundation::PWSTR, lpszusername: super::super::Foundation::PWSTR, lpszpassword: super::super::Foundation::PWSTR, dwreserved: ADS_AUTHENTICATION_ENUM, riid: *const ::windows_sys::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropCheckIfWritable(pwzattr: super::super::Foundation::PWSTR, pwritableattrs: *const ADS_ATTR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ADsPropCreateNotifyObj(pappthddataobj: super::super::System::Com::IDataObject, pwzadsobjname: super::super::Foundation::PWSTR, phnotifyobj: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ADsPropGetInitInfo(hnotifyobj: super::super::Foundation::HWND, pinitparams: *mut ADSPROPINITPARAMS) -> super::super::Foundation::BOOL;
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
    pub fn AdsTypeToPropVariant(padsvalues: *mut ADSVALUE, dwnumvalues: u32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn AllocADsMem(cb: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocADsStr(pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn BinarySDToSecurityDescriptor(psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, pvarsec: *mut super::super::System::Com::VARIANT, pszservername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
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
    pub fn DsBindByInstanceA(servername: super::super::Foundation::PSTR, annotation: super::super::Foundation::PSTR, instanceguid: *const ::windows_sys::core::GUID, dnsdomainname: super::super::Foundation::PSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsBindByInstanceW(servername: super::super::Foundation::PWSTR, annotation: super::super::Foundation::PWSTR, instanceguid: *const ::windows_sys::core::GUID, dnsdomainname: super::super::Foundation::PWSTR, authidentity: *const ::core::ffi::c_void, serviceprincipalname: super::super::Foundation::PWSTR, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32;
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
    pub fn DsBrowseForContainerA(pinfo: *mut DSBROWSEINFOA) -> i32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn DsBrowseForContainerW(pinfo: *mut DSBROWSEINFOW) -> i32;
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
    pub fn DsCrackUnquotedMangledRdnA(pszrdn: super::super::Foundation::PSTR, cchrdn: u32, pguid: *mut ::windows_sys::core::GUID, pedsmanglefor: *mut DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsCrackUnquotedMangledRdnW(pszrdn: super::super::Foundation::PWSTR, cchrdn: u32, pguid: *mut ::windows_sys::core::GUID, pedsmanglefor: *mut DS_MANGLE_FOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsA(servername: super::super::Foundation::PSTR, dnsdomainname: super::super::Foundation::PSTR, domainguid: *const ::windows_sys::core::GUID, dsaguid: *const ::windows_sys::core::GUID, dnshostname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsDeregisterDnsHostRecordsW(servername: super::super::Foundation::PWSTR, dnsdomainname: super::super::Foundation::PWSTR, domainguid: *const ::windows_sys::core::GUID, dsaguid: *const ::windows_sys::core::GUID, dnshostname: super::super::Foundation::PWSTR) -> u32;
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
    pub fn DsGetDcNameA(computername: super::super::Foundation::PSTR, domainname: super::super::Foundation::PSTR, domainguid: *const ::windows_sys::core::GUID, sitename: super::super::Foundation::PSTR, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcNameW(computername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, domainguid: *const ::windows_sys::core::GUID, sitename: super::super::Foundation::PWSTR, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextA(getdccontexthandle: super::super::Foundation::HANDLE, sockaddresscount: *mut u32, sockaddresses: *mut *mut super::WinSock::SOCKET_ADDRESS, dnshostname: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn DsGetDcNextW(getdccontexthandle: super::super::Foundation::HANDLE, sockaddresscount: *mut u32, sockaddresses: *mut *mut super::WinSock::SOCKET_ADDRESS, dnshostname: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenA(dnsname: super::super::Foundation::PSTR, optionflags: u32, sitename: super::super::Foundation::PSTR, domainguid: *const ::windows_sys::core::GUID, dnsforestname: super::super::Foundation::PSTR, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsGetDcOpenW(dnsname: super::super::Foundation::PWSTR, optionflags: u32, sitename: super::super::Foundation::PWSTR, domainguid: *const ::windows_sys::core::GUID, dnsforestname: super::super::Foundation::PWSTR, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32;
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
    pub fn DsGetFriendlyClassName(pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows_sys::core::HRESULT;
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
    pub fn DsMapSchemaGuidsA(hds: super::super::Foundation::HANDLE, cguids: u32, rguids: *const ::windows_sys::core::GUID, ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsMapSchemaGuidsW(hds: super::super::Foundation::HANDLE, cguids: u32, rguids: *const ::windows_sys::core::GUID, ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPW) -> u32;
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
    pub fn DsReplicaGetInfo2W(hds: super::super::Foundation::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: super::super::Foundation::PWSTR, puuidforsourcedsaobjguid: *const ::windows_sys::core::GUID, pszattributename: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, dwflags: u32, dwenumerationcontext: u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaGetInfoW(hds: super::super::Foundation::HANDLE, infotype: DS_REPL_INFO_TYPE, pszobject: super::super::Foundation::PWSTR, puuidforsourcedsaobjguid: *const ::windows_sys::core::GUID, ppinfo: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuidsourcedsa: *const ::windows_sys::core::GUID, transportdn: super::super::Foundation::PSTR, sourcedsaaddress: super::super::Foundation::PSTR, pschedule: *const SCHEDULE, replicaflags: u32, modifyfields: u32, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaModifyW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuidsourcedsa: *const ::windows_sys::core::GUID, transportdn: super::super::Foundation::PWSTR, sourcedsaaddress: super::super::Foundation::PWSTR, pschedule: *const SCHEDULE, replicaflags: u32, modifyfields: u32, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuiddsasrc: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllA(hds: super::super::Foundation::HANDLE, psznamecontext: super::super::Foundation::PSTR, ulflags: u32, pfncallback: isize, pcallbackdata: *const ::core::ffi::c_void, perrors: *mut *mut *mut DS_REPSYNCALL_ERRINFOA) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncAllW(hds: super::super::Foundation::HANDLE, psznamecontext: super::super::Foundation::PWSTR, ulflags: u32, pfncallback: isize, pcallbackdata: *const ::core::ffi::c_void, perrors: *mut *mut *mut DS_REPSYNCALL_ERRINFOW) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaSyncW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuiddsasrc: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, dsadest: super::super::Foundation::PSTR, puuiddsadest: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaUpdateRefsW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, dsadest: super::super::Foundation::PWSTR, puuiddsadest: *const ::windows_sys::core::GUID, options: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsA(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PSTR, puuiddsasrc: *const ::windows_sys::core::GUID, uloptions: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DsReplicaVerifyObjectsW(hds: super::super::Foundation::HANDLE, namecontext: super::super::Foundation::PWSTR, puuiddsasrc: *const ::windows_sys::core::GUID, uloptions: u32) -> u32;
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
    pub fn PropVariantToAdsType(pvariant: *mut super::super::System::Com::VARIANT, dwnumvariant: u32, ppadsvalues: *mut *mut ADSVALUE, pdwnumvalues: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
    pub fn ReallocADsMem(poldmem: *mut ::core::ffi::c_void, cbold: u32, cbnew: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReallocADsStr(ppstr: *mut super::super::Foundation::PWSTR, pstr: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_ActiveDirectory`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn SecurityDescriptorToBinarySD(vvarsecdes: super::super::System::Com::VARIANT, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, pdwsdlength: *mut u32, pszservername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_CONTROL_ACCESS: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_CREATE_CHILD: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_DELETE_CHILD: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_DELETE_TREE: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_LIST: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_LIST_OBJECT: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_OPEN: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_READ_PROP: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_SELF: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ACTRL_DS_WRITE_PROP: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH: u32 = 0u32;
pub struct ADSI_DIALECT_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADSPROPERROR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADSPROPINITPARAMS(i32);
pub struct ADSTYPEENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADSVALUE(i32);
pub struct ADS_ACEFLAG_ENUM(i32);
pub struct ADS_ACETYPE_ENUM(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_ATTR_APPEND: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_ATTR_CLEAR: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_ATTR_DEF(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_ATTR_DELETE: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_ATTR_INFO(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_ATTR_UPDATE: u32 = 2u32;
pub struct ADS_AUTHENTICATION_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_BACKLINK(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_CASEIGNORE_LIST(i32);
pub struct ADS_CHASE_REFERRALS_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_CLASS_DEF(i32);
pub struct ADS_DEREFENUM(i32);
pub struct ADS_DISPLAY_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_DN_WITH_BINARY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_DN_WITH_STRING(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_EMAIL(i32);
pub struct ADS_ESCAPE_MODE_ENUM(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_EXT_INITCREDENTIALS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_EXT_INITIALIZE_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_EXT_MAXEXTDISPID: u32 = 16777215u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const ADS_EXT_MINEXTDISPID: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_FAXNUMBER(i32);
pub struct ADS_FLAGTYPE_ENUM(i32);
pub struct ADS_FORMAT_ENUM(i32);
pub struct ADS_GROUP_TYPE_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_HOLD(i32);
pub struct ADS_NAME_INITTYPE_ENUM(i32);
pub struct ADS_NAME_TYPE_ENUM(i32);
pub struct ADS_NETADDRESS(i32);
pub struct ADS_NT_SECURITY_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_OBJECT_INFO(i32);
pub struct ADS_OCTET_LIST(i32);
pub struct ADS_OCTET_STRING(i32);
pub struct ADS_OPTION_ENUM(i32);
pub struct ADS_PASSWORD_ENCODING_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_PATH(i32);
pub struct ADS_PATHTYPE_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_POSTALADDRESS(i32);
pub struct ADS_PREFERENCES_ENUM(i32);
pub struct ADS_PROPERTY_OPERATION_ENUM(i32);
pub struct ADS_PROV_SPECIFIC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_REPLICAPOINTER(i32);
pub struct ADS_RIGHTS_ENUM(i32);
pub struct ADS_SCOPEENUM(i32);
pub struct ADS_SD_CONTROL_ENUM(i32);
pub struct ADS_SD_FORMAT_ENUM(i32);
pub struct ADS_SD_REVISION_ENUM(i32);
pub struct ADS_SEARCHPREF_ENUM(i32);
pub struct ADS_SECURITY_INFO_ENUM(i32);
pub struct ADS_SETTYPE_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_SORTKEY(i32);
pub struct ADS_STATUSENUM(i32);
pub struct ADS_SYSTEMFLAG_ENUM(i32);
pub struct ADS_TIMESTAMP(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_TYPEDNAME(i32);
pub struct ADS_USER_FLAG_ENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_VLV(i32);
pub struct ADSystemInfo(i32);
pub struct ADsSecurityUtility(i32);
pub struct AccessControlEntry(i32);
pub struct AccessControlList(i32);
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
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQFF_ISOPTIONAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQFF_NOGLOBALPAGES: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CQFORM(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CQPAGE(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_CLEARFORM: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_ENABLE: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_GETPARAMETERS: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_HANDLERSPECIFIC: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_HELP: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_INITIALIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_PERSIST: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_RELEASE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const CQPM_SETDEFAULTPARAMETERS: u32 = 9u32;
pub struct CaseIgnoreList(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DBDTF_RETURNEXTERNAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DBDTF_RETURNFQDN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DBDTF_RETURNINBOUND: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DBDTF_RETURNINOUTBOUND: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DBDTF_RETURNMIXEDDOMAINS: u32 = 2u32;
pub struct DNWithBinary(i32);
pub struct DNWithString(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAINDESC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAIN_CONTROLLER_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAIN_CONTROLLER_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAIN_TREE(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NEWOBJ_CTX_CLEANUP: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NEWOBJ_CTX_COMMIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NEWOBJ_CTX_POSTCOMMIT: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NEWOBJ_CTX_PRECOMMIT: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct DSA_NEWOBJ_DISPINFO(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NOTIFY_DEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NOTIFY_FLAG_ADDITIONAL_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NOTIFY_FLAG_FORCE_ADDITIONAL_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NOTIFY_MOV: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NOTIFY_PROP: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSA_NOTIFY_REN: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBF_DISPLAYNAME: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBF_ICONLOCATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBF_STATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBID_BANNER: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBID_CONTAINERLIST: u32 = 257u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DSBITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSBITEMW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_CHECKBOXES: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_DONTSIGNSEAL: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_ENTIREDIRECTORY: u32 = 589824u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_EXPANDONOPEN: u32 = 262144u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_HASCREDENTIALS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_IGNORETREATASLEAF: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_INCLUDEHIDDEN: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_NOBUTTONS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_NOLINES: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_NOLINESATROOT: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_NOROOT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_RETURNOBJECTCLASS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_RETURN_FORMAT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBI_SIMPLEAUTHENTICATE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBM_CHANGEIMAGESTATE: u32 = 102u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBM_CONTEXTMENU: u32 = 104u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBM_HELP: u32 = 103u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBM_QUERYINSERT: u32 = 100u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBM_QUERYINSERTA: u32 = 101u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBM_QUERYINSERTW: u32 = 100u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct DSBROWSEINFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct DSBROWSEINFOW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBS_CHECKED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBS_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSBS_ROOT: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSB_MAX_DISPLAYNAME_CHARS: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSCCIF_HASWIZARDDIALOG: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSCCIF_HASWIZARDPRIMARYPAGE: u32 = 2u32;
pub struct DSCLASSCREATIONINFO(i32);
pub struct DSCOLUMN(i32);
pub struct DSDISPLAYSPECOPTIONS(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSDSOF_DONTSIGNSEAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSDSOF_DSAVAILABLE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSDSOF_HASUSERANDSERVERINFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSDSOF_SIMPLEAUTHENTICATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSECAF_NOTLISTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSGIF_DEFAULTISCONTAINER: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSGIF_GETDEFAULTICON: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSGIF_ISDISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSGIF_ISMASK: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSGIF_ISNORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSGIF_ISOPEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSICCF_IGNORETREATASLEAF: u32 = 1u32;
pub struct DSOBJECT(i32);
pub struct DSOBJECTNAMES(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOBJECT_ISCONTAINER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOBJECT_READONLYPAGES: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_ALL_APP_PACKAGES: u32 = 2281701376u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_ALL_WELLKNOWN_SIDS: u32 = 2147614720u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_ANONYMOUS: u32 = 2147483712u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_AUTHENTICATED_USER: u32 = 2147483680u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_BATCH: u32 = 2147483776u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_COMPUTERS: u32 = 2147483656u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_GROUP: u32 = 2147484160u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_OWNER: u32 = 2147483904u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_DIALUP: u32 = 2147484672u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_EXCLUDE_BUILTIN_GROUPS: u32 = 2147516416u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_GLOBAL_GROUPS: u32 = 2147483652u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_IIS_APP_POOL: u32 = 2214592512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_INTERACTIVE: u32 = 2147485696u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_INTERNET_USER: u32 = 2149580800u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_ACCOUNTS: u32 = 2415919104u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_GROUPS: u32 = 2147483650u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_LOGON: u32 = 2164260864u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_SERVICE: u32 = 2147745792u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_NETWORK: u32 = 2147487744u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_NETWORK_SERVICE: u32 = 2148007936u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_OWNER_RIGHTS: u32 = 2151677952u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_REMOTE_LOGON: u32 = 2148532224u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_SERVICE: u32 = 2147491840u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_SERVICES: u32 = 2155872256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_SYSTEM: u32 = 2147500032u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_TERMINAL_SERVER: u32 = 2147549184u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_THIS_ORG_CERT: u32 = 2181038080u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_USERS: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_DOWNLEVEL_FILTER_WORLD: u32 = 2147483664u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_BUILTIN_GROUPS: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_COMPUTERS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_CONTACTS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_DL: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_SE: u32 = 512u32;
pub struct DSOP_FILTER_FLAGS(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_GLOBAL_GROUPS_DL: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_GLOBAL_GROUPS_SE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_INCLUDE_ADVANCED_VIEW: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_SERVICE_ACCOUNTS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_UNIVERSAL_GROUPS_DL: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_UNIVERSAL_GROUPS_SE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_USERS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FILTER_WELL_KNOWN_PRINCIPALS: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FLAG_MULTISELECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_FLAG_SKIP_TARGET_COMPUTER_DC_CHECK: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DSOP_INIT_INFO(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_COMPUTERS: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_CONTACTS: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_GROUPS: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_SERVICE_ACCOUNTS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_USERS: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_STARTING_SCOPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_WANT_DOWNLEVEL_BUILTIN_PATH: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_GC: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_LDAP: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_WINNT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_FLAG_WANT_SID_PATH: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DSOP_SCOPE_INIT_INFO(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_DOWNLEVEL_JOINED_DOMAIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_ENTERPRISE_DOMAIN: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_EXTERNAL_DOWNLEVEL_DOMAIN: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_EXTERNAL_UPLEVEL_DOMAIN: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_GLOBAL_CATALOG: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_TARGET_COMPUTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_UPLEVEL_JOINED_DOMAIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_USER_ENTERED_DOWNLEVEL_SCOPE: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_USER_ENTERED_UPLEVEL_SCOPE: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSOP_SCOPE_TYPE_WORKGROUP: u32 = 128u32;
pub struct DSOP_UPLEVEL_FILTER_FLAGS(i32);
pub struct DSPROPERTYPAGEINFO(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSPROVIDER_ADVANCED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSPROVIDER_AD_LDS: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSPROVIDER_UNUSED_0: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSPROVIDER_UNUSED_1: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSPROVIDER_UNUSED_2: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSPROVIDER_UNUSED_3: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPF_ENABLEADMINFEATURES: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPF_ENABLEADVANCEDFEATURES: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPF_HASCREDENTIALS: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPF_NOCHOOSECOLUMNS: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPF_NOSAVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPF_SAVELOCATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPF_SHOWHIDDENOBJECTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPM_GETCLASSLIST: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSQPM_HELPTOPICS: u32 = 268435457u32;
pub struct DSQUERYCLASSLIST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSQUERYINITPARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSQUERYPARAMS(i32);
pub struct DSROLE_MACHINE_ROLE(i32);
pub struct DSROLE_OPERATION_STATE(i32);
pub struct DSROLE_OPERATION_STATE_INFO(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: u32 = 16777216u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC(i32);
pub struct DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSROLE_PRIMARY_DS_MIXED_MODE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSROLE_PRIMARY_DS_READONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSROLE_PRIMARY_DS_RUNNING: u32 = 1u32;
pub struct DSROLE_SERVER_STATE(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSROLE_UPGRADE_IN_PROGRESS: u32 = 4u32;
pub struct DSROLE_UPGRADE_STATUS_INFO(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSSSF_DONTSIGNSEAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSSSF_DSAVAILABLE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DSSSF_SIMPLEAUTHENTICATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_AVOID_SELF: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BACKGROUND_ONLY: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_LONGHORN: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2000: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2003: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2008: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2008R2: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2012: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2012R2: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN2016: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN7: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WIN8: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WINBLUE: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_BEHAVIOR_WINTHRESHOLD: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_CLOSEST_FLAG: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: u32 = 524288u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DIRECTORY_SERVICE_PREFERRED: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DIRECTORY_SERVICE_REQUIRED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DNS_CONTROLLER_FLAG: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DNS_DOMAIN_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DNS_FOREST_FLAG: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_1A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_1W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_2A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_2W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_3A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_3W(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DOMAIN_DIRECT_INBOUND: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DOMAIN_DIRECT_OUTBOUND: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DOMAIN_IN_FOREST: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DOMAIN_NATIVE_MODE: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DOMAIN_PRIMARY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DOMAIN_TREE_ROOT: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_TRUSTSA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_TRUSTSW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DS_10_FLAG: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DS_8_FLAG: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DS_9_FLAG: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_DS_FLAG: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_EXIST_ADVISORY_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_FORCE_REDISCOVERY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_GC_FLAG: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_GC_SERVER_REQUIRED: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_GFTI_UPDATE_TDO: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_GFTI_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_GOOD_TIMESERV_FLAG: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_GOOD_TIMESERV_PREFERRED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_INSTANCETYPE_IS_NC_HEAD: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_INSTANCETYPE_NC_COMING: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_INSTANCETYPE_NC_GOING: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_INSTANCETYPE_NC_IS_WRITEABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_IP_REQUIRED: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_IS_DNS_NAME: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_IS_FLAT_NAME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_KCC_FLAG_ASYNC_OP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_KCC_FLAG_DAMPED: u32 = 2u32;
pub struct DS_KCC_TASKID(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_KDC_FLAG: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_KDC_REQUIRED: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_KEY_LIST_FLAG: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_KEY_LIST_SUPPORT_REQUIRED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_LDAP_FLAG: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_LIST_ACCOUNT_OBJECT_FOR_SERVER: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_LIST_DNS_HOST_NAME_FOR_SERVER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_LIST_DSA_OBJECT_FOR_SERVER: u32 = 0u32;
pub struct DS_MANGLE_FOR(i32);
pub struct DS_NAME_ERROR(i32);
pub struct DS_NAME_FLAGS(i32);
pub struct DS_NAME_FORMAT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_NAME_RESULTA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_NAME_RESULTW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_NAME_RESULT_ITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_NAME_RESULT_ITEMW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_NDNC_FLAG: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_NOTIFY_AFTER_SITE_RECORDS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_ONLY_DO_SITE_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_ONLY_LDAP_NEEDED: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_PDC_FLAG: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_PDC_REQUIRED: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_PING_FLAGS: u32 = 1048575u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_ASYNCHRONOUS_REPLICA: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_CRITICAL: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_DISABLE_NOTIFICATION: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_DISABLE_PERIODIC: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_INITIAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_INTERSITE_MESSAGING: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_NEVER_NOTIFY: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_NONGC_RO_REPLICA: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_PERIODIC: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_SELECT_SECRETS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_TWO_WAY: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_USE_COMPRESSION: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPADD_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPDEL_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPDEL_IGNORE_ERRORS: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPDEL_INTERSITE_MESSAGING: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPDEL_LOCAL_ONLY: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPDEL_NO_SOURCE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPDEL_REF_OK: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPDEL_WRITEABLE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA_EXT(i32);
pub struct DS_REPL_CURSOR(i32);
pub struct DS_REPL_CURSORS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSORS_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSORS_3W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_3W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_BLOB(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS: u32 = 1u32;
pub struct DS_REPL_INFO_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILURESW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILUREW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILUREW_BLOB(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_COMPRESS_CHANGES: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_DISABLE_SCHEDULED_SYNC: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_DO_SCHEDULED_SYNCS: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_FULL_SYNC_IN_PROGRESS: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_FULL_SYNC_NEXT_PACKET: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_GCSPN: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_NEVER_SYNCED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_NONGC_RO_REPLICA: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_PREEMPTED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_RETURN_OBJECT_PARENTS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_SELECT_SECRETS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_SYNC_ON_STARTUP: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_TWO_WAY_SYNC: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPL_NBR_WRITEABLE: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORSW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORW_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OBJ_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OBJ_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OPW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OPW_BLOB(i32);
pub struct DS_REPL_OP_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_PENDING_OPSW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_QUEUE_STATISTICSW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_BLOB_EXT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_EXT(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_UPDATE_ADDRESS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_UPDATE_FLAGS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_UPDATE_INSTANCE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_UPDATE_RESULT: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_UPDATE_SCHEDULE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_UPDATE_TRANSPORT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPMOD_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_CROSS_SITE_BOUNDARIES: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_DO_NOT_SYNC: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPSYNCALL_ERRINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPSYNCALL_ERRINFOW(i32);
pub struct DS_REPSYNCALL_ERROR(i32);
pub struct DS_REPSYNCALL_EVENT(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_ID_SERVERS_BY_DN: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_NO_OPTIONS: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_PUSH_CHANGES_OUTWARD: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_SKIP_INITIAL_CHECK: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPSYNCALL_SYNCA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPSYNCALL_SYNCW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPSYNCALL_UPDATEA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPSYNCALL_UPDATEW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_ABANDONED: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_ADD_REFERENCE: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_ASYNCHRONOUS_REPLICA: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_CRITICAL: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_FORCE: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_FULL: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_FULL_IN_PROGRESS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_INITIAL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_INITIAL_IN_PROGRESS: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_INTERSITE_MESSAGING: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_NEVER_COMPLETED: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_NEVER_NOTIFY: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_NONGC_RO_REPLICA: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_NOTIFICATION: u32 = 524288u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_NO_DISCARD: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_PARTIAL_ATTRIBUTE_SET: u32 = 131072u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_PERIODIC: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_PREEMPTED: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_REQUEUE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_SELECT_SECRETS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_TWO_WAY: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_URGENT: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_USE_COMPRESSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPSYNC_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPUPD_ADD_REFERENCE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPUPD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPUPD_DELETE_REFERENCE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPUPD_REFERENCE_GCSPN: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_REPUPD_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_RETURN_DNS_NAME: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_RETURN_FLAT_NAME: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_ROLE_DOMAIN_OWNER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_ROLE_INFRASTRUCTURE_OWNER: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_ROLE_PDC_OWNER: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_ROLE_RID_OWNER: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_ROLE_SCHEMA_OWNER: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_SCHEMA_GUID_ATTR: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_SCHEMA_GUID_ATTR_SET: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_SCHEMA_GUID_CLASS: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_SCHEMA_GUID_CONTROL_RIGHT: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DS_SCHEMA_GUID_MAPA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DS_SCHEMA_GUID_MAPW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_SCHEMA_GUID_NOT_FOUND: u32 = 0u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DS_SELECTION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DS_SELECTION_LIST(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: u32 = 2048u32;
pub struct DS_SITE_COST_INFO(i32);
pub struct DS_SPN_NAME_TYPE(i32);
pub struct DS_SPN_WRITE_OP(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_TIMESERV_FLAG: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_TIMESERV_REQUIRED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_TRY_NEXTCLOSEST_SITE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_WEB_SERVICE_REQUIRED: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_WRITABLE_FLAG: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_WRITABLE_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const DS_WS_FLAG: u32 = 8192u32;
pub struct Email(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FACILITY_BACKUP: u32 = 2047u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FACILITY_NTDSB: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FACILITY_SYSTEM: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FLAG_DISABLABLE_OPTIONAL_FEATURE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FLAG_DOMAIN_OPTIONAL_FEATURE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FLAG_FOREST_OPTIONAL_FEATURE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FLAG_SERVER_OPTIONAL_FEATURE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FRSCONN_MAX_PRIORITY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const FRSCONN_PRIORITY_MASK: u32 = 1879048192u32;
pub struct FaxNumber(i32);
pub struct GetDcContextHandle(i32);
pub struct Hold(i32);
pub struct IADs(i32);
pub struct IADsADSystemInfo(i32);
pub struct IADsAccessControlEntry(i32);
pub struct IADsAccessControlList(i32);
pub struct IADsAcl(i32);
pub struct IADsAggregatee(i32);
pub struct IADsAggregator(i32);
pub struct IADsBackLink(i32);
pub struct IADsCaseIgnoreList(i32);
pub struct IADsClass(i32);
pub struct IADsCollection(i32);
pub struct IADsComputer(i32);
pub struct IADsComputerOperations(i32);
pub struct IADsContainer(i32);
pub struct IADsDNWithBinary(i32);
pub struct IADsDNWithString(i32);
pub struct IADsDeleteOps(i32);
pub struct IADsDomain(i32);
pub struct IADsEmail(i32);
pub struct IADsExtension(i32);
pub struct IADsFaxNumber(i32);
pub struct IADsFileService(i32);
pub struct IADsFileServiceOperations(i32);
pub struct IADsFileShare(i32);
pub struct IADsGroup(i32);
pub struct IADsHold(i32);
pub struct IADsLargeInteger(i32);
pub struct IADsLocality(i32);
pub struct IADsMembers(i32);
pub struct IADsNameTranslate(i32);
pub struct IADsNamespaces(i32);
pub struct IADsNetAddress(i32);
pub struct IADsO(i32);
pub struct IADsOU(i32);
pub struct IADsObjectOptions(i32);
pub struct IADsOctetList(i32);
pub struct IADsOpenDSObject(i32);
pub struct IADsPath(i32);
pub struct IADsPathname(i32);
pub struct IADsPostalAddress(i32);
pub struct IADsPrintJob(i32);
pub struct IADsPrintJobOperations(i32);
pub struct IADsPrintQueue(i32);
pub struct IADsPrintQueueOperations(i32);
pub struct IADsProperty(i32);
pub struct IADsPropertyEntry(i32);
pub struct IADsPropertyList(i32);
pub struct IADsPropertyValue(i32);
pub struct IADsPropertyValue2(i32);
pub struct IADsReplicaPointer(i32);
pub struct IADsResource(i32);
pub struct IADsSecurityDescriptor(i32);
pub struct IADsSecurityUtility(i32);
pub struct IADsService(i32);
pub struct IADsServiceOperations(i32);
pub struct IADsSession(i32);
pub struct IADsSyntax(i32);
pub struct IADsTimestamp(i32);
pub struct IADsTypedName(i32);
pub struct IADsUser(i32);
pub struct IADsWinNTSystemInfo(i32);
pub struct ICommonQuery(i32);
pub struct IDirectoryObject(i32);
pub struct IDirectorySchemaMgmt(i32);
pub struct IDirectorySearch(i32);
pub struct IDsAdminCreateObj(i32);
pub struct IDsAdminNewObj(i32);
pub struct IDsAdminNewObjExt(i32);
pub struct IDsAdminNewObjPrimarySite(i32);
pub struct IDsAdminNotifyHandler(i32);
pub struct IDsBrowseDomainTree(i32);
pub struct IDsDisplaySpecifier(i32);
pub struct IDsObjectPicker(i32);
pub struct IDsObjectPickerCredentials(i32);
pub struct IPersistQuery(i32);
pub struct IPrivateDispatch(i32);
pub struct IPrivateUnknown(i32);
pub struct IQueryForm(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct LPCQADDFORMSPROC(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct LPCQADDPAGESPROC(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct LPCQPAGEPROC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LPDSENUMATTRIBUTES(i32);
pub struct LargeInteger(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSAPI_BIND_ALLOW_DELEGATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSAPI_BIND_FIND_BINDING: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSAPI_BIND_FORCE_KERBEROS: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_GC_TOPOLOGY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_INTERSITE_TOPOLOGY: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_NO_REASON: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_RING_TOPOLOGY: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_IGNORE_SCHEDULE_MASK: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_IS_GENERATED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_RODC_TOPOLOGY: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_TWOWAY_SYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_USER_OWNED_SCHEDULE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSCONN_OPT_USE_NOTIFY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSDSA_OPT_BLOCK_RPC: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSDSA_OPT_DISABLE_INBOUND_REPL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSDSA_OPT_DISABLE_OUTBOUND_REPL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSDSA_OPT_DISABLE_SPN_REGISTRATION: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSDSA_OPT_GENERATE_OWN_TOPO: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSDSA_OPT_IS_GC: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSITECONN_OPT_DISABLE_COMPRESSION: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSITECONN_OPT_TWOWAY_SYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSITECONN_OPT_USE_NOTIFY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSITELINK_OPT_DISABLE_COMPRESSION: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSITELINK_OPT_TWOWAY_SYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSSITELINK_OPT_USE_NOTIFY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSTRANSPORT_OPT_BRIDGES_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const NTDSTRANSPORT_OPT_IGNORE_SCHEDULES: u32 = 1u32;
pub struct NameTranslate(i32);
pub struct NetAddress(i32);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct OPENQUERYWINDOW(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_DEFAULTFORM: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_HIDEMENUS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_HIDESEARCHUI: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_ISSUEONOPEN: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_LOADQUERY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_OKCANCEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_PARAMISPROPERTYBAG: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_REMOVEFORMS: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_REMOVESCOPES: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_SAVEQUERYONOK: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_SHOWOPTIONAL: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const OQWF_SINGLESELECT: u32 = 4u32;
pub struct OctetList(i32);
pub struct Path(i32);
pub struct Pathname(i32);
pub struct PostalAddress(i32);
pub struct PropertyEntry(i32);
pub struct PropertyValue(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const QUERYFORM_CHANGESFORMLIST: u64 = 1u64;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const QUERYFORM_CHANGESOPTFORMLIST: u64 = 2u64;
pub struct ReplicaPointer(i32);
pub struct SCHEDULE(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const SCHEDULE_BANDWIDTH: u32 = 1u32;
pub struct SCHEDULE_HEADER(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const SCHEDULE_INTERVAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const SCHEDULE_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const STATUS_SEVERITY_ERROR: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const STATUS_SEVERITY_INFORMATIONAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const STATUS_SEVERITY_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const STATUS_SEVERITY_WARNING: u32 = 2u32;
pub struct SecurityDescriptor(i32);
pub struct Timestamp(i32);
pub struct TypedName(i32);
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_APPLY: u32 = 2128u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_CHANGE: u32 = 2127u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_ERROR: u32 = 2134u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_EXIT: u32 = 2131u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_FOREGROUND: u32 = 2130u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_PAGEHWND: u32 = 2126u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_PAGEINIT: u32 = 2125u32;
#[doc = "*Required features: `Win32_Networking_ActiveDirectory`*"]
pub const WM_ADSPROP_NOTIFY_SETFOCUS: u32 = 2129u32;
pub struct WinNTSystemInfo(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ads_search_column(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ads_searchpref_info(i32);
