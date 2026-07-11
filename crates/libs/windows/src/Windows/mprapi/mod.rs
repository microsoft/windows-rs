#[inline]
pub unsafe fn MprAdminBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprAdminBufferFree(pbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminConnectionClearStats(hrasserver: RAS_SERVER_HANDLE, hrasconnection: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionClearStats(hrasserver : RAS_SERVER_HANDLE, hrasconnection : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminConnectionClearStats(hrasserver, hrasconnection) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionEnum(hrasserver: RAS_SERVER_HANDLE, dwlevel: u32, lplpbbuffer: *mut super::minwindef::LPBYTE, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionEnum(hrasserver : RAS_SERVER_HANDLE, dwlevel : u32, lplpbbuffer : *mut super::minwindef::LPBYTE, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminConnectionEnum(hrasserver, dwlevel, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionEnumEx(hrasserver: RAS_SERVER_HANDLE, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut PRAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionEnumEx(hrasserver : RAS_SERVER_HANDLE, pobjectheader : *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, pprasconn : *mut PRAS_CONNECTION_EX, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminConnectionEnumEx(hrasserver, pobjectheader, dwpreferedmaxlen, lpdwentriesread as _, lpdwtotalentries as _, pprasconn as _, lpdwresumehandle) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionGetInfo(hrasserver: RAS_SERVER_HANDLE, dwlevel: u32, hrasconnection: super::winnt::HANDLE, lplpbbuffer: *mut super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfo(hrasserver : RAS_SERVER_HANDLE, dwlevel : u32, hrasconnection : super::winnt::HANDLE, lplpbbuffer : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { MprAdminConnectionGetInfo(hrasserver, dwlevel, hrasconnection, lplpbbuffer as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminConnectionGetInfoEx(hrasserver: RAS_SERVER_HANDLE, hrasconnection: super::winnt::HANDLE, prasconnection: *mut RAS_CONNECTION_EX) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfoEx(hrasserver : RAS_SERVER_HANDLE, hrasconnection : super::winnt::HANDLE, prasconnection : *mut RAS_CONNECTION_EX) -> u32);
    unsafe { MprAdminConnectionGetInfoEx(hrasserver, hrasconnection, prasconnection as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminConnectionRemoveQuarantine(hrasserver: super::winnt::HANDLE, hrasconnection: super::winnt::HANDLE, fisipaddress: bool) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminConnectionRemoveQuarantine(hrasserver : super::winnt::HANDLE, hrasconnection : super::winnt::HANDLE, fisipaddress : windows_core::BOOL) -> u32);
    unsafe { MprAdminConnectionRemoveQuarantine(hrasserver, hrasconnection, fisipaddress.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminDeregisterConnectionNotification(hmprserver: MPR_SERVER_HANDLE, heventnotification: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminDeregisterConnectionNotification(hmprserver : MPR_SERVER_HANDLE, heventnotification : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminDeregisterConnectionNotification(hmprserver, heventnotification) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminDeviceEnum(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lplpbbuffer: *mut super::minwindef::LPBYTE, lpdwtotalentries: *mut u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminDeviceEnum(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lplpbbuffer : *mut super::minwindef::LPBYTE, lpdwtotalentries : *mut u32) -> u32);
    unsafe { MprAdminDeviceEnum(hmprserver, dwlevel, lplpbbuffer as _, lpdwtotalentries as _) }
}
#[inline]
pub unsafe fn MprAdminEstablishDomainRasServer<P0, P1>(pszdomain: P0, pszmachine: P1, benable: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminEstablishDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, benable : windows_core::BOOL) -> u32);
    unsafe { MprAdminEstablishDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), benable.into()) }
}
#[inline]
pub unsafe fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut windows_core::PWSTR) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminGetErrorString(dwerror : u32, lplpwserrorstring : *mut windows_core::PWSTR) -> u32);
    unsafe { MprAdminGetErrorString(dwerror, lplpwserrorstring as _) }
}
#[inline]
pub unsafe fn MprAdminGetPDCServer<P0, P1>(lpszdomain: P0, lpszserver: P1, lpszpdcserver: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminGetPDCServer(lpszdomain : windows_core::PCWSTR, lpszserver : windows_core::PCWSTR, lpszpdcserver : windows_core::PWSTR) -> u32);
    unsafe { MprAdminGetPDCServer(lpszdomain.param().abi(), lpszserver.param().abi(), core::mem::transmute(lpszpdcserver)) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceConnect(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, hevent: super::winnt::HANDLE, fsynchronous: bool) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceConnect(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, hevent : super::winnt::HANDLE, fsynchronous : windows_core::BOOL) -> u32);
    unsafe { MprAdminInterfaceConnect(hmprserver, hinterface, hevent, fsynchronous.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceCreate(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceCreate(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lpbbuffer : *const u8, phinterface : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprAdminInterfaceCreate(hmprserver, dwlevel, lpbbuffer, phinterface as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceDelete(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDelete(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminInterfaceDelete(hmprserver, hinterface) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceGetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwindex: u32, dwlevel: u32, lplpbuffer: *mut super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceGetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwindex : u32, dwlevel : u32, lplpbuffer : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { MprAdminInterfaceDeviceGetInfo(hmprserver, hinterface, dwindex, dwlevel, lplpbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceSetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceSetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwindex : u32, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminInterfaceDeviceSetInfo(hmprserver, hinterface, dwindex, dwlevel, lpbbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceDisconnect(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceDisconnect(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminInterfaceDisconnect(hmprserver, hinterface) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceEnum(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lplpbbuffer: *mut super::minwindef::LPBYTE, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceEnum(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lplpbbuffer : *mut super::minwindef::LPBYTE, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminInterfaceEnum(hmprserver, dwlevel, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentials<P0, P1>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: Option<windows_core::PWSTR>, lpwspassword: Option<windows_core::PWSTR>, lpwsdomainname: Option<windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PWSTR, lpwspassword : windows_core::PWSTR, lpwsdomainname : windows_core::PWSTR) -> u32);
    unsafe { MprAdminInterfaceGetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), lpwsusername.unwrap_or(core::mem::zeroed()) as _, lpwspassword.unwrap_or(core::mem::zeroed()) as _, lpwsdomainname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentialsEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwlevel: u32, lplpbbuffer: *mut super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentialsEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwlevel : u32, lplpbbuffer : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { MprAdminInterfaceGetCredentialsEx(hmprserver, hinterface, dwlevel, lplpbbuffer as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCustomInfoEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCustomInfoEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, pcustominfo : *mut MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprAdminInterfaceGetCustomInfoEx(hmprserver, hinterface, pcustominfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceGetHandle<P1>(hmprserver: MPR_SERVER_HANDLE, lpwsinterfacename: P1, phinterface: *mut super::winnt::HANDLE, fincludeclientinterfaces: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetHandle(hmprserver : MPR_SERVER_HANDLE, lpwsinterfacename : windows_core::PCWSTR, phinterface : *mut super::winnt::HANDLE, fincludeclientinterfaces : windows_core::BOOL) -> u32);
    unsafe { MprAdminInterfaceGetHandle(hmprserver, lpwsinterfacename.param().abi(), phinterface as _, fincludeclientinterfaces.into()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwlevel: u32, lplpbbuffer: *const super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceGetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwlevel : u32, lplpbbuffer : *const super::minwindef::LPBYTE) -> u32);
    unsafe { MprAdminInterfaceGetInfo(hmprserver, hinterface, dwlevel, lplpbbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceQueryUpdateResult(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceQueryUpdateResult(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwprotocolid : u32, lpdwupdateresult : *mut u32) -> u32);
    unsafe { MprAdminInterfaceQueryUpdateResult(hmprserver, hinterface, dwprotocolid, lpdwupdateresult as _) }
}
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentials<P0, P1, P2, P3, P4>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: P2, lpwsdomainname: P3, lpwspassword: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PCWSTR, lpwsdomainname : windows_core::PCWSTR, lpwspassword : windows_core::PCWSTR) -> u32);
    unsafe { MprAdminInterfaceSetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), lpwsusername.param().abi(), lpwsdomainname.param().abi(), lpwspassword.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentialsEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentialsEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminInterfaceSetCredentialsEx(hmprserver, hinterface, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetCustomInfoEx(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCustomInfoEx(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, pcustominfo : *const MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprAdminInterfaceSetCustomInfoEx(hmprserver, hinterface, pcustominfo) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceSetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceSetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminInterfaceSetInfo(hmprserver, hinterface, dwlevel, lpbbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportAdd(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportAdd(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwtransportid : u32, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    unsafe { MprAdminInterfaceTransportAdd(hmprserver, hinterface, dwtransportid, pinterfaceinfo, dwinterfaceinfosize) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminInterfaceTransportGetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwtransportid: u32, ppinterfaceinfo: *mut super::minwindef::LPBYTE, lpdwinterfaceinfosize: Option<*mut u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportGetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwtransportid : u32, ppinterfaceinfo : *mut super::minwindef::LPBYTE, lpdwinterfaceinfosize : *mut u32) -> u32);
    unsafe { MprAdminInterfaceTransportGetInfo(hmprserver, hinterface, dwtransportid, ppinterfaceinfo as _, lpdwinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportRemove(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwtransportid: u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportRemove(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwtransportid : u32) -> u32);
    unsafe { MprAdminInterfaceTransportRemove(hmprserver, hinterface, dwtransportid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportSetInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportSetInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwtransportid : u32, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    unsafe { MprAdminInterfaceTransportSetInfo(hmprserver, hinterface, dwtransportid, pinterfaceinfo, dwinterfaceinfosize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminInterfaceUpdatePhonebookInfo(hmprserver, hinterface) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminInterfaceUpdateRoutes(hmprserver: MPR_SERVER_HANDLE, hinterface: super::winnt::HANDLE, dwprotocolid: u32, hevent: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdateRoutes(hmprserver : MPR_SERVER_HANDLE, hinterface : super::winnt::HANDLE, dwprotocolid : u32, hevent : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminInterfaceUpdateRoutes(hmprserver, hinterface, dwprotocolid, hevent) }
}
#[inline]
pub unsafe fn MprAdminIsDomainRasServer<P0, P1>(pszdomain: P0, pszmachine: P1, pbisrasserver: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminIsDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, pbisrasserver : *mut windows_core::BOOL) -> u32);
    unsafe { MprAdminIsDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), pbisrasserver as _) }
}
#[inline]
pub unsafe fn MprAdminIsServiceInitialized<P0>(lpwsservername: P0, fisserviceinitialized: *const windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminIsServiceInitialized(lpwsservername : windows_core::PCWSTR, fisserviceinitialized : *const windows_core::BOOL) -> u32);
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
#[inline]
pub unsafe fn MprAdminMIBBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprAdminMIBBufferFree(pbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBEntryCreate(hmibserver: MIB_SERVER_HANDLE, dwpid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryCreate(hmibserver : MIB_SERVER_HANDLE, dwpid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    unsafe { MprAdminMIBEntryCreate(hmibserver, dwpid, dwroutingpid, lpentry, dwentrysize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBEntryDelete(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryDelete(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    unsafe { MprAdminMIBEntryDelete(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBEntryGet(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryGet(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    unsafe { MprAdminMIBEntryGet(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBEntryGetFirst(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetFirst(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    unsafe { MprAdminMIBEntryGetFirst(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBEntryGetNext(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetNext(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    unsafe { MprAdminMIBEntryGetNext(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry as _, lpoutentrysize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBEntrySet(hmibserver: MIB_SERVER_HANDLE, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBEntrySet(hmibserver : MIB_SERVER_HANDLE, dwprotocolid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    unsafe { MprAdminMIBEntrySet(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBServerConnect<P0>(lpwsservername: P0, phmibserver: *mut MIB_SERVER_HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBServerConnect(lpwsservername : windows_core::PCWSTR, phmibserver : *mut MIB_SERVER_HANDLE) -> u32);
    unsafe { MprAdminMIBServerConnect(lpwsservername.param().abi(), phmibserver as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminMIBServerDisconnect(hmibserver: MIB_SERVER_HANDLE) {
    windows_core::link!("mprapi.dll" "system" fn MprAdminMIBServerDisconnect(hmibserver : MIB_SERVER_HANDLE));
    unsafe { MprAdminMIBServerDisconnect(hmibserver) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminPortClearStats(hrasserver: RAS_SERVER_HANDLE, hport: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortClearStats(hrasserver : RAS_SERVER_HANDLE, hport : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminPortClearStats(hrasserver, hport) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminPortDisconnect(hrasserver: RAS_SERVER_HANDLE, hport: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortDisconnect(hrasserver : RAS_SERVER_HANDLE, hport : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminPortDisconnect(hrasserver, hport) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminPortEnum(hrasserver: RAS_SERVER_HANDLE, dwlevel: u32, hrasconnection: super::winnt::HANDLE, lplpbbuffer: *mut super::minwindef::LPBYTE, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortEnum(hrasserver : RAS_SERVER_HANDLE, dwlevel : u32, hrasconnection : super::winnt::HANDLE, lplpbbuffer : *mut super::minwindef::LPBYTE, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    unsafe { MprAdminPortEnum(hrasserver, dwlevel, hrasconnection, lplpbbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminPortGetInfo(hrasserver: RAS_SERVER_HANDLE, dwlevel: u32, hport: super::winnt::HANDLE, lplpbbuffer: *mut super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortGetInfo(hrasserver : RAS_SERVER_HANDLE, dwlevel : u32, hport : super::winnt::HANDLE, lplpbbuffer : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { MprAdminPortGetInfo(hrasserver, dwlevel, hport, lplpbbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminPortReset(hrasserver: RAS_SERVER_HANDLE, hport: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminPortReset(hrasserver : RAS_SERVER_HANDLE, hport : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminPortReset(hrasserver, hport) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminRegisterConnectionNotification(hmprserver: MPR_SERVER_HANDLE, heventnotification: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminRegisterConnectionNotification(hmprserver : MPR_SERVER_HANDLE, heventnotification : super::winnt::HANDLE) -> u32);
    unsafe { MprAdminRegisterConnectionNotification(hmprserver, heventnotification) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminSendUserMessage<P2>(hmprserver: MPR_SERVER_HANDLE, hconnection: super::winnt::HANDLE, lpwszmessage: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminSendUserMessage(hmprserver : MPR_SERVER_HANDLE, hconnection : super::winnt::HANDLE, lpwszmessage : windows_core::PCWSTR) -> u32);
    unsafe { MprAdminSendUserMessage(hmprserver, hconnection, lpwszmessage.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminServerConnect<P0>(lpwsservername: P0, phmprserver: *mut MPR_SERVER_HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerConnect(lpwsservername : windows_core::PCWSTR, phmprserver : *mut MPR_SERVER_HANDLE) -> u32);
    unsafe { MprAdminServerConnect(lpwsservername.param().abi(), phmprserver as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminServerDisconnect(hmprserver: MPR_SERVER_HANDLE) {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerDisconnect(hmprserver : MPR_SERVER_HANDLE));
    unsafe { MprAdminServerDisconnect(hmprserver) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminServerGetCredentials(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lplpbbuffer: *const super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerGetCredentials(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lplpbbuffer : *const super::minwindef::LPBYTE) -> u32);
    unsafe { MprAdminServerGetCredentials(hmprserver, dwlevel, lplpbbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminServerGetInfo(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lplpbbuffer: *mut super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerGetInfo(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lplpbbuffer : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { MprAdminServerGetInfo(hmprserver, dwlevel, lplpbbuffer as _) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminServerGetInfoEx(hmprserver: MPR_SERVER_HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerGetInfoEx(hmprserver : MPR_SERVER_HANDLE, pserverinfo : *mut MPR_SERVER_EX1) -> u32);
    unsafe { MprAdminServerGetInfoEx(hmprserver, pserverinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminServerSetCredentials(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerSetCredentials(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminServerSetCredentials(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminServerSetInfo(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerSetInfo(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminServerSetInfo(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminServerSetInfoEx(hmprserver: MPR_SERVER_HANDLE, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminServerSetInfoEx(hmprserver : MPR_SERVER_HANDLE, pserverinfo : *const MPR_SERVER_SET_CONFIG_EX1) -> u32);
    unsafe { MprAdminServerSetInfoEx(hmprserver, pserverinfo) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminTransportCreate<P2, P7>(hmprserver: MPR_SERVER_HANDLE, dwtransportid: u32, lpwstransportname: P2, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: Option<*const u8>, dwclientinterfaceinfosize: Option<u32>, lpwsdllpath: P7) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminTransportCreate(hmprserver : MPR_SERVER_HANDLE, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR) -> u32);
    unsafe { MprAdminTransportCreate(hmprserver, dwtransportid, lpwstransportname.param().abi(), pglobalinfo, dwglobalinfosize, pclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, dwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _, lpwsdllpath.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprAdminTransportGetInfo(hmprserver: MPR_SERVER_HANDLE, dwtransportid: u32, ppglobalinfo: Option<*mut super::minwindef::LPBYTE>, lpdwglobalinfosize: Option<*mut u32>, ppclientinterfaceinfo: Option<*mut super::minwindef::LPBYTE>, lpdwclientinterfaceinfosize: Option<*mut u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminTransportGetInfo(hmprserver : MPR_SERVER_HANDLE, dwtransportid : u32, ppglobalinfo : *mut super::minwindef::LPBYTE, lpdwglobalinfosize : *mut u32, ppclientinterfaceinfo : *mut super::minwindef::LPBYTE, lpdwclientinterfaceinfosize : *mut u32) -> u32);
    unsafe { MprAdminTransportGetInfo(hmprserver, dwtransportid, ppglobalinfo.unwrap_or(core::mem::zeroed()) as _, lpdwglobalinfosize.unwrap_or(core::mem::zeroed()) as _, ppclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminTransportSetInfo(hmprserver: MPR_SERVER_HANDLE, dwtransportid: u32, pglobalinfo: Option<*const u8>, dwglobalinfosize: u32, pclientinterfaceinfo: Option<*const u8>, dwclientinterfaceinfosize: u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminTransportSetInfo(hmprserver : MPR_SERVER_HANDLE, dwtransportid : u32, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32) -> u32);
    unsafe { MprAdminTransportSetInfo(hmprserver, dwtransportid, pglobalinfo.unwrap_or(core::mem::zeroed()) as _, dwglobalinfosize, pclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, dwclientinterfaceinfosize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprAdminUpdateConnection(hrasserver: RAS_SERVER_HANDLE, hrasconnection: super::winnt::HANDLE, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprAdminUpdateConnection(hrasserver : RAS_SERVER_HANDLE, hrasconnection : super::winnt::HANDLE, prasupdateconnection : *const RAS_UPDATE_CONNECTION) -> u32);
    unsafe { MprAdminUpdateConnection(hrasserver, hrasconnection, prasupdateconnection) }
}
#[inline]
pub unsafe fn MprAdminUserGetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminUserGetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : u32, lpbbuffer : *mut u8) -> u32);
    unsafe { MprAdminUserGetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer as _) }
}
#[inline]
pub unsafe fn MprAdminUserSetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprAdminUserSetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprAdminUserSetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer) }
}
#[inline]
pub unsafe fn MprConfigBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprConfigBufferFree(pbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigFilterGetInfo(hmprconfig: super::winnt::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigFilterGetInfo(hmprconfig : super::winnt::HANDLE, dwlevel : u32, dwtransportid : u32, lpbuffer : *mut u8) -> u32);
    unsafe { MprConfigFilterGetInfo(hmprconfig, dwlevel, dwtransportid, lpbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigFilterSetInfo(hmprconfig: super::winnt::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigFilterSetInfo(hmprconfig : super::winnt::HANDLE, dwlevel : u32, dwtransportid : u32, lpbuffer : *const u8) -> u32);
    unsafe { MprConfigFilterSetInfo(hmprconfig, dwlevel, dwtransportid, lpbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigGetFriendlyName<P1>(hmprconfig: super::winnt::HANDLE, pszguidname: P1, pszbuffer: *mut u16, dwbuffersize: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigGetFriendlyName(hmprconfig : super::winnt::HANDLE, pszguidname : windows_core::PCWSTR, pszbuffer : *mut u16, dwbuffersize : u32) -> u32);
    unsafe { MprConfigGetFriendlyName(hmprconfig, pszguidname.param().abi(), pszbuffer as _, dwbuffersize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigGetGuidName<P1>(hmprconfig: super::winnt::HANDLE, pszfriendlyname: P1, pszbuffer: *mut u16, dwbuffersize: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigGetGuidName(hmprconfig : super::winnt::HANDLE, pszfriendlyname : windows_core::PCWSTR, pszbuffer : *mut u16, dwbuffersize : u32) -> u32);
    unsafe { MprConfigGetGuidName(hmprconfig, pszfriendlyname.param().abi(), pszbuffer as _, dwbuffersize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceCreate(hmprconfig: super::winnt::HANDLE, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceCreate(hmprconfig : super::winnt::HANDLE, dwlevel : u32, lpbbuffer : *const u8, phrouterinterface : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprConfigInterfaceCreate(hmprconfig, dwlevel, lpbbuffer, phrouterinterface as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceDelete(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceDelete(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE) -> u32);
    unsafe { MprConfigInterfaceDelete(hmprconfig, hrouterinterface) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceEnum(hmprconfig: super::winnt::HANDLE, dwlevel: u32, lplpbuffer: *mut super::minwindef::LPBYTE, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceEnum(hmprconfig : super::winnt::HANDLE, dwlevel : u32, lplpbuffer : *mut super::minwindef::LPBYTE, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    unsafe { MprConfigInterfaceEnum(hmprconfig, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetCustomInfoEx(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceGetCustomInfoEx(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, pcustominfo : *mut MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprConfigInterfaceGetCustomInfoEx(hmprconfig, hrouterinterface, pcustominfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceGetHandle<P1>(hmprconfig: super::winnt::HANDLE, lpwsinterfacename: P1, phrouterinterface: *mut super::winnt::HANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceGetHandle(hmprconfig : super::winnt::HANDLE, lpwsinterfacename : windows_core::PCWSTR, phrouterinterface : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprConfigInterfaceGetHandle(hmprconfig, lpwsinterfacename.param().abi(), phrouterinterface as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetInfo(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, dwlevel: u32, lplpbuffer: *mut super::minwindef::LPBYTE, lpdwbuffersize: *mut u32) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceGetInfo(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, dwlevel : u32, lplpbuffer : *mut super::minwindef::LPBYTE, lpdwbuffersize : *mut u32) -> u32);
    unsafe { MprConfigInterfaceGetInfo(hmprconfig, hrouterinterface, dwlevel, lplpbuffer as _, lpdwbuffersize as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceSetCustomInfoEx(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceSetCustomInfoEx(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, pcustominfo : *const MPR_IF_CUSTOMINFOEX2) -> u32);
    unsafe { MprConfigInterfaceSetCustomInfoEx(hmprconfig, hrouterinterface, pcustominfo) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceSetInfo(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceSetInfo(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprConfigInterfaceSetInfo(hmprconfig, hrouterinterface, dwlevel, lpbbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportAdd<P3>(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, dwtransportid: u32, lpwstransportname: P3, pinterfaceinfo: &[u8], phrouteriftransport: *mut super::winnt::HANDLE) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportAdd(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32, phrouteriftransport : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprConfigInterfaceTransportAdd(hmprconfig, hrouterinterface, dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pinterfaceinfo.as_ptr()), pinterfaceinfo.len().try_into().unwrap(), phrouteriftransport as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportEnum(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, dwlevel: u32, lplpbuffer: *mut super::minwindef::LPBYTE, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportEnum(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, dwlevel : u32, lplpbuffer : *mut super::minwindef::LPBYTE, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    unsafe { MprConfigInterfaceTransportEnum(hmprconfig, hrouterinterface, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetHandle(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, dwtransportid: u32, phrouteriftransport: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetHandle(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, dwtransportid : u32, phrouteriftransport : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprConfigInterfaceTransportGetHandle(hmprconfig, hrouterinterface, dwtransportid, phrouteriftransport as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetInfo(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, hrouteriftransport: super::winnt::HANDLE, ppinterfaceinfo: *mut super::minwindef::LPBYTE, lpdwinterfaceinfosize: Option<*mut u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetInfo(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, hrouteriftransport : super::winnt::HANDLE, ppinterfaceinfo : *mut super::minwindef::LPBYTE, lpdwinterfaceinfosize : *mut u32) -> u32);
    unsafe { MprConfigInterfaceTransportGetInfo(hmprconfig, hrouterinterface, hrouteriftransport, ppinterfaceinfo as _, lpdwinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportRemove(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, hrouteriftransport: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportRemove(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, hrouteriftransport : super::winnt::HANDLE) -> u32);
    unsafe { MprConfigInterfaceTransportRemove(hmprconfig, hrouterinterface, hrouteriftransport) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportSetInfo(hmprconfig: super::winnt::HANDLE, hrouterinterface: super::winnt::HANDLE, hrouteriftransport: super::winnt::HANDLE, pinterfaceinfo: Option<&[u8]>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportSetInfo(hmprconfig : super::winnt::HANDLE, hrouterinterface : super::winnt::HANDLE, hrouteriftransport : super::winnt::HANDLE, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    unsafe { MprConfigInterfaceTransportSetInfo(hmprconfig, hrouterinterface, hrouteriftransport, core::mem::transmute(pinterfaceinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pinterfaceinfo.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigServerBackup<P1>(hmprconfig: super::winnt::HANDLE, lpwspath: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerBackup(hmprconfig : super::winnt::HANDLE, lpwspath : windows_core::PCWSTR) -> u32);
    unsafe { MprConfigServerBackup(hmprconfig, lpwspath.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigServerConnect<P0>(lpwsservername: P0, phmprconfig: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerConnect(lpwsservername : windows_core::PCWSTR, phmprconfig : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprConfigServerConnect(lpwsservername.param().abi(), phmprconfig as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigServerDisconnect(hmprconfig: super::winnt::HANDLE) {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerDisconnect(hmprconfig : super::winnt::HANDLE));
    unsafe { MprConfigServerDisconnect(hmprconfig) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerGetInfo(hmprconfig: super::winnt::HANDLE, dwlevel: u32, lplpbbuffer: *mut super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerGetInfo(hmprconfig : super::winnt::HANDLE, dwlevel : u32, lplpbbuffer : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { MprConfigServerGetInfo(hmprconfig, dwlevel, lplpbbuffer as _) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerGetInfoEx(hmprconfig: super::winnt::HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerGetInfoEx(hmprconfig : super::winnt::HANDLE, pserverinfo : *mut MPR_SERVER_EX1) -> u32);
    unsafe { MprConfigServerGetInfoEx(hmprconfig, pserverinfo as _) }
}
#[inline]
pub unsafe fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerInstall(dwlevel : u32, pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { MprConfigServerInstall(dwlevel, pbuffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigServerRefresh(hmprconfig: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerRefresh(hmprconfig : super::winnt::HANDLE) -> u32);
    unsafe { MprConfigServerRefresh(hmprconfig) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigServerRestore<P1>(hmprconfig: super::winnt::HANDLE, lpwspath: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerRestore(hmprconfig : super::winnt::HANDLE, lpwspath : windows_core::PCWSTR) -> u32);
    unsafe { MprConfigServerRestore(hmprconfig, lpwspath.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigServerSetInfo(hmprserver: MPR_SERVER_HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerSetInfo(hmprserver : MPR_SERVER_HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    unsafe { MprConfigServerSetInfo(hmprserver, dwlevel, lpbbuffer) }
}
#[cfg(all(feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigServerSetInfoEx(hmprconfig: super::winnt::HANDLE, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigServerSetInfoEx(hmprconfig : super::winnt::HANDLE, psetserverconfig : *const MPR_SERVER_SET_CONFIG_EX1) -> u32);
    unsafe { MprConfigServerSetInfoEx(hmprconfig, psetserverconfig) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigTransportCreate<P2, P7>(hmprconfig: super::winnt::HANDLE, dwtransportid: u32, lpwstransportname: P2, pglobalinfo: &[u8], pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P7, phroutertransport: *mut super::winnt::HANDLE) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportCreate(hmprconfig : super::winnt::HANDLE, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR, phroutertransport : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprConfigTransportCreate(hmprconfig, dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pglobalinfo.as_ptr()), pglobalinfo.len().try_into().unwrap(), core::mem::transmute(pclientinterfaceinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi(), phroutertransport as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigTransportDelete(hmprconfig: super::winnt::HANDLE, hroutertransport: super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportDelete(hmprconfig : super::winnt::HANDLE, hroutertransport : super::winnt::HANDLE) -> u32);
    unsafe { MprConfigTransportDelete(hmprconfig, hroutertransport) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportEnum(hmprconfig: super::winnt::HANDLE, dwlevel: u32, lplpbuffer: *mut super::minwindef::LPBYTE, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportEnum(hmprconfig : super::winnt::HANDLE, dwlevel : u32, lplpbuffer : *mut super::minwindef::LPBYTE, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    unsafe { MprConfigTransportEnum(hmprconfig, dwlevel, lplpbuffer as _, dwprefmaxlen, lpdwentriesread as _, lpdwtotalentries as _, lpdwresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigTransportGetHandle(hmprconfig: super::winnt::HANDLE, dwtransportid: u32, phroutertransport: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportGetHandle(hmprconfig : super::winnt::HANDLE, dwtransportid : u32, phroutertransport : *mut super::winnt::HANDLE) -> u32);
    unsafe { MprConfigTransportGetHandle(hmprconfig, dwtransportid, phroutertransport as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn MprConfigTransportGetInfo(hmprconfig: super::winnt::HANDLE, hroutertransport: super::winnt::HANDLE, ppglobalinfo: Option<*mut super::minwindef::LPBYTE>, lpdwglobalinfosize: Option<*mut u32>, ppclientinterfaceinfo: Option<*mut super::minwindef::LPBYTE>, lpdwclientinterfaceinfosize: Option<*mut u32>, lplpwsdllpath: *mut windows_core::PWSTR) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportGetInfo(hmprconfig : super::winnt::HANDLE, hroutertransport : super::winnt::HANDLE, ppglobalinfo : *mut super::minwindef::LPBYTE, lpdwglobalinfosize : *mut u32, ppclientinterfaceinfo : *mut super::minwindef::LPBYTE, lpdwclientinterfaceinfosize : *mut u32, lplpwsdllpath : *mut windows_core::PWSTR) -> u32);
    unsafe { MprConfigTransportGetInfo(hmprconfig, hroutertransport, ppglobalinfo.unwrap_or(core::mem::zeroed()) as _, lpdwglobalinfosize.unwrap_or(core::mem::zeroed()) as _, ppclientinterfaceinfo.unwrap_or(core::mem::zeroed()) as _, lpdwclientinterfaceinfosize.unwrap_or(core::mem::zeroed()) as _, lplpwsdllpath as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MprConfigTransportSetInfo<P6>(hmprconfig: super::winnt::HANDLE, hroutertransport: super::winnt::HANDLE, pglobalinfo: Option<&[u8]>, pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P6) -> u32
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mprapi.dll" "system" fn MprConfigTransportSetInfo(hmprconfig : super::winnt::HANDLE, hroutertransport : super::winnt::HANDLE, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR) -> u32);
    unsafe { MprConfigTransportSetInfo(hmprconfig, hroutertransport, core::mem::transmute(pglobalinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pglobalinfo.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pclientinterfaceinfo.map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi()) }
}
#[inline]
pub unsafe fn MprInfoBlockAdd(lpheader: *const core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockAdd(lpheader : *const core::ffi::c_void, dwinfotype : u32, dwitemsize : u32, dwitemcount : u32, lpitemdata : *const u8, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockAdd(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MprInfoBlockFind(lpheader: *const core::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut super::minwindef::LPBYTE) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockFind(lpheader : *const core::ffi::c_void, dwinfotype : u32, lpdwitemsize : *mut u32, lpdwitemcount : *mut u32, lplpitemdata : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { MprInfoBlockFind(lpheader, dwinfotype, lpdwitemsize as _, lpdwitemcount as _, lplpitemdata as _) }
}
#[inline]
pub unsafe fn MprInfoBlockQuerySize(lpheader: *const core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockQuerySize(lpheader : *const core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockQuerySize(lpheader) }
}
#[inline]
pub unsafe fn MprInfoBlockRemove(lpheader: *const core::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockRemove(lpheader : *const core::ffi::c_void, dwinfotype : u32, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockRemove(lpheader, dwinfotype, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoBlockSet(lpheader: *const core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoBlockSet(lpheader : *const core::ffi::c_void, dwinfotype : u32, dwitemsize : u32, dwitemcount : u32, lpitemdata : *const u8, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoBlockSet(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoCreate(dwversion : u32, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoCreate(dwversion, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoDelete(lpheader: *const core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoDelete(lpheader : *const core::ffi::c_void) -> u32);
    unsafe { MprInfoDelete(lpheader) }
}
#[inline]
pub unsafe fn MprInfoDuplicate(lpheader: *const core::ffi::c_void, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoDuplicate(lpheader : *const core::ffi::c_void, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoDuplicate(lpheader, lplpnewheader as _) }
}
#[inline]
pub unsafe fn MprInfoRemoveAll(lpheader: *const core::ffi::c_void, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mprapi.dll" "system" fn MprInfoRemoveAll(lpheader : *const core::ffi::c_void, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    unsafe { MprInfoRemoveAll(lpheader, lplpnewheader as _) }
}
pub const ALLOW_NO_AUTH: u32 = 1;
pub const ATADDRESSLEN: u32 = 32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTH_VALIDATION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub hRasConnection: super::winnt::HANDLE,
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub AuthInfoSize: u32,
    pub AuthInfo: [u8; 1],
}
#[cfg(feature = "winnt")]
impl Default for AUTH_VALIDATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DO_NOT_ALLOW_NO_AUTH: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GRE_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IKEV2_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: IKEV2_TUNNEL_CONFIG_PARAMS4,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl Default for IKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl Default for IKEV2_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::wincrypt::CERT_NAME_BLOB,
    pub machineCertificateName: super::wincrypt::CERT_NAME_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: PROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "wincrypt")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS3 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::wincrypt::CERT_NAME_BLOB,
    pub machineCertificateName: super::wincrypt::CERT_NAME_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: PROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: PMPR_CERT_EKU,
    pub machineCertificateHash: super::wincrypt::CRYPT_HASH_BLOB,
}
#[cfg(feature = "wincrypt")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS4 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::wincrypt::CERT_NAME_BLOB,
    pub machineCertificateName: super::wincrypt::CERT_NAME_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: PROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: PMPR_CERT_EKU,
    pub machineCertificateHash: super::wincrypt::CRYPT_HASH_BLOB,
    pub dwMmSaLifeTime: u32,
}
#[cfg(feature = "wincrypt")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPADDRESSLEN: u32 = 15;
pub const IPXADDRESSLEN: u32 = 22;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct L2TP_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct L2TP_CONFIG_PARAMS1 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: L2TP_TUNNEL_CONFIG_PARAMS2,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS1 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: PROUTER_CUSTOM_L2TP_POLICY0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: PROUTER_CUSTOM_L2TP_POLICY0,
    pub dwMmSaLifeTime: u32,
}
pub const MAXIPADRESSLEN: u32 = 64;
pub const MAX_SSTP_HASH_SIZE: u32 = 32;
#[cfg(feature = "winnt")]
pub type MIB_SERVER_HANDLE = super::winnt::HANDLE;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MPRAPI_ADMIN_DLL_CALLBACKS {
    pub revision: u8,
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
pub const MPRAPI_ADMIN_DLL_VERSION_1: u32 = 1;
pub const MPRAPI_ADMIN_DLL_VERSION_2: u32 = 2;
pub const MPRAPI_IF_CUSTOM_CONFIG_FOR_IKEV2: u32 = 1;
pub const MPRAPI_IKEV2_AUTH_USING_CERT: u32 = 1;
pub const MPRAPI_IKEV2_AUTH_USING_EAP: u32 = 2;
pub const MPRAPI_IKEV2_PROJECTION_INFO_TYPE: u32 = 2;
pub const MPRAPI_IKEV2_SET_TUNNEL_CONFIG_PARAMS: u32 = 1;
pub const MPRAPI_L2TP_SET_TUNNEL_CONFIG_PARAMS: u32 = 1;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_1: u32 = 1;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_2: u32 = 2;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_3: u32 = 3;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_1: u32 = 1;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_2: u32 = 2;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_3: u32 = 3;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_4: u32 = 4;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_5: u32 = 5;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_1: u32 = 1;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_2: u32 = 2;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_3: u32 = 3;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_4: u32 = 4;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_5: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPRAPI_OBJECT_HEADER {
    pub revision: u8,
    pub r#type: u8,
    pub size: u16,
}
pub type MPRAPI_OBJECT_TYPE = i32;
pub const MPRAPI_OBJECT_TYPE_AUTH_VALIDATION_OBJECT: MPRAPI_OBJECT_TYPE = 4;
pub const MPRAPI_OBJECT_TYPE_IF_CUSTOM_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = 6;
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_OBJECT: MPRAPI_OBJECT_TYPE = 2;
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_SET_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = 3;
pub const MPRAPI_OBJECT_TYPE_RAS_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = 1;
pub const MPRAPI_OBJECT_TYPE_UPDATE_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = 5;
pub const MPRAPI_PPP_PROJECTION_INFO_TYPE: u32 = 1;
pub const MPRAPI_RAS_CONNECTION_OBJECT_REVISION_1: u32 = 1;
pub const MPRAPI_RAS_UPDATE_CONNECTION_OBJECT_REVISION_1: u32 = 1;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_GRE: u32 = 16;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_IKEV2: u32 = 8;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_L2TP: u32 = 2;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_PPTP: u32 = 1;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_SSTP: u32 = 4;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
    pub GREConfigParams: GRE_CONFIG_PARAMS0,
}
pub const MPRDM_DialAll: u32 = 1;
pub const MPRDM_DialAsNeeded: u32 = 2;
pub const MPRDM_DialFirst: u32 = 0;
pub const MPRET_Phone: u32 = 1;
pub const MPRET_Vpn: u32 = 2;
pub const MPRIDS_Disabled: i32 = -1;
pub const MPRIDS_UseGlobalValue: u32 = 0;
pub const MPRIO_DisableLcpExtensions: u32 = 32;
pub const MPRIO_IpHeaderCompression: u32 = 8;
pub const MPRIO_IpSecPreSharedKey: u32 = 2147483648;
pub const MPRIO_NetworkLogon: u32 = 8192;
pub const MPRIO_PromoteAlternates: u32 = 32768;
pub const MPRIO_RemoteDefaultGateway: u32 = 16;
pub const MPRIO_RequireCHAP: u32 = 134217728;
pub const MPRIO_RequireDataEncryption: u32 = 4096;
pub const MPRIO_RequireEAP: u32 = 131072;
pub const MPRIO_RequireEncryptedPw: u32 = 1024;
pub const MPRIO_RequireMachineCertificates: u32 = 16777216;
pub const MPRIO_RequireMsCHAP: u32 = 268435456;
pub const MPRIO_RequireMsCHAP2: u32 = 536870912;
pub const MPRIO_RequireMsEncryptedPw: u32 = 2048;
pub const MPRIO_RequirePAP: u32 = 262144;
pub const MPRIO_RequireSPAP: u32 = 524288;
pub const MPRIO_SecureLocalFiles: u32 = 65536;
pub const MPRIO_SharedPhoneNumbers: u32 = 8388608;
pub const MPRIO_SpecificIpAddr: u32 = 2;
pub const MPRIO_SpecificNameServers: u32 = 4;
pub const MPRIO_SwCompression: u32 = 512;
pub const MPRIO_UsePreSharedKeyForIkev2Initiator: u32 = 33554432;
pub const MPRIO_UsePreSharedKeyForIkev2Responder: u32 = 67108864;
pub const MPRNP_Ip: u32 = 4;
pub const MPRNP_Ipv6: u32 = 8;
pub const MPRNP_Ipx: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_CERT_EKU {
    pub dwSize: u32,
    pub IsEKUOID: windows_core::BOOL,
    pub pwszEKU: *mut u16,
}
impl Default for MPR_CERT_EKU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_CREDENTIALSEX_0 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: super::minwindef::LPBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_CREDENTIALSEX_1 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: super::minwindef::LPBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_DEVICE_0 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl Default for MPR_DEVICE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_DEVICE_1 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: super::winnt::PWCHAR,
}
#[cfg(feature = "winnt")]
impl Default for MPR_DEVICE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_ENABLE_RAS_ON_DEVICE: u32 = 1;
pub const MPR_ENABLE_ROUTING_ON_DEVICE: u32 = 2;
pub const MPR_ET_None: u32 = 0;
pub const MPR_ET_Optional: u32 = 3;
pub const MPR_ET_Require: u32 = 1;
pub const MPR_ET_RequireMax: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_FILTER_0 {
    pub fEnable: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_IFTRANSPORT_0 {
    pub dwTransportId: u32,
    pub hIfTransport: super::winnt::HANDLE,
    pub wszIfTransportName: [u16; 41],
}
#[cfg(feature = "winnt")]
impl Default for MPR_IFTRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG0,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG1,
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX2 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG2,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_INTERFACE_0 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::winnt::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
}
#[cfg(feature = "winnt")]
impl Default for MPR_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_INTERFACE_1 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::winnt::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub lpwsDialoutHoursRestriction: windows_core::PWSTR,
}
#[cfg(feature = "winnt")]
impl Default for MPR_INTERFACE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_INTERFACE_2 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::winnt::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: super::winnt::PWCHAR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: u32,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: super::minwindef::LPBYTE,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for MPR_INTERFACE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MPR_INTERFACE_3 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::winnt::HANDLE,
    pub fEnabled: windows_core::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: super::winnt::PWCHAR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: u32,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: super::minwindef::LPBYTE,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: u32,
    pub AddressCount: u32,
    pub ipv6addrDns: super::in6addr::IN6_ADDR,
    pub ipv6addrDnsAlt: super::in6addr::IN6_ADDR,
    pub ipv6addr: *mut super::in6addr::IN6_ADDR,
}
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "winnt"))]
impl Default for MPR_INTERFACE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_INTERFACE_ADMIN_DISABLED: u32 = 2;
pub const MPR_INTERFACE_CONNECTION_FAILURE: u32 = 4;
pub const MPR_INTERFACE_DIALOUT_HOURS_RESTRICTION: u32 = 16;
pub const MPR_INTERFACE_NO_DEVICE: u32 = 64;
pub const MPR_INTERFACE_NO_MEDIA_SENSE: u32 = 32;
pub const MPR_INTERFACE_OUT_OF_RESOURCES: u32 = 1;
pub const MPR_INTERFACE_SERVICE_PAUSED: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_IPINIP_INTERFACE_0 {
    pub wszFriendlyName: [u16; 257],
    pub Guid: windows_core::GUID,
}
impl Default for MPR_IPINIP_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MPR_MaxAreaCode: u32 = 10;
pub const MPR_MaxCallbackNumber: u32 = 128;
pub const MPR_MaxDeviceName: u32 = 128;
pub const MPR_MaxDeviceType: u32 = 16;
pub const MPR_MaxEntryName: u32 = 256;
pub const MPR_MaxFacilities: u32 = 200;
pub const MPR_MaxIpAddress: u32 = 15;
pub const MPR_MaxIpxAddress: u32 = 21;
pub const MPR_MaxPadType: u32 = 32;
pub const MPR_MaxPhoneNumber: u32 = 128;
pub const MPR_MaxUserData: u32 = 200;
pub const MPR_MaxX25Address: u32 = 200;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_0 {
    pub fLanOnlyMode: windows_core::BOOL,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_1 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_2 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
    pub dwNumSstpPorts: u32,
    pub dwSstpPortFlags: u32,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[cfg(feature = "winnt")]
pub type MPR_SERVER_HANDLE = super::winnt::HANDLE;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPR_TRANSPORT_0 {
    pub dwTransportId: u32,
    pub hTransport: super::winnt::HANDLE,
    pub wszTransportName: [u16; 41],
}
#[cfg(feature = "winnt")]
impl Default for MPR_TRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct MPR_VPN_TRAFFIC_SELECTOR {
    pub r#type: MPR_VPN_TS_TYPE,
    pub protocolId: u8,
    pub portStart: u16,
    pub portEnd: u16,
    pub tsPayloadId: u16,
    pub addrStart: VPN_TS_IP_ADDRESS,
    pub addrEnd: VPN_TS_IP_ADDRESS,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for MPR_VPN_TRAFFIC_SELECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MPR_VPN_TRAFFIC_SELECTORS {
    pub numTsi: u32,
    pub numTsr: u32,
    pub tsI: PMPR_VPN_TRAFFIC_SELECTOR,
    pub tsR: PMPR_VPN_TRAFFIC_SELECTOR,
}
pub const MPR_VPN_TS_IPv4_ADDR_RANGE: MPR_VPN_TS_TYPE = 7;
pub const MPR_VPN_TS_IPv6_ADDR_RANGE: MPR_VPN_TS_TYPE = 8;
pub type MPR_VPN_TS_TYPE = i32;
pub const MPR_VS_Default: u32 = 0;
pub const MPR_VS_Ikev2First: u32 = 8;
pub const MPR_VS_Ikev2Only: u32 = 7;
pub const MPR_VS_L2tpFirst: u32 = 4;
pub const MPR_VS_L2tpOnly: u32 = 3;
pub const MPR_VS_PptpFirst: u32 = 2;
pub const MPR_VS_PptpOnly: u32 = 1;
pub type PGRE_CONFIG_PARAMS0 = *mut GRE_CONFIG_PARAMS0;
pub const PID_ATALK: u32 = 41;
pub const PID_IP: u32 = 33;
pub const PID_IPV6: u32 = 87;
pub const PID_IPX: u32 = 43;
pub const PID_NBF: u32 = 63;
#[cfg(feature = "wincrypt")]
pub type PIKEV2_CONFIG_PARAMS = *mut IKEV2_CONFIG_PARAMS;
pub type PIKEV2_PROJECTION_INFO = *mut IKEV2_PROJECTION_INFO;
pub type PIKEV2_PROJECTION_INFO2 = *mut IKEV2_PROJECTION_INFO2;
#[cfg(feature = "wincrypt")]
pub type PIKEV2_TUNNEL_CONFIG_PARAMS2 = *mut IKEV2_TUNNEL_CONFIG_PARAMS2;
#[cfg(feature = "wincrypt")]
pub type PIKEV2_TUNNEL_CONFIG_PARAMS3 = *mut IKEV2_TUNNEL_CONFIG_PARAMS3;
#[cfg(feature = "wincrypt")]
pub type PIKEV2_TUNNEL_CONFIG_PARAMS4 = *mut IKEV2_TUNNEL_CONFIG_PARAMS4;
pub type PL2TP_CONFIG_PARAMS0 = *mut L2TP_CONFIG_PARAMS0;
pub type PL2TP_CONFIG_PARAMS1 = *mut L2TP_CONFIG_PARAMS1;
pub type PL2TP_TUNNEL_CONFIG_PARAMS1 = *mut L2TP_TUNNEL_CONFIG_PARAMS1;
pub type PL2TP_TUNNEL_CONFIG_PARAMS2 = *mut L2TP_TUNNEL_CONFIG_PARAMS2;
#[cfg(feature = "winnt")]
pub type PMPRADMINACCEPTNEWCONNECTION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PMPRADMINACCEPTNEWCONNECTION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPRADMINACCEPTNEWCONNECTION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPRADMINACCEPTNEWCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PMPRADMINACCEPTNEWLINK = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPRADMINACCEPTREAUTHENTICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPRADMINACCEPTREAUTHENTICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1)>;
#[cfg(feature = "winnt")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2)>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: RAS_CONNECTION_3)>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX)>;
pub type PMPRADMINGETIPADDRESSFORUSER = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: *mut u32, param3: *mut windows_core::BOOL) -> u32>;
#[cfg(feature = "in6addr")]
pub type PMPRADMINGETIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: *mut super::in6addr::IN6_ADDR, param3: *mut windows_core::BOOL) -> u32>;
#[cfg(feature = "winnt")]
pub type PMPRADMINLINKHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1)>;
#[cfg(feature = "winnt")]
pub type PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut AUTH_VALIDATION_EX) -> u32>;
pub type PMPRADMINRELEASEIPADRESS = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: *mut u32)>;
#[cfg(feature = "in6addr")]
pub type PMPRADMINRELEASEIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: *mut u16, param1: *mut u16, param2: *mut super::in6addr::IN6_ADDR)>;
pub type PMPRADMINTERMINATEDLL = Option<unsafe extern "system" fn() -> u32>;
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "winnt"))]
pub type PMPRAPI_ADMIN_DLL_CALLBACKS = *mut MPRAPI_ADMIN_DLL_CALLBACKS;
pub type PMPRAPI_OBJECT_HEADER = *mut MPRAPI_OBJECT_HEADER;
pub type PMPRAPI_OBJECT_TYPE = *mut MPRAPI_OBJECT_TYPE;
#[cfg(feature = "wincrypt")]
pub type PMPRAPI_TUNNEL_CONFIG_PARAMS0 = *mut MPRAPI_TUNNEL_CONFIG_PARAMS0;
#[cfg(feature = "wincrypt")]
pub type PMPRAPI_TUNNEL_CONFIG_PARAMS1 = *mut MPRAPI_TUNNEL_CONFIG_PARAMS1;
pub type PMPR_CERT_EKU = *mut MPR_CERT_EKU;
#[cfg(feature = "minwindef")]
pub type PMPR_CREDENTIALSEX_0 = *mut MPR_CREDENTIALSEX_0;
#[cfg(feature = "minwindef")]
pub type PMPR_CREDENTIALSEX_1 = *mut MPR_CREDENTIALSEX_1;
pub type PMPR_DEVICE_0 = *mut MPR_DEVICE_0;
#[cfg(feature = "winnt")]
pub type PMPR_DEVICE_1 = *mut MPR_DEVICE_1;
pub type PMPR_FILTER_0 = *mut MPR_FILTER_0;
#[cfg(feature = "winnt")]
pub type PMPR_IFTRANSPORT_0 = *mut MPR_IFTRANSPORT_0;
#[cfg(feature = "wincrypt")]
pub type PMPR_IF_CUSTOMINFOEX0 = *mut MPR_IF_CUSTOMINFOEX0;
#[cfg(feature = "wincrypt")]
pub type PMPR_IF_CUSTOMINFOEX1 = *mut MPR_IF_CUSTOMINFOEX1;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt"))]
pub type PMPR_IF_CUSTOMINFOEX2 = *mut MPR_IF_CUSTOMINFOEX2;
#[cfg(feature = "winnt")]
pub type PMPR_INTERFACE_0 = *mut MPR_INTERFACE_0;
#[cfg(feature = "winnt")]
pub type PMPR_INTERFACE_1 = *mut MPR_INTERFACE_1;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PMPR_INTERFACE_2 = *mut MPR_INTERFACE_2;
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "winnt"))]
pub type PMPR_INTERFACE_3 = *mut MPR_INTERFACE_3;
pub type PMPR_IPINIP_INTERFACE_0 = *mut MPR_IPINIP_INTERFACE_0;
pub type PMPR_SERVER_0 = *mut MPR_SERVER_0;
pub type PMPR_SERVER_1 = *mut MPR_SERVER_1;
pub type PMPR_SERVER_2 = *mut MPR_SERVER_2;
#[cfg(feature = "wincrypt")]
pub type PMPR_SERVER_EX0 = *mut MPR_SERVER_EX0;
#[cfg(feature = "wincrypt")]
pub type PMPR_SERVER_EX1 = *mut MPR_SERVER_EX1;
#[cfg(feature = "wincrypt")]
pub type PMPR_SERVER_SET_CONFIG_EX0 = *mut MPR_SERVER_SET_CONFIG_EX0;
#[cfg(feature = "wincrypt")]
pub type PMPR_SERVER_SET_CONFIG_EX1 = *mut MPR_SERVER_SET_CONFIG_EX1;
#[cfg(feature = "winnt")]
pub type PMPR_TRANSPORT_0 = *mut MPR_TRANSPORT_0;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type PMPR_VPN_TRAFFIC_SELECTOR = *mut MPR_VPN_TRAFFIC_SELECTOR;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type PMPR_VPN_TRAFFIC_SELECTORS = *mut MPR_VPN_TRAFFIC_SELECTORS;
pub type PPPP_PROJECTION_INFO = *mut PPP_PROJECTION_INFO;
pub type PPPP_PROJECTION_INFO2 = *mut PPP_PROJECTION_INFO2;
pub type PPPTP_CONFIG_PARAMS = *mut PPTP_CONFIG_PARAMS;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_ATCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 33],
}
impl Default for PPP_ATCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_CCP_COMPRESSION: u32 = 1;
pub const PPP_CCP_ENCRYPTION128BIT: u32 = 64;
pub const PPP_CCP_ENCRYPTION40BIT: u32 = 32;
pub const PPP_CCP_ENCRYPTION40BITOLD: u32 = 16;
pub const PPP_CCP_ENCRYPTION56BIT: u32 = 128;
pub const PPP_CCP_HISTORYLESS: u32 = 16777216;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_CCP_INFO {
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwRemoteOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_INFO {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_INFO_2 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_INFO_3 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipv6: PPP_IPV6_CP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
}
impl Default for PPP_IPCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPCP_INFO2 {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
}
impl Default for PPP_IPCP_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_IPCP_VJ: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPV6_CP_INFO {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
}
impl Default for PPP_IPV6_CP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_IPXCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 23],
}
impl Default for PPP_IPXCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPP_LCP_3_DES: u32 = 32;
pub const PPP_LCP_ACFC: u32 = 4;
pub const PPP_LCP_AES_128: u32 = 64;
pub const PPP_LCP_AES_192: u32 = 256;
pub const PPP_LCP_AES_256: u32 = 128;
pub const PPP_LCP_CHAP: u32 = 49699;
pub const PPP_LCP_CHAP_MD5: u32 = 5;
pub const PPP_LCP_CHAP_MS: u32 = 128;
pub const PPP_LCP_CHAP_MSV2: u32 = 129;
pub const PPP_LCP_DES_56: u32 = 16;
pub const PPP_LCP_EAP: u32 = 49703;
pub const PPP_LCP_GCM_AES_128: u32 = 512;
pub const PPP_LCP_GCM_AES_192: u32 = 1024;
pub const PPP_LCP_GCM_AES_256: u32 = 2048;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_LCP_INFO {
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwRemoteAuthenticationProtocol: u32,
    pub dwRemoteAuthenticationData: u32,
    pub dwTerminateReason: u32,
    pub dwRemoteTerminateReason: u32,
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
}
pub const PPP_LCP_MULTILINK_FRAMING: u32 = 1;
pub const PPP_LCP_PAP: u32 = 49187;
pub const PPP_LCP_PFC: u32 = 2;
pub const PPP_LCP_SPAP: u32 = 49191;
pub const PPP_LCP_SSHF: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_NBFCP_INFO {
    pub dwError: u32,
    pub wszWksta: [u16; 17],
}
impl Default for PPP_NBFCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwRemoteAuthenticationProtocol: u32,
    pub dwRemoteAuthenticationData: u32,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl Default for PPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwRemoteAuthenticationProtocol: u32,
    pub dwRemoteAuthenticationData: u32,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl Default for PPP_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PPROJECTION_INFO = *mut PROJECTION_INFO;
pub type PPROJECTION_INFO2 = *mut PROJECTION_INFO2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
#[cfg(feature = "winnt")]
pub type PRAS_CONNECTION_0 = *mut RAS_CONNECTION_0;
#[cfg(feature = "winnt")]
pub type PRAS_CONNECTION_1 = *mut RAS_CONNECTION_1;
#[cfg(feature = "winnt")]
pub type PRAS_CONNECTION_2 = *mut RAS_CONNECTION_2;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PRAS_CONNECTION_3 = *mut RAS_CONNECTION_3;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PRAS_CONNECTION_4 = *mut RAS_CONNECTION_4;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PRAS_CONNECTION_EX = *mut RAS_CONNECTION_EX;
#[cfg(feature = "winnt")]
pub type PRAS_PORT_0 = *mut RAS_PORT_0;
#[cfg(feature = "winnt")]
pub type PRAS_PORT_1 = *mut RAS_PORT_1;
#[cfg(feature = "winnt")]
pub type PRAS_PORT_2 = *mut RAS_PORT_2;
pub type PRAS_UPDATE_CONNECTION = *mut RAS_UPDATE_CONNECTION;
pub type PRAS_USER_0 = *mut RAS_USER_0;
pub type PRAS_USER_1 = *mut RAS_USER_1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO_0,
}
impl Default for PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO,
}
impl Default for PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO2 {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO2_0,
}
impl Default for PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO2_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO2,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO2,
}
impl Default for PROJECTION_INFO2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROUTER_CUSTOM_IKEv2_POLICY0 = *mut ROUTER_CUSTOM_IKEv2_POLICY0;
pub type PROUTER_CUSTOM_L2TP_POLICY0 = *mut ROUTER_CUSTOM_IKEv2_POLICY0;
#[cfg(feature = "wincrypt")]
pub type PROUTER_IKEv2_IF_CUSTOM_CONFIG0 = *mut ROUTER_IKEv2_IF_CUSTOM_CONFIG0;
#[cfg(feature = "wincrypt")]
pub type PROUTER_IKEv2_IF_CUSTOM_CONFIG1 = *mut ROUTER_IKEv2_IF_CUSTOM_CONFIG1;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt"))]
pub type PROUTER_IKEv2_IF_CUSTOM_CONFIG2 = *mut ROUTER_IKEv2_IF_CUSTOM_CONFIG2;
#[cfg(feature = "wincrypt")]
pub type PSSTP_CERT_INFO = *mut SSTP_CERT_INFO;
#[cfg(feature = "wincrypt")]
pub type PSSTP_CONFIG_PARAMS = *mut SSTP_CONFIG_PARAMS;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type PVPN_TS_IP_ADDRESS = *mut VPN_TS_IP_ADDRESS;
pub const RASPRIV2_DialinPolicy: u32 = 1;
pub const RASPRIV_AdminSetCallback: u32 = 2;
pub const RASPRIV_CallbackType: u32 = 7;
pub const RASPRIV_CallerSetCallback: u32 = 4;
pub const RASPRIV_DialinPrivilege: u32 = 8;
pub const RASPRIV_NoCallback: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_CONNECTION_0 {
    pub hConnection: super::winnt::HANDLE,
    pub hInterface: super::winnt::HANDLE,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: u32,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
}
#[cfg(feature = "winnt")]
impl Default for RAS_CONNECTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RAS_CONNECTION_1 {
    pub hConnection: super::winnt::HANDLE,
    pub hInterface: super::winnt::HANDLE,
    pub PppInfo: PPP_INFO,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_CONNECTION_2 {
    pub hConnection: super::winnt::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo2: PPP_INFO_2,
}
#[cfg(feature = "winnt")]
impl Default for RAS_CONNECTION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_CONNECTION_3 {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub hConnection: super::winnt::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo3: PPP_INFO_3,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub timer: super::minwindef::FILETIME,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for RAS_CONNECTION_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_4 {
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: u32,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::minwindef::FILETIME,
    pub connectionStartTime: super::minwindef::FILETIME,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO2,
    pub hConnection: super::winnt::HANDLE,
    pub hInterface: super::winnt::HANDLE,
    pub dwDeviceType: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for RAS_CONNECTION_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: u32,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::minwindef::FILETIME,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO,
    pub hConnection: super::winnt::HANDLE,
    pub hInterface: super::winnt::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for RAS_CONNECTION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RAS_FLAGS_DORMANT: u32 = 32;
pub const RAS_FLAGS_MESSENGER_PRESENT: u32 = 2;
pub const RAS_FLAGS_PPP_CONNECTION: u32 = 1;
pub const RAS_FLAGS_QUARANTINE_PRESENT: u32 = 8;
pub type RAS_HARDWARE_CONDITION = i32;
pub const RAS_HARDWARE_FAILURE: RAS_HARDWARE_CONDITION = 1;
pub const RAS_HARDWARE_OPERATIONAL: RAS_HARDWARE_CONDITION = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_PORT_0 {
    pub hPort: super::winnt::HANDLE,
    pub hConnection: super::winnt::HANDLE,
    pub dwPortCondition: RAS_PORT_CONDITION,
    pub dwTotalNumberOfCalls: u32,
    pub dwConnectDuration: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
}
#[cfg(feature = "winnt")]
impl Default for RAS_PORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RAS_PORT_1 {
    pub hPort: super::winnt::HANDLE,
    pub hConnection: super::winnt::HANDLE,
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_PORT_2 {
    pub hPort: super::winnt::HANDLE,
    pub hConnection: super::winnt::HANDLE,
    pub dwConn_State: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwCrcErr: u32,
    pub dwSerialOverRunErrs: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwTotalErrors: u32,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub ullFramesXmited: u64,
    pub ullFramesRcved: u64,
    pub ullBytesTxUncompressed: u64,
    pub ullBytesTxCompressed: u64,
    pub ullBytesRcvUncompressed: u64,
    pub ullBytesRcvCompressed: u64,
}
#[cfg(feature = "winnt")]
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
#[cfg(feature = "winnt")]
pub type RAS_SERVER_HANDLE = super::winnt::HANDLE;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_UPDATE_CONNECTION {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwIfIndex: u32,
    pub wszLocalEndpointAddress: [u16; 65],
    pub wszRemoteEndpointAddress: [u16; 65],
}
impl Default for RAS_UPDATE_CONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_USER_0 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
}
impl Default for RAS_USER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_USER_1 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
    pub bfPrivilege2: u8,
}
impl Default for RAS_USER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ROUTER_CONNECTION_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ROUTER_CUSTOM_IKEv2_POLICY0 {
    pub dwIntegrityMethod: u32,
    pub dwEncryptionMethod: u32,
    pub dwCipherTransformConstant: u32,
    pub dwAuthTransformConstant: u32,
    pub dwPfsGroup: u32,
    pub dwDhGroup: u32,
}
pub type ROUTER_CUSTOM_L2TP_POLICY0 = ROUTER_CUSTOM_IKEv2_POLICY0;
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
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::wincrypt::CERT_NAME_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "wincrypt")]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::wincrypt::CERT_NAME_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::wincrypt::CRYPT_HASH_BLOB,
}
#[cfg(feature = "wincrypt")]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::wincrypt::CERT_NAME_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::wincrypt::CRYPT_HASH_BLOB,
    pub dwMmSaLifeTime: u32,
    pub vpnTrafficSelectors: MPR_VPN_TRAFFIC_SELECTORS,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "wincrypt"))]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ROUTER_INTERFACE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSTP_CERT_INFO {
    pub isDefault: windows_core::BOOL,
    pub certBlob: super::wincrypt::CRYPT_HASH_BLOB,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SSTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub isUseHttps: windows_core::BOOL,
    pub certAlgorithm: u32,
    pub sstpCertDetails: SSTP_CERT_INFO,
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct VPN_TS_IP_ADDRESS {
    pub Type: u16,
    pub Anonymous: VPN_TS_IP_ADDRESS_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for VPN_TS_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub union VPN_TS_IP_ADDRESS_0 {
    pub v4: super::inaddr::IN_ADDR,
    pub v6: super::in6addr::IN6_ADDR,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for VPN_TS_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
