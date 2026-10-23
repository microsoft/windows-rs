#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminBufferFree(pbuffer: super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminBufferFree(pbuffer : super::LPVOID) -> super::DWORD);
    unsafe { MprAdminBufferFree(pbuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionClearStats(hrasserver: RAS_SERVER_HANDLE, hrasconnection: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionClearStats(hrasserver : RAS_SERVER_HANDLE, hrasconnection : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminConnectionClearStats(hrasserver, hrasconnection) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminConnectionEnum(hrasserver: RAS_SERVER_HANDLE, dwlevel: super::DWORD, lplpbbuffer: *mut super::LPBYTE, dwprefmaxlen: super::DWORD, lpdwentriesread: super::LPDWORD, lpdwtotalentries: super::LPDWORD, lpdwresumehandle: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionEnum(hrasserver : RAS_SERVER_HANDLE, dwlevel : super::DWORD, lplpbbuffer : *mut super::LPBYTE, dwprefmaxlen : super::DWORD, lpdwentriesread : super::LPDWORD, lpdwtotalentries : super::LPDWORD, lpdwresumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminConnectionEnum(hrasserver, dwlevel, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionEnumEx(hrasserver: RAS_SERVER_HANDLE, pobjectheader: PMPRAPI_OBJECT_HEADER, dwpreferedmaxlen: super::DWORD, lpdwentriesread: super::LPDWORD, lpdwtotalentries: super::LPDWORD, pprasconn: *mut PRAS_CONNECTION_EX, lpdwresumehandle: super::LPDWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionEnumEx(hrasserver : RAS_SERVER_HANDLE, pobjectheader : PMPRAPI_OBJECT_HEADER, dwpreferedmaxlen : super::DWORD, lpdwentriesread : super::LPDWORD, lpdwtotalentries : super::LPDWORD, pprasconn : *mut PRAS_CONNECTION_EX, lpdwresumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminConnectionEnumEx(hrasserver, pobjectheader, dwpreferedmaxlen, lpdwentriesread as _, lpdwtotalentries as _, pprasconn as _, lpdwresumehandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionGetInfo(hrasserver: RAS_SERVER_HANDLE, dwlevel: super::DWORD, hrasconnection: super::HANDLE, lplpbbuffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfo(hrasserver : RAS_SERVER_HANDLE, dwlevel : super::DWORD, hrasconnection : super::HANDLE, lplpbbuffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminConnectionGetInfo(hrasserver, dwlevel, hrasconnection, lplpbbuffer as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionGetInfoEx(hrasserver: RAS_SERVER_HANDLE, hrasconnection: super::HANDLE, prasconnection: PRAS_CONNECTION_EX) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfoEx(hrasserver : RAS_SERVER_HANDLE, hrasconnection : super::HANDLE, prasconnection : PRAS_CONNECTION_EX) -> super::DWORD);
    unsafe { MprAdminConnectionGetInfoEx(hrasserver, hrasconnection, prasconnection as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionRemoveQuarantine(hrasserver: super::HANDLE, hrasconnection: super::HANDLE, fisipaddress: bool) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionRemoveQuarantine(hrasserver : super::HANDLE, hrasconnection : super::HANDLE, fisipaddress : windows_core::BOOL) -> super::DWORD);
    unsafe { MprAdminConnectionRemoveQuarantine(hrasserver, hrasconnection, fisipaddress.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminDeregisterConnectionNotification(hmprserver: MPR_SERVER_HANDLE, heventnotification: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminDeregisterConnectionNotification(hmprserver : MPR_SERVER_HANDLE, heventnotification : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminDeregisterConnectionNotification(hmprserver, heventnotification) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminDeviceEnum(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lplpbbuffer: *mut super::LPBYTE, lpdwtotalentries: super::LPDWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminDeviceEnum(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lplpbbuffer : *mut super::LPBYTE, lpdwtotalentries : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminDeviceEnum(hmprserver, dwlevel, lplpbbuffer as _, lpdwtotalentries as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminEstablishDomainRasServer<P0, P1>(pszdomain: P0, pszmachine: P1, benable: bool) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminEstablishDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, benable : windows_core::BOOL) -> super::DWORD);
    unsafe { MprAdminEstablishDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), benable.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminGetErrorString(dwerror: super::DWORD, lplpwserrorstring: *mut windows_core::PWSTR) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminGetErrorString(dwerror : super::DWORD, lplpwserrorstring : *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { MprAdminGetErrorString(dwerror, lplpwserrorstring as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminGetPDCServer<P0, P1>(lpszdomain: P0, lpszserver: P1, lpszpdcserver: windows_core::PWSTR) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminGetPDCServer(lpszdomain : windows_core::PCWSTR, lpszserver : windows_core::PCWSTR, lpszpdcserver : windows_core::PWSTR) -> super::DWORD);
    unsafe { MprAdminGetPDCServer(lpszdomain.param().abi(), lpszserver.param().abi(), lpszpdcserver) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceConnect(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, hevent: super::HANDLE, fsynchronous: bool) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceConnect(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, hevent : super::HANDLE, fsynchronous : windows_core::BOOL) -> super::DWORD);
    unsafe { MprAdminInterfaceConnect(hmprserver, hinterface, hevent, fsynchronous.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceCreate(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE, phinterface: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceCreate(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE, phinterface : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprAdminInterfaceCreate(hmprserver, dwlevel, lpbbuffer, phinterface as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceDelete(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDelete(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminInterfaceDelete(hmprserver, hinterface) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceGetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwindex: super::DWORD, dwlevel: super::DWORD, lplpbuffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceGetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwindex : super::DWORD, dwlevel : super::DWORD, lplpbuffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminInterfaceDeviceGetInfo(hmprserver, hinterface, dwindex, dwlevel, lplpbuffer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceSetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwindex: super::DWORD, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceSetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwindex : super::DWORD, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminInterfaceDeviceSetInfo(hmprserver, hinterface, dwindex, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceDisconnect(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDisconnect(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminInterfaceDisconnect(hmprserver, hinterface) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminInterfaceEnum(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lplpbbuffer: *mut super::LPBYTE, dwprefmaxlen: super::DWORD, lpdwentriesread: super::LPDWORD, lpdwtotalentries: super::LPDWORD, lpdwresumehandle: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceEnum(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lplpbbuffer : *mut super::LPBYTE, dwprefmaxlen : super::DWORD, lpdwentriesread : super::LPDWORD, lpdwtotalentries : super::LPDWORD, lpdwresumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminInterfaceEnum(hmprserver, dwlevel, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentials<P0, P1>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: Option<windows_core::PWSTR>, lpwspassword: Option<windows_core::PWSTR>, lpwsdomainname: Option<windows_core::PWSTR>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PWSTR, lpwspassword : windows_core::PWSTR, lpwsdomainname : windows_core::PWSTR) -> super::DWORD);
    unsafe { MprAdminInterfaceGetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), lpwsusername.unwrap_or(core::mem::zeroed()) as _, lpwspassword.unwrap_or(core::mem::zeroed()) as _, lpwsdomainname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentialsEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwlevel: super::DWORD, lplpbbuffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentialsEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwlevel : super::DWORD, lplpbbuffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminInterfaceGetCredentialsEx(hmprserver, hinterface, dwlevel, lplpbbuffer as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCustomInfoEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, pcustominfo: PMPR_IF_CUSTOMINFOEX2) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCustomInfoEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, pcustominfo : PMPR_IF_CUSTOMINFOEX2) -> super::DWORD);
    unsafe { MprAdminInterfaceGetCustomInfoEx(hmprserver, hinterface, pcustominfo as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetHandle<P1>(hmprserver: MPR_SERVER_HANDLE, lpwsinterfacename: P1, phinterface: *mut super::HANDLE, fincludeclientinterfaces: bool) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetHandle(hmprserver : MPR_SERVER_HANDLE, lpwsinterfacename : windows_core::PCWSTR, phinterface : *mut super::HANDLE, fincludeclientinterfaces : windows_core::BOOL) -> super::DWORD);
    unsafe { MprAdminInterfaceGetHandle(hmprserver, lpwsinterfacename.param().abi(), phinterface as _, fincludeclientinterfaces.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwlevel: super::DWORD, lplpbbuffer: *const super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwlevel : super::DWORD, lplpbbuffer : *const super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminInterfaceGetInfo(hmprserver, hinterface, dwlevel, lplpbbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceQueryUpdateResult(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwprotocolid: super::DWORD, lpdwupdateresult: super::LPDWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceQueryUpdateResult(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwprotocolid : super::DWORD, lpdwupdateresult : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminInterfaceQueryUpdateResult(hmprserver, hinterface, dwprotocolid, lpdwupdateresult as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentials<P0, P1, P2, P3, P4>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: P2, lpwsdomainname: P3, lpwspassword: P4) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PCWSTR, lpwsdomainname : windows_core::PCWSTR, lpwspassword : windows_core::PCWSTR) -> super::DWORD);
    unsafe { MprAdminInterfaceSetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), lpwsusername.param().abi(), lpwsdomainname.param().abi(), lpwspassword.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentialsEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentialsEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminInterfaceSetCredentialsEx(hmprserver, hinterface, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetCustomInfoEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, pcustominfo: PMPR_IF_CUSTOMINFOEX2) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCustomInfoEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, pcustominfo : PMPR_IF_CUSTOMINFOEX2) -> super::DWORD);
    unsafe { MprAdminInterfaceSetCustomInfoEx(hmprserver, hinterface, pcustominfo) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminInterfaceSetInfo(hmprserver, hinterface, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceTransportAdd(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwtransportid: super::DWORD, pinterfaceinfo: super::LPBYTE, dwinterfaceinfosize: super::DWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportAdd(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwtransportid : super::DWORD, pinterfaceinfo : super::LPBYTE, dwinterfaceinfosize : super::DWORD) -> super::DWORD);
    unsafe { MprAdminInterfaceTransportAdd(hmprserver, hinterface, dwtransportid, pinterfaceinfo, dwinterfaceinfosize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceTransportGetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwtransportid: super::DWORD, ppinterfaceinfo: *mut super::LPBYTE, lpdwinterfaceinfosize: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportGetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwtransportid : super::DWORD, ppinterfaceinfo : *mut super::LPBYTE, lpdwinterfaceinfosize : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminInterfaceTransportGetInfo(hmprserver, hinterface, dwtransportid, ppinterfaceinfo as _, lpdwinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceTransportRemove(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwtransportid: super::DWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportRemove(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwtransportid : super::DWORD) -> super::DWORD);
    unsafe { MprAdminInterfaceTransportRemove(hmprserver, hinterface, dwtransportid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceTransportSetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwtransportid: super::DWORD, pinterfaceinfo: super::LPBYTE, dwinterfaceinfosize: super::DWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportSetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwtransportid : super::DWORD, pinterfaceinfo : super::LPBYTE, dwinterfaceinfosize : super::DWORD) -> super::DWORD);
    unsafe { MprAdminInterfaceTransportSetInfo(hmprserver, hinterface, dwtransportid, pinterfaceinfo, dwinterfaceinfosize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminInterfaceUpdatePhonebookInfo(hmprserver, hinterface) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceUpdateRoutes(hmprserver: MPR_SERVER_HANDLE, hinterface: super::HANDLE, dwprotocolid: super::DWORD, hevent: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdateRoutes(hmprserver : MPR_SERVER_HANDLE, hinterface : super::HANDLE, dwprotocolid : super::DWORD, hevent : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminInterfaceUpdateRoutes(hmprserver, hinterface, dwprotocolid, hevent) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminIsDomainRasServer<P0, P1>(pszdomain: P0, pszmachine: P1, pbisrasserver: super::PBOOL) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminIsDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, pbisrasserver : super::PBOOL) -> super::DWORD);
    unsafe { MprAdminIsDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), pbisrasserver as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminIsServiceInitialized<P0>(lpwsservername: P0, fisserviceinitialized: *const windows_core::BOOL) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminIsServiceInitialized(lpwsservername : windows_core::PCWSTR, fisserviceinitialized : *const windows_core::BOOL) -> super::DWORD);
    unsafe { MprAdminIsServiceInitialized(lpwsservername.param().abi(), fisserviceinitialized) }
}
#[inline]
pub unsafe fn MprAdminIsServiceRunning<P0>(lpwsservername: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminIsServiceRunning(lpwsservername : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { MprAdminIsServiceRunning(lpwsservername.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminMIBBufferFree(pbuffer: super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBBufferFree(pbuffer : super::LPVOID) -> super::DWORD);
    unsafe { MprAdminMIBBufferFree(pbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminMIBEntryCreate(hmibserver: MIB_SERVER_HANDLE, dwpid: super::DWORD, dwroutingpid: super::DWORD, lpentry: super::LPVOID, dwentrysize: super::DWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryCreate(hmibserver : MIB_SERVER_HANDLE, dwpid : super::DWORD, dwroutingpid : super::DWORD, lpentry : super::LPVOID, dwentrysize : super::DWORD) -> super::DWORD);
    unsafe { MprAdminMIBEntryCreate(hmibserver, dwpid, dwroutingpid, lpentry, dwentrysize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminMIBEntryDelete(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: super::DWORD, dwroutingpid: super::DWORD, lpentry: super::LPVOID, dwentrysize: super::DWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryDelete(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : super::DWORD, dwroutingpid : super::DWORD, lpentry : super::LPVOID, dwentrysize : super::DWORD) -> super::DWORD);
    unsafe { MprAdminMIBEntryDelete(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminMIBEntryGet(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: super::DWORD, dwroutingpid: super::DWORD, lpinentry: super::LPVOID, dwinentrysize: super::DWORD, lplpoutentry: *mut super::LPVOID, lpoutentrysize: super::LPDWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryGet(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : super::DWORD, dwroutingpid : super::DWORD, lpinentry : super::LPVOID, dwinentrysize : super::DWORD, lplpoutentry : *mut super::LPVOID, lpoutentrysize : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminMIBEntryGet(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminMIBEntryGetFirst(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: super::DWORD, dwroutingpid: super::DWORD, lpinentry: super::LPVOID, dwinentrysize: super::DWORD, lplpoutentry: *mut super::LPVOID, lpoutentrysize: super::LPDWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetFirst(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : super::DWORD, dwroutingpid : super::DWORD, lpinentry : super::LPVOID, dwinentrysize : super::DWORD, lplpoutentry : *mut super::LPVOID, lpoutentrysize : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminMIBEntryGetFirst(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminMIBEntryGetNext(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: super::DWORD, dwroutingpid: super::DWORD, lpinentry: super::LPVOID, dwinentrysize: super::DWORD, lplpoutentry: *mut super::LPVOID, lpoutentrysize: super::LPDWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetNext(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : super::DWORD, dwroutingpid : super::DWORD, lpinentry : super::LPVOID, dwinentrysize : super::DWORD, lplpoutentry : *mut super::LPVOID, lpoutentrysize : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminMIBEntryGetNext(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminMIBEntrySet(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: super::DWORD, dwroutingpid: super::DWORD, lpentry: super::LPVOID, dwentrysize: super::DWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntrySet(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : super::DWORD, dwroutingpid : super::DWORD, lpentry : super::LPVOID, dwentrysize : super::DWORD) -> super::DWORD);
    unsafe { MprAdminMIBEntrySet(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminMIBServerConnect<P0>(lpwsservername: P0, phmibserver: *mut MIB_SERVER_HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBServerConnect(lpwsservername : windows_core::PCWSTR, phmibserver : *mut MIB_SERVER_HANDLE) -> super::DWORD);
    unsafe { MprAdminMIBServerConnect(lpwsservername.param().abi(), phmibserver as _) }
}
#[inline]
pub unsafe fn MprAdminMIBServerDisconnect(hmibserver: MIB_SERVER_HANDLE) {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBServerDisconnect(hmibserver : MIB_SERVER_HANDLE));
    unsafe { MprAdminMIBServerDisconnect(hmibserver) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminPortClearStats(hrasserver: RAS_SERVER_HANDLE, hport: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortClearStats(hrasserver : RAS_SERVER_HANDLE, hport : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminPortClearStats(hrasserver, hport) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminPortDisconnect(hrasserver: RAS_SERVER_HANDLE, hport: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortDisconnect(hrasserver : RAS_SERVER_HANDLE, hport : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminPortDisconnect(hrasserver, hport) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminPortEnum(hrasserver: RAS_SERVER_HANDLE, dwlevel: super::DWORD, hrasconnection: super::HANDLE, lplpbbuffer: *mut super::LPBYTE, dwprefmaxlen: super::DWORD, lpdwentriesread: super::LPDWORD, lpdwtotalentries: super::LPDWORD, lpdwresumehandle: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortEnum(hrasserver : RAS_SERVER_HANDLE, dwlevel : super::DWORD, hrasconnection : super::HANDLE, lplpbbuffer : *mut super::LPBYTE, dwprefmaxlen : super::DWORD, lpdwentriesread : super::LPDWORD, lpdwtotalentries : super::LPDWORD, lpdwresumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminPortEnum(hrasserver, dwlevel, hrasconnection, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminPortGetInfo(hrasserver: RAS_SERVER_HANDLE, dwlevel: super::DWORD, hport: super::HANDLE, lplpbbuffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortGetInfo(hrasserver : RAS_SERVER_HANDLE, dwlevel : super::DWORD, hport : super::HANDLE, lplpbbuffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminPortGetInfo(hrasserver, dwlevel, hport, lplpbbuffer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminPortReset(hrasserver: RAS_SERVER_HANDLE, hport: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortReset(hrasserver : RAS_SERVER_HANDLE, hport : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminPortReset(hrasserver, hport) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminRegisterConnectionNotification(hmprserver: MPR_SERVER_HANDLE, heventnotification: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminRegisterConnectionNotification(hmprserver : MPR_SERVER_HANDLE, heventnotification : super::HANDLE) -> super::DWORD);
    unsafe { MprAdminRegisterConnectionNotification(hmprserver, heventnotification) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminSendUserMessage<P2>(hmprserver: MPR_SERVER_HANDLE, hconnection: super::HANDLE, lpwszmessage: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminSendUserMessage(hmprserver : MPR_SERVER_HANDLE, hconnection : super::HANDLE, lpwszmessage : windows_core::PCWSTR) -> super::DWORD);
    unsafe { MprAdminSendUserMessage(hmprserver, hconnection, lpwszmessage.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminServerConnect<P0>(lpwsservername: P0, phmprserver: *mut MPR_SERVER_HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerConnect(lpwsservername : windows_core::PCWSTR, phmprserver : *mut MPR_SERVER_HANDLE) -> super::DWORD);
    unsafe { MprAdminServerConnect(lpwsservername.param().abi(), phmprserver as _) }
}
#[inline]
pub unsafe fn MprAdminServerDisconnect(hmprserver: MPR_SERVER_HANDLE) {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerDisconnect(hmprserver : MPR_SERVER_HANDLE));
    unsafe { MprAdminServerDisconnect(hmprserver) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminServerGetCredentials(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lplpbbuffer: *const super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerGetCredentials(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lplpbbuffer : *const super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminServerGetCredentials(hmprserver, dwlevel, lplpbbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminServerGetInfo(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lplpbbuffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerGetInfo(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lplpbbuffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminServerGetInfo(hmprserver, dwlevel, lplpbbuffer as _) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminServerGetInfoEx(hmprserver: MPR_SERVER_HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerGetInfoEx(hmprserver : MPR_SERVER_HANDLE, pserverinfo : *mut MPR_SERVER_EX1) -> super::DWORD);
    unsafe { MprAdminServerGetInfoEx(hmprserver, pserverinfo as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminServerSetCredentials(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerSetCredentials(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminServerSetCredentials(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminServerSetInfo(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerSetInfo(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminServerSetInfo(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminServerSetInfoEx(hmprserver: MPR_SERVER_HANDLE, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerSetInfoEx(hmprserver : MPR_SERVER_HANDLE, pserverinfo : *const MPR_SERVER_SET_CONFIG_EX1) -> super::DWORD);
    unsafe { MprAdminServerSetInfoEx(hmprserver, pserverinfo) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminTransportCreate<P2, P7>(hmprserver: MPR_SERVER_HANDLE, dwtransportid: super::DWORD, lpwstransportname: P2, pglobalinfo: super::LPBYTE, dwglobalinfosize: super::DWORD, pclientinterfaceinfo: Option<super::LPBYTE>, dwclientinterfaceinfosize: Option<super::DWORD>, lpwsdllpath: P7) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminTransportCreate(hmprserver : MPR_SERVER_HANDLE, dwtransportid : super::DWORD, lpwstransportname : windows_core::PCWSTR, pglobalinfo : super::LPBYTE, dwglobalinfosize : super::DWORD, pclientinterfaceinfo : super::LPBYTE, dwclientinterfaceinfosize : super::DWORD, lpwsdllpath : windows_core::PCWSTR) -> super::DWORD);
    unsafe { MprAdminTransportCreate(hmprserver, dwtransportid, lpwstransportname.param().abi(), pglobalinfo, dwglobalinfosize, pclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, dwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _, lpwsdllpath.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprAdminTransportGetInfo(hmprserver: MPR_SERVER_HANDLE, dwtransportid: super::DWORD, ppglobalinfo: Option<*mut super::LPBYTE>, lpdwglobalinfosize: Option<super::LPDWORD>, ppclientinterfaceinfo: Option<*mut super::LPBYTE>, lpdwclientinterfaceinfosize: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminTransportGetInfo(hmprserver : MPR_SERVER_HANDLE, dwtransportid : super::DWORD, ppglobalinfo : *mut super::LPBYTE, lpdwglobalinfosize : super::LPDWORD, ppclientinterfaceinfo : *mut super::LPBYTE, lpdwclientinterfaceinfosize : super::LPDWORD) -> super::DWORD);
    unsafe { MprAdminTransportGetInfo(hmprserver, dwtransportid, ppglobalinfo.unwrap_or(core::mem::zeroed()) as _, lpdwglobalinfosize.unwrap_or(core::mem::zeroed()) as _, ppclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminTransportSetInfo(hmprserver: MPR_SERVER_HANDLE, dwtransportid: super::DWORD, pglobalinfo: Option<super::LPBYTE>, dwglobalinfosize: super::DWORD, pclientinterfaceinfo: Option<super::LPBYTE>, dwclientinterfaceinfosize: super::DWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminTransportSetInfo(hmprserver : MPR_SERVER_HANDLE, dwtransportid : super::DWORD, pglobalinfo : super::LPBYTE, dwglobalinfosize : super::DWORD, pclientinterfaceinfo : super::LPBYTE, dwclientinterfaceinfosize : super::DWORD) -> super::DWORD);
    unsafe { MprAdminTransportSetInfo(hmprserver, dwtransportid, pglobalinfo.unwrap_or(core::mem::zeroed()) as _, dwglobalinfosize, pclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, dwclientinterfaceinfosize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminUpdateConnection(hrasserver: RAS_SERVER_HANDLE, hrasconnection: super::HANDLE, prasupdateconnection: PRAS_UPDATE_CONNECTION) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprAdminUpdateConnection(hrasserver : RAS_SERVER_HANDLE, hrasconnection : super::HANDLE, prasupdateconnection : PRAS_UPDATE_CONNECTION) -> super::DWORD);
    unsafe { MprAdminUpdateConnection(hrasserver, hrasconnection, prasupdateconnection) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminUserGetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminUserGetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminUserGetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprAdminUserSetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminUserSetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprAdminUserSetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprConfigBufferFree(pbuffer: super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigBufferFree(pbuffer : super::LPVOID) -> super::DWORD);
    unsafe { MprConfigBufferFree(pbuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigFilterGetInfo(hmprconfig: super::HANDLE, dwlevel: super::DWORD, dwtransportid: super::DWORD, lpbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigFilterGetInfo(hmprconfig : super::HANDLE, dwlevel : super::DWORD, dwtransportid : super::DWORD, lpbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprConfigFilterGetInfo(hmprconfig, dwlevel, dwtransportid, lpbuffer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigFilterSetInfo(hmprconfig: super::HANDLE, dwlevel: super::DWORD, dwtransportid: super::DWORD, lpbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigFilterSetInfo(hmprconfig : super::HANDLE, dwlevel : super::DWORD, dwtransportid : super::DWORD, lpbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprConfigFilterSetInfo(hmprconfig, dwlevel, dwtransportid, lpbuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigGetFriendlyName<P1>(hmprconfig: super::HANDLE, pszguidname: P1, pszbuffer: super::PWCHAR, dwbuffersize: super::DWORD) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigGetFriendlyName(hmprconfig : super::HANDLE, pszguidname : windows_core::PCWSTR, pszbuffer : super::PWCHAR, dwbuffersize : super::DWORD) -> super::DWORD);
    unsafe { MprConfigGetFriendlyName(hmprconfig, pszguidname.param().abi(), pszbuffer as _, dwbuffersize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigGetGuidName<P1>(hmprconfig: super::HANDLE, pszfriendlyname: P1, pszbuffer: super::PWCHAR, dwbuffersize: super::DWORD) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigGetGuidName(hmprconfig : super::HANDLE, pszfriendlyname : windows_core::PCWSTR, pszbuffer : super::PWCHAR, dwbuffersize : super::DWORD) -> super::DWORD);
    unsafe { MprConfigGetGuidName(hmprconfig, pszfriendlyname.param().abi(), pszbuffer as _, dwbuffersize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceCreate(hmprconfig: super::HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE, phrouterinterface: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceCreate(hmprconfig : super::HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE, phrouterinterface : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprConfigInterfaceCreate(hmprconfig, dwlevel, lpbbuffer, phrouterinterface as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceDelete(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceDelete(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE) -> super::DWORD);
    unsafe { MprConfigInterfaceDelete(hmprconfig, hrouterinterface) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceEnum(hmprconfig: super::HANDLE, dwlevel: super::DWORD, lplpbuffer: *mut super::LPBYTE, dwprefmaxlen: super::DWORD, lpdwentriesread: super::LPDWORD, lpdwtotalentries: super::LPDWORD, lpdwresumehandle: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceEnum(hmprconfig : super::HANDLE, dwlevel : super::DWORD, lplpbuffer : *mut super::LPBYTE, dwprefmaxlen : super::DWORD, lpdwentriesread : super::LPDWORD, lpdwtotalentries : super::LPDWORD, lpdwresumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { MprConfigInterfaceEnum(hmprconfig, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetCustomInfoEx(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, pcustominfo: PMPR_IF_CUSTOMINFOEX2) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceGetCustomInfoEx(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, pcustominfo : PMPR_IF_CUSTOMINFOEX2) -> super::DWORD);
    unsafe { MprConfigInterfaceGetCustomInfoEx(hmprconfig, hrouterinterface, pcustominfo as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetHandle<P1>(hmprconfig: super::HANDLE, lpwsinterfacename: P1, phrouterinterface: *mut super::HANDLE) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceGetHandle(hmprconfig : super::HANDLE, lpwsinterfacename : windows_core::PCWSTR, phrouterinterface : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprConfigInterfaceGetHandle(hmprconfig, lpwsinterfacename.param().abi(), phrouterinterface as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetInfo(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, dwlevel: super::DWORD, lplpbuffer: *mut super::LPBYTE, lpdwbuffersize: super::LPDWORD) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceGetInfo(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, dwlevel : super::DWORD, lplpbuffer : *mut super::LPBYTE, lpdwbuffersize : super::LPDWORD) -> super::DWORD);
    unsafe { MprConfigInterfaceGetInfo(hmprconfig, hrouterinterface, dwlevel, lplpbuffer as _, lpdwbuffersize as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceSetCustomInfoEx(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, pcustominfo: PMPR_IF_CUSTOMINFOEX2) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceSetCustomInfoEx(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, pcustominfo : PMPR_IF_CUSTOMINFOEX2) -> super::DWORD);
    unsafe { MprConfigInterfaceSetCustomInfoEx(hmprconfig, hrouterinterface, pcustominfo) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceSetInfo(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceSetInfo(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprConfigInterfaceSetInfo(hmprconfig, hrouterinterface, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportAdd<P3>(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, dwtransportid: super::DWORD, lpwstransportname: P3, pinterfaceinfo: &[u8], phrouteriftransport: *mut super::HANDLE) -> super::DWORD
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportAdd(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, dwtransportid : super::DWORD, lpwstransportname : windows_core::PCWSTR, pinterfaceinfo : super::LPBYTE, dwinterfaceinfosize : super::DWORD, phrouteriftransport : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprConfigInterfaceTransportAdd(hmprconfig, hrouterinterface, dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pinterfaceinfo.as_ptr()), pinterfaceinfo.len().try_into().unwrap(), phrouteriftransport as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportEnum(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, dwlevel: super::DWORD, lplpbuffer: *mut super::LPBYTE, dwprefmaxlen: super::DWORD, lpdwentriesread: super::LPDWORD, lpdwtotalentries: super::LPDWORD, lpdwresumehandle: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportEnum(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, dwlevel : super::DWORD, lplpbuffer : *mut super::LPBYTE, dwprefmaxlen : super::DWORD, lpdwentriesread : super::LPDWORD, lpdwtotalentries : super::LPDWORD, lpdwresumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { MprConfigInterfaceTransportEnum(hmprconfig, hrouterinterface, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetHandle(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, dwtransportid: super::DWORD, phrouteriftransport: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetHandle(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, dwtransportid : super::DWORD, phrouteriftransport : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprConfigInterfaceTransportGetHandle(hmprconfig, hrouterinterface, dwtransportid, phrouteriftransport as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetInfo(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, hrouteriftransport: super::HANDLE, ppinterfaceinfo: Option<*mut super::LPBYTE>, lpdwinterfaceinfosize: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetInfo(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, hrouteriftransport : super::HANDLE, ppinterfaceinfo : *mut super::LPBYTE, lpdwinterfaceinfosize : super::LPDWORD) -> super::DWORD);
    unsafe { MprConfigInterfaceTransportGetInfo(hmprconfig, hrouterinterface, hrouteriftransport, ppinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportRemove(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, hrouteriftransport: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportRemove(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, hrouteriftransport : super::HANDLE) -> super::DWORD);
    unsafe { MprConfigInterfaceTransportRemove(hmprconfig, hrouterinterface, hrouteriftransport) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportSetInfo(hmprconfig: super::HANDLE, hrouterinterface: super::HANDLE, hrouteriftransport: super::HANDLE, pinterfaceinfo: Option<&[u8]>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportSetInfo(hmprconfig : super::HANDLE, hrouterinterface : super::HANDLE, hrouteriftransport : super::HANDLE, pinterfaceinfo : super::LPBYTE, dwinterfaceinfosize : super::DWORD) -> super::DWORD);
    unsafe { MprConfigInterfaceTransportSetInfo(hmprconfig, hrouterinterface, hrouteriftransport, core::mem::transmute(pinterfaceinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pinterfaceinfo.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerBackup<P1>(hmprconfig: super::HANDLE, lpwspath: P1) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerBackup(hmprconfig : super::HANDLE, lpwspath : windows_core::PCWSTR) -> super::DWORD);
    unsafe { MprConfigServerBackup(hmprconfig, lpwspath.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerConnect<P0>(lpwsservername: P0, phmprconfig: *mut super::HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerConnect(lpwsservername : windows_core::PCWSTR, phmprconfig : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprConfigServerConnect(lpwsservername.param().abi(), phmprconfig as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigServerDisconnect(hmprconfig: super::HANDLE) {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerDisconnect(hmprconfig : super::HANDLE));
    unsafe { MprConfigServerDisconnect(hmprconfig) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerGetInfo(hmprconfig: super::HANDLE, dwlevel: super::DWORD, lplpbbuffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerGetInfo(hmprconfig : super::HANDLE, dwlevel : super::DWORD, lplpbbuffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { MprConfigServerGetInfo(hmprconfig, dwlevel, lplpbbuffer as _) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerGetInfoEx(hmprconfig: super::HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerGetInfoEx(hmprconfig : super::HANDLE, pserverinfo : *mut MPR_SERVER_EX1) -> super::DWORD);
    unsafe { MprConfigServerGetInfoEx(hmprconfig, pserverinfo as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerInstall(dwlevel: super::DWORD, pbuffer: super::PVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerInstall(dwlevel : super::DWORD, pbuffer : super::PVOID) -> super::DWORD);
    unsafe { MprConfigServerInstall(dwlevel, pbuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerRefresh(hmprconfig: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerRefresh(hmprconfig : super::HANDLE) -> super::DWORD);
    unsafe { MprConfigServerRefresh(hmprconfig) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerRestore<P1>(hmprconfig: super::HANDLE, lpwspath: P1) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerRestore(hmprconfig : super::HANDLE, lpwspath : windows_core::PCWSTR) -> super::DWORD);
    unsafe { MprConfigServerRestore(hmprconfig, lpwspath.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MprConfigServerSetInfo(hmprserver: MPR_SERVER_HANDLE, dwlevel: super::DWORD, lpbbuffer: super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerSetInfo(hmprserver : MPR_SERVER_HANDLE, dwlevel : super::DWORD, lpbbuffer : super::LPBYTE) -> super::DWORD);
    unsafe { MprConfigServerSetInfo(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerSetInfoEx(hmprconfig: super::HANDLE, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerSetInfoEx(hmprconfig : super::HANDLE, psetserverconfig : *const MPR_SERVER_SET_CONFIG_EX1) -> super::DWORD);
    unsafe { MprConfigServerSetInfoEx(hmprconfig, psetserverconfig) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportCreate<P2, P7>(hmprconfig: super::HANDLE, dwtransportid: super::DWORD, lpwstransportname: P2, pglobalinfo: &[u8], pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P7, phroutertransport: *mut super::HANDLE) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportCreate(hmprconfig : super::HANDLE, dwtransportid : super::DWORD, lpwstransportname : windows_core::PCWSTR, pglobalinfo : super::LPBYTE, dwglobalinfosize : super::DWORD, pclientinterfaceinfo : super::LPBYTE, dwclientinterfaceinfosize : super::DWORD, lpwsdllpath : windows_core::PCWSTR, phroutertransport : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprConfigTransportCreate(hmprconfig, dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pglobalinfo.as_ptr()), pglobalinfo.len().try_into().unwrap(), core::mem::transmute(pclientinterfaceinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi(), phroutertransport as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportDelete(hmprconfig: super::HANDLE, hroutertransport: super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportDelete(hmprconfig : super::HANDLE, hroutertransport : super::HANDLE) -> super::DWORD);
    unsafe { MprConfigTransportDelete(hmprconfig, hroutertransport) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportEnum(hmprconfig: super::HANDLE, dwlevel: super::DWORD, lplpbuffer: *mut super::LPBYTE, dwprefmaxlen: super::DWORD, lpdwentriesread: super::LPDWORD, lpdwtotalentries: super::LPDWORD, lpdwresumehandle: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportEnum(hmprconfig : super::HANDLE, dwlevel : super::DWORD, lplpbuffer : *mut super::LPBYTE, dwprefmaxlen : super::DWORD, lpdwentriesread : super::LPDWORD, lpdwtotalentries : super::LPDWORD, lpdwresumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { MprConfigTransportEnum(hmprconfig, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportGetHandle(hmprconfig: super::HANDLE, dwtransportid: super::DWORD, phroutertransport: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportGetHandle(hmprconfig : super::HANDLE, dwtransportid : super::DWORD, phroutertransport : *mut super::HANDLE) -> super::DWORD);
    unsafe { MprConfigTransportGetHandle(hmprconfig, dwtransportid, phroutertransport as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportGetInfo(hmprconfig: super::HANDLE, hroutertransport: super::HANDLE, ppglobalinfo: Option<*mut super::LPBYTE>, lpdwglobalinfosize: Option<super::LPDWORD>, ppclientinterfaceinfo: Option<*mut super::LPBYTE>, lpdwclientinterfaceinfosize: Option<super::LPDWORD>, lplpwsdllpath: Option<*mut windows_core::PWSTR>) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportGetInfo(hmprconfig : super::HANDLE, hroutertransport : super::HANDLE, ppglobalinfo : *mut super::LPBYTE, lpdwglobalinfosize : super::LPDWORD, ppclientinterfaceinfo : *mut super::LPBYTE, lpdwclientinterfaceinfosize : super::LPDWORD, lplpwsdllpath : *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { MprConfigTransportGetInfo(hmprconfig, hroutertransport, ppglobalinfo.unwrap_or(core::mem::zeroed()) as _, lpdwglobalinfosize.unwrap_or(core::mem::zeroed()) as _, ppclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _, lplpwsdllpath.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportSetInfo<P6>(hmprconfig: super::HANDLE, hroutertransport: super::HANDLE, pglobalinfo: Option<&[u8]>, pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P6) -> super::DWORD
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportSetInfo(hmprconfig : super::HANDLE, hroutertransport : super::HANDLE, pglobalinfo : super::LPBYTE, dwglobalinfosize : super::DWORD, pclientinterfaceinfo : super::LPBYTE, dwclientinterfaceinfosize : super::DWORD, lpwsdllpath : windows_core::PCWSTR) -> super::DWORD);
    unsafe { MprConfigTransportSetInfo(hmprconfig, hroutertransport, core::mem::transmute(pglobalinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pglobalinfo.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pclientinterfaceinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoBlockAdd(lpheader: super::LPVOID, dwinfotype: super::DWORD, dwitemsize: super::DWORD, dwitemcount: super::DWORD, lpitemdata: super::LPBYTE, lplpnewheader: *mut super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockAdd(lpheader : super::LPVOID, dwinfotype : super::DWORD, dwitemsize : super::DWORD, dwitemcount : super::DWORD, lpitemdata : super::LPBYTE, lplpnewheader : *mut super::LPVOID) -> super::DWORD);
    unsafe { MprInfoBlockAdd(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoBlockFind(lpheader: super::LPVOID, dwinfotype: super::DWORD, lpdwitemsize: super::LPDWORD, lpdwitemcount: super::LPDWORD, lplpitemdata: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockFind(lpheader : super::LPVOID, dwinfotype : super::DWORD, lpdwitemsize : super::LPDWORD, lpdwitemcount : super::LPDWORD, lplpitemdata : *mut super::LPBYTE) -> super::DWORD);
    unsafe { MprInfoBlockFind(lpheader, dwinfotype, lpdwitemsize as _, lpdwitemcount as _, lplpitemdata as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoBlockQuerySize(lpheader: super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockQuerySize(lpheader : super::LPVOID) -> super::DWORD);
    unsafe { MprInfoBlockQuerySize(lpheader) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoBlockRemove(lpheader: super::LPVOID, dwinfotype: super::DWORD, lplpnewheader: *mut super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockRemove(lpheader : super::LPVOID, dwinfotype : super::DWORD, lplpnewheader : *mut super::LPVOID) -> super::DWORD);
    unsafe { MprInfoBlockRemove(lpheader, dwinfotype, lplpnewheader as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoBlockSet(lpheader: super::LPVOID, dwinfotype: super::DWORD, dwitemsize: super::DWORD, dwitemcount: super::DWORD, lpitemdata: super::LPBYTE, lplpnewheader: *mut super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockSet(lpheader : super::LPVOID, dwinfotype : super::DWORD, dwitemsize : super::DWORD, dwitemcount : super::DWORD, lpitemdata : super::LPBYTE, lplpnewheader : *mut super::LPVOID) -> super::DWORD);
    unsafe { MprInfoBlockSet(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoCreate(dwversion: super::DWORD, lplpnewheader: *mut super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoCreate(dwversion : super::DWORD, lplpnewheader : *mut super::LPVOID) -> super::DWORD);
    unsafe { MprInfoCreate(dwversion, lplpnewheader as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoDelete(lpheader: super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoDelete(lpheader : super::LPVOID) -> super::DWORD);
    unsafe { MprInfoDelete(lpheader) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoDuplicate(lpheader: super::LPVOID, lplpnewheader: *mut super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoDuplicate(lpheader : super::LPVOID, lplpnewheader : *mut super::LPVOID) -> super::DWORD);
    unsafe { MprInfoDuplicate(lpheader, lplpnewheader as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MprInfoRemoveAll(lpheader: super::LPVOID, lplpnewheader: *mut super::LPVOID) -> super::DWORD {
    windows_core::link!("mprapi.dll" "system" fn MprInfoRemoveAll(lpheader : super::LPVOID, lplpnewheader : *mut super::LPVOID) -> super::DWORD);
    unsafe { MprInfoRemoveAll(lpheader, lplpnewheader as _) }
}
pub const ALLOW_NO_AUTH: i32 = 1;
pub const ATADDRESSLEN: i32 = 32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTH_VALIDATION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub hRasConnection: super::HANDLE,
    pub wszUserName: [super::WCHAR; 257],
    pub wszLogonDomain: [super::WCHAR; 16],
    pub AuthInfoSize: super::DWORD,
    pub AuthInfo: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AUTH_VALIDATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DO_NOT_ALLOW_NO_AUTH: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRE_CONFIG_PARAMS0 {
    pub dwNumPorts: super::DWORD,
    pub dwPortFlags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEV2_CONFIG_PARAMS {
    pub dwNumPorts: super::DWORD,
    pub dwPortFlags: super::DWORD,
    pub dwTunnelConfigParamFlags: super::DWORD,
    pub TunnelConfigParams: IKEV2_TUNNEL_CONFIG_PARAMS4,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: super::DWORD,
    pub wszAddress: [super::WCHAR; 16],
    pub wszRemoteAddress: [super::WCHAR; 16],
    pub IPv4SubInterfaceIndex: super::ULONG64,
    pub dwIPv6NegotiationError: super::DWORD,
    pub bInterfaceIdentifier: [super::BYTE; 8],
    pub bRemoteInterfaceIdentifier: [super::BYTE; 8],
    pub bPrefix: [super::BYTE; 8],
    pub dwPrefixLength: super::DWORD,
    pub IPv6SubInterfaceIndex: super::ULONG64,
    pub dwOptions: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwCompressionAlgorithm: super::DWORD,
    pub dwEncryptionMethod: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: super::DWORD,
    pub wszAddress: [super::WCHAR; 16],
    pub wszRemoteAddress: [super::WCHAR; 16],
    pub IPv4SubInterfaceIndex: super::ULONG64,
    pub dwIPv6NegotiationError: super::DWORD,
    pub bInterfaceIdentifier: [super::BYTE; 8],
    pub bRemoteInterfaceIdentifier: [super::BYTE; 8],
    pub bPrefix: [super::BYTE; 8],
    pub dwPrefixLength: super::DWORD,
    pub IPv6SubInterfaceIndex: super::ULONG64,
    pub dwOptions: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwEmbeddedEAPTypeId: super::DWORD,
    pub dwCompressionAlgorithm: super::DWORD,
    pub dwEncryptionMethod: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IKEV2_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: super::DWORD,
    pub dwNetworkBlackoutTime: super::DWORD,
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSizeForRenegotiation: super::DWORD,
    pub dwConfigOptions: super::DWORD,
    pub dwTotalCertificates: super::DWORD,
    pub certificateNames: *mut super::CERT_NAME_BLOB,
    pub machineCertificateName: super::CERT_NAME_BLOB,
    pub dwEncryptionType: super::DWORD,
    pub customPolicy: PROUTER_CUSTOM_IKEv2_POLICY0,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS3 {
    pub dwIdleTimeout: super::DWORD,
    pub dwNetworkBlackoutTime: super::DWORD,
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSizeForRenegotiation: super::DWORD,
    pub dwConfigOptions: super::DWORD,
    pub dwTotalCertificates: super::DWORD,
    pub certificateNames: *mut super::CERT_NAME_BLOB,
    pub machineCertificateName: super::CERT_NAME_BLOB,
    pub dwEncryptionType: super::DWORD,
    pub customPolicy: PROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: super::DWORD,
    pub certificateEKUs: PMPR_CERT_EKU,
    pub machineCertificateHash: super::CRYPT_HASH_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS4 {
    pub dwIdleTimeout: super::DWORD,
    pub dwNetworkBlackoutTime: super::DWORD,
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSizeForRenegotiation: super::DWORD,
    pub dwConfigOptions: super::DWORD,
    pub dwTotalCertificates: super::DWORD,
    pub certificateNames: *mut super::CERT_NAME_BLOB,
    pub machineCertificateName: super::CERT_NAME_BLOB,
    pub dwEncryptionType: super::DWORD,
    pub customPolicy: PROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: super::DWORD,
    pub certificateEKUs: PMPR_CERT_EKU,
    pub machineCertificateHash: super::CRYPT_HASH_BLOB,
    pub dwMmSaLifeTime: super::DWORD,
}
pub const IPADDRESSLEN: i32 = 15;
pub const IPXADDRESSLEN: i32 = 22;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct L2TP_CONFIG_PARAMS0 {
    pub dwNumPorts: super::DWORD,
    pub dwPortFlags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct L2TP_CONFIG_PARAMS1 {
    pub dwNumPorts: super::DWORD,
    pub dwPortFlags: super::DWORD,
    pub dwTunnelConfigParamFlags: super::DWORD,
    pub TunnelConfigParams: L2TP_TUNNEL_CONFIG_PARAMS2,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS1 {
    pub dwIdleTimeout: super::DWORD,
    pub dwEncryptionType: super::DWORD,
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSizeForRenegotiation: super::DWORD,
    pub customPolicy: PROUTER_CUSTOM_L2TP_POLICY0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: super::DWORD,
    pub dwEncryptionType: super::DWORD,
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSizeForRenegotiation: super::DWORD,
    pub customPolicy: PROUTER_CUSTOM_L2TP_POLICY0,
    pub dwMmSaLifeTime: super::DWORD,
}
pub const MAXIPADRESSLEN: i32 = 64;
pub const MAX_SSTP_HASH_SIZE: i32 = 32;
pub type MIB_SERVER_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MPRAPI_ADMIN_DLL_CALLBACKS {
    pub revision: super::UCHAR,
    pub lpfnMprAdminGetIpAddressForUser: PMPRADMINGETIPADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpAddress: PMPRADMINRELEASEIPADRESS,
    pub lpfnMprAdminGetIpv6AddressForUser: PMPRADMINGETIPV6ADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpV6AddressForUser: PMPRADMINRELEASEIPV6ADDRESSFORUSER,
    pub lpfnRasAdminAcceptNewLink: PMPRADMINACCEPTNEWLINK,
    pub lpfnRasAdminLinkHangupNotification: PMPRADMINLINKHANGUPNOTIFICATION,
    pub lpfnRasAdminTerminateDll: PMPRADMINTERMINATEDLL,
    pub lpfnRasAdminAcceptNewConnectionEx: PMPRADMINACCEPTNEWCONNECTIONEX,
    pub lpfnRasAdminAcceptEndpointChangeEx: PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX,
    pub lpfnRasAdminAcceptReauthenticationEx: PMPRADMINACCEPTREAUTHENTICATIONEX,
    pub lpfnRasAdminConnectionHangupNotificationEx: PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX,
    pub lpfnRASValidatePreAuthenticatedConnectionEx: PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX,
}
pub const MPRAPI_ADMIN_DLL_VERSION_1: i32 = 1;
pub const MPRAPI_ADMIN_DLL_VERSION_2: i32 = 2;
pub const MPRAPI_IF_CUSTOM_CONFIG_FOR_IKEV2: i32 = 1;
pub const MPRAPI_IKEV2_AUTH_USING_CERT: i32 = 1;
pub const MPRAPI_IKEV2_AUTH_USING_EAP: i32 = 2;
pub const MPRAPI_IKEV2_PROJECTION_INFO_TYPE: i32 = 2;
pub const MPRAPI_IKEV2_SET_TUNNEL_CONFIG_PARAMS: i32 = 1;
pub const MPRAPI_L2TP_SET_TUNNEL_CONFIG_PARAMS: i32 = 1;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_1: i32 = 1;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_2: i32 = 2;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_3: i32 = 3;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_1: i32 = 1;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_2: i32 = 2;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_3: i32 = 3;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_4: i32 = 4;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_5: i32 = 5;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_1: i32 = 1;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_2: i32 = 2;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_3: i32 = 3;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_4: i32 = 4;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_5: i32 = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPRAPI_OBJECT_HEADER {
    pub revision: super::UCHAR,
    pub r#type: super::UCHAR,
    pub size: super::USHORT,
}
pub type MPRAPI_OBJECT_TYPE = i32;
pub const MPRAPI_OBJECT_TYPE_AUTH_VALIDATION_OBJECT: MPRAPI_OBJECT_TYPE = 4;
pub const MPRAPI_OBJECT_TYPE_IF_CUSTOM_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = 6;
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_OBJECT: MPRAPI_OBJECT_TYPE = 2;
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_SET_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = 3;
pub const MPRAPI_OBJECT_TYPE_RAS_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = 1;
pub const MPRAPI_OBJECT_TYPE_UPDATE_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = 5;
pub const MPRAPI_PPP_PROJECTION_INFO_TYPE: i32 = 1;
pub const MPRAPI_RAS_CONNECTION_OBJECT_REVISION_1: i32 = 1;
pub const MPRAPI_RAS_UPDATE_CONNECTION_OBJECT_REVISION_1: i32 = 1;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_GRE: i32 = 16;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_IKEV2: i32 = 8;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_L2TP: i32 = 2;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_PPTP: i32 = 1;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_SSTP: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
    pub GREConfigParams: GRE_CONFIG_PARAMS0,
}
pub const MPRDM_DialAll: i32 = 1;
pub const MPRDM_DialAsNeeded: i32 = 2;
pub const MPRDM_DialFirst: i32 = 0;
pub const MPRDT_Atm: windows_core::PCSTR = windows_core::s!("ATM");
pub const MPRDT_FrameRelay: windows_core::PCSTR = windows_core::s!("FRAMERELAY");
pub const MPRDT_Generic: windows_core::PCSTR = windows_core::s!("GENERIC");
pub const MPRDT_Irda: windows_core::PCSTR = windows_core::s!("IRDA");
pub const MPRDT_Isdn: windows_core::PCSTR = windows_core::s!("isdn");
pub const MPRDT_Modem: windows_core::PCSTR = windows_core::s!("modem");
pub const MPRDT_Pad: windows_core::PCSTR = windows_core::s!("pad");
pub const MPRDT_Parallel: windows_core::PCSTR = windows_core::s!("PARALLEL");
pub const MPRDT_SW56: windows_core::PCSTR = windows_core::s!("SW56");
pub const MPRDT_Serial: windows_core::PCSTR = windows_core::s!("SERIAL");
pub const MPRDT_Sonet: windows_core::PCSTR = windows_core::s!("SONET");
pub const MPRDT_Vpn: windows_core::PCSTR = windows_core::s!("vpn");
pub const MPRDT_X25: windows_core::PCSTR = windows_core::s!("x25");
pub const MPRET_Phone: i32 = 1;
pub const MPRET_Vpn: i32 = 2;
pub const MPRIDS_Disabled: u32 = 4294967295;
pub const MPRIDS_UseGlobalValue: i32 = 0;
pub const MPRIO_DisableLcpExtensions: i32 = 32;
pub const MPRIO_IpHeaderCompression: i32 = 8;
pub const MPRIO_IpSecPreSharedKey: u32 = 2147483648;
pub const MPRIO_NetworkLogon: i32 = 8192;
pub const MPRIO_PromoteAlternates: i32 = 32768;
pub const MPRIO_RemoteDefaultGateway: i32 = 16;
pub const MPRIO_RequireCHAP: i32 = 134217728;
pub const MPRIO_RequireDataEncryption: i32 = 4096;
pub const MPRIO_RequireEAP: i32 = 131072;
pub const MPRIO_RequireEncryptedPw: i32 = 1024;
pub const MPRIO_RequireMachineCertificates: i32 = 16777216;
pub const MPRIO_RequireMsCHAP: i32 = 268435456;
pub const MPRIO_RequireMsCHAP2: i32 = 536870912;
pub const MPRIO_RequireMsEncryptedPw: i32 = 2048;
pub const MPRIO_RequirePAP: i32 = 262144;
pub const MPRIO_RequireSPAP: i32 = 524288;
pub const MPRIO_SecureLocalFiles: i32 = 65536;
pub const MPRIO_SharedPhoneNumbers: i32 = 8388608;
pub const MPRIO_SpecificIpAddr: i32 = 2;
pub const MPRIO_SpecificNameServers: i32 = 4;
pub const MPRIO_SwCompression: i32 = 512;
pub const MPRIO_UsePreSharedKeyForIkev2Initiator: i32 = 33554432;
pub const MPRIO_UsePreSharedKeyForIkev2Responder: i32 = 67108864;
pub const MPRNP_Ip: i32 = 4;
pub const MPRNP_Ipv6: i32 = 8;
pub const MPRNP_Ipx: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_CERT_EKU {
    pub dwSize: super::DWORD,
    pub IsEKUOID: windows_core::BOOL,
    pub pwszEKU: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_CREDENTIALSEX_0 {
    pub dwSize: super::DWORD,
    pub lpbCredentialsInfo: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_CREDENTIALSEX_1 {
    pub dwSize: super::DWORD,
    pub lpbCredentialsInfo: super::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_DEVICE_0 {
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
}
#[cfg(feature = "winnt")]
impl Default for MPR_DEVICE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_DEVICE_1 {
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
    pub szLocalPhoneNumber: [super::WCHAR; 129],
    pub szAlternates: super::PWCHAR,
}
#[cfg(feature = "winnt")]
impl Default for MPR_DEVICE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_ENABLE_RAS_ON_DEVICE: i32 = 1;
pub const MPR_ENABLE_ROUTING_ON_DEVICE: i32 = 2;
pub const MPR_ET_None: i32 = 0;
pub const MPR_ET_Optional: i32 = 3;
pub const MPR_ET_Require: i32 = 1;
pub const MPR_ET_RequireMax: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_FILTER_0 {
    pub fEnable: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_IFTRANSPORT_0 {
    pub dwTransportId: super::DWORD,
    pub hIfTransport: super::HANDLE,
    pub wszIfTransportName: [super::WCHAR; 41],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MPR_IFTRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: super::DWORD,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG0,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: super::DWORD,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG1,
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX2 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: super::DWORD,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG2,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_INTERFACE_0 {
    pub wszInterfaceName: [super::WCHAR; 257],
    pub hInterface: super::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: super::DWORD,
    pub dwLastError: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MPR_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_INTERFACE_1 {
    pub wszInterfaceName: [super::WCHAR; 257],
    pub hInterface: super::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: super::DWORD,
    pub dwLastError: super::DWORD,
    pub lpwsDialoutHoursRestriction: windows_core::PWSTR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MPR_INTERFACE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_INTERFACE_2 {
    pub wszInterfaceName: [super::WCHAR; 257],
    pub hInterface: super::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: super::DWORD,
    pub dwLastError: super::DWORD,
    pub dwfOptions: super::DWORD,
    pub szLocalPhoneNumber: [super::WCHAR; 129],
    pub szAlternates: super::PWCHAR,
    pub ipaddr: super::DWORD,
    pub ipaddrDns: super::DWORD,
    pub ipaddrDnsAlt: super::DWORD,
    pub ipaddrWins: super::DWORD,
    pub ipaddrWinsAlt: super::DWORD,
    pub dwfNetProtocols: super::DWORD,
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
    pub szX25PadType: [super::WCHAR; 33],
    pub szX25Address: [super::WCHAR; 201],
    pub szX25Facilities: [super::WCHAR; 201],
    pub szX25UserData: [super::WCHAR; 201],
    pub dwChannels: super::DWORD,
    pub dwSubEntries: super::DWORD,
    pub dwDialMode: super::DWORD,
    pub dwDialExtraPercent: super::DWORD,
    pub dwDialExtraSampleSeconds: super::DWORD,
    pub dwHangUpExtraPercent: super::DWORD,
    pub dwHangUpExtraSampleSeconds: super::DWORD,
    pub dwIdleDisconnectSeconds: super::DWORD,
    pub dwType: super::DWORD,
    pub dwEncryptionType: super::DWORD,
    pub dwCustomAuthKey: super::DWORD,
    pub dwCustomAuthDataSize: super::DWORD,
    pub lpbCustomAuthData: super::LPBYTE,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MPR_INTERFACE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MPR_INTERFACE_3 {
    pub wszInterfaceName: [super::WCHAR; 257],
    pub hInterface: super::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: super::DWORD,
    pub dwLastError: super::DWORD,
    pub dwfOptions: super::DWORD,
    pub szLocalPhoneNumber: [super::WCHAR; 129],
    pub szAlternates: super::PWCHAR,
    pub ipaddr: super::DWORD,
    pub ipaddrDns: super::DWORD,
    pub ipaddrDnsAlt: super::DWORD,
    pub ipaddrWins: super::DWORD,
    pub ipaddrWinsAlt: super::DWORD,
    pub dwfNetProtocols: super::DWORD,
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
    pub szX25PadType: [super::WCHAR; 33],
    pub szX25Address: [super::WCHAR; 201],
    pub szX25Facilities: [super::WCHAR; 201],
    pub szX25UserData: [super::WCHAR; 201],
    pub dwChannels: super::DWORD,
    pub dwSubEntries: super::DWORD,
    pub dwDialMode: super::DWORD,
    pub dwDialExtraPercent: super::DWORD,
    pub dwDialExtraSampleSeconds: super::DWORD,
    pub dwHangUpExtraPercent: super::DWORD,
    pub dwHangUpExtraSampleSeconds: super::DWORD,
    pub dwIdleDisconnectSeconds: super::DWORD,
    pub dwType: super::DWORD,
    pub dwEncryptionType: super::DWORD,
    pub dwCustomAuthKey: super::DWORD,
    pub dwCustomAuthDataSize: super::DWORD,
    pub lpbCustomAuthData: super::LPBYTE,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: super::DWORD,
    pub AddressCount: super::ULONG,
    pub ipv6addrDns: super::IN6_ADDR,
    pub ipv6addrDnsAlt: super::IN6_ADDR,
    pub ipv6addr: *mut super::IN6_ADDR,
}
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
impl Default for MPR_INTERFACE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_INTERFACE_ADMIN_DISABLED: i32 = 2;
pub const MPR_INTERFACE_CONNECTION_FAILURE: i32 = 4;
pub const MPR_INTERFACE_DIALOUT_HOURS_RESTRICTION: i32 = 16;
pub const MPR_INTERFACE_NO_DEVICE: i32 = 64;
pub const MPR_INTERFACE_NO_MEDIA_SENSE: i32 = 32;
pub const MPR_INTERFACE_OUT_OF_RESOURCES: i32 = 1;
pub const MPR_INTERFACE_SERVICE_PAUSED: i32 = 8;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_IPINIP_INTERFACE_0 {
    pub wszFriendlyName: [super::WCHAR; 257],
    pub Guid: windows_core::GUID,
}
#[cfg(feature = "winnt")]
impl Default for MPR_IPINIP_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_MaxAreaCode: i32 = 10;
pub const MPR_MaxCallbackNumber: i32 = 128;
pub const MPR_MaxDeviceName: i32 = 128;
pub const MPR_MaxDeviceType: i32 = 16;
pub const MPR_MaxEntryName: i32 = 256;
pub const MPR_MaxFacilities: i32 = 200;
pub const MPR_MaxIpAddress: i32 = 15;
pub const MPR_MaxIpxAddress: i32 = 21;
pub const MPR_MaxPadType: i32 = 32;
pub const MPR_MaxPhoneNumber: i32 = 128;
pub const MPR_MaxUserData: i32 = 200;
pub const MPR_MaxX25Address: i32 = 200;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_SERVER_0 {
    pub fLanOnlyMode: windows_core::BOOL,
    pub dwUpTime: super::DWORD,
    pub dwTotalPorts: super::DWORD,
    pub dwPortsInUse: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_SERVER_1 {
    pub dwNumPptpPorts: super::DWORD,
    pub dwPptpPortFlags: super::DWORD,
    pub dwNumL2tpPorts: super::DWORD,
    pub dwL2tpPortFlags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_SERVER_2 {
    pub dwNumPptpPorts: super::DWORD,
    pub dwPptpPortFlags: super::DWORD,
    pub dwNumL2tpPorts: super::DWORD,
    pub dwL2tpPortFlags: super::DWORD,
    pub dwNumSstpPorts: super::DWORD,
    pub dwSstpPortFlags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_SERVER_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: super::DWORD,
    pub dwUpTime: super::DWORD,
    pub dwTotalPorts: super::DWORD,
    pub dwPortsInUse: super::DWORD,
    pub Reserved: super::DWORD,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_SERVER_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: super::DWORD,
    pub dwUpTime: super::DWORD,
    pub dwTotalPorts: super::DWORD,
    pub dwPortsInUse: super::DWORD,
    pub Reserved: super::DWORD,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
pub type MPR_SERVER_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: super::DWORD,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: super::DWORD,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_TRANSPORT_0 {
    pub dwTransportId: super::DWORD,
    pub hTransport: super::HANDLE,
    pub wszTransportName: [super::WCHAR; 41],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MPR_TRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MPR_VPN_TRAFFIC_SELECTOR {
    pub r#type: MPR_VPN_TS_TYPE,
    pub protocolId: super::UINT8,
    pub portStart: super::UINT16,
    pub portEnd: super::UINT16,
    pub tsPayloadId: super::UINT16,
    pub addrStart: VPN_TS_IP_ADDRESS,
    pub addrEnd: VPN_TS_IP_ADDRESS,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for MPR_VPN_TRAFFIC_SELECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPR_VPN_TRAFFIC_SELECTORS {
    pub numTsi: super::DWORD,
    pub numTsr: super::DWORD,
    pub tsI: PMPR_VPN_TRAFFIC_SELECTOR,
    pub tsR: PMPR_VPN_TRAFFIC_SELECTOR,
}
pub const MPR_VPN_TS_IPv4_ADDR_RANGE: MPR_VPN_TS_TYPE = 7;
pub const MPR_VPN_TS_IPv6_ADDR_RANGE: MPR_VPN_TS_TYPE = 8;
pub type MPR_VPN_TS_TYPE = i32;
pub const MPR_VS_Default: i32 = 0;
pub const MPR_VS_Ikev2First: i32 = 8;
pub const MPR_VS_Ikev2Only: i32 = 7;
pub const MPR_VS_L2tpFirst: i32 = 4;
pub const MPR_VS_L2tpOnly: i32 = 3;
pub const MPR_VS_PptpFirst: i32 = 2;
pub const MPR_VS_PptpOnly: i32 = 1;
#[cfg(feature = "shared")]
pub type PGRE_CONFIG_PARAMS0 = *mut GRE_CONFIG_PARAMS0;
pub const PID_ATALK: i32 = 41;
pub const PID_IP: i32 = 33;
pub const PID_IPV6: i32 = 87;
pub const PID_IPX: i32 = 43;
pub const PID_NBF: i32 = 63;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PIKEV2_CONFIG_PARAMS = *mut IKEV2_CONFIG_PARAMS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIKEV2_PROJECTION_INFO = *mut IKEV2_PROJECTION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIKEV2_PROJECTION_INFO2 = *mut IKEV2_PROJECTION_INFO2;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PIKEV2_TUNNEL_CONFIG_PARAMS2 = *mut IKEV2_TUNNEL_CONFIG_PARAMS2;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PIKEV2_TUNNEL_CONFIG_PARAMS3 = *mut IKEV2_TUNNEL_CONFIG_PARAMS3;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PIKEV2_TUNNEL_CONFIG_PARAMS4 = *mut IKEV2_TUNNEL_CONFIG_PARAMS4;
#[cfg(feature = "shared")]
pub type PL2TP_CONFIG_PARAMS0 = *mut L2TP_CONFIG_PARAMS0;
#[cfg(feature = "shared")]
pub type PL2TP_CONFIG_PARAMS1 = *mut L2TP_CONFIG_PARAMS1;
#[cfg(feature = "shared")]
pub type PL2TP_TUNNEL_CONFIG_PARAMS1 = *mut L2TP_TUNNEL_CONFIG_PARAMS1;
#[cfg(feature = "shared")]
pub type PL2TP_TUNNEL_CONFIG_PARAMS2 = *mut L2TP_TUNNEL_CONFIG_PARAMS2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTNEWCONNECTION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTNEWCONNECTION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTNEWCONNECTION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTNEWCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTNEWLINK = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTREAUTHENTICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTREAUTHENTICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2)>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: RAS_CONNECTION_3)>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINGETIPADDRESSFORUSER = Option<unsafe extern "system" fn(param0: *mut super::WCHAR, param1: *mut super::WCHAR, param2: *mut super::DWORD, param3: *mut windows_core::BOOL) -> super::DWORD>;
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
pub type PMPRADMINGETIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: *mut super::WCHAR, param1: *mut super::WCHAR, param2: *mut super::IN6_ADDR, param3: *mut windows_core::BOOL) -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINLINKHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut AUTH_VALIDATION_EX) -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPRADMINRELEASEIPADRESS = Option<unsafe extern "system" fn(param0: *mut super::WCHAR, param1: *mut super::WCHAR, param2: *mut super::DWORD)>;
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
pub type PMPRADMINRELEASEIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: *mut super::WCHAR, param1: *mut super::WCHAR, param2: *mut super::IN6_ADDR)>;
#[cfg(feature = "shared")]
pub type PMPRADMINTERMINATEDLL = Option<unsafe extern "system" fn() -> super::DWORD>;
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMPRAPI_ADMIN_DLL_CALLBACKS = *mut MPRAPI_ADMIN_DLL_CALLBACKS;
#[cfg(feature = "shared")]
pub type PMPRAPI_OBJECT_HEADER = *mut MPRAPI_OBJECT_HEADER;
pub type PMPRAPI_OBJECT_TYPE = *mut MPRAPI_OBJECT_TYPE;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PMPRAPI_TUNNEL_CONFIG_PARAMS0 = *mut MPRAPI_TUNNEL_CONFIG_PARAMS0;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PMPRAPI_TUNNEL_CONFIG_PARAMS1 = *mut MPRAPI_TUNNEL_CONFIG_PARAMS1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPR_CERT_EKU = *mut MPR_CERT_EKU;
#[cfg(feature = "shared")]
pub type PMPR_CREDENTIALSEX_0 = *mut MPR_CREDENTIALSEX_0;
#[cfg(feature = "shared")]
pub type PMPR_CREDENTIALSEX_1 = *mut MPR_CREDENTIALSEX_1;
#[cfg(feature = "winnt")]
pub type PMPR_DEVICE_0 = *mut MPR_DEVICE_0;
#[cfg(feature = "winnt")]
pub type PMPR_DEVICE_1 = *mut MPR_DEVICE_1;
pub type PMPR_FILTER_0 = *mut MPR_FILTER_0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPR_IFTRANSPORT_0 = *mut MPR_IFTRANSPORT_0;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PMPR_IF_CUSTOMINFOEX0 = *mut MPR_IF_CUSTOMINFOEX0;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PMPR_IF_CUSTOMINFOEX1 = *mut MPR_IF_CUSTOMINFOEX1;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt"))]
pub type PMPR_IF_CUSTOMINFOEX2 = *mut MPR_IF_CUSTOMINFOEX2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPR_INTERFACE_0 = *mut MPR_INTERFACE_0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPR_INTERFACE_1 = *mut MPR_INTERFACE_1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPR_INTERFACE_2 = *mut MPR_INTERFACE_2;
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
pub type PMPR_INTERFACE_3 = *mut MPR_INTERFACE_3;
#[cfg(feature = "winnt")]
pub type PMPR_IPINIP_INTERFACE_0 = *mut MPR_IPINIP_INTERFACE_0;
#[cfg(feature = "shared")]
pub type PMPR_SERVER_0 = *mut MPR_SERVER_0;
#[cfg(feature = "shared")]
pub type PMPR_SERVER_1 = *mut MPR_SERVER_1;
#[cfg(feature = "shared")]
pub type PMPR_SERVER_2 = *mut MPR_SERVER_2;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PMPR_SERVER_EX0 = *mut MPR_SERVER_EX0;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PMPR_SERVER_EX1 = *mut MPR_SERVER_EX1;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PMPR_SERVER_SET_CONFIG_EX0 = *mut MPR_SERVER_SET_CONFIG_EX0;
#[cfg(all(feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PMPR_SERVER_SET_CONFIG_EX1 = *mut MPR_SERVER_SET_CONFIG_EX1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMPR_TRANSPORT_0 = *mut MPR_TRANSPORT_0;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
pub type PMPR_VPN_TRAFFIC_SELECTOR = *mut MPR_VPN_TRAFFIC_SELECTOR;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
pub type PMPR_VPN_TRAFFIC_SELECTORS = *mut MPR_VPN_TRAFFIC_SELECTORS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPPP_PROJECTION_INFO = *mut PPP_PROJECTION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPPP_PROJECTION_INFO2 = *mut PPP_PROJECTION_INFO2;
#[cfg(feature = "shared")]
pub type PPPTP_CONFIG_PARAMS = *mut PPTP_CONFIG_PARAMS;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_ATCP_INFO {
    pub dwError: super::DWORD,
    pub wszAddress: [super::WCHAR; 33],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PPP_ATCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_CCP_COMPRESSION: i32 = 1;
pub const PPP_CCP_ENCRYPTION128BIT: i32 = 64;
pub const PPP_CCP_ENCRYPTION40BIT: i32 = 32;
pub const PPP_CCP_ENCRYPTION40BITOLD: i32 = 16;
pub const PPP_CCP_ENCRYPTION56BIT: i32 = 128;
pub const PPP_CCP_HISTORYLESS: i32 = 16777216;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPP_CCP_INFO {
    pub dwError: super::DWORD,
    pub dwCompressionAlgorithm: super::DWORD,
    pub dwOptions: super::DWORD,
    pub dwRemoteCompressionAlgorithm: super::DWORD,
    pub dwRemoteOptions: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPP_INFO {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPP_INFO_2 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPP_INFO_3 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipv6: PPP_IPV6_CP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPCP_INFO {
    pub dwError: super::DWORD,
    pub wszAddress: [super::WCHAR; 16],
    pub wszRemoteAddress: [super::WCHAR; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PPP_IPCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPCP_INFO2 {
    pub dwError: super::DWORD,
    pub wszAddress: [super::WCHAR; 16],
    pub wszRemoteAddress: [super::WCHAR; 16],
    pub dwOptions: super::DWORD,
    pub dwRemoteOptions: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PPP_IPCP_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_IPCP_VJ: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPV6_CP_INFO {
    pub dwVersion: super::DWORD,
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub bInterfaceIdentifier: [super::BYTE; 8],
    pub bRemoteInterfaceIdentifier: [super::BYTE; 8],
    pub dwOptions: super::DWORD,
    pub dwRemoteOptions: super::DWORD,
    pub bPrefix: [super::BYTE; 8],
    pub dwPrefixLength: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for PPP_IPV6_CP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPXCP_INFO {
    pub dwError: super::DWORD,
    pub wszAddress: [super::WCHAR; 23],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PPP_IPXCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_LCP_3_DES: i32 = 32;
pub const PPP_LCP_ACFC: i32 = 4;
pub const PPP_LCP_AES_128: i32 = 64;
pub const PPP_LCP_AES_192: i32 = 256;
pub const PPP_LCP_AES_256: i32 = 128;
pub const PPP_LCP_CHAP: i32 = 49699;
pub const PPP_LCP_CHAP_MD5: i32 = 5;
pub const PPP_LCP_CHAP_MS: i32 = 128;
pub const PPP_LCP_CHAP_MSV2: i32 = 129;
pub const PPP_LCP_DES_56: i32 = 16;
pub const PPP_LCP_EAP: i32 = 49703;
pub const PPP_LCP_GCM_AES_128: i32 = 512;
pub const PPP_LCP_GCM_AES_192: i32 = 1024;
pub const PPP_LCP_GCM_AES_256: i32 = 2048;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPP_LCP_INFO {
    pub dwError: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwAuthenticationData: super::DWORD,
    pub dwRemoteAuthenticationProtocol: super::DWORD,
    pub dwRemoteAuthenticationData: super::DWORD,
    pub dwTerminateReason: super::DWORD,
    pub dwRemoteTerminateReason: super::DWORD,
    pub dwOptions: super::DWORD,
    pub dwRemoteOptions: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwRemoteEapTypeId: super::DWORD,
}
pub const PPP_LCP_MULTILINK_FRAMING: i32 = 1;
pub const PPP_LCP_PAP: i32 = 49187;
pub const PPP_LCP_PFC: i32 = 2;
pub const PPP_LCP_SPAP: i32 = 49191;
pub const PPP_LCP_SSHF: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_NBFCP_INFO {
    pub dwError: super::DWORD,
    pub wszWksta: [super::WCHAR; 17],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PPP_NBFCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: super::DWORD,
    pub wszAddress: [super::WCHAR; 16],
    pub wszRemoteAddress: [super::WCHAR; 16],
    pub dwIPv4Options: super::DWORD,
    pub dwIPv4RemoteOptions: super::DWORD,
    pub IPv4SubInterfaceIndex: super::ULONG64,
    pub dwIPv6NegotiationError: super::DWORD,
    pub bInterfaceIdentifier: [super::BYTE; 8],
    pub bRemoteInterfaceIdentifier: [super::BYTE; 8],
    pub bPrefix: [super::BYTE; 8],
    pub dwPrefixLength: super::DWORD,
    pub IPv6SubInterfaceIndex: super::ULONG64,
    pub dwLcpError: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwAuthenticationData: super::DWORD,
    pub dwRemoteAuthenticationProtocol: super::DWORD,
    pub dwRemoteAuthenticationData: super::DWORD,
    pub dwLcpTerminateReason: super::DWORD,
    pub dwLcpRemoteTerminateReason: super::DWORD,
    pub dwLcpOptions: super::DWORD,
    pub dwLcpRemoteOptions: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwRemoteEapTypeId: super::DWORD,
    pub dwCcpError: super::DWORD,
    pub dwCompressionAlgorithm: super::DWORD,
    pub dwCcpOptions: super::DWORD,
    pub dwRemoteCompressionAlgorithm: super::DWORD,
    pub dwCcpRemoteOptions: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: super::DWORD,
    pub wszAddress: [super::WCHAR; 16],
    pub wszRemoteAddress: [super::WCHAR; 16],
    pub dwIPv4Options: super::DWORD,
    pub dwIPv4RemoteOptions: super::DWORD,
    pub IPv4SubInterfaceIndex: super::ULONG64,
    pub dwIPv6NegotiationError: super::DWORD,
    pub bInterfaceIdentifier: [super::BYTE; 8],
    pub bRemoteInterfaceIdentifier: [super::BYTE; 8],
    pub bPrefix: [super::BYTE; 8],
    pub dwPrefixLength: super::DWORD,
    pub IPv6SubInterfaceIndex: super::ULONG64,
    pub dwLcpError: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwAuthenticationData: super::DWORD,
    pub dwRemoteAuthenticationProtocol: super::DWORD,
    pub dwRemoteAuthenticationData: super::DWORD,
    pub dwLcpTerminateReason: super::DWORD,
    pub dwLcpRemoteTerminateReason: super::DWORD,
    pub dwLcpOptions: super::DWORD,
    pub dwLcpRemoteOptions: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwEmbeddedEAPTypeId: super::DWORD,
    pub dwRemoteEapTypeId: super::DWORD,
    pub dwCcpError: super::DWORD,
    pub dwCompressionAlgorithm: super::DWORD,
    pub dwCcpOptions: super::DWORD,
    pub dwRemoteCompressionAlgorithm: super::DWORD,
    pub dwCcpRemoteOptions: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PPP_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROJECTION_INFO = *mut PROJECTION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROJECTION_INFO2 = *mut PROJECTION_INFO2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PPTP_CONFIG_PARAMS {
    pub dwNumPorts: super::DWORD,
    pub dwPortFlags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_CONNECTION_0 = *mut RAS_CONNECTION_0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_CONNECTION_1 = *mut RAS_CONNECTION_1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_CONNECTION_2 = *mut RAS_CONNECTION_2;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PRAS_CONNECTION_3 = *mut RAS_CONNECTION_3;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PRAS_CONNECTION_4 = *mut RAS_CONNECTION_4;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PRAS_CONNECTION_EX = *mut RAS_CONNECTION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_PORT_0 = *mut RAS_PORT_0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_PORT_1 = *mut RAS_PORT_1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_PORT_2 = *mut RAS_PORT_2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_UPDATE_CONNECTION = *mut RAS_UPDATE_CONNECTION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_USER_0 = *mut RAS_USER_0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAS_USER_1 = *mut RAS_USER_1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO {
    pub projectionInfoType: super::UCHAR,
    pub Anonymous: PROJECTION_INFO_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO2 {
    pub projectionInfoType: super::UCHAR,
    pub Anonymous: PROJECTION_INFO2_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO2_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO2,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO2,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROJECTION_INFO2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PROUTER_CUSTOM_IKEv2_POLICY0 = *mut _ROUTER_CUSTOM_IKEv2_POLICY0;
#[cfg(feature = "shared")]
pub type PROUTER_CUSTOM_L2TP_POLICY0 = *mut _ROUTER_CUSTOM_IKEv2_POLICY0;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PROUTER_IKEv2_IF_CUSTOM_CONFIG0 = *mut ROUTER_IKEv2_IF_CUSTOM_CONFIG0;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PROUTER_IKEv2_IF_CUSTOM_CONFIG1 = *mut ROUTER_IKEv2_IF_CUSTOM_CONFIG1;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt"))]
pub type PROUTER_IKEv2_IF_CUSTOM_CONFIG2 = *mut ROUTER_IKEv2_IF_CUSTOM_CONFIG2;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PSSTP_CERT_INFO = *mut SSTP_CERT_INFO;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PSSTP_CONFIG_PARAMS = *mut SSTP_CONFIG_PARAMS;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
pub type PVPN_TS_IP_ADDRESS = *mut VPN_TS_IP_ADDRESS;
pub const RASCCPCA_MPPC: i32 = 6;
pub const RASCCPCA_STAC: i32 = 5;
pub const RASPRIV2_DialinPolicy: i32 = 1;
pub const RASPRIV_AdminSetCallback: i32 = 2;
pub const RASPRIV_CallbackType: i32 = 7;
pub const RASPRIV_CallerSetCallback: i32 = 4;
pub const RASPRIV_DialinPrivilege: i32 = 8;
pub const RASPRIV_NoCallback: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_CONNECTION_0 {
    pub hConnection: super::HANDLE,
    pub hInterface: super::HANDLE,
    pub dwConnectDuration: super::DWORD,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: super::DWORD,
    pub wszInterfaceName: [super::WCHAR; 257],
    pub wszUserName: [super::WCHAR; 257],
    pub wszLogonDomain: [super::WCHAR; 16],
    pub wszRemoteComputer: [super::WCHAR; 17],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAS_CONNECTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAS_CONNECTION_1 {
    pub hConnection: super::HANDLE,
    pub hInterface: super::HANDLE,
    pub PppInfo: PPP_INFO,
    pub dwBytesXmited: super::DWORD,
    pub dwBytesRcved: super::DWORD,
    pub dwFramesXmited: super::DWORD,
    pub dwFramesRcved: super::DWORD,
    pub dwCrcErr: super::DWORD,
    pub dwTimeoutErr: super::DWORD,
    pub dwAlignmentErr: super::DWORD,
    pub dwHardwareOverrunErr: super::DWORD,
    pub dwFramingErr: super::DWORD,
    pub dwBufferOverrunErr: super::DWORD,
    pub dwCompressionRatioIn: super::DWORD,
    pub dwCompressionRatioOut: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_CONNECTION_2 {
    pub hConnection: super::HANDLE,
    pub wszUserName: [super::WCHAR; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo2: PPP_INFO_2,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAS_CONNECTION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_CONNECTION_3 {
    pub dwVersion: super::DWORD,
    pub dwSize: super::DWORD,
    pub hConnection: super::HANDLE,
    pub wszUserName: [super::WCHAR; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo3: PPP_INFO_3,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub timer: super::FILETIME,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for RAS_CONNECTION_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_4 {
    pub dwConnectDuration: super::DWORD,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: super::DWORD,
    pub wszInterfaceName: [super::WCHAR; 257],
    pub wszUserName: [super::WCHAR; 257],
    pub wszLogonDomain: [super::WCHAR; 16],
    pub wszRemoteComputer: [super::WCHAR; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::FILETIME,
    pub connectionStartTime: super::FILETIME,
    pub ullBytesXmited: super::ULONGLONG,
    pub ullBytesRcved: super::ULONGLONG,
    pub dwFramesXmited: super::DWORD,
    pub dwFramesRcved: super::DWORD,
    pub dwCrcErr: super::DWORD,
    pub dwTimeoutErr: super::DWORD,
    pub dwAlignmentErr: super::DWORD,
    pub dwHardwareOverrunErr: super::DWORD,
    pub dwFramingErr: super::DWORD,
    pub dwBufferOverrunErr: super::DWORD,
    pub dwCompressionRatioIn: super::DWORD,
    pub dwCompressionRatioOut: super::DWORD,
    pub dwNumSwitchOvers: super::DWORD,
    pub wszRemoteEndpointAddress: [super::WCHAR; 65],
    pub wszLocalEndpointAddress: [super::WCHAR; 65],
    pub ProjectionInfo: PROJECTION_INFO2,
    pub hConnection: super::HANDLE,
    pub hInterface: super::HANDLE,
    pub dwDeviceType: super::DWORD,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for RAS_CONNECTION_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwConnectDuration: super::DWORD,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: super::DWORD,
    pub wszInterfaceName: [super::WCHAR; 257],
    pub wszUserName: [super::WCHAR; 257],
    pub wszLogonDomain: [super::WCHAR; 16],
    pub wszRemoteComputer: [super::WCHAR; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::FILETIME,
    pub dwBytesXmited: super::DWORD,
    pub dwBytesRcved: super::DWORD,
    pub dwFramesXmited: super::DWORD,
    pub dwFramesRcved: super::DWORD,
    pub dwCrcErr: super::DWORD,
    pub dwTimeoutErr: super::DWORD,
    pub dwAlignmentErr: super::DWORD,
    pub dwHardwareOverrunErr: super::DWORD,
    pub dwFramingErr: super::DWORD,
    pub dwBufferOverrunErr: super::DWORD,
    pub dwCompressionRatioIn: super::DWORD,
    pub dwCompressionRatioOut: super::DWORD,
    pub dwNumSwitchOvers: super::DWORD,
    pub wszRemoteEndpointAddress: [super::WCHAR; 65],
    pub wszLocalEndpointAddress: [super::WCHAR; 65],
    pub ProjectionInfo: PROJECTION_INFO,
    pub hConnection: super::HANDLE,
    pub hInterface: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for RAS_CONNECTION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RAS_FLAGS_DORMANT: i32 = 32;
pub const RAS_FLAGS_MESSENGER_PRESENT: i32 = 2;
pub const RAS_FLAGS_PPP_CONNECTION: i32 = 1;
pub const RAS_FLAGS_QUARANTINE_PRESENT: i32 = 8;
pub type RAS_HARDWARE_CONDITION = i32;
pub const RAS_HARDWARE_FAILURE: RAS_HARDWARE_CONDITION = 1;
pub const RAS_HARDWARE_OPERATIONAL: RAS_HARDWARE_CONDITION = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_PORT_0 {
    pub hPort: super::HANDLE,
    pub hConnection: super::HANDLE,
    pub dwPortCondition: RAS_PORT_CONDITION,
    pub dwTotalNumberOfCalls: super::DWORD,
    pub dwConnectDuration: super::DWORD,
    pub wszPortName: [super::WCHAR; 17],
    pub wszMediaName: [super::WCHAR; 17],
    pub wszDeviceName: [super::WCHAR; 129],
    pub wszDeviceType: [super::WCHAR; 17],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAS_PORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAS_PORT_1 {
    pub hPort: super::HANDLE,
    pub hConnection: super::HANDLE,
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: super::DWORD,
    pub dwBytesXmited: super::DWORD,
    pub dwBytesRcved: super::DWORD,
    pub dwFramesXmited: super::DWORD,
    pub dwFramesRcved: super::DWORD,
    pub dwCrcErr: super::DWORD,
    pub dwTimeoutErr: super::DWORD,
    pub dwAlignmentErr: super::DWORD,
    pub dwHardwareOverrunErr: super::DWORD,
    pub dwFramingErr: super::DWORD,
    pub dwBufferOverrunErr: super::DWORD,
    pub dwCompressionRatioIn: super::DWORD,
    pub dwCompressionRatioOut: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_PORT_2 {
    pub hPort: super::HANDLE,
    pub hConnection: super::HANDLE,
    pub dwConn_State: super::DWORD,
    pub wszPortName: [super::WCHAR; 17],
    pub wszMediaName: [super::WCHAR; 17],
    pub wszDeviceName: [super::WCHAR; 129],
    pub wszDeviceType: [super::WCHAR; 17],
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: super::DWORD,
    pub dwCrcErr: super::DWORD,
    pub dwSerialOverRunErrs: super::DWORD,
    pub dwTimeoutErr: super::DWORD,
    pub dwAlignmentErr: super::DWORD,
    pub dwHardwareOverrunErr: super::DWORD,
    pub dwFramingErr: super::DWORD,
    pub dwBufferOverrunErr: super::DWORD,
    pub dwCompressionRatioIn: super::DWORD,
    pub dwCompressionRatioOut: super::DWORD,
    pub dwTotalErrors: super::DWORD,
    pub ullBytesXmited: super::ULONGLONG,
    pub ullBytesRcved: super::ULONGLONG,
    pub ullFramesXmited: super::ULONGLONG,
    pub ullFramesRcved: super::ULONGLONG,
    pub ullBytesTxUncompressed: super::ULONGLONG,
    pub ullBytesTxCompressed: super::ULONGLONG,
    pub ullBytesRcvUncompressed: super::ULONGLONG,
    pub ullBytesRcvCompressed: super::ULONGLONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAS_PORT_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RAS_PORT_AUTHENTICATED: RAS_PORT_CONDITION = 5;
pub const RAS_PORT_AUTHENTICATING: RAS_PORT_CONDITION = 4;
pub const RAS_PORT_CALLING_BACK: RAS_PORT_CONDITION = 2;
pub type RAS_PORT_CONDITION = i32;
pub const RAS_PORT_DISCONNECTED: RAS_PORT_CONDITION = 1;
pub const RAS_PORT_INITIALIZING: RAS_PORT_CONDITION = 6;
pub const RAS_PORT_LISTENING: RAS_PORT_CONDITION = 3;
pub const RAS_PORT_NON_OPERATIONAL: RAS_PORT_CONDITION = 0;
pub type RAS_QUARANTINE_STATE = i32;
pub const RAS_QUAR_STATE_NORMAL: RAS_QUARANTINE_STATE = 0;
pub const RAS_QUAR_STATE_NOT_CAPABLE: RAS_QUARANTINE_STATE = 3;
pub const RAS_QUAR_STATE_PROBATION: RAS_QUARANTINE_STATE = 2;
pub const RAS_QUAR_STATE_QUARANTINE: RAS_QUARANTINE_STATE = 1;
pub type RAS_SERVER_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_UPDATE_CONNECTION {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwIfIndex: super::DWORD,
    pub wszLocalEndpointAddress: [super::WCHAR; 65],
    pub wszRemoteEndpointAddress: [super::WCHAR; 65],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAS_UPDATE_CONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_USER_0 {
    pub bfPrivilege: super::BYTE,
    pub wszPhoneNumber: [super::WCHAR; 129],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAS_USER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_USER_1 {
    pub bfPrivilege: super::BYTE,
    pub wszPhoneNumber: [super::WCHAR; 129],
    pub bfPrivilege2: super::BYTE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAS_USER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ROUTER_CONNECTION_STATE = i32;
#[cfg(feature = "shared")]
pub type ROUTER_CUSTOM_IKEv2_POLICY0 = _ROUTER_CUSTOM_IKEv2_POLICY0;
#[cfg(feature = "shared")]
pub type ROUTER_CUSTOM_L2TP_POLICY0 = _ROUTER_CUSTOM_IKEv2_POLICY0;
pub const ROUTER_IF_STATE_CONNECTED: ROUTER_CONNECTION_STATE = 3;
pub const ROUTER_IF_STATE_CONNECTING: ROUTER_CONNECTION_STATE = 2;
pub const ROUTER_IF_STATE_DISCONNECTED: ROUTER_CONNECTION_STATE = 1;
pub const ROUTER_IF_STATE_UNREACHABLE: ROUTER_CONNECTION_STATE = 0;
pub const ROUTER_IF_TYPE_CLIENT: ROUTER_INTERFACE_TYPE = 0;
pub const ROUTER_IF_TYPE_DEDICATED: ROUTER_INTERFACE_TYPE = 3;
pub const ROUTER_IF_TYPE_DIALOUT: ROUTER_INTERFACE_TYPE = 7;
pub const ROUTER_IF_TYPE_FULL_ROUTER: ROUTER_INTERFACE_TYPE = 2;
pub const ROUTER_IF_TYPE_HOME_ROUTER: ROUTER_INTERFACE_TYPE = 1;
pub const ROUTER_IF_TYPE_INTERNAL: ROUTER_INTERFACE_TYPE = 4;
pub const ROUTER_IF_TYPE_LOOPBACK: ROUTER_INTERFACE_TYPE = 5;
pub const ROUTER_IF_TYPE_MAX: ROUTER_INTERFACE_TYPE = 8;
pub const ROUTER_IF_TYPE_TUNNEL1: ROUTER_INTERFACE_TYPE = 6;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSize: super::DWORD,
    pub certificateName: super::CERT_NAME_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSize: super::DWORD,
    pub certificateName: super::CERT_NAME_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::CRYPT_HASH_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    pub dwSaLifeTime: super::DWORD,
    pub dwSaDataSize: super::DWORD,
    pub certificateName: super::CERT_NAME_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::CRYPT_HASH_BLOB,
    pub dwMmSaLifeTime: super::DWORD,
    pub vpnTrafficSelectors: MPR_VPN_TRAFFIC_SELECTORS,
}
pub type ROUTER_INTERFACE_TYPE = i32;
pub const RRAS_SERVICE_NAME: windows_core::PCSTR = windows_core::s!("RemoteAccess");
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SSTP_CERT_INFO {
    pub isDefault: windows_core::BOOL,
    pub certBlob: super::CRYPT_HASH_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SSTP_CONFIG_PARAMS {
    pub dwNumPorts: super::DWORD,
    pub dwPortFlags: super::DWORD,
    pub isUseHttps: windows_core::BOOL,
    pub certAlgorithm: super::DWORD,
    pub sstpCertDetails: SSTP_CERT_INFO,
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct VPN_TS_IP_ADDRESS {
    pub Type: super::USHORT,
    pub Anonymous: VPN_TS_IP_ADDRESS_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for VPN_TS_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub union VPN_TS_IP_ADDRESS_0 {
    pub v4: super::IN_ADDR,
    pub v6: super::IN6_ADDR,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for VPN_TS_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _ROUTER_CUSTOM_IKEv2_POLICY0 {
    pub dwIntegrityMethod: super::DWORD,
    pub dwEncryptionMethod: super::DWORD,
    pub dwCipherTransformConstant: super::DWORD,
    pub dwAuthTransformConstant: super::DWORD,
    pub dwPfsGroup: super::DWORD,
    pub dwDhGroup: super::DWORD,
}
