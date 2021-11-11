#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmAddGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmDeRegisterMProtocol(hprotocol: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmDeleteGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MgmGetFirstMfe(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MgmGetFirstMfeStats(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetMfe(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetMfeStats(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetNextMfe(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_NetworkManagement_IpHelper`*"]
    #[cfg(feature = "Win32_NetworkManagement_IpHelper")]
    pub fn MgmGetNextMfeStats(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MgmGetProtocolOnInterface(dwifindex: u32, dwifnexthopaddr: u32, pdwifprotocolid: *mut u32, pdwifcomponentid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmGroupEnumerationEnd(henum: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmGroupEnumerationGetNext(henum: super::super::Foundation::HANDLE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmGroupEnumerationStart(hprotocol: super::super::Foundation::HANDLE, metenumtype: MGM_ENUM_TYPES, phenumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmRegisterMProtocol(prpiinfo: *mut ::core::mem::ManuallyDrop<ROUTING_PROTOCOL_CONFIG>, dwprotocolid: u32, dwcomponentid: u32, phprotocol: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmReleaseInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MgmTakeInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionClearStats(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminConnectionEnum(hrasserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionEnumEx(hrasserver: isize, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut *mut RAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionGetInfo(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionGetInfoEx(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasconnection: *mut RAS_CONNECTION_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminConnectionRemoveQuarantine(hrasserver: super::super::Foundation::HANDLE, hrasconnection: super::super::Foundation::HANDLE, fisipaddress: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminDeregisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminDeviceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, lpdwtotalentries: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminEstablishDomainRasServer(pszdomain: super::super::Foundation::PWSTR, pszmachine: super::super::Foundation::PWSTR, benable: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminGetPDCServer(lpszdomain: super::super::Foundation::PWSTR, lpszserver: super::super::Foundation::PWSTR, lpszpdcserver: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceConnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE, fsynchronous: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceCreate(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDelete(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDeviceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lplpbuffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDeviceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceDisconnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminInterfaceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetCredentials(lpwsserver: super::super::Foundation::PWSTR, lpwsinterfacename: super::super::Foundation::PWSTR, lpwsusername: super::super::Foundation::PWSTR, lpwspassword: super::super::Foundation::PWSTR, lpwsdomainname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminInterfaceGetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetHandle(hmprserver: isize, lpwsinterfacename: super::super::Foundation::PWSTR, phinterface: *mut super::super::Foundation::HANDLE, fincludeclientinterfaces: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceQueryUpdateResult(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceSetCredentials(lpwsserver: super::super::Foundation::PWSTR, lpwsinterfacename: super::super::Foundation::PWSTR, lpwsusername: super::super::Foundation::PWSTR, lpwsdomainname: super::super::Foundation::PWSTR, lpwspassword: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceSetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminInterfaceSetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportAdd(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportRemove(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceTransportSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminInterfaceUpdateRoutes(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, hevent: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminIsDomainRasServer(pszdomain: super::super::Foundation::PWSTR, pszmachine: super::super::Foundation::PWSTR, pbisrasserver: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminIsServiceInitialized(lpwsservername: super::super::Foundation::PWSTR, fisserviceinitialized: *const super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminIsServiceRunning(lpwsservername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryCreate(hmibserver: isize, dwpid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryDelete(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryGet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryGetFirst(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntryGetNext(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBEntrySet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminMIBServerConnect(lpwsservername: super::super::Foundation::PWSTR, phmibserver: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminMIBServerDisconnect(hmibserver: isize);
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortClearStats(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortDisconnect(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortEnum(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortGetInfo(hrasserver: isize, dwlevel: u32, hport: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminPortReset(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminRegisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminSendUserMessage(hmprserver: isize, hconnection: super::super::Foundation::HANDLE, lpwszmessage: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminServerConnect(lpwsservername: super::super::Foundation::PWSTR, phmprserver: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerDisconnect(hmprserver: isize);
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerGetCredentials(hmprserver: isize, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerGetInfo(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminServerGetInfoEx(hmprserver: isize, pserverinfo: *mut MPR_SERVER_EX1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerSetCredentials(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprAdminServerSetInfoEx(hmprserver: isize, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminTransportCreate(hmprserver: isize, dwtransportid: u32, lpwstransportname: super::super::Foundation::PWSTR, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminTransportGetInfo(hmprserver: isize, dwtransportid: u32, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprAdminTransportSetInfo(hmprserver: isize, dwtransportid: u32, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminUpdateConnection(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminUserGetInfo(lpszserver: super::super::Foundation::PWSTR, lpszuser: super::super::Foundation::PWSTR, dwlevel: u32, lpbbuffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprAdminUserSetInfo(lpszserver: super::super::Foundation::PWSTR, lpszuser: super::super::Foundation::PWSTR, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprConfigBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigFilterGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigFilterSetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigGetFriendlyName(hmprconfig: super::super::Foundation::HANDLE, pszguidname: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigGetGuidName(hmprconfig: super::super::Foundation::HANDLE, pszfriendlyname: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceCreate(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceDelete(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigInterfaceGetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceGetHandle(hmprconfig: super::super::Foundation::HANDLE, lpwsinterfacename: super::super::Foundation::PWSTR, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, lpdwbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigInterfaceSetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportAdd(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: super::super::Foundation::PWSTR, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportEnum(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportRemove(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigInterfaceTransportSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerBackup(hmprconfig: super::super::Foundation::HANDLE, lpwspath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerConnect(lpwsservername: super::super::Foundation::PWSTR, phmprconfig: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerDisconnect(hmprconfig: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigServerGetInfoEx(hmprconfig: super::super::Foundation::HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerRefresh(hmprconfig: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigServerRestore(hmprconfig: super::super::Foundation::HANDLE, lpwspath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprConfigServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MprConfigServerSetInfoEx(hmprconfig: super::super::Foundation::HANDLE, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportCreate(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: super::super::Foundation::PWSTR, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: super::super::Foundation::PWSTR, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportDelete(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32, lplpwsdllpath: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MprConfigTransportSetInfo(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockAdd(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockFind(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockQuerySize(lpheader: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockRemove(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoBlockSet(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoDelete(lpheader: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoDuplicate(lpheader: *const ::core::ffi::c_void, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn MprInfoRemoveAll(lpheader: *const ::core::ffi::c_void, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasClearConnectionStatistics(hrasconn: HRASCONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasClearLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasConnectionNotificationA(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasConnectionNotificationW(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasCreatePhonebookEntryA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasCreatePhonebookEntryW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteEntryA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteEntryW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteSubEntryA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, dwsubentryid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDeleteSubEntryW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, dwsubentryid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialA(param0: *const RASDIALEXTENSIONS, param1: super::super::Foundation::PSTR, param2: *const RASDIALPARAMSA, param3: u32, param4: *const ::core::ffi::c_void, param5: *mut HRASCONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialDlgA(lpszphonebook: super::super::Foundation::PSTR, lpszentry: super::super::Foundation::PSTR, lpszphonenumber: super::super::Foundation::PSTR, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialDlgW(lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpszphonenumber: super::super::Foundation::PWSTR, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasDialW(param0: *const RASDIALEXTENSIONS, param1: super::super::Foundation::PWSTR, param2: *const RASDIALPARAMSW, param3: u32, param4: *const ::core::ffi::c_void, param5: *mut HRASCONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEditPhonebookEntryA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEditPhonebookEntryW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEntryDlgA(lpszphonebook: super::super::Foundation::PSTR, lpszentry: super::super::Foundation::PSTR, lpinfo: *mut RASENTRYDLGA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEntryDlgW(lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpinfo: *mut RASENTRYDLGW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumAutodialAddressesA(lpprasautodialaddresses: *mut super::super::Foundation::PSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumAutodialAddressesW(lpprasautodialaddresses: *mut super::super::Foundation::PWSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumConnectionsA(param0: *mut RASCONNA, param1: *mut u32, param2: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumConnectionsW(param0: *mut RASCONNW, param1: *mut u32, param2: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumDevicesA(param0: *mut RASDEVINFOA, param1: *mut u32, param2: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasEnumDevicesW(param0: *mut RASDEVINFOW, param1: *mut u32, param2: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumEntriesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut RASENTRYNAMEA, param3: *mut u32, param4: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasEnumEntriesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut RASENTRYNAMEW, param3: *mut u32, param4: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasFreeEapUserIdentityA(praseapuseridentity: *const RASEAPUSERIDENTITYA);
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasFreeEapUserIdentityW(praseapuseridentity: *const RASEAPUSERIDENTITYW);
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetAutodialAddressA(param0: super::super::Foundation::PSTR, param1: *const u32, param2: *mut RASAUTODIALENTRYA, param3: *mut u32, param4: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetAutodialAddressW(param0: super::super::Foundation::PWSTR, param1: *const u32, param2: *mut RASAUTODIALENTRYW, param3: *mut u32, param4: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialEnableA(param0: u32, param1: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialEnableW(param0: u32, param1: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialParamA(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetAutodialParamW(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetConnectStatusA(param0: HRASCONN, param1: *mut RASCONNSTATUSA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RasGetConnectStatusW(param0: HRASCONN, param1: *mut RASCONNSTATUSW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetConnectionStatistics(hrasconn: HRASCONN, lpstatistics: *mut RAS_STATS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetCountryInfoA(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetCountryInfoW(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCredentialsA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut RASCREDENTIALSA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCredentialsW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut RASCREDENTIALSW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCustomAuthDataA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetCustomAuthDataW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserDataA(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserDataW(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserIdentityA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEapUserIdentityW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEntryDialParamsA(param0: super::super::Foundation::PSTR, param1: *mut RASDIALPARAMSA, param2: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetEntryDialParamsW(param0: super::super::Foundation::PWSTR, param1: *mut RASDIALPARAMSW, param2: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut RASENTRYA, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut RASENTRYW, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetErrorStringA(resourceid: u32, lpszstring: super::super::Foundation::PSTR, inbufsize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetErrorStringW(resourceid: u32, lpszstring: super::super::Foundation::PWSTR, inbufsize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32, lpstatistics: *mut RAS_STATS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetPCscf(lpszpcscf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetProjectionInfoA(param0: HRASCONN, param1: RASPROJECTION, param2: *mut ::core::ffi::c_void, param3: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasGetProjectionInfoEx(hrasconn: HRASCONN, prasprojection: *mut RAS_PROJECTION_INFO, lpdwsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetProjectionInfoW(param0: HRASCONN, param1: RASPROJECTION, param2: *mut ::core::ffi::c_void, param3: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetSubEntryHandleA(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasGetSubEntryHandleW(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetSubEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: u32, param3: *mut RASSUBENTRYA, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasGetSubEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut RASSUBENTRYW, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasHangUpA(param0: HRASCONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasHangUpW(param0: HRASCONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasInvokeEapUI(param0: HRASCONN, param1: u32, param2: *const RASDIALEXTENSIONS, param3: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasPhonebookDlgA(lpszphonebook: super::super::Foundation::PSTR, lpszentry: super::super::Foundation::PSTR, lpinfo: *mut ::core::mem::ManuallyDrop<RASPBDLGA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasPhonebookDlgW(lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpinfo: *mut ::core::mem::ManuallyDrop<RASPBDLGW>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasRenameEntryA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasRenameEntryW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialAddressA(param0: super::super::Foundation::PSTR, param1: u32, param2: *const RASAUTODIALENTRYA, param3: u32, param4: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialAddressW(param0: super::super::Foundation::PWSTR, param1: u32, param2: *const RASAUTODIALENTRYW, param3: u32, param4: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialEnableA(param0: u32, param1: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetAutodialEnableW(param0: u32, param1: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasSetAutodialParamA(param0: u32, param1: *const ::core::ffi::c_void, param2: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RasSetAutodialParamW(param0: u32, param1: *const ::core::ffi::c_void, param2: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCredentialsA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *const RASCREDENTIALSA, param3: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCredentialsW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *const RASCREDENTIALSW, param3: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCustomAuthDataA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetCustomAuthDataW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEapUserDataA(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEapUserDataW(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEntryDialParamsA(param0: super::super::Foundation::PSTR, param1: *const RASDIALPARAMSA, param2: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetEntryDialParamsW(param0: super::super::Foundation::PWSTR, param1: *const RASDIALPARAMSW, param2: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasSetEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *const RASENTRYA, param3: u32, param4: *const u8, param5: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn RasSetEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *const RASENTRYW, param3: u32, param4: *const u8, param5: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetSubEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: u32, param3: *const RASSUBENTRYA, param4: u32, param5: *const u8, param6: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasSetSubEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *const RASSUBENTRYW, param4: u32, param5: *const u8, param6: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RasUpdateConnection(hrasconn: HRASCONN, lprasupdateconn: *const RASUPDATECONN) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasValidateEntryNameA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RasValidateEntryNameW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmAddNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, changeflags: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmAddRouteToDest(rtmreghandle: isize, routehandle: *mut isize, destaddress: *mut RTM_NET_ADDRESS, routeinfo: *mut RTM_ROUTE_INFO, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmBlockMethods(rtmreghandle: isize, targethandle: super::super::Foundation::HANDLE, targettype: u8, blockingflag: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress: *mut RTM_NET_ADDRESS, address: super::super::Networking::WinSock::IN6_ADDR, dwlength: u32, dwaddresssize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Networking_WinSock`*"]
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress: *mut RTM_NET_ADDRESS, paddress: *mut super::super::Networking::WinSock::IN6_ADDR, plength: *mut u32, dwaddresssize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateDestEnum(rtmreghandle: isize, targetviews: u32, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, protocolid: u32, rtmenumhandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateNextHopEnum(rtmreghandle: isize, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, rtmenumhandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateRouteEnum(rtmreghandle: isize, desthandle: isize, targetviews: u32, enumflags: u32, startdest: *mut RTM_NET_ADDRESS, matchingflags: u32, criteriaroute: *mut RTM_ROUTE_INFO, criteriainterface: u32, rtmenumhandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateRouteList(rtmreghandle: isize, routelisthandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmCreateRouteListEnum(rtmreghandle: isize, routelisthandle: isize, rtmenumhandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteEnumHandle(rtmreghandle: isize, enumhandle: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteNextHop(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteRouteList(rtmreghandle: isize, routelisthandle: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeleteRouteToDest(rtmreghandle: isize, routehandle: isize, changeflags: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeregisterEntity(rtmreghandle: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmDeregisterFromChangeNotification(rtmreghandle: isize, notifyhandle: isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmFindNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetChangeStatus(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, changestatus: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: *mut u32, changeddests: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetDestInfo(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEntityInfo(rtmreghandle: isize, entityhandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEntityMethods(rtmreghandle: isize, entityhandle: isize, nummethods: *mut u32, exptmethods: *mut ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetEnumDests(rtmreghandle: isize, enumhandle: isize, numdests: *mut u32, destinfos: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEnumNextHops(rtmreghandle: isize, enumhandle: isize, numnexthops: *mut u32, nexthophandles: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetExactMatchDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetExactMatchRoute(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, matchingflags: u32, routeinfo: *mut RTM_ROUTE_INFO, interfaceindex: u32, targetviews: u32, routehandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetLessSpecificDestination(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetListEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmGetMostSpecificDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetNextHopInfo(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetNextHopPointer(rtmreghandle: isize, nexthophandle: isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetOpaqueInformationPointer(rtmreghandle: isize, desthandle: isize, opaqueinfopointer: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetRegisteredEntities(rtmreghandle: isize, numentities: *mut u32, entityhandles: *mut isize, entityinfos: *mut RTM_ENTITY_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetRouteInfo(rtmreghandle: isize, routehandle: isize, routeinfo: *mut RTM_ROUTE_INFO, destaddress: *mut RTM_NET_ADDRESS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmGetRoutePointer(rtmreghandle: isize, routehandle: isize, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmHoldDestination(rtmreghandle: isize, desthandle: isize, targetviews: u32, holdtime: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmIgnoreChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmInsertInRouteList(rtmreghandle: isize, routelisthandle: isize, numroutes: u32, routehandles: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmInvokeMethod(rtmreghandle: isize, entityhandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, outputsize: *mut u32, output: *mut RTM_ENTITY_METHOD_OUTPUT) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmIsBestRoute(rtmreghandle: isize, routehandle: isize, bestinviews: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmIsMarkedForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, destmarked: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmLockDestination(rtmreghandle: isize, desthandle: isize, exclusive: super::super::Foundation::BOOL, lockdest: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmLockNextHop(rtmreghandle: isize, nexthophandle: isize, exclusive: super::super::Foundation::BOOL, locknexthop: super::super::Foundation::BOOL, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmLockRoute(rtmreghandle: isize, routehandle: isize, exclusive: super::super::Foundation::BOOL, lockroute: super::super::Foundation::BOOL, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmMarkDestForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, markdest: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReferenceHandles(rtmreghandle: isize, numhandles: u32, rtmhandles: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmRegisterEntity(rtmentityinfo: *mut RTM_ENTITY_INFO, exportmethods: *mut ::core::mem::ManuallyDrop<RTM_ENTITY_EXPORT_METHODS>, eventcallback: ::windows::runtime::RawPtr, reserveopaquepointer: super::super::Foundation::BOOL, rtmregprofile: *mut RTM_REGN_PROFILE, rtmreghandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmRegisterForChangeNotification(rtmreghandle: isize, targetviews: u32, notifyflags: u32, notifycontext: *mut ::core::ffi::c_void, notifyhandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReleaseChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReleaseDestInfo(rtmreghandle: isize, destinfo: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtmReleaseDests(rtmreghandle: isize, numdests: u32, destinfos: *mut RTM_DEST_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseEntities(rtmreghandle: isize, numentities: u32, entityhandles: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseEntityInfo(rtmreghandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseNextHopInfo(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseNextHops(rtmreghandle: isize, numnexthops: u32, nexthophandles: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseRouteInfo(rtmreghandle: isize, routeinfo: *mut RTM_ROUTE_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmReleaseRoutes(rtmreghandle: isize, numroutes: u32, routehandles: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Rras`*"]
    pub fn RtmUpdateAndUnlockRoute(rtmreghandle: isize, routehandle: isize, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32;
}
