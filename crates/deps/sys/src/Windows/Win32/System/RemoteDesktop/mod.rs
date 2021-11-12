#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCloseServer(hserver: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionA(logonid: u32, targetlogonid: u32, ppassword: super::super::Foundation::PSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionW(logonid: u32, targetlogonid: u32, ppassword: super::super::Foundation::PWSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSDisconnectSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnableChildSessions(benable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut i8, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut u16, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PWSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersA(pdomainname: super::super::Foundation::PSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersW(pdomainname: super::super::Foundation::PWSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub fn WTSGetActiveConsoleSessionId() -> u32;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSGetChildSessionId(psessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSIsChildSessionsEnabled(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSLogoffSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerA(pservername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExA(pservername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExW(pservername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerW(pservername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, pbuffer: *mut WTSLISTENERCONFIGA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, pbuffer: *mut WTSLISTENERCONFIGW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationA(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationW(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigA(pservername: super::super::Foundation::PSTR, pusername: super::super::Foundation::PSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigW(pservername: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotification(hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageA(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: super::super::Foundation::PSTR, titlelength: u32, pmessage: super::super::Foundation::PSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageW(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: super::super::Foundation::PWSTR, titlelength: u32, pmessage: super::super::Foundation::PWSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetRenderHint(prenderhintid: *mut u64, hwndowner: super::super::Foundation::HWND, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigA(pservername: super::super::Foundation::PSTR, pusername: super::super::Foundation::PSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: super::super::Foundation::PSTR, datalength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigW(pservername: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: super::super::Foundation::PWSTR, datalength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSShutdownSystem(hserver: super::super::Foundation::HANDLE, shutdownflag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionA(ptargetservername: super::super::Foundation::PSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionW(ptargetservername: super::super::Foundation::PWSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStopRemoteControlSession(logonid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSTerminateProcess(hserver: super::super::Foundation::HANDLE, processid: u32, exitcode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotification(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelClose(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelOpen(hserver: super::super::Foundation::HANDLE, sessionid: u32, pvirtualname: super::super::Foundation::PSTR) -> HwtsVirtualChannelHandle;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelOpenEx(sessionid: u32, pvirtualname: super::super::Foundation::PSTR, flags: u32) -> HwtsVirtualChannelHandle;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeInput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeOutput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelQuery(hchannelhandle: super::super::Foundation::HANDLE, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut ::core::ffi::c_void, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelRead(hchannelhandle: super::super::Foundation::HANDLE, timeout: u32, buffer: super::super::Foundation::PSTR, buffersize: u32, pbytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelWrite(hchannelhandle: super::super::Foundation::HANDLE, buffer: super::super::Foundation::PSTR, length: u32, pbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSWaitSystemEvent(hserver: super::super::Foundation::HANDLE, eventmask: u32, peventflags: *mut u32) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
pub struct AAAccountingData(i32);
pub struct AAAccountingDataType(i32);
pub struct AAAuthSchemes(i32);
pub struct AATrustClassID(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub struct ADsTSUserEx(i32);
pub struct AE_CURRENT_POSITION(i32);
pub struct AE_POSITION_FLAGS(i32);
pub struct BITMAP_RENDERER_STATISTICS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_DEF(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_ENTRY_POINTS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_DATA_RECEIVED: u32 = 10u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_DISCONNECTED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_INITIALIZED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_TERMINATED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_V1_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_WRITE_CANCELLED: u32 = 12u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_EVENT_WRITE_COMPLETE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_FAIL: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_FIRST: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_LAST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_FLAG_MIDDLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_MAX_COUNT: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_NAME_LEN: u32 = 7u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_COMPRESS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_COMPRESS_RDP: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_ENCRYPT_CS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_ENCRYPT_RDP: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_ENCRYPT_SC: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_INITIALIZED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_PRI_HIGH: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_PRI_LOW: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_PRI_MED: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_OPTION_SHOW_PROTOCOL: u32 = 2097152u32;
pub struct CHANNEL_PDU_HEADER(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ALREADY_CONNECTED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ALREADY_INITIALIZED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ALREADY_OPEN: u32 = 14u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_CHANNEL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_CHANNEL_HANDLE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_INIT_HANDLE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_BAD_PROC: u32 = 11u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_INITIALIZATION_ERROR: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_INVALID_INSTANCE: u32 = 18u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_INITIALIZED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY: u32 = 15u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NOT_OPEN: u32 = 10u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NO_BUFFER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NO_MEMORY: u32 = 12u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_NULL_DATA: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_TOO_MANY_CHANNELS: u32 = 5u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_UNKNOWN_CHANNEL_NAME: u32 = 13u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_UNSUPPORTED_VERSION: u32 = 19u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CHANNEL_RC_ZERO_LENGTH: u32 = 17u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const CLIENTNAME_LENGTH: u32 = 20u32;
pub struct CLIENT_DISPLAY(i32);
pub struct CLIENT_MESSAGE_TYPE(i32);
pub struct CONNECTION_CHANGE_NOTIFICATION(i32);
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1259668864,
    data2: 65188,
    data3: 19772,
    data4: [157, 228, 116, 51, 166, 102, 24, 247],
};
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1765769205, data2: 3150, data3: 19735, data4: [184, 224, 31, 112, 50, 94, 93, 88] };
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_ADMINMESSAGERECEIVED: u32 = 760u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_AUTORECONNECTED: u32 = 756u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_AUTORECONNECTING: u32 = 755u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_CONNECTED: u32 = 751u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_CONNECTING: u32 = 750u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_DIALOGDISMISSED: u32 = 758u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_DIALOGDISPLAYING: u32 = 757u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_DISCONNECTED: u32 = 753u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_KEYCOMBINATIONPRESSED: u32 = 761u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_LOGINCOMPLETED: u32 = 752u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_NETWORKSTATUSCHANGED: u32 = 759u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_REMOTEDESKTOPSIZECHANGED: u32 = 762u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_STATUSCHANGED: u32 = 754u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_AX_TOUCHPOINTERCURSORMOVED: u32 = 800u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS: u32 = 722u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT: u32 = 706u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT: u32 = 701u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS: u32 = 704u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT: u32 = 707u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT: u32 = 702u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION: u32 = 732u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY: u32 = 721u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT: u32 = 733u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT: u32 = 703u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES: u32 = 731u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS: u32 = 723u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY: u32 = 720u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES: u32 = 730u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS: u32 = 705u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS: u32 = 711u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS: u32 = 710u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED: u32 = 740u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED: u32 = 741u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED: u32 = 742u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER: u32 = 712u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const DOMAIN_LENGTH: u32 = 17u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const FORCE_REJOIN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
pub struct HwtsVirtualChannelHandle(i32);
pub struct IADsTSUserEx(pub *mut ::core::ffi::c_void);
pub struct IAudioDeviceEndpoint(pub *mut ::core::ffi::c_void);
pub struct IAudioEndpoint(pub *mut ::core::ffi::c_void);
pub struct IAudioEndpointControl(pub *mut ::core::ffi::c_void);
pub struct IAudioEndpointRT(pub *mut ::core::ffi::c_void);
pub struct IAudioInputEndpointRT(pub *mut ::core::ffi::c_void);
pub struct IAudioOutputEndpointRT(pub *mut ::core::ffi::c_void);
pub struct IRemoteDesktopClient(pub *mut ::core::ffi::c_void);
pub struct IRemoteDesktopClientActions(pub *mut ::core::ffi::c_void);
pub struct IRemoteDesktopClientSettings(pub *mut ::core::ffi::c_void);
pub struct IRemoteDesktopClientTouchPointer(pub *mut ::core::ffi::c_void);
pub struct IRemoteSystemAdditionalInfoProvider(pub *mut ::core::ffi::c_void);
pub struct ITSGAccountingEngine(pub *mut ::core::ffi::c_void);
pub struct ITSGAuthenticateUserSink(pub *mut ::core::ffi::c_void);
pub struct ITSGAuthenticationEngine(pub *mut ::core::ffi::c_void);
pub struct ITSGAuthorizeConnectionSink(pub *mut ::core::ffi::c_void);
pub struct ITSGAuthorizeResourceSink(pub *mut ::core::ffi::c_void);
pub struct ITSGPolicyEngine(pub *mut ::core::ffi::c_void);
pub struct ITsSbBaseNotifySink(pub *mut ::core::ffi::c_void);
pub struct ITsSbClientConnection(pub *mut ::core::ffi::c_void);
pub struct ITsSbClientConnectionPropertySet(pub *mut ::core::ffi::c_void);
pub struct ITsSbEnvironment(pub *mut ::core::ffi::c_void);
pub struct ITsSbEnvironmentPropertySet(pub *mut ::core::ffi::c_void);
pub struct ITsSbFilterPluginStore(pub *mut ::core::ffi::c_void);
pub struct ITsSbGenericNotifySink(pub *mut ::core::ffi::c_void);
pub struct ITsSbGlobalStore(pub *mut ::core::ffi::c_void);
pub struct ITsSbLoadBalanceResult(pub *mut ::core::ffi::c_void);
pub struct ITsSbLoadBalancing(pub *mut ::core::ffi::c_void);
pub struct ITsSbLoadBalancingNotifySink(pub *mut ::core::ffi::c_void);
pub struct ITsSbOrchestration(pub *mut ::core::ffi::c_void);
pub struct ITsSbOrchestrationNotifySink(pub *mut ::core::ffi::c_void);
pub struct ITsSbPlacement(pub *mut ::core::ffi::c_void);
pub struct ITsSbPlacementNotifySink(pub *mut ::core::ffi::c_void);
pub struct ITsSbPlugin(pub *mut ::core::ffi::c_void);
pub struct ITsSbPluginNotifySink(pub *mut ::core::ffi::c_void);
pub struct ITsSbPluginPropertySet(pub *mut ::core::ffi::c_void);
pub struct ITsSbPropertySet(pub *mut ::core::ffi::c_void);
pub struct ITsSbProvider(pub *mut ::core::ffi::c_void);
pub struct ITsSbProvisioning(pub *mut ::core::ffi::c_void);
pub struct ITsSbProvisioningPluginNotifySink(pub *mut ::core::ffi::c_void);
pub struct ITsSbResourceNotification(pub *mut ::core::ffi::c_void);
pub struct ITsSbResourceNotificationEx(pub *mut ::core::ffi::c_void);
pub struct ITsSbResourcePlugin(pub *mut ::core::ffi::c_void);
pub struct ITsSbResourcePluginStore(pub *mut ::core::ffi::c_void);
pub struct ITsSbServiceNotification(pub *mut ::core::ffi::c_void);
pub struct ITsSbSession(pub *mut ::core::ffi::c_void);
pub struct ITsSbTarget(pub *mut ::core::ffi::c_void);
pub struct ITsSbTargetPropertySet(pub *mut ::core::ffi::c_void);
pub struct ITsSbTaskInfo(pub *mut ::core::ffi::c_void);
pub struct ITsSbTaskPlugin(pub *mut ::core::ffi::c_void);
pub struct ITsSbTaskPluginNotifySink(pub *mut ::core::ffi::c_void);
pub struct IWRdsEnhancedFastReconnectArbitrator(pub *mut ::core::ffi::c_void);
pub struct IWRdsGraphicsChannel(pub *mut ::core::ffi::c_void);
pub struct IWRdsGraphicsChannelEvents(pub *mut ::core::ffi::c_void);
pub struct IWRdsGraphicsChannelManager(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolConnection(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolConnectionCallback(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolConnectionSettings(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolLicenseConnection(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolListener(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolListenerCallback(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolLogonErrorRedirector(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolManager(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolSettings(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolShadowCallback(pub *mut ::core::ffi::c_void);
pub struct IWRdsProtocolShadowConnection(pub *mut ::core::ffi::c_void);
pub struct IWRdsWddmIddProps(pub *mut ::core::ffi::c_void);
pub struct IWTSBitmapRenderService(pub *mut ::core::ffi::c_void);
pub struct IWTSBitmapRenderer(pub *mut ::core::ffi::c_void);
pub struct IWTSBitmapRendererCallback(pub *mut ::core::ffi::c_void);
pub struct IWTSListener(pub *mut ::core::ffi::c_void);
pub struct IWTSListenerCallback(pub *mut ::core::ffi::c_void);
pub struct IWTSPlugin(pub *mut ::core::ffi::c_void);
pub struct IWTSPluginServiceProvider(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolConnection(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolConnectionCallback(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolLicenseConnection(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolListener(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolListenerCallback(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolLogonErrorRedirector(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolManager(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolShadowCallback(pub *mut ::core::ffi::c_void);
pub struct IWTSProtocolShadowConnection(pub *mut ::core::ffi::c_void);
pub struct IWTSSBPlugin(pub *mut ::core::ffi::c_void);
pub struct IWTSVirtualChannel(pub *mut ::core::ffi::c_void);
pub struct IWTSVirtualChannelCallback(pub *mut ::core::ffi::c_void);
pub struct IWTSVirtualChannelManager(pub *mut ::core::ffi::c_void);
pub struct IWorkspace(pub *mut ::core::ffi::c_void);
pub struct IWorkspace2(pub *mut ::core::ffi::c_void);
pub struct IWorkspace3(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceClientExt(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceRegistration(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceRegistration2(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceReportMessage(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceResTypeRegistry(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceScriptable(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceScriptable2(pub *mut ::core::ffi::c_void);
pub struct IWorkspaceScriptable3(pub *mut ::core::ffi::c_void);
pub struct ItsPubPlugin(pub *mut ::core::ffi::c_void);
pub struct ItsPubPlugin2(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
pub struct KeyCombinationType(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MAX_DATE_TIME_LENGTH: u32 = 56u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MAX_POLICY_ATTRIBUTES: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxAppName_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxDomainName_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxFQDN_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxFarm_Len: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxNetBiosName_Len: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxNumOfExposed_IPs: u32 = 12u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const MaxUserName_Len: u32 = 104u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
pub struct PCHANNEL_INIT_EVENT_FN(i32);
pub struct PCHANNEL_OPEN_EVENT_FN(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
pub struct PLUGIN_TYPE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 215995022,
    data2: 53433,
    data3: 19487,
    data4: [165, 235, 109, 31, 108, 101, 53, 185],
};
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3979100122,
    data2: 13197,
    data3: 19775,
    data4: [129, 163, 231, 103, 49, 13, 144, 142],
};
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1645401943, data2: 67, data3: 18530, data4: [153, 195, 159, 48, 89, 172, 42, 59] };
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 427573882, data2: 309, data3: 19309, data4: [156, 94, 230, 87, 154, 10, 182, 37] };
pub struct PVIRTUALCHANNELCLOSE(i32);
pub struct PVIRTUALCHANNELENTRY(i32);
pub struct PVIRTUALCHANNELINIT(i32);
pub struct PVIRTUALCHANNELOPEN(i32);
pub struct PVIRTUALCHANNELWRITE(i32);
pub struct PasswordEncodingType(i32);
pub struct PolicyAttributeType(i32);
pub const RDCLIENT_BITMAP_RENDER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3838576843, data2: 37934, data3: 19225, data4: [133, 4, 189, 90, 137, 167, 71, 245] };
pub struct RDV_TASK_STATUS(i32);
pub struct RD_FARM_TYPE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RENDER_HINT_CLEAR: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RENDER_HINT_VIDEO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_CLIENT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_GFX_MAX_SUPPORTED_MONITORS: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MONITOR_INFO(i32);
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE(i32);
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM(i32);
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY(i32);
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET(i32);
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST(i32);
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY(i32);
pub struct RFX_GFX_MSG_HEADER(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
pub struct RFX_GFX_MSG_RDP_DATA(i32);
pub struct RFX_GFX_RECT(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
pub struct RemoteActionType(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
pub struct SESSION_TIMEOUT_ACTION_TYPE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const SINGLE_SESSION: u32 = 1u32;
pub struct SnapshotEncodingType(i32);
pub struct SnapshotFormatType(i32);
pub struct TARGET_CHANGE_TYPE(i32);
pub struct TARGET_OWNER(i32);
pub struct TARGET_PATCH_STATE(i32);
pub struct TARGET_STATE(i32);
pub struct TARGET_TYPE(i32);
pub struct TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(i32);
pub struct TSPUB_PLUGIN_PD_RESOLUTION_TYPE(i32);
pub struct TSSB_NOTIFICATION_TYPE(i32);
pub struct TSSD_AddrV46Type(i32);
pub struct TSSD_ConnectionPoint(i32);
pub struct TSSESSION_STATE(i32);
pub struct TSUserExInterfaces(i32);
pub struct TS_SB_SORT_BY(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const USERNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
pub struct VM_HOST_NOTIFY_STATUS(i32);
pub struct VM_NOTIFY_ENTRY(i32);
pub struct VM_NOTIFY_INFO(i32);
pub struct VM_NOTIFY_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct VM_PATCH_INFO(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTING(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS_1(i32);
pub struct WRDS_CONNECTION_SETTING_LEVEL(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
pub struct WRDS_DYNAMIC_TIME_ZONE_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
pub struct WRDS_LISTENER_SETTING(i32);
pub struct WRDS_LISTENER_SETTINGS(i32);
pub struct WRDS_LISTENER_SETTINGS_1(i32);
pub struct WRDS_LISTENER_SETTING_LEVEL(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3533258573, data2: 719, data3: 17024, data4: [140, 72, 22, 36, 180, 79, 135, 6] };
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTING(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS_1(i32);
pub struct WRDS_SETTING_LEVEL(i32);
pub struct WRDS_SETTING_STATUS(i32);
pub struct WRDS_SETTING_TYPE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
pub struct WRdsGraphicsChannelType(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCLIENTA(i32);
pub struct WTSCLIENTW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCONFIGINFOA(i32);
pub struct WTSCONFIGINFOW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEXA(i32);
pub struct WTSINFOEXW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEX_LEVEL1_A(i32);
pub struct WTSINFOEX_LEVEL1_W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEX_LEVEL_A(i32);
pub struct WTSINFOEX_LEVEL_W(i32);
pub struct WTSINFOW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSLISTENERCONFIGA(i32);
pub struct WTSLISTENERCONFIGW(i32);
pub struct WTSSBX_ADDRESS_FAMILY(i32);
pub struct WTSSBX_IP_ADDRESS(i32);
pub struct WTSSBX_MACHINE_CONNECT_INFO(i32);
pub struct WTSSBX_MACHINE_DRAIN(i32);
pub struct WTSSBX_MACHINE_INFO(i32);
pub struct WTSSBX_MACHINE_SESSION_MODE(i32);
pub struct WTSSBX_MACHINE_STATE(i32);
pub struct WTSSBX_NOTIFICATION_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSSBX_SESSION_INFO(i32);
pub struct WTSSBX_SESSION_STATE(i32);
pub struct WTSSESSION_NOTIFICATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTSUSERCONFIGA(i32);
pub struct WTSUSERCONFIGW(i32);
pub struct WTS_CACHE_STATS(i32);
pub struct WTS_CACHE_STATS_UN(i32);
pub struct WTS_CERT_TYPE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
pub struct WTS_CLIENT_ADDRESS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_CLIENT_DATA(i32);
pub struct WTS_CLIENT_DISPLAY(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
pub struct WTS_CONFIG_CLASS(i32);
pub struct WTS_CONFIG_SOURCE(i32);
pub struct WTS_CONNECTSTATE_CLASS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
pub struct WTS_DISPLAY_IOCTL(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DOMAIN_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_DRIVE_LENGTH: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_ALL: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_CONNECT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_CREATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_DELETE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_DISCONNECT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_FLUSH: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_LICENSE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_LOGON: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_RENAME: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_EVENT_STATECHANGE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_IMEFILENAME_LENGTH: u32 = 32u32;
pub struct WTS_INFO_CLASS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_LICENSE_CAPABILITIES(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LISTENER_CREATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
pub struct WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_CACHE_RESERVED: u32 = 20u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_COUNTERS: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_PROTOCOL_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_MAX_RESERVED: u32 = 100u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PASSWORD_LENGTH: u32 = 255u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_THEMING: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_POLICY_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXW(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROPERTY_VALUE(i32);
pub struct WTS_PROTOCOL_CACHE(i32);
pub struct WTS_PROTOCOL_COUNTERS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub struct WTS_PROTOCOL_STATUS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3346864944,
    data2: 23521,
    data3: 19563,
    data4: [160, 225, 189, 109, 46, 92, 159, 204],
};
pub const WTS_QUERY_AUDIOENUM_DLL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2616523415, data2: 51331, data3: 19498, data4: [128, 171, 90, 57, 201, 175, 0, 219] };
pub const WTS_QUERY_LOGON_SCREEN_SIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2341343207, data2: 2052, data3: 18958, data4: [178, 121, 134, 96, 177, 223, 0, 73] };
pub const WTS_QUERY_MF_FORMAT_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1099340496,
    data2: 25394,
    data3: 19912,
    data4: [149, 213, 219, 116, 158, 47, 29, 148],
};
pub struct WTS_RCM_DRAIN_STATE(i32);
pub struct WTS_RCM_SERVICE_STATE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_CONNECT: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_DISCONNECT: u32 = 512u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_GUEST_ACCESS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_LOGOFF: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_LOGON: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_MESSAGE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_REMOTE_CONTROL: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_RESET: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_SET_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SECURITY_VIRTUAL_CHANNELS: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SERVER_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SERVER_INFOW(i32);
pub struct WTS_SERVICE_STATE(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
pub struct WTS_SESSION_ADDRESS(i32);
pub struct WTS_SESSION_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFO_1A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFO_1W(i32);
pub struct WTS_SMALL_RECT(i32);
pub struct WTS_SOCKADDR(i32);
pub struct WTS_SYSTEMTIME(i32);
pub struct WTS_TIME_ZONE_INFORMATION(i32);
pub struct WTS_TYPE_CLASS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
pub struct WTS_USER_CREDENTIAL(i32);
pub struct WTS_USER_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_VALIDATION_INFORMATIONA(i32);
pub struct WTS_VALIDATION_INFORMATIONW(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
pub struct WTS_VIRTUAL_CLASS(i32);
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_LOGOFF: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_POWEROFF: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_REBOOT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub struct Workspace(i32);
pub struct _ITSWkspEvents(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub struct _WTS_PRODUCT_INFOA(i32);
pub struct _WTS_PRODUCT_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct pluginResource(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct pluginResource2(i32);
pub struct pluginResource2FileAssociation(i32);
