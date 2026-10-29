#[cfg(feature = "minwindef")]
windows_link::link!("wtsapi32.dll" "system" fn WTSActiveSessionExists(pbactivesessionexists : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSCloseServer(hserver : super::HANDLE));
windows_link::link!("wtsapi32.dll" "system" fn WTSCloudAuthClose(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE));
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSCloudAuthConvertAssertionToSerializedUserCredential(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE, assertion : windows_sys::core::PCSTR, assertionlength : super::ULONG, resourceid : windows_sys::core::PCWSTR, usercredential : *mut *mut WTS_SERIALIZED_USER_CREDENTIAL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSCloudAuthDuplicateSerializedUserCredential(usercredential : *const WTS_SERIALIZED_USER_CREDENTIAL, duplicatedusercredential : *mut *mut WTS_SERIALIZED_USER_CREDENTIAL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSCloudAuthGetServerNonce(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE, servernonce : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSCloudAuthNetworkLogonWithSerializedCredential(cloudauthhandle : WTS_CLOUD_AUTH_HANDLE, usercredential : *const WTS_SERIALIZED_USER_CREDENTIAL, token : *mut super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSCloudAuthOpen(activityid : *const windows_sys::core::GUID) -> WTS_CLOUD_AUTH_HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSConnectSessionA(logonid : super::ULONG, targetlogonid : super::ULONG, ppassword : windows_sys::core::PCSTR, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSConnectSessionW(logonid : super::ULONG, targetlogonid : super::ULONG, ppassword : windows_sys::core::PCWSTR, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSCreateListenerA(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCSTR, pbuffer : PWTSLISTENERCONFIGA, flag : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSCreateListenerW(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCWSTR, pbuffer : PWTSLISTENERCONFIGW, flag : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSDisconnectSession(hserver : super::HANDLE, sessionid : super::DWORD, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnableChildSessions(benable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersA(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plisteners : PWTSLISTENERNAMEA, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersW(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plisteners : PWTSLISTENERNAMEW, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesA(hserver : super::HANDLE, reserved : super::DWORD, version : super::DWORD, ppprocessinfo : *mut PWTS_PROCESS_INFOA, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExA(hserver : super::HANDLE, plevel : *mut super::DWORD, sessionid : super::DWORD, ppprocessinfo : *mut windows_sys::core::PSTR, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExW(hserver : super::HANDLE, plevel : *mut super::DWORD, sessionid : super::DWORD, ppprocessinfo : *mut windows_sys::core::PWSTR, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesW(hserver : super::HANDLE, reserved : super::DWORD, version : super::DWORD, ppprocessinfo : *mut PWTS_PROCESS_INFOW, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateServersA(pdomainname : windows_sys::core::PCSTR, reserved : super::DWORD, version : super::DWORD, ppserverinfo : *mut PWTS_SERVER_INFOA, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateServersW(pdomainname : windows_sys::core::PCWSTR, reserved : super::DWORD, version : super::DWORD, ppserverinfo : *mut PWTS_SERVER_INFOW, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsA(hserver : super::HANDLE, reserved : super::DWORD, version : super::DWORD, ppsessioninfo : *mut PWTS_SESSION_INFOA, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExA(hserver : super::HANDLE, plevel : *mut super::DWORD, filter : super::DWORD, ppsessioninfo : *mut PWTS_SESSION_INFO_1A, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExW(hserver : super::HANDLE, plevel : *mut super::DWORD, filter : super::DWORD, ppsessioninfo : *mut PWTS_SESSION_INFO_1W, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsW(hserver : super::HANDLE, reserved : super::DWORD, version : super::DWORD, ppsessioninfo : *mut PWTS_SESSION_INFOW, pcount : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSFreeMemory(pmemory : super::PVOID));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExA(wtstypeclass : WTS_TYPE_CLASS, pmemory : super::PVOID, numberofentries : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExW(wtstypeclass : WTS_TYPE_CLASS, pmemory : super::PVOID, numberofentries : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wtsapi32.dll" "system" fn WTSGetChildSessionId(psessionid : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityA(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : super::DWORD, lpnlengthneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityW(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCWSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : super::DWORD, lpnlengthneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("wtsapi32.dll" "system" fn WTSIsChildSessionsEnabled(pbenabled : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSLogoffSession(hserver : super::HANDLE, sessionid : super::DWORD, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerA(pservername : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerExA(pservername : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerExW(pservername : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerW(pservername : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigA(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCSTR, pbuffer : PWTSLISTENERCONFIGA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigW(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCWSTR, pbuffer : PWTSLISTENERCONFIGW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationA(hserver : super::HANDLE, sessionid : super::DWORD, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_sys::core::PSTR, pbytesreturned : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationW(hserver : super::HANDLE, sessionid : super::DWORD, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_sys::core::PWSTR, pbytesreturned : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigA(pservername : windows_sys::core::PCSTR, pusername : windows_sys::core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_sys::core::PSTR, pbytesreturned : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigW(pservername : windows_sys::core::PCWSTR, pusername : windows_sys::core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_sys::core::PWSTR, pbytesreturned : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryUserToken(sessionid : super::ULONG, phtoken : super::PHANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotification(hwnd : super::HWND, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotificationEx(hserver : super::HANDLE, hwnd : super::HWND, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSSendMessageA(hserver : super::HANDLE, sessionid : super::DWORD, ptitle : windows_sys::core::PCSTR, titlelength : super::DWORD, pmessage : windows_sys::core::PCSTR, messagelength : super::DWORD, style : super::DWORD, timeout : super::DWORD, presponse : *mut super::DWORD, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSSendMessageW(hserver : super::HANDLE, sessionid : super::DWORD, ptitle : windows_sys::core::PCWSTR, titlelength : super::DWORD, pmessage : windows_sys::core::PCWSTR, messagelength : super::DWORD, style : super::DWORD, timeout : super::DWORD, presponse : *mut super::DWORD, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityA(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityW(hserver : super::HANDLE, preserved : super::PVOID, reserved : super::DWORD, plistenername : windows_sys::core::PCWSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSSetUserConfigA(pservername : windows_sys::core::PCSTR, pusername : windows_sys::core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_sys::core::PCSTR, datalength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSSetUserConfigW(pservername : windows_sys::core::PCWSTR, pusername : windows_sys::core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_sys::core::PCWSTR, datalength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSShutdownSystem(hserver : super::HANDLE, shutdownflag : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionA(ptargetservername : windows_sys::core::PCSTR, targetlogonid : super::ULONG, hotkeyvk : super::BYTE, hotkeymodifiers : super::USHORT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionW(ptargetservername : windows_sys::core::PCWSTR, targetlogonid : super::ULONG, hotkeyvk : super::BYTE, hotkeymodifiers : super::USHORT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("wtsapi32.dll" "system" fn WTSStopRemoteControlSession(logonid : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSTerminateProcess(hserver : super::HANDLE, processid : super::DWORD, exitcode : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotification(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotificationEx(hserver : super::HANDLE, hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelClose(hchannelhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpen(hserver : super::HANDLE, sessionid : super::DWORD, pvirtualname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpenEx(sessionid : super::DWORD, pvirtualname : windows_sys::core::PCSTR, flags : super::DWORD) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeInput(hchannelhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeOutput(hchannelhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelQuery(hchannelhandle : super::HANDLE, param1 : WTS_VIRTUAL_CLASS, ppbuffer : *mut super::PVOID, pbytesreturned : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelRead(hchannelhandle : super::HANDLE, timeout : super::ULONG, buffer : super::PCHAR, buffersize : super::ULONG, pbytesread : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelWrite(hchannelhandle : super::HANDLE, buffer : super::PCHAR, length : super::ULONG, pbyteswritten : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wtsapi32.dll" "system" fn WTSWaitSystemEvent(hserver : super::HANDLE, eventmask : super::DWORD, peventflags : *mut super::DWORD) -> windows_sys::core::BOOL);
pub const CLIENTADDRESS_LENGTH: i32 = 30;
pub const CLIENTNAME_LENGTH: i32 = 20;
pub const DOMAIN_LENGTH: i32 = 17;
pub const IDASYNC: i32 = 32001;
pub const MAX_DATE_TIME_LENGTH: i32 = 56;
pub const MAX_ELAPSED_TIME_LENGTH: i32 = 15;
pub const NOTIFY_FOR_ALL_SESSIONS: i32 = 1;
pub const NOTIFY_FOR_THIS_SESSION: i32 = 0;
pub const PRODUCTINFO_COMPANYNAME_LENGTH: i32 = 256;
pub const PRODUCTINFO_PRODUCTID_LENGTH: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PRODUCT_INFOA {
    pub CompanyName: [super::CHAR; 256],
    pub ProductID: [super::CHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PRODUCT_INFOW {
    pub CompanyName: [super::WCHAR; 256],
    pub ProductID: [super::WCHAR; 4],
}
#[cfg(feature = "winnt")]
impl Default for PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PWRDS_SERIALIZED_USER_CREDENTIAL = *mut _WTS_SERIALIZED_USER_CREDENTIAL;
#[cfg(feature = "shared")]
pub type PWTSCLIENTA = *mut WTSCLIENTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSCLIENTW = *mut WTSCLIENTW;
#[cfg(feature = "shared")]
pub type PWTSCONFIGINFOA = *mut WTSCONFIGINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSCONFIGINFOW = *mut WTSCONFIGINFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOA = *mut WTSINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOEXA = *mut WTSINFOEXA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOEXW = *mut WTSINFOEXW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOEX_LEVEL1_A = *mut WTSINFOEX_LEVEL1_A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOEX_LEVEL1_W = *mut WTSINFOEX_LEVEL1_W;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOEX_LEVEL_A = *mut WTSINFOEX_LEVEL_A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOEX_LEVEL_W = *mut WTSINFOEX_LEVEL_W;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSINFOW = *mut WTSINFOW;
#[cfg(feature = "shared")]
pub type PWTSLISTENERCONFIGA = *mut WTSLISTENERCONFIGA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSLISTENERCONFIGW = *mut WTSLISTENERCONFIGW;
pub type PWTSLISTENERNAMEA = *mut [i8; 33];
pub type PWTSLISTENERNAMEW = *mut [u16; 33];
#[cfg(feature = "shared")]
pub type PWTSUSERCONFIGA = *mut WTSUSERCONFIGA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTSUSERCONFIGW = *mut WTSUSERCONFIGW;
#[cfg(feature = "shared")]
pub type PWTS_CLIENT_ADDRESS = *mut WTS_CLIENT_ADDRESS;
#[cfg(feature = "shared")]
pub type PWTS_CLIENT_DISPLAY = *mut WTS_CLIENT_DISPLAY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTS_PROCESS_INFOA = *mut WTS_PROCESS_INFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTS_PROCESS_INFOW = *mut WTS_PROCESS_INFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTS_PROCESS_INFO_EXA = *mut WTS_PROCESS_INFO_EXA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTS_PROCESS_INFO_EXW = *mut WTS_PROCESS_INFO_EXW;
#[cfg(feature = "shared")]
pub type PWTS_SERIALIZED_USER_CREDENTIAL = *mut _WTS_SERIALIZED_USER_CREDENTIAL;
pub type PWTS_SERVER_INFOA = *mut WTS_SERVER_INFOA;
pub type PWTS_SERVER_INFOW = *mut WTS_SERVER_INFOW;
#[cfg(feature = "shared")]
pub type PWTS_SESSION_ADDRESS = *mut WTS_SESSION_ADDRESS;
#[cfg(feature = "shared")]
pub type PWTS_SESSION_INFOA = *mut WTS_SESSION_INFOA;
#[cfg(feature = "shared")]
pub type PWTS_SESSION_INFOW = *mut WTS_SESSION_INFOW;
#[cfg(feature = "shared")]
pub type PWTS_SESSION_INFO_1A = *mut WTS_SESSION_INFO_1A;
#[cfg(feature = "shared")]
pub type PWTS_SESSION_INFO_1W = *mut WTS_SESSION_INFO_1W;
#[cfg(feature = "shared")]
pub type PWTS_VALIDATION_INFORMATIONA = *mut WTS_VALIDATION_INFORMATIONA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWTS_VALIDATION_INFORMATIONW = *mut WTS_VALIDATION_INFORMATIONW;
pub const REMOTECONTROL_KBDALT_HOTKEY: i32 = 4;
pub const REMOTECONTROL_KBDCTRL_HOTKEY: i32 = 2;
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: i32 = 1;
pub const USERNAME_LENGTH: i32 = 20;
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: i32 = 20;
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: i32 = 16384;
pub const WINSTATIONNAME_LENGTH: i32 = 32;
#[cfg(feature = "shared")]
pub type WRDS_SERIALIZED_USER_CREDENTIAL = _WTS_SERIALIZED_USER_CREDENTIAL;
pub const WTSActive: WTS_CONNECTSTATE_CLASS = 0;
pub const WTSApplicationName: WTS_INFO_CLASS = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WTSCLIENTA {
    pub ClientName: [super::CHAR; 21],
    pub Domain: [super::CHAR; 18],
    pub UserName: [super::CHAR; 21],
    pub WorkDirectory: [super::CHAR; 261],
    pub InitialProgram: [super::CHAR; 261],
    pub EncryptionLevel: super::BYTE,
    pub ClientAddressFamily: super::ULONG,
    pub ClientAddress: [super::USHORT; 31],
    pub HRes: super::USHORT,
    pub VRes: super::USHORT,
    pub ColorDepth: super::USHORT,
    pub ClientDirectory: [super::CHAR; 261],
    pub ClientBuildNumber: super::ULONG,
    pub ClientHardwareId: super::ULONG,
    pub ClientProductId: super::USHORT,
    pub OutBufCountHost: super::USHORT,
    pub OutBufCountClient: super::USHORT,
    pub OutBufLength: super::USHORT,
    pub DeviceId: [super::CHAR; 261],
}
#[cfg(feature = "shared")]
impl Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSCLIENTW {
    pub ClientName: [super::WCHAR; 21],
    pub Domain: [super::WCHAR; 18],
    pub UserName: [super::WCHAR; 21],
    pub WorkDirectory: [super::WCHAR; 261],
    pub InitialProgram: [super::WCHAR; 261],
    pub EncryptionLevel: super::BYTE,
    pub ClientAddressFamily: super::ULONG,
    pub ClientAddress: [super::USHORT; 31],
    pub HRes: super::USHORT,
    pub VRes: super::USHORT,
    pub ColorDepth: super::USHORT,
    pub ClientDirectory: [super::WCHAR; 261],
    pub ClientBuildNumber: super::ULONG,
    pub ClientHardwareId: super::ULONG,
    pub ClientProductId: super::USHORT,
    pub OutBufCountHost: super::USHORT,
    pub OutBufCountClient: super::USHORT,
    pub OutBufLength: super::USHORT,
    pub DeviceId: [super::WCHAR; 261],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WTSCONFIGINFOA {
    pub version: super::ULONG,
    pub fConnectClientDrivesAtLogon: super::ULONG,
    pub fConnectPrinterAtLogon: super::ULONG,
    pub fDisablePrinterRedirection: super::ULONG,
    pub fDisableDefaultMainClientPrinter: super::ULONG,
    pub ShadowSettings: super::ULONG,
    pub LogonUserName: [super::CHAR; 21],
    pub LogonDomain: [super::CHAR; 18],
    pub WorkDirectory: [super::CHAR; 261],
    pub InitialProgram: [super::CHAR; 261],
    pub ApplicationName: [super::CHAR; 261],
}
#[cfg(feature = "shared")]
impl Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSCONFIGINFOW {
    pub version: super::ULONG,
    pub fConnectClientDrivesAtLogon: super::ULONG,
    pub fConnectPrinterAtLogon: super::ULONG,
    pub fDisablePrinterRedirection: super::ULONG,
    pub fDisableDefaultMainClientPrinter: super::ULONG,
    pub ShadowSettings: super::ULONG,
    pub LogonUserName: [super::WCHAR; 21],
    pub LogonDomain: [super::WCHAR; 18],
    pub WorkDirectory: [super::WCHAR; 261],
    pub InitialProgram: [super::WCHAR; 261],
    pub ApplicationName: [super::WCHAR; 261],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSINFOA {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: super::DWORD,
    pub IncomingBytes: super::DWORD,
    pub OutgoingBytes: super::DWORD,
    pub IncomingFrames: super::DWORD,
    pub OutgoingFrames: super::DWORD,
    pub IncomingCompressedBytes: super::DWORD,
    pub OutgoingCompressedBy: super::DWORD,
    pub WinStationName: [super::CHAR; 32],
    pub Domain: [super::CHAR; 17],
    pub UserName: [super::CHAR; 21],
    pub ConnectTime: super::LARGE_INTEGER,
    pub DisconnectTime: super::LARGE_INTEGER,
    pub LastInputTime: super::LARGE_INTEGER,
    pub LogonTime: super::LARGE_INTEGER,
    pub CurrentTime: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSINFOEXA {
    pub Level: super::DWORD,
    pub Data: WTSINFOEX_LEVEL_A,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSINFOEXW {
    pub Level: super::DWORD,
    pub Data: WTSINFOEX_LEVEL_W,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSINFOEX_LEVEL1_A {
    pub SessionId: super::ULONG,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: super::LONG,
    pub WinStationName: [super::CHAR; 33],
    pub UserName: [super::CHAR; 21],
    pub DomainName: [super::CHAR; 18],
    pub LogonTime: super::LARGE_INTEGER,
    pub ConnectTime: super::LARGE_INTEGER,
    pub DisconnectTime: super::LARGE_INTEGER,
    pub LastInputTime: super::LARGE_INTEGER,
    pub CurrentTime: super::LARGE_INTEGER,
    pub IncomingBytes: super::DWORD,
    pub OutgoingBytes: super::DWORD,
    pub IncomingFrames: super::DWORD,
    pub OutgoingFrames: super::DWORD,
    pub IncomingCompressedBytes: super::DWORD,
    pub OutgoingCompressedBytes: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSINFOEX_LEVEL1_W {
    pub SessionId: super::ULONG,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: super::LONG,
    pub WinStationName: [super::WCHAR; 33],
    pub UserName: [super::WCHAR; 21],
    pub DomainName: [super::WCHAR; 18],
    pub LogonTime: super::LARGE_INTEGER,
    pub ConnectTime: super::LARGE_INTEGER,
    pub DisconnectTime: super::LARGE_INTEGER,
    pub LastInputTime: super::LARGE_INTEGER,
    pub CurrentTime: super::LARGE_INTEGER,
    pub IncomingBytes: super::DWORD,
    pub OutgoingBytes: super::DWORD,
    pub IncomingFrames: super::DWORD,
    pub OutgoingFrames: super::DWORD,
    pub IncomingCompressedBytes: super::DWORD,
    pub OutgoingCompressedBytes: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSINFOEX_LEVEL1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSINFOEX_LEVEL_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSINFOW {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: super::DWORD,
    pub IncomingBytes: super::DWORD,
    pub OutgoingBytes: super::DWORD,
    pub IncomingFrames: super::DWORD,
    pub OutgoingFrames: super::DWORD,
    pub IncomingCompressedBytes: super::DWORD,
    pub OutgoingCompressedBytes: super::DWORD,
    pub WinStationName: [super::WCHAR; 32],
    pub Domain: [super::WCHAR; 17],
    pub UserName: [super::WCHAR; 21],
    pub ConnectTime: super::LARGE_INTEGER,
    pub DisconnectTime: super::LARGE_INTEGER,
    pub LastInputTime: super::LARGE_INTEGER,
    pub LogonTime: super::LARGE_INTEGER,
    pub CurrentTime: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WTSLISTENERCONFIGA {
    pub version: super::ULONG,
    pub fEnableListener: super::ULONG,
    pub MaxConnectionCount: super::ULONG,
    pub fPromptForPassword: super::ULONG,
    pub fInheritColorDepth: super::ULONG,
    pub ColorDepth: super::ULONG,
    pub fInheritBrokenTimeoutSettings: super::ULONG,
    pub BrokenTimeoutSettings: super::ULONG,
    pub fDisablePrinterRedirection: super::ULONG,
    pub fDisableDriveRedirection: super::ULONG,
    pub fDisableComPortRedirection: super::ULONG,
    pub fDisableLPTPortRedirection: super::ULONG,
    pub fDisableClipboardRedirection: super::ULONG,
    pub fDisableAudioRedirection: super::ULONG,
    pub fDisablePNPRedirection: super::ULONG,
    pub fDisableDefaultMainClientPrinter: super::ULONG,
    pub LanAdapter: super::ULONG,
    pub PortNumber: super::ULONG,
    pub fInheritShadowSettings: super::ULONG,
    pub ShadowSettings: super::ULONG,
    pub TimeoutSettingsConnection: super::ULONG,
    pub TimeoutSettingsDisconnection: super::ULONG,
    pub TimeoutSettingsIdle: super::ULONG,
    pub SecurityLayer: super::ULONG,
    pub MinEncryptionLevel: super::ULONG,
    pub UserAuthentication: super::ULONG,
    pub Comment: [super::CHAR; 61],
    pub LogonUserName: [super::CHAR; 21],
    pub LogonDomain: [super::CHAR; 18],
    pub WorkDirectory: [super::CHAR; 261],
    pub InitialProgram: [super::CHAR; 261],
}
#[cfg(feature = "shared")]
impl Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSLISTENERCONFIGW {
    pub version: super::ULONG,
    pub fEnableListener: super::ULONG,
    pub MaxConnectionCount: super::ULONG,
    pub fPromptForPassword: super::ULONG,
    pub fInheritColorDepth: super::ULONG,
    pub ColorDepth: super::ULONG,
    pub fInheritBrokenTimeoutSettings: super::ULONG,
    pub BrokenTimeoutSettings: super::ULONG,
    pub fDisablePrinterRedirection: super::ULONG,
    pub fDisableDriveRedirection: super::ULONG,
    pub fDisableComPortRedirection: super::ULONG,
    pub fDisableLPTPortRedirection: super::ULONG,
    pub fDisableClipboardRedirection: super::ULONG,
    pub fDisableAudioRedirection: super::ULONG,
    pub fDisablePNPRedirection: super::ULONG,
    pub fDisableDefaultMainClientPrinter: super::ULONG,
    pub LanAdapter: super::ULONG,
    pub PortNumber: super::ULONG,
    pub fInheritShadowSettings: super::ULONG,
    pub ShadowSettings: super::ULONG,
    pub TimeoutSettingsConnection: super::ULONG,
    pub TimeoutSettingsDisconnection: super::ULONG,
    pub TimeoutSettingsIdle: super::ULONG,
    pub SecurityLayer: super::ULONG,
    pub MinEncryptionLevel: super::ULONG,
    pub UserAuthentication: super::ULONG,
    pub Comment: [super::WCHAR; 61],
    pub LogonUserName: [super::WCHAR; 21],
    pub LogonDomain: [super::WCHAR; 18],
    pub WorkDirectory: [super::WCHAR; 261],
    pub InitialProgram: [super::WCHAR; 261],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WTSUSERCONFIGA {
    pub Source: super::DWORD,
    pub InheritInitialProgram: super::DWORD,
    pub AllowLogonTerminalServer: super::DWORD,
    pub TimeoutSettingsConnections: super::DWORD,
    pub TimeoutSettingsDisconnections: super::DWORD,
    pub TimeoutSettingsIdle: super::DWORD,
    pub DeviceClientDrives: super::DWORD,
    pub DeviceClientPrinters: super::DWORD,
    pub ClientDefaultPrinter: super::DWORD,
    pub BrokenTimeoutSettings: super::DWORD,
    pub ReconnectSettings: super::DWORD,
    pub ShadowingSettings: super::DWORD,
    pub TerminalServerRemoteHomeDir: super::DWORD,
    pub InitialProgram: [super::CHAR; 261],
    pub WorkDirectory: [super::CHAR; 261],
    pub TerminalServerProfilePath: [super::CHAR; 261],
    pub TerminalServerHomeDir: [super::CHAR; 261],
    pub TerminalServerHomeDirDrive: [super::CHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTSUSERCONFIGW {
    pub Source: super::DWORD,
    pub InheritInitialProgram: super::DWORD,
    pub AllowLogonTerminalServer: super::DWORD,
    pub TimeoutSettingsConnections: super::DWORD,
    pub TimeoutSettingsDisconnections: super::DWORD,
    pub TimeoutSettingsIdle: super::DWORD,
    pub DeviceClientDrives: super::DWORD,
    pub DeviceClientPrinters: super::DWORD,
    pub ClientDefaultPrinter: super::DWORD,
    pub BrokenTimeoutSettings: super::DWORD,
    pub ReconnectSettings: super::DWORD,
    pub ShadowingSettings: super::DWORD,
    pub TerminalServerRemoteHomeDir: super::DWORD,
    pub InitialProgram: [super::WCHAR; 261],
    pub WorkDirectory: [super::WCHAR; 261],
    pub TerminalServerProfilePath: [super::WCHAR; 261],
    pub TerminalServerHomeDir: [super::WCHAR; 261],
    pub TerminalServerHomeDirDrive: [super::WCHAR; 4],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
pub const WTS_ANY_SESSION: super::DWORD = 4294967294;
pub const WTS_CHANNEL_OPTION_DYNAMIC: i32 = 1;
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: i32 = 8;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: i32 = 4;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: i32 = 0;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: i32 = 2;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: i32 = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: super::DWORD,
    pub Address: [super::BYTE; 20],
}
#[cfg(feature = "shared")]
impl Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: super::DWORD,
    pub VerticalResolution: super::DWORD,
    pub ColorDepth: super::DWORD,
}
pub type WTS_CLOUD_AUTH_HANDLE = *mut WTS_CLOUD_AUTH_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_CLOUD_AUTH_HANDLE__ {
    pub unused: i32,
}
pub const WTS_COMMENT_LENGTH: i32 = 60;
pub type WTS_CONFIG_CLASS = i32;
pub type WTS_CONFIG_SOURCE = i32;
pub type WTS_CONNECTSTATE_CLASS = i32;
#[cfg(feature = "winnt")]
pub const WTS_CURRENT_SERVER: super::HANDLE = 0 as _;
#[cfg(feature = "winnt")]
pub const WTS_CURRENT_SERVER_HANDLE: super::HANDLE = 0 as _;
pub const WTS_CURRENT_SERVER_NAME: i32 = 0;
#[cfg(feature = "shared")]
pub const WTS_CURRENT_SESSION: super::DWORD = 4294967295;
pub const WTS_DRIVE_LENGTH: i32 = 3;
pub const WTS_EVENT_ALL: i32 = 2147483647;
pub const WTS_EVENT_CONNECT: i32 = 8;
pub const WTS_EVENT_CREATE: i32 = 1;
pub const WTS_EVENT_DELETE: i32 = 2;
pub const WTS_EVENT_DISCONNECT: i32 = 16;
pub const WTS_EVENT_FLUSH: u32 = 2147483648;
pub const WTS_EVENT_LICENSE: i32 = 256;
pub const WTS_EVENT_LOGOFF: i32 = 64;
pub const WTS_EVENT_LOGON: i32 = 32;
pub const WTS_EVENT_NONE: i32 = 0;
pub const WTS_EVENT_RENAME: i32 = 4;
pub const WTS_EVENT_STATECHANGE: i32 = 128;
pub type WTS_INFO_CLASS = i32;
pub const WTS_LISTENER_CREATE: i32 = 1;
pub const WTS_LISTENER_NAME_LENGTH: i32 = 32;
pub const WTS_LISTENER_UPDATE: i32 = 16;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: super::DWORD,
    pub ProcessId: super::DWORD,
    pub pProcessName: windows_sys::core::PSTR,
    pub pUserSid: super::PSID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: super::DWORD,
    pub ProcessId: super::DWORD,
    pub pProcessName: windows_sys::core::PWSTR,
    pub pUserSid: super::PSID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: super::DWORD,
    pub ProcessId: super::DWORD,
    pub pProcessName: windows_sys::core::PSTR,
    pub pUserSid: super::PSID,
    pub NumberOfThreads: super::DWORD,
    pub HandleCount: super::DWORD,
    pub PagefileUsage: super::DWORD,
    pub PeakPagefileUsage: super::DWORD,
    pub WorkingSetSize: super::DWORD,
    pub PeakWorkingSetSize: super::DWORD,
    pub UserTime: super::LARGE_INTEGER,
    pub KernelTime: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTS_PROCESS_INFO_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: super::DWORD,
    pub ProcessId: super::DWORD,
    pub pProcessName: windows_sys::core::PWSTR,
    pub pUserSid: super::PSID,
    pub NumberOfThreads: super::DWORD,
    pub HandleCount: super::DWORD,
    pub PagefileUsage: super::DWORD,
    pub PeakPagefileUsage: super::DWORD,
    pub WorkingSetSize: super::DWORD,
    pub PeakWorkingSetSize: super::DWORD,
    pub UserTime: super::LARGE_INTEGER,
    pub KernelTime: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTS_PROCESS_INFO_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_PROCESS_INFO_LEVEL_0: i32 = 0;
pub const WTS_PROCESS_INFO_LEVEL_1: i32 = 1;
pub const WTS_PROTOCOL_TYPE_CONSOLE: i32 = 0;
pub const WTS_PROTOCOL_TYPE_ICA: i32 = 1;
pub const WTS_PROTOCOL_TYPE_RDP: i32 = 2;
pub const WTS_SECURITY_ALL_ACCESS: i32 = 983999;
pub const WTS_SECURITY_CONNECT: i32 = 256;
pub const WTS_SECURITY_CURRENT_GUEST_ACCESS: i32 = 72;
pub const WTS_SECURITY_DISCONNECT: i32 = 512;
pub const WTS_SECURITY_GUEST_ACCESS: i32 = 32;
pub const WTS_SECURITY_LOGOFF: i32 = 64;
pub const WTS_SECURITY_LOGON: i32 = 32;
pub const WTS_SECURITY_MESSAGE: i32 = 128;
pub const WTS_SECURITY_QUERY_INFORMATION: i32 = 1;
pub const WTS_SECURITY_REMOTE_CONTROL: i32 = 16;
pub const WTS_SECURITY_RESET: i32 = 4;
pub const WTS_SECURITY_SET_INFORMATION: i32 = 2;
pub const WTS_SECURITY_USER_ACCESS: i32 = 329;
pub const WTS_SECURITY_VIRTUAL_CHANNELS: i32 = 8;
#[cfg(feature = "shared")]
pub type WTS_SERIALIZED_USER_CREDENTIAL = _WTS_SERIALIZED_USER_CREDENTIAL;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_SERVER_INFOA {
    pub pServerName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_SERVER_INFOW {
    pub pServerName: windows_sys::core::PWSTR,
}
pub const WTS_SESSIONSTATE_LOCK: i32 = 0;
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295;
pub const WTS_SESSIONSTATE_UNLOCK: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: super::DWORD,
    pub Address: [super::BYTE; 20],
}
#[cfg(feature = "shared")]
impl Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WTS_SESSION_INFOA {
    pub SessionId: super::DWORD,
    pub pWinStationName: windows_sys::core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WTS_SESSION_INFOW {
    pub SessionId: super::DWORD,
    pub pWinStationName: windows_sys::core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: super::DWORD,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: super::DWORD,
    pub pSessionName: windows_sys::core::PSTR,
    pub pHostName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDomainName: windows_sys::core::PSTR,
    pub pFarmName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: super::DWORD,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: super::DWORD,
    pub pSessionName: windows_sys::core::PWSTR,
    pub pHostName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDomainName: windows_sys::core::PWSTR,
    pub pFarmName: windows_sys::core::PWSTR,
}
pub type WTS_TYPE_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: PRODUCT_INFOA,
    pub License: [super::BYTE; 16384],
    pub LicenseLength: super::DWORD,
    pub HardwareID: [super::BYTE; 20],
    pub HardwareIDLength: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: PRODUCT_INFOW,
    pub License: [super::BYTE; 16384],
    pub LicenseLength: super::DWORD,
    pub HardwareID: [super::BYTE; 20],
    pub HardwareIDLength: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTS_VIRTUAL_CLASS = i32;
pub const WTS_WSD_FASTREBOOT: i32 = 16;
pub const WTS_WSD_LOGOFF: i32 = 1;
pub const WTS_WSD_POWEROFF: i32 = 8;
pub const WTS_WSD_REBOOT: i32 = 4;
pub const WTS_WSD_SHUTDOWN: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct _WTS_SERIALIZED_USER_CREDENTIAL {
    pub SerializationLength: super::ULONG,
    pub Serialization: *mut super::BYTE,
}
