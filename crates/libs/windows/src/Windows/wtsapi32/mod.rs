#[inline]
pub unsafe fn WTSActiveSessionExists(pbactivesessionexists: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSActiveSessionExists(pbactivesessionexists : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSActiveSessionExists(pbactivesessionexists as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSCloseServer(hserver: super::winnt::HANDLE) {
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloseServer(hserver : super::winnt::HANDLE));
    unsafe { WTSCloseServer(hserver) }
}
#[inline]
pub unsafe fn WTSCloudAuthClose(cloudauthhandle: WTS_CLOUD_AUTH_HANDLE) {
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloudAuthClose(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE));
    unsafe { WTSCloudAuthClose(cloudauthhandle) }
}
#[inline]
pub unsafe fn WTSCloudAuthConvertAssertionToSerializedUserCredential<P3>(cloudauthhandle: WTS_CLOUD_AUTH_HANDLE, assertion: &[u8], resourceid: P3, usercredential: *mut *mut WTS_SERIALIZED_USER_CREDENTIAL) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloudAuthConvertAssertionToSerializedUserCredential(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE, assertion : windows_core::PCSTR, assertionlength : u32, resourceid : windows_core::PCWSTR, usercredential : *mut *mut WTS_SERIALIZED_USER_CREDENTIAL) -> windows_core::BOOL);
    unsafe { WTSCloudAuthConvertAssertionToSerializedUserCredential(cloudauthhandle, core::mem::transmute(assertion.as_ptr()), assertion.len().try_into().unwrap(), resourceid.param().abi(), usercredential as _) }
}
#[inline]
pub unsafe fn WTSCloudAuthDuplicateSerializedUserCredential(usercredential: *const WTS_SERIALIZED_USER_CREDENTIAL, duplicatedusercredential: *mut *mut WTS_SERIALIZED_USER_CREDENTIAL) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloudAuthDuplicateSerializedUserCredential(usercredential : *const WTS_SERIALIZED_USER_CREDENTIAL, duplicatedusercredential : *mut *mut WTS_SERIALIZED_USER_CREDENTIAL) -> windows_core::BOOL);
    unsafe { WTSCloudAuthDuplicateSerializedUserCredential(usercredential, duplicatedusercredential as _) }
}
#[inline]
pub unsafe fn WTSCloudAuthGetServerNonce(cloudauthhandle: WTS_CLOUD_AUTH_HANDLE, servernonce: *mut windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloudAuthGetServerNonce(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE, servernonce : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { WTSCloudAuthGetServerNonce(cloudauthhandle, servernonce as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSCloudAuthNetworkLogonWithSerializedCredential(cloudauthhandle: WTS_CLOUD_AUTH_HANDLE, usercredential: *const WTS_SERIALIZED_USER_CREDENTIAL, token: *mut super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloudAuthNetworkLogonWithSerializedCredential(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE, usercredential : *const WTS_SERIALIZED_USER_CREDENTIAL, token : *mut super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WTSCloudAuthNetworkLogonWithSerializedCredential(cloudauthhandle, usercredential, token as _) }
}
#[inline]
pub unsafe fn WTSCloudAuthOpen(activityid: *const windows_core::GUID) -> WTS_CLOUD_AUTH_HANDLE {
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloudAuthOpen(activityid : *const windows_core::GUID) -> WTS_CLOUD_AUTH_HANDLE);
    unsafe { WTSCloudAuthOpen(activityid) }
}
#[inline]
pub unsafe fn WTSConnectSessionA<P2>(logonid: u32, targetlogonid: u32, ppassword: P2, bwait: bool) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSConnectSessionA(logonid : u32, targetlogonid : u32, ppassword : windows_core::PCSTR, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSConnectSessionA(logonid, targetlogonid, ppassword.param().abi(), bwait.into()) }
}
#[inline]
pub unsafe fn WTSConnectSessionW<P2>(logonid: u32, targetlogonid: u32, ppassword: P2, bwait: bool) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSConnectSessionW(logonid : u32, targetlogonid : u32, ppassword : windows_core::PCWSTR, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSConnectSessionW(logonid, targetlogonid, ppassword.param().abi(), bwait.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSCreateListenerA<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSCreateListenerA(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, pbuffer : *const WTSLISTENERCONFIGA, flag : u32) -> windows_core::BOOL);
    unsafe { WTSCreateListenerA(hserver, preserved, reserved, plistenername.param().abi(), pbuffer, flag) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSCreateListenerW<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSCreateListenerW(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, pbuffer : *const WTSLISTENERCONFIGW, flag : u32) -> windows_core::BOOL);
    unsafe { WTSCreateListenerW(hserver, preserved, reserved, plistenername.param().abi(), pbuffer, flag) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSDisconnectSession(hserver: super::winnt::HANDLE, sessionid: u32, bwait: bool) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSDisconnectSession(hserver : super::winnt::HANDLE, sessionid : u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSDisconnectSession(hserver, sessionid, bwait.into()) }
}
#[inline]
pub unsafe fn WTSEnableChildSessions(benable: bool) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "C" fn WTSEnableChildSessions(benable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSEnableChildSessions(benable.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateListenersA(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plisteners: Option<*mut WTSLISTENERNAMEA>, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersA(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut WTSLISTENERNAMEA, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateListenersA(hserver, preserved, reserved, plisteners.unwrap_or(core::mem::zeroed()) as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateListenersW(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plisteners: Option<*mut WTSLISTENERNAMEW>, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersW(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut WTSLISTENERNAMEW, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateListenersW(hserver, preserved, reserved, plisteners.unwrap_or(core::mem::zeroed()) as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateProcessesA(hserver: super::winnt::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut PWTS_PROCESS_INFOA, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesA(hserver : super::winnt::HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut PWTS_PROCESS_INFOA, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesA(hserver, reserved, version, ppprocessinfo as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExA(hserver: super::winnt::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut windows_core::PSTR, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExA(hserver : super::winnt::HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_core::PSTR, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesExA(hserver, plevel as _, sessionid, ppprocessinfo as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateProcessesExW(hserver: super::winnt::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut windows_core::PWSTR, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExW(hserver : super::winnt::HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_core::PWSTR, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesExW(hserver, plevel as _, sessionid, ppprocessinfo as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateProcessesW(hserver: super::winnt::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut PWTS_PROCESS_INFOW, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesW(hserver : super::winnt::HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut PWTS_PROCESS_INFOW, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesW(hserver, reserved, version, ppprocessinfo as _, pcount as _) }
}
#[inline]
pub unsafe fn WTSEnumerateServersA<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut PWTS_SERVER_INFOA, pcount: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateServersA(pdomainname : windows_core::PCSTR, reserved : u32, version : u32, ppserverinfo : *mut PWTS_SERVER_INFOA, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateServersA(pdomainname.param().abi(), reserved, version, ppserverinfo as _, pcount as _) }
}
#[inline]
pub unsafe fn WTSEnumerateServersW<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut PWTS_SERVER_INFOW, pcount: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateServersW(pdomainname : windows_core::PCWSTR, reserved : u32, version : u32, ppserverinfo : *mut PWTS_SERVER_INFOW, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateServersW(pdomainname.param().abi(), reserved, version, ppserverinfo as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateSessionsA(hserver: super::winnt::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut PWTS_SESSION_INFOA, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsA(hserver : super::winnt::HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut PWTS_SESSION_INFOA, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsA(hserver, reserved, version, ppsessioninfo as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExA(hserver: super::winnt::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut PWTS_SESSION_INFO_1A, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExA(hserver : super::winnt::HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut PWTS_SESSION_INFO_1A, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsExA(hserver, plevel as _, filter, ppsessioninfo as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateSessionsExW(hserver: super::winnt::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut PWTS_SESSION_INFO_1W, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExW(hserver : super::winnt::HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut PWTS_SESSION_INFO_1W, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsExW(hserver, plevel as _, filter, ppsessioninfo as _, pcount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSEnumerateSessionsW(hserver: super::winnt::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut PWTS_SESSION_INFOW, pcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsW(hserver : super::winnt::HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut PWTS_SESSION_INFOW, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsW(hserver, reserved, version, ppsessioninfo as _, pcount as _) }
}
#[inline]
pub unsafe fn WTSFreeMemory(pmemory: *mut core::ffi::c_void) {
    windows_core::link!("wtsapi32.dll" "system" fn WTSFreeMemory(pmemory : *mut core::ffi::c_void));
    unsafe { WTSFreeMemory(pmemory as _) }
}
#[inline]
pub unsafe fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const core::ffi::c_void, numberofentries: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExA(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> windows_core::BOOL);
    unsafe { WTSFreeMemoryExA(wtstypeclass, pmemory, numberofentries) }
}
#[inline]
pub unsafe fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const core::ffi::c_void, numberofentries: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExW(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> windows_core::BOOL);
    unsafe { WTSFreeMemoryExW(wtstypeclass, pmemory, numberofentries) }
}
#[inline]
pub unsafe fn WTSGetChildSessionId(psessionid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "C" fn WTSGetChildSessionId(psessionid : *mut u32) -> windows_core::BOOL);
    unsafe { WTSGetChildSessionId(psessionid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSGetListenerSecurityA<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityA(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { WTSGetListenerSecurityA(hserver, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSGetListenerSecurityW<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityW(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { WTSGetListenerSecurityW(hserver, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[inline]
pub unsafe fn WTSIsChildSessionsEnabled(pbenabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "C" fn WTSIsChildSessionsEnabled(pbenabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSIsChildSessionsEnabled(pbenabled as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSLogoffSession(hserver: super::winnt::HANDLE, sessionid: u32, bwait: bool) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSLogoffSession(hserver : super::winnt::HANDLE, sessionid : u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSLogoffSession(hserver, sessionid, bwait.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSOpenServerA<P0>(pservername: P0) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerA(pservername : windows_core::PCSTR) -> super::winnt::HANDLE);
    unsafe { WTSOpenServerA(pservername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSOpenServerExA<P0>(pservername: P0) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerExA(pservername : windows_core::PCSTR) -> super::winnt::HANDLE);
    unsafe { WTSOpenServerExA(pservername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSOpenServerExW<P0>(pservername: P0) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerExW(pservername : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { WTSOpenServerExW(pservername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSOpenServerW<P0>(pservername: P0) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerW(pservername : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { WTSOpenServerW(pservername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSQueryListenerConfigA<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *mut WTSLISTENERCONFIGA) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigA(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, pbuffer : *mut WTSLISTENERCONFIGA) -> windows_core::BOOL);
    unsafe { WTSQueryListenerConfigA(hserver, preserved, reserved, plistenername.param().abi(), pbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSQueryListenerConfigW<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *mut WTSLISTENERCONFIGW) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigW(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, pbuffer : *mut WTSLISTENERCONFIGW) -> windows_core::BOOL);
    unsafe { WTSQueryListenerConfigW(hserver, preserved, reserved, plistenername.param().abi(), pbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSQuerySessionInformationA(hserver: super::winnt::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut windows_core::PSTR, pbytesreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationA(hserver : super::winnt::HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_core::PSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQuerySessionInformationA(hserver, sessionid, wtsinfoclass, ppbuffer as _, pbytesreturned as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSQuerySessionInformationW(hserver: super::winnt::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut windows_core::PWSTR, pbytesreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationW(hserver : super::winnt::HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_core::PWSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQuerySessionInformationW(hserver, sessionid, wtsinfoclass, ppbuffer as _, pbytesreturned as _) }
}
#[inline]
pub unsafe fn WTSQueryUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut windows_core::PSTR, pbytesreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigA(pservername : windows_core::PCSTR, pusername : windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_core::PSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQueryUserConfigA(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, ppbuffer as _, pbytesreturned as _) }
}
#[inline]
pub unsafe fn WTSQueryUserConfigW<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut windows_core::PWSTR, pbytesreturned: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigW(pservername : windows_core::PCWSTR, pusername : windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_core::PWSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQueryUserConfigW(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, ppbuffer as _, pbytesreturned as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryUserToken(sessionid : u32, phtoken : *mut super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WTSQueryUserToken(sessionid, phtoken as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WTSRegisterSessionNotification(hwnd: super::windef::HWND, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotification(hwnd : super::windef::HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { WTSRegisterSessionNotification(hwnd, dwflags) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WTSRegisterSessionNotificationEx(hserver: super::winnt::HANDLE, hwnd: super::windef::HWND, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotificationEx(hserver : super::winnt::HANDLE, hwnd : super::windef::HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { WTSRegisterSessionNotificationEx(hserver, hwnd, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSSendMessageA(hserver: super::winnt::HANDLE, sessionid: u32, ptitle: &[u8], pmessage: &[u8], style: u32, timeout: u32, presponse: *mut u32, bwait: bool) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSSendMessageA(hserver : super::winnt::HANDLE, sessionid : u32, ptitle : windows_core::PCSTR, titlelength : u32, pmessage : windows_core::PCSTR, messagelength : u32, style : u32, timeout : u32, presponse : *mut u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSSendMessageA(hserver, sessionid, core::mem::transmute(ptitle.as_ptr()), ptitle.len().try_into().unwrap(), core::mem::transmute(pmessage.as_ptr()), pmessage.len().try_into().unwrap(), style, timeout, presponse as _, bwait.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSSendMessageW<P2, P4>(hserver: super::winnt::HANDLE, sessionid: u32, ptitle: P2, titlelength: u32, pmessage: P4, messagelength: u32, style: u32, timeout: u32, presponse: *mut u32, bwait: bool) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSendMessageW(hserver : super::winnt::HANDLE, sessionid : u32, ptitle : windows_core::PCWSTR, titlelength : u32, pmessage : windows_core::PCWSTR, messagelength : u32, style : u32, timeout : u32, presponse : *mut u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSSendMessageW(hserver, sessionid, ptitle.param().abi(), titlelength, pmessage.param().abi(), messagelength, style, timeout, presponse as _, bwait.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSSetListenerSecurityA<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityA(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { WTSSetListenerSecurityA(hserver, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSSetListenerSecurityW<P3>(hserver: super::winnt::HANDLE, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityW(hserver : super::winnt::HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { WTSSetListenerSecurityW(hserver, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor) }
}
#[inline]
pub unsafe fn WTSSetUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: &[u8]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetUserConfigA(pservername : windows_core::PCSTR, pusername : windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_core::PCSTR, datalength : u32) -> windows_core::BOOL);
    unsafe { WTSSetUserConfigA(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn WTSSetUserConfigW<P0, P1, P3>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: P3, datalength: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetUserConfigW(pservername : windows_core::PCWSTR, pusername : windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_core::PCWSTR, datalength : u32) -> windows_core::BOOL);
    unsafe { WTSSetUserConfigW(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, pbuffer.param().abi(), datalength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSShutdownSystem(hserver: super::winnt::HANDLE, shutdownflag: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSShutdownSystem(hserver : super::winnt::HANDLE, shutdownflag : u32) -> windows_core::BOOL);
    unsafe { WTSShutdownSystem(hserver, shutdownflag) }
}
#[inline]
pub unsafe fn WTSStartRemoteControlSessionA<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionA(ptargetservername : windows_core::PCSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> windows_core::BOOL);
    unsafe { WTSStartRemoteControlSessionA(ptargetservername.param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers) }
}
#[inline]
pub unsafe fn WTSStartRemoteControlSessionW<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionW(ptargetservername : windows_core::PCWSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> windows_core::BOOL);
    unsafe { WTSStartRemoteControlSessionW(ptargetservername.param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers) }
}
#[inline]
pub unsafe fn WTSStopRemoteControlSession(logonid: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSStopRemoteControlSession(logonid : u32) -> windows_core::BOOL);
    unsafe { WTSStopRemoteControlSession(logonid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSTerminateProcess(hserver: super::winnt::HANDLE, processid: u32, exitcode: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSTerminateProcess(hserver : super::winnt::HANDLE, processid : u32, exitcode : u32) -> windows_core::BOOL);
    unsafe { WTSTerminateProcess(hserver, processid, exitcode) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotification(hwnd: super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotification(hwnd : super::windef::HWND) -> windows_core::BOOL);
    unsafe { WTSUnRegisterSessionNotification(hwnd) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WTSUnRegisterSessionNotificationEx(hserver: super::winnt::HANDLE, hwnd: super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotificationEx(hserver : super::winnt::HANDLE, hwnd : super::windef::HWND) -> windows_core::BOOL);
    unsafe { WTSUnRegisterSessionNotificationEx(hserver, hwnd) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelClose(hchannelhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelClose(hchannelhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelClose(hchannelhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelOpen<P2>(hserver: super::winnt::HANDLE, sessionid: u32, pvirtualname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpen(hserver : super::winnt::HANDLE, sessionid : u32, pvirtualname : windows_core::PCSTR) -> super::winnt::HANDLE);
    unsafe { WTSVirtualChannelOpen(hserver, sessionid, pvirtualname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelOpenEx<P1>(sessionid: u32, pvirtualname: P1, flags: u32) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpenEx(sessionid : u32, pvirtualname : windows_core::PCSTR, flags : u32) -> super::winnt::HANDLE);
    unsafe { WTSVirtualChannelOpenEx(sessionid, pvirtualname.param().abi(), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeInput(hchannelhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeInput(hchannelhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelPurgeInput(hchannelhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelPurgeOutput(hchannelhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeOutput(hchannelhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelPurgeOutput(hchannelhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelQuery(hchannelhandle: super::winnt::HANDLE, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut core::ffi::c_void, pbytesreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelQuery(hchannelhandle : super::winnt::HANDLE, param1 : WTS_VIRTUAL_CLASS, ppbuffer : *mut *mut core::ffi::c_void, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelQuery(hchannelhandle, param1, ppbuffer as _, pbytesreturned as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelRead(hchannelhandle: super::winnt::HANDLE, timeout: u32, buffer: &mut [u8], pbytesread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelRead(hchannelhandle : super::winnt::HANDLE, timeout : u32, buffer : *mut i8, buffersize : u32, pbytesread : *mut u32) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelRead(hchannelhandle, timeout, core::mem::transmute(buffer.as_mut_ptr()), buffer.len().try_into().unwrap(), pbytesread as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSVirtualChannelWrite(hchannelhandle: super::winnt::HANDLE, buffer: &[u8], pbyteswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelWrite(hchannelhandle : super::winnt::HANDLE, buffer : *const i8, length : u32, pbyteswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelWrite(hchannelhandle, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), pbyteswritten as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WTSWaitSystemEvent(hserver: super::winnt::HANDLE, eventmask: u32, peventflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSWaitSystemEvent(hserver : super::winnt::HANDLE, eventmask : u32, peventflags : *mut u32) -> windows_core::BOOL);
    unsafe { WTSWaitSystemEvent(hserver, eventmask, peventflags as _) }
}
pub const CLIENTADDRESS_LENGTH: u32 = 30;
pub const CLIENTNAME_LENGTH: u32 = 20;
pub const DOMAIN_LENGTH: u32 = 17;
pub const IDASYNC: u32 = 32001;
pub const MAX_DATE_TIME_LENGTH: u32 = 56;
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15;
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1;
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0;
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256;
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRODUCT_INFOA {
    pub CompanyName: [i8; 256],
    pub ProductID: [i8; 4],
}
impl Default for PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl Default for PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PWRDS_SERIALIZED_USER_CREDENTIAL = *mut WTS_SERIALIZED_USER_CREDENTIAL;
pub type PWTSCLIENTA = *mut WTSCLIENTA;
pub type PWTSCLIENTW = *mut WTSCLIENTW;
pub type PWTSCONFIGINFOA = *mut WTSCONFIGINFOA;
pub type PWTSCONFIGINFOW = *mut WTSCONFIGINFOW;
pub type PWTSINFOA = *mut WTSINFOA;
pub type PWTSINFOEXA = *mut WTSINFOEXA;
pub type PWTSINFOEXW = *mut WTSINFOEXW;
pub type PWTSINFOEX_LEVEL1_A = *mut WTSINFOEX_LEVEL1_A;
pub type PWTSINFOEX_LEVEL1_W = *mut WTSINFOEX_LEVEL1_W;
pub type PWTSINFOEX_LEVEL_A = *mut WTSINFOEX_LEVEL_A;
pub type PWTSINFOEX_LEVEL_W = *mut WTSINFOEX_LEVEL_W;
pub type PWTSINFOW = *mut WTSINFOW;
pub type PWTSLISTENERCONFIGA = *mut WTSLISTENERCONFIGA;
pub type PWTSLISTENERCONFIGW = *mut WTSLISTENERCONFIGW;
pub type PWTSLISTENERNAMEA = *mut WTSLISTENERNAMEA;
pub type PWTSLISTENERNAMEW = *mut WTSLISTENERNAMEW;
pub type PWTSUSERCONFIGA = *mut WTSUSERCONFIGA;
pub type PWTSUSERCONFIGW = *mut WTSUSERCONFIGW;
pub type PWTS_CLIENT_ADDRESS = *mut WTS_CLIENT_ADDRESS;
pub type PWTS_CLIENT_DISPLAY = *mut WTS_CLIENT_DISPLAY;
#[cfg(feature = "winnt")]
pub type PWTS_PROCESS_INFOA = *mut WTS_PROCESS_INFOA;
#[cfg(feature = "winnt")]
pub type PWTS_PROCESS_INFOW = *mut WTS_PROCESS_INFOW;
#[cfg(feature = "winnt")]
pub type PWTS_PROCESS_INFO_EXA = *mut WTS_PROCESS_INFO_EXA;
#[cfg(feature = "winnt")]
pub type PWTS_PROCESS_INFO_EXW = *mut WTS_PROCESS_INFO_EXW;
pub type PWTS_SERIALIZED_USER_CREDENTIAL = *mut WTS_SERIALIZED_USER_CREDENTIAL;
pub type PWTS_SERVER_INFOA = *mut WTS_SERVER_INFOA;
pub type PWTS_SERVER_INFOW = *mut WTS_SERVER_INFOW;
pub type PWTS_SESSION_ADDRESS = *mut WTS_SESSION_ADDRESS;
pub type PWTS_SESSION_INFOA = *mut WTS_SESSION_INFOA;
pub type PWTS_SESSION_INFOW = *mut WTS_SESSION_INFOW;
pub type PWTS_SESSION_INFO_1A = *mut WTS_SESSION_INFO_1A;
pub type PWTS_SESSION_INFO_1W = *mut WTS_SESSION_INFO_1W;
pub type PWTS_VALIDATION_INFORMATIONA = *mut WTS_VALIDATION_INFORMATIONA;
pub type PWTS_VALIDATION_INFORMATIONW = *mut WTS_VALIDATION_INFORMATIONW;
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4;
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2;
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1;
pub const USERNAME_LENGTH: u32 = 20;
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20;
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384;
pub const WINSTATIONNAME_LENGTH: u32 = 32;
pub type WRDS_SERIALIZED_USER_CREDENTIAL = WTS_SERIALIZED_USER_CREDENTIAL;
pub const WTSActive: WTS_CONNECTSTATE_CLASS = 0;
pub const WTSApplicationName: WTS_INFO_CLASS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSCLIENTA {
    pub ClientName: [i8; 21],
    pub Domain: [i8; 18],
    pub UserName: [i8; 21],
    pub WorkDirectory: [i8; 261],
    pub InitialProgram: [i8; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [i8; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [i8; 261],
}
impl Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSCLIENTW {
    pub ClientName: [u16; 21],
    pub Domain: [u16; 18],
    pub UserName: [u16; 21],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [u16; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [u16; 261],
}
impl Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSCONFIGINFOA {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [i8; 21],
    pub LogonDomain: [i8; 18],
    pub WorkDirectory: [i8; 261],
    pub InitialProgram: [i8; 261],
    pub ApplicationName: [i8; 261],
}
impl Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSCONFIGINFOW {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub ApplicationName: [u16; 261],
}
impl Default for WTSCONFIGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTSCapabilityCheck: WTS_INFO_CLASS = 31;
pub const WTSClientAddress: WTS_INFO_CLASS = 14;
pub const WTSClientBuildNumber: WTS_INFO_CLASS = 9;
pub const WTSClientDirectory: WTS_INFO_CLASS = 11;
pub const WTSClientDisplay: WTS_INFO_CLASS = 15;
pub const WTSClientHardwareId: WTS_INFO_CLASS = 13;
pub const WTSClientInfo: WTS_INFO_CLASS = 23;
pub const WTSClientName: WTS_INFO_CLASS = 10;
pub const WTSClientProductId: WTS_INFO_CLASS = 12;
pub const WTSClientProtocolType: WTS_INFO_CLASS = 16;
pub const WTSConfigInfo: WTS_INFO_CLASS = 26;
pub const WTSConnectQuery: WTS_CONNECTSTATE_CLASS = 2;
pub const WTSConnectState: WTS_INFO_CLASS = 8;
pub const WTSConnected: WTS_CONNECTSTATE_CLASS = 1;
pub const WTSDisconnected: WTS_CONNECTSTATE_CLASS = 4;
pub const WTSDomainName: WTS_INFO_CLASS = 7;
pub const WTSDown: WTS_CONNECTSTATE_CLASS = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSINFOA {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBy: u32,
    pub WinStationName: [i8; 32],
    pub Domain: [i8; 17],
    pub UserName: [i8; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl Default for WTSINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOEXA {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_A,
}
impl Default for WTSINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOEXW {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_W,
}
impl Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSINFOEX_LEVEL1_A {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [i8; 33],
    pub UserName: [i8; 21],
    pub DomainName: [i8; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSINFOEX_LEVEL1_W {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u16; 33],
    pub UserName: [u16; 21],
    pub DomainName: [u16; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl Default for WTSINFOEX_LEVEL1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
impl Default for WTSINFOEX_LEVEL_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
impl Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSINFOW {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
    pub WinStationName: [u16; 32],
    pub Domain: [u16; 17],
    pub UserName: [u16; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl Default for WTSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = 5;
pub const WTSIdleTime: WTS_INFO_CLASS = 17;
pub const WTSIncomingBytes: WTS_INFO_CLASS = 19;
pub const WTSIncomingFrames: WTS_INFO_CLASS = 21;
pub const WTSInit: WTS_CONNECTSTATE_CLASS = 9;
pub const WTSInitialProgram: WTS_INFO_CLASS = 0;
pub const WTSIsRemoteSession: WTS_INFO_CLASS = 29;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSLISTENERCONFIGA {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [i8; 61],
    pub LogonUserName: [i8; 21],
    pub LogonDomain: [i8; 18],
    pub WorkDirectory: [i8; 261],
    pub InitialProgram: [i8; 261],
}
impl Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSLISTENERCONFIGW {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [u16; 61],
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
}
impl Default for WTSLISTENERCONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTSLISTENERNAMEA = [i8; 33];
pub type WTSLISTENERNAMEW = [u16; 33];
pub const WTSListen: WTS_CONNECTSTATE_CLASS = 6;
pub const WTSLogonTime: WTS_INFO_CLASS = 18;
pub const WTSOEMId: WTS_INFO_CLASS = 3;
pub const WTSOutgoingBytes: WTS_INFO_CLASS = 20;
pub const WTSOutgoingFrames: WTS_INFO_CLASS = 22;
pub const WTSReset: WTS_CONNECTSTATE_CLASS = 7;
pub const WTSSessionActivityId: WTS_INFO_CLASS = 30;
pub const WTSSessionAddressV4: WTS_INFO_CLASS = 28;
pub const WTSSessionId: WTS_INFO_CLASS = 4;
pub const WTSSessionInfo: WTS_INFO_CLASS = 24;
pub const WTSSessionInfoEx: WTS_INFO_CLASS = 25;
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = 3;
pub const WTSTypeCloudAuthServerNonce: WTS_TYPE_CLASS = 3;
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = 0;
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = 1;
pub const WTSTypeSerializedUserCredential: WTS_TYPE_CLASS = 4;
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSUSERCONFIGA {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [i8; 261],
    pub WorkDirectory: [i8; 261],
    pub TerminalServerProfilePath: [i8; 261],
    pub TerminalServerHomeDir: [i8; 261],
    pub TerminalServerHomeDirDrive: [i8; 4],
}
impl Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTSUSERCONFIGW {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [u16; 261],
    pub WorkDirectory: [u16; 261],
    pub TerminalServerProfilePath: [u16; 261],
    pub TerminalServerHomeDir: [u16; 261],
    pub TerminalServerHomeDirDrive: [u16; 4],
}
impl Default for WTSUSERCONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTSUserConfigBrokenTimeoutSettings: WTS_CONFIG_CLASS = 10;
pub const WTSUserConfigInitialProgram: WTS_CONFIG_CLASS = 0;
pub const WTSUserConfigModemCallbackPhoneNumber: WTS_CONFIG_CLASS = 13;
pub const WTSUserConfigModemCallbackSettings: WTS_CONFIG_CLASS = 12;
pub const WTSUserConfigReconnectSettings: WTS_CONFIG_CLASS = 11;
pub const WTSUserConfigShadowingSettings: WTS_CONFIG_CLASS = 14;
pub const WTSUserConfigSourceSAM: WTS_CONFIG_SOURCE = 0;
pub const WTSUserConfigTerminalServerHomeDir: WTS_CONFIG_CLASS = 16;
pub const WTSUserConfigTerminalServerHomeDirDrive: WTS_CONFIG_CLASS = 17;
pub const WTSUserConfigTerminalServerProfilePath: WTS_CONFIG_CLASS = 15;
pub const WTSUserConfigTimeoutSettingsConnections: WTS_CONFIG_CLASS = 4;
pub const WTSUserConfigTimeoutSettingsDisconnections: WTS_CONFIG_CLASS = 5;
pub const WTSUserConfigTimeoutSettingsIdle: WTS_CONFIG_CLASS = 6;
pub const WTSUserConfigUser: WTS_CONFIG_CLASS = 19;
pub const WTSUserConfigWorkingDirectory: WTS_CONFIG_CLASS = 1;
pub const WTSUserConfigfAllowLogonTerminalServer: WTS_CONFIG_CLASS = 3;
pub const WTSUserConfigfDeviceClientDefaultPrinter: WTS_CONFIG_CLASS = 9;
pub const WTSUserConfigfDeviceClientDrives: WTS_CONFIG_CLASS = 7;
pub const WTSUserConfigfDeviceClientPrinters: WTS_CONFIG_CLASS = 8;
pub const WTSUserConfigfInheritInitialProgram: WTS_CONFIG_CLASS = 2;
pub const WTSUserConfigfTerminalServerRemoteHomeDir: WTS_CONFIG_CLASS = 18;
pub const WTSUserName: WTS_INFO_CLASS = 5;
pub const WTSValidationInfo: WTS_INFO_CLASS = 27;
pub const WTSVirtualClientData: WTS_VIRTUAL_CLASS = 0;
pub const WTSVirtualFileHandle: WTS_VIRTUAL_CLASS = 1;
pub const WTSWinStationName: WTS_INFO_CLASS = 6;
pub const WTSWorkingDirectory: WTS_INFO_CLASS = 2;
pub const WTS_ANY_SESSION: u32 = 4294967294;
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1;
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WTS_CLOUD_AUTH_HANDLE(pub *mut core::ffi::c_void);
impl Default for WTS_CLOUD_AUTH_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_COMMENT_LENGTH: u32 = 60;
pub type WTS_CONFIG_CLASS = i32;
pub type WTS_CONFIG_SOURCE = i32;
pub type WTS_CONNECTSTATE_CLASS = i32;
pub const WTS_CURRENT_SERVER_NAME: u32 = 0;
pub const WTS_CURRENT_SESSION: u32 = 4294967295;
pub const WTS_DRIVE_LENGTH: u32 = 3;
pub const WTS_EVENT_ALL: u32 = 2147483647;
pub const WTS_EVENT_CONNECT: u32 = 8;
pub const WTS_EVENT_CREATE: u32 = 1;
pub const WTS_EVENT_DELETE: u32 = 2;
pub const WTS_EVENT_DISCONNECT: u32 = 16;
pub const WTS_EVENT_FLUSH: u32 = 2147483648;
pub const WTS_EVENT_LICENSE: u32 = 256;
pub const WTS_EVENT_LOGOFF: u32 = 64;
pub const WTS_EVENT_LOGON: u32 = 32;
pub const WTS_EVENT_NONE: u32 = 0;
pub const WTS_EVENT_RENAME: u32 = 4;
pub const WTS_EVENT_STATECHANGE: u32 = 128;
pub type WTS_INFO_CLASS = i32;
pub const WTS_LISTENER_CREATE: u32 = 1;
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32;
pub const WTS_LISTENER_UPDATE: u32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PSTR,
    pub pUserSid: super::winnt::PSID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PWSTR,
    pub pUserSid: super::winnt::PSID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PSTR,
    pub pUserSid: super::winnt::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PWSTR,
    pub pUserSid: super::winnt::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0;
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1;
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0;
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1;
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2;
pub const WTS_SECURITY_ALL_ACCESS: u32 = 983999;
pub const WTS_SECURITY_CONNECT: u32 = 256;
pub const WTS_SECURITY_CURRENT_GUEST_ACCESS: u32 = 72;
pub const WTS_SECURITY_CURRENT_USER_ACCESS: u32 = 6;
pub const WTS_SECURITY_DISCONNECT: u32 = 512;
pub const WTS_SECURITY_GUEST_ACCESS: u32 = 32;
pub const WTS_SECURITY_LOGOFF: u32 = 64;
pub const WTS_SECURITY_LOGON: u32 = 32;
pub const WTS_SECURITY_MESSAGE: u32 = 128;
pub const WTS_SECURITY_QUERY_INFORMATION: u32 = 1;
pub const WTS_SECURITY_REMOTE_CONTROL: u32 = 16;
pub const WTS_SECURITY_RESET: u32 = 4;
pub const WTS_SECURITY_SET_INFORMATION: u32 = 2;
pub const WTS_SECURITY_USER_ACCESS: u32 = 329;
pub const WTS_SECURITY_VIRTUAL_CHANNELS: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTS_SERIALIZED_USER_CREDENTIAL {
    pub SerializationLength: u32,
    pub Serialization: *mut u8,
}
impl Default for WTS_SERIALIZED_USER_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SERVER_INFOA {
    pub pServerName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SERVER_INFOW {
    pub pServerName: windows_core::PWSTR,
}
pub const WTS_SESSIONSTATE_LOCK: u32 = 0;
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295;
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: windows_core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: windows_core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: windows_core::PSTR,
    pub pHostName: windows_core::PSTR,
    pub pUserName: windows_core::PSTR,
    pub pDomainName: windows_core::PSTR,
    pub pFarmName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: windows_core::PWSTR,
    pub pHostName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pDomainName: windows_core::PWSTR,
    pub pFarmName: windows_core::PWSTR,
}
pub type WTS_TYPE_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTS_VIRTUAL_CLASS = i32;
pub const WTS_WSD_FASTREBOOT: u32 = 16;
pub const WTS_WSD_LOGOFF: u32 = 1;
pub const WTS_WSD_POWEROFF: u32 = 8;
pub const WTS_WSD_REBOOT: u32 = 4;
pub const WTS_WSD_SHUTDOWN: u32 = 2;
