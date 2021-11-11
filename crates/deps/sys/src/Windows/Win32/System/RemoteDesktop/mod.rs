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
    pub fn WTSSetRenderHint(prenderhintid: *mut u64, hwndowner: super::super::Foundation::HWND, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows::runtime::HRESULT;
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
