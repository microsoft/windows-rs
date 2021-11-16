#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCloseServer(hserver: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionA(logonid: u32, targetlogonid: u32, ppassword: super::super::Foundation::PSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionW(logonid: u32, targetlogonid: u32, ppassword: super::super::Foundation::PWSTR, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSDisconnectSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnableChildSessions(benable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut i8, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plisteners: *mut *mut u16, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut super::super::Foundation::PWSTR, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersA(pdomainname: super::super::Foundation::PSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersW(pdomainname: super::super::Foundation::PWSTR, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsA(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExA(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExW(hserver: super::super::Foundation::HANDLE, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsW(hserver: super::super::Foundation::HANDLE, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> super::super::Foundation::BOOL;
    pub fn WTSFreeMemory(pmemory: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const ::core::ffi::c_void, numberofentries: u32) -> super::super::Foundation::BOOL;
    pub fn WTSGetActiveConsoleSessionId() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSGetChildSessionId(psessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSIsChildSessionsEnabled(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSLogoffSession(hserver: super::super::Foundation::HANDLE, sessionid: u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerA(pservername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExA(pservername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExW(pservername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerW(pservername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, pbuffer: *mut WTSLISTENERCONFIGA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, pbuffer: *mut WTSLISTENERCONFIGW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationA(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationW(hserver: super::super::Foundation::HANDLE, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigA(pservername: super::super::Foundation::PSTR, pusername: super::super::Foundation::PSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigW(pservername: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut super::super::Foundation::PWSTR, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotification(hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageA(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: super::super::Foundation::PSTR, titlelength: u32, pmessage: super::super::Foundation::PSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageW(hserver: super::super::Foundation::HANDLE, sessionid: u32, ptitle: super::super::Foundation::PWSTR, titlelength: u32, pmessage: super::super::Foundation::PWSTR, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityA(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityW(hserver: super::super::Foundation::HANDLE, preserved: *const ::core::ffi::c_void, reserved: u32, plistenername: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetRenderHint(prenderhintid: *mut u64, hwndowner: super::super::Foundation::HWND, renderhinttype: u32, cbhintdatalength: u32, phintdata: *const u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigA(pservername: super::super::Foundation::PSTR, pusername: super::super::Foundation::PSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: super::super::Foundation::PSTR, datalength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigW(pservername: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: super::super::Foundation::PWSTR, datalength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSShutdownSystem(hserver: super::super::Foundation::HANDLE, shutdownflag: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionA(ptargetservername: super::super::Foundation::PSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionW(ptargetservername: super::super::Foundation::PWSTR, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStopRemoteControlSession(logonid: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSTerminateProcess(hserver: super::super::Foundation::HANDLE, processid: u32, exitcode: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotification(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotificationEx(hserver: super::super::Foundation::HANDLE, hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelClose(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelOpen(hserver: super::super::Foundation::HANDLE, sessionid: u32, pvirtualname: super::super::Foundation::PSTR) -> HwtsVirtualChannelHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelOpenEx(sessionid: u32, pvirtualname: super::super::Foundation::PSTR, flags: u32) -> HwtsVirtualChannelHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeInput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeOutput(hchannelhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelQuery(hchannelhandle: super::super::Foundation::HANDLE, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut ::core::ffi::c_void, pbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelRead(hchannelhandle: super::super::Foundation::HANDLE, timeout: u32, buffer: super::super::Foundation::PSTR, buffersize: u32, pbytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelWrite(hchannelhandle: super::super::Foundation::HANDLE, buffer: super::super::Foundation::PSTR, length: u32, pbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSWaitSystemEvent(hserver: super::super::Foundation::HANDLE, eventmask: u32, peventflags: *mut u32) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AAAccountingData {
    pub userName: super::super::Foundation::BSTR,
    pub clientName: super::super::Foundation::BSTR,
    pub authType: AAAuthSchemes,
    pub resourceName: super::super::Foundation::BSTR,
    pub portNumber: i32,
    pub protocolName: super::super::Foundation::BSTR,
    pub numberOfBytesReceived: i32,
    pub numberOfBytesTransfered: i32,
    pub reasonForDisconnect: super::super::Foundation::BSTR,
    pub mainSessionId: ::windows_sys::core::GUID,
    pub subSessionId: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AAAccountingData {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AAAccountingData {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AA_MAIN_SESSION_CREATION: i32 = 0i32;
pub const AA_SUB_SESSION_CREATION: i32 = 1i32;
pub const AA_SUB_SESSION_CLOSED: i32 = 2i32;
pub const AA_MAIN_SESSION_CLOSED: i32 = 3i32;
pub const AA_AUTH_MIN: i32 = 0i32;
pub const AA_AUTH_BASIC: i32 = 1i32;
pub const AA_AUTH_NTLM: i32 = 2i32;
pub const AA_AUTH_SC: i32 = 3i32;
pub const AA_AUTH_LOGGEDONCREDENTIALS: i32 = 4i32;
pub const AA_AUTH_NEGOTIATE: i32 = 5i32;
pub const AA_AUTH_ANY: i32 = 6i32;
pub const AA_AUTH_COOKIE: i32 = 7i32;
pub const AA_AUTH_DIGEST: i32 = 8i32;
pub const AA_AUTH_ORGID: i32 = 9i32;
pub const AA_AUTH_CONID: i32 = 10i32;
pub const AA_AUTH_SSPI_NTLM: i32 = 11i32;
pub const AA_AUTH_MAX: i32 = 12i32;
pub const AA_UNTRUSTED: i32 = 0i32;
pub const AA_TRUSTEDUSER_UNTRUSTEDCLIENT: i32 = 1i32;
pub const AA_TRUSTEDUSER_TRUSTEDCLIENT: i32 = 2i32;
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub const ADsTSUserEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3806972646,
    data2: 7803,
    data3: 19342,
    data4: [186, 189, 233, 191, 98, 146, 172, 41],
};
#[repr(C)]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
impl ::core::marker::Copy for AE_CURRENT_POSITION {}
impl ::core::clone::Clone for AE_CURRENT_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const POSITION_INVALID: i32 = 0i32;
pub const POSITION_DISCONTINUOUS: i32 = 1i32;
pub const POSITION_CONTINUOUS: i32 = 2i32;
pub const POSITION_QPC_ERROR: i32 = 4i32;
#[repr(C)]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
impl ::core::marker::Copy for BITMAP_RENDERER_STATISTICS {}
impl ::core::clone::Clone for BITMAP_RENDERER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_DEF {
    pub name: [super::super::Foundation::CHAR; 8],
    pub options: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANNEL_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANNEL_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANNEL_ENTRY_POINTS {
    pub cbSize: u32,
    pub protocolVersion: u32,
    pub pVirtualChannelInit: PVIRTUALCHANNELINIT,
    pub pVirtualChannelOpen: PVIRTUALCHANNELOPEN,
    pub pVirtualChannelClose: PVIRTUALCHANNELCLOSE,
    pub pVirtualChannelWrite: PVIRTUALCHANNELWRITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANNEL_ENTRY_POINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANNEL_ENTRY_POINTS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CHANNEL_EVENT_CONNECTED: u32 = 1u32;
pub const CHANNEL_EVENT_DATA_RECEIVED: u32 = 10u32;
pub const CHANNEL_EVENT_DISCONNECTED: u32 = 3u32;
pub const CHANNEL_EVENT_INITIALIZED: u32 = 0u32;
pub const CHANNEL_EVENT_TERMINATED: u32 = 4u32;
pub const CHANNEL_EVENT_V1_CONNECTED: u32 = 2u32;
pub const CHANNEL_EVENT_WRITE_CANCELLED: u32 = 12u32;
pub const CHANNEL_EVENT_WRITE_COMPLETE: u32 = 11u32;
pub const CHANNEL_FLAG_FAIL: u32 = 256u32;
pub const CHANNEL_FLAG_FIRST: u32 = 1u32;
pub const CHANNEL_FLAG_LAST: u32 = 2u32;
pub const CHANNEL_FLAG_MIDDLE: u32 = 0u32;
pub const CHANNEL_MAX_COUNT: u32 = 30u32;
pub const CHANNEL_NAME_LEN: u32 = 7u32;
pub const CHANNEL_OPTION_COMPRESS: u32 = 4194304u32;
pub const CHANNEL_OPTION_COMPRESS_RDP: u32 = 8388608u32;
pub const CHANNEL_OPTION_ENCRYPT_CS: u32 = 268435456u32;
pub const CHANNEL_OPTION_ENCRYPT_RDP: u32 = 1073741824u32;
pub const CHANNEL_OPTION_ENCRYPT_SC: u32 = 536870912u32;
pub const CHANNEL_OPTION_INITIALIZED: u32 = 2147483648u32;
pub const CHANNEL_OPTION_PRI_HIGH: u32 = 134217728u32;
pub const CHANNEL_OPTION_PRI_LOW: u32 = 33554432u32;
pub const CHANNEL_OPTION_PRI_MED: u32 = 67108864u32;
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: u32 = 1048576u32;
pub const CHANNEL_OPTION_SHOW_PROTOCOL: u32 = 2097152u32;
#[repr(C)]
pub struct CHANNEL_PDU_HEADER {
    pub length: u32,
    pub flags: u32,
}
impl ::core::marker::Copy for CHANNEL_PDU_HEADER {}
impl ::core::clone::Clone for CHANNEL_PDU_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CHANNEL_RC_ALREADY_CONNECTED: u32 = 3u32;
pub const CHANNEL_RC_ALREADY_INITIALIZED: u32 = 1u32;
pub const CHANNEL_RC_ALREADY_OPEN: u32 = 14u32;
pub const CHANNEL_RC_BAD_CHANNEL: u32 = 6u32;
pub const CHANNEL_RC_BAD_CHANNEL_HANDLE: u32 = 7u32;
pub const CHANNEL_RC_BAD_INIT_HANDLE: u32 = 9u32;
pub const CHANNEL_RC_BAD_PROC: u32 = 11u32;
pub const CHANNEL_RC_INITIALIZATION_ERROR: u32 = 20u32;
pub const CHANNEL_RC_INVALID_INSTANCE: u32 = 18u32;
pub const CHANNEL_RC_NOT_CONNECTED: u32 = 4u32;
pub const CHANNEL_RC_NOT_INITIALIZED: u32 = 2u32;
pub const CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY: u32 = 15u32;
pub const CHANNEL_RC_NOT_OPEN: u32 = 10u32;
pub const CHANNEL_RC_NO_BUFFER: u32 = 8u32;
pub const CHANNEL_RC_NO_MEMORY: u32 = 12u32;
pub const CHANNEL_RC_NULL_DATA: u32 = 16u32;
pub const CHANNEL_RC_OK: u32 = 0u32;
pub const CHANNEL_RC_TOO_MANY_CHANNELS: u32 = 5u32;
pub const CHANNEL_RC_UNKNOWN_CHANNEL_NAME: u32 = 13u32;
pub const CHANNEL_RC_UNSUPPORTED_VERSION: u32 = 19u32;
pub const CHANNEL_RC_ZERO_LENGTH: u32 = 17u32;
pub const CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for CLIENT_DISPLAY {}
impl ::core::clone::Clone for CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLIENT_MESSAGE_CONNECTION_INVALID: i32 = 0i32;
pub const CLIENT_MESSAGE_CONNECTION_STATUS: i32 = 1i32;
pub const CLIENT_MESSAGE_CONNECTION_ERROR: i32 = 2i32;
pub const CONNECTION_REQUEST_INVALID: i32 = 0i32;
pub const CONNECTION_REQUEST_PENDING: i32 = 1i32;
pub const CONNECTION_REQUEST_FAILED: i32 = 2i32;
pub const CONNECTION_REQUEST_TIMEDOUT: i32 = 3i32;
pub const CONNECTION_REQUEST_SUCCEEDED: i32 = 4i32;
pub const CONNECTION_REQUEST_CANCELLED: i32 = 5i32;
pub const CONNECTION_REQUEST_LB_COMPLETED: i32 = 6i32;
pub const CONNECTION_REQUEST_QUERY_PL_COMPLETED: i32 = 7i32;
pub const CONNECTION_REQUEST_ORCH_COMPLETED: i32 = 8i32;
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1259668864,
    data2: 65188,
    data3: 19772,
    data4: [157, 228, 116, 51, 166, 102, 24, 247],
};
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1765769205, data2: 3150, data3: 19735, data4: [184, 224, 31, 112, 50, 94, 93, 88] };
pub const DISPID_AX_ADMINMESSAGERECEIVED: u32 = 760u32;
pub const DISPID_AX_AUTORECONNECTED: u32 = 756u32;
pub const DISPID_AX_AUTORECONNECTING: u32 = 755u32;
pub const DISPID_AX_CONNECTED: u32 = 751u32;
pub const DISPID_AX_CONNECTING: u32 = 750u32;
pub const DISPID_AX_DIALOGDISMISSED: u32 = 758u32;
pub const DISPID_AX_DIALOGDISPLAYING: u32 = 757u32;
pub const DISPID_AX_DISCONNECTED: u32 = 753u32;
pub const DISPID_AX_KEYCOMBINATIONPRESSED: u32 = 761u32;
pub const DISPID_AX_LOGINCOMPLETED: u32 = 752u32;
pub const DISPID_AX_NETWORKSTATUSCHANGED: u32 = 759u32;
pub const DISPID_AX_REMOTEDESKTOPSIZECHANGED: u32 = 762u32;
pub const DISPID_AX_STATUSCHANGED: u32 = 754u32;
pub const DISPID_AX_TOUCHPOINTERCURSORMOVED: u32 = 800u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS: u32 = 722u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT: u32 = 706u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT: u32 = 701u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS: u32 = 704u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT: u32 = 707u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT: u32 = 702u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION: u32 = 732u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY: u32 = 721u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT: u32 = 733u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT: u32 = 703u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES: u32 = 731u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS: u32 = 723u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY: u32 = 720u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES: u32 = 730u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS: u32 = 705u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS: u32 = 711u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS: u32 = 710u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED: u32 = 740u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED: u32 = 741u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED: u32 = 742u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER: u32 = 712u32;
pub const DOMAIN_LENGTH: u32 = 17u32;
pub const FORCE_REJOIN: u32 = 2u32;
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
pub type HwtsVirtualChannelHandle = isize;
#[repr(transparent)]
pub struct IADsTSUserEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IADsTSUserEx {}
impl ::core::clone::Clone for IADsTSUserEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceEndpoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceEndpoint {}
impl ::core::clone::Clone for IAudioDeviceEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpoint {}
impl ::core::clone::Clone for IAudioEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointControl {}
impl ::core::clone::Clone for IAudioEndpointControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointRT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointRT {}
impl ::core::clone::Clone for IAudioEndpointRT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioInputEndpointRT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioInputEndpointRT {}
impl ::core::clone::Clone for IAudioInputEndpointRT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioOutputEndpointRT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioOutputEndpointRT {}
impl ::core::clone::Clone for IAudioOutputEndpointRT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteDesktopClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteDesktopClient {}
impl ::core::clone::Clone for IRemoteDesktopClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteDesktopClientActions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteDesktopClientActions {}
impl ::core::clone::Clone for IRemoteDesktopClientActions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteDesktopClientSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteDesktopClientSettings {}
impl ::core::clone::Clone for IRemoteDesktopClientSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteDesktopClientTouchPointer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteDesktopClientTouchPointer {}
impl ::core::clone::Clone for IRemoteDesktopClientTouchPointer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteSystemAdditionalInfoProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteSystemAdditionalInfoProvider {}
impl ::core::clone::Clone for IRemoteSystemAdditionalInfoProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITSGAccountingEngine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITSGAccountingEngine {}
impl ::core::clone::Clone for ITSGAccountingEngine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITSGAuthenticateUserSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITSGAuthenticateUserSink {}
impl ::core::clone::Clone for ITSGAuthenticateUserSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITSGAuthenticationEngine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITSGAuthenticationEngine {}
impl ::core::clone::Clone for ITSGAuthenticationEngine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITSGAuthorizeConnectionSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITSGAuthorizeConnectionSink {}
impl ::core::clone::Clone for ITSGAuthorizeConnectionSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITSGAuthorizeResourceSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITSGAuthorizeResourceSink {}
impl ::core::clone::Clone for ITSGAuthorizeResourceSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITSGPolicyEngine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITSGPolicyEngine {}
impl ::core::clone::Clone for ITSGPolicyEngine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbBaseNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbBaseNotifySink {}
impl ::core::clone::Clone for ITsSbBaseNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbClientConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbClientConnection {}
impl ::core::clone::Clone for ITsSbClientConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbClientConnectionPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbClientConnectionPropertySet {}
impl ::core::clone::Clone for ITsSbClientConnectionPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbEnvironment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbEnvironment {}
impl ::core::clone::Clone for ITsSbEnvironment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbEnvironmentPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbEnvironmentPropertySet {}
impl ::core::clone::Clone for ITsSbEnvironmentPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbFilterPluginStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbFilterPluginStore {}
impl ::core::clone::Clone for ITsSbFilterPluginStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbGenericNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbGenericNotifySink {}
impl ::core::clone::Clone for ITsSbGenericNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbGlobalStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbGlobalStore {}
impl ::core::clone::Clone for ITsSbGlobalStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbLoadBalanceResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbLoadBalanceResult {}
impl ::core::clone::Clone for ITsSbLoadBalanceResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbLoadBalancing(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbLoadBalancing {}
impl ::core::clone::Clone for ITsSbLoadBalancing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbLoadBalancingNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbLoadBalancingNotifySink {}
impl ::core::clone::Clone for ITsSbLoadBalancingNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbOrchestration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbOrchestration {}
impl ::core::clone::Clone for ITsSbOrchestration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbOrchestrationNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbOrchestrationNotifySink {}
impl ::core::clone::Clone for ITsSbOrchestrationNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbPlacement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbPlacement {}
impl ::core::clone::Clone for ITsSbPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbPlacementNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbPlacementNotifySink {}
impl ::core::clone::Clone for ITsSbPlacementNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbPlugin {}
impl ::core::clone::Clone for ITsSbPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbPluginNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbPluginNotifySink {}
impl ::core::clone::Clone for ITsSbPluginNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbPluginPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbPluginPropertySet {}
impl ::core::clone::Clone for ITsSbPluginPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbPropertySet {}
impl ::core::clone::Clone for ITsSbPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbProvider {}
impl ::core::clone::Clone for ITsSbProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbProvisioning(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbProvisioning {}
impl ::core::clone::Clone for ITsSbProvisioning {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbProvisioningPluginNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbProvisioningPluginNotifySink {}
impl ::core::clone::Clone for ITsSbProvisioningPluginNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbResourceNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbResourceNotification {}
impl ::core::clone::Clone for ITsSbResourceNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbResourceNotificationEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbResourceNotificationEx {}
impl ::core::clone::Clone for ITsSbResourceNotificationEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbResourcePlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbResourcePlugin {}
impl ::core::clone::Clone for ITsSbResourcePlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbResourcePluginStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbResourcePluginStore {}
impl ::core::clone::Clone for ITsSbResourcePluginStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbServiceNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbServiceNotification {}
impl ::core::clone::Clone for ITsSbServiceNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbSession {}
impl ::core::clone::Clone for ITsSbSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbTarget {}
impl ::core::clone::Clone for ITsSbTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbTargetPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbTargetPropertySet {}
impl ::core::clone::Clone for ITsSbTargetPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbTaskInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbTaskInfo {}
impl ::core::clone::Clone for ITsSbTaskInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbTaskPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbTaskPlugin {}
impl ::core::clone::Clone for ITsSbTaskPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITsSbTaskPluginNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITsSbTaskPluginNotifySink {}
impl ::core::clone::Clone for ITsSbTaskPluginNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsEnhancedFastReconnectArbitrator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsEnhancedFastReconnectArbitrator {}
impl ::core::clone::Clone for IWRdsEnhancedFastReconnectArbitrator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsGraphicsChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsGraphicsChannel {}
impl ::core::clone::Clone for IWRdsGraphicsChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsGraphicsChannelEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsGraphicsChannelEvents {}
impl ::core::clone::Clone for IWRdsGraphicsChannelEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsGraphicsChannelManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsGraphicsChannelManager {}
impl ::core::clone::Clone for IWRdsGraphicsChannelManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolConnection {}
impl ::core::clone::Clone for IWRdsProtocolConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolConnectionCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolConnectionCallback {}
impl ::core::clone::Clone for IWRdsProtocolConnectionCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolConnectionSettings {}
impl ::core::clone::Clone for IWRdsProtocolConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolLicenseConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolLicenseConnection {}
impl ::core::clone::Clone for IWRdsProtocolLicenseConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolListener {}
impl ::core::clone::Clone for IWRdsProtocolListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolListenerCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolListenerCallback {}
impl ::core::clone::Clone for IWRdsProtocolListenerCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolLogonErrorRedirector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolLogonErrorRedirector {}
impl ::core::clone::Clone for IWRdsProtocolLogonErrorRedirector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolManager {}
impl ::core::clone::Clone for IWRdsProtocolManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolSettings {}
impl ::core::clone::Clone for IWRdsProtocolSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolShadowCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolShadowCallback {}
impl ::core::clone::Clone for IWRdsProtocolShadowCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsProtocolShadowConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsProtocolShadowConnection {}
impl ::core::clone::Clone for IWRdsProtocolShadowConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWRdsWddmIddProps(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWRdsWddmIddProps {}
impl ::core::clone::Clone for IWRdsWddmIddProps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSBitmapRenderService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSBitmapRenderService {}
impl ::core::clone::Clone for IWTSBitmapRenderService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSBitmapRenderer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSBitmapRenderer {}
impl ::core::clone::Clone for IWTSBitmapRenderer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSBitmapRendererCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSBitmapRendererCallback {}
impl ::core::clone::Clone for IWTSBitmapRendererCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSListener {}
impl ::core::clone::Clone for IWTSListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSListenerCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSListenerCallback {}
impl ::core::clone::Clone for IWTSListenerCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSPlugin {}
impl ::core::clone::Clone for IWTSPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSPluginServiceProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSPluginServiceProvider {}
impl ::core::clone::Clone for IWTSPluginServiceProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolConnection {}
impl ::core::clone::Clone for IWTSProtocolConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolConnectionCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolConnectionCallback {}
impl ::core::clone::Clone for IWTSProtocolConnectionCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolLicenseConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolLicenseConnection {}
impl ::core::clone::Clone for IWTSProtocolLicenseConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolListener {}
impl ::core::clone::Clone for IWTSProtocolListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolListenerCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolListenerCallback {}
impl ::core::clone::Clone for IWTSProtocolListenerCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolLogonErrorRedirector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolLogonErrorRedirector {}
impl ::core::clone::Clone for IWTSProtocolLogonErrorRedirector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolManager {}
impl ::core::clone::Clone for IWTSProtocolManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolShadowCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolShadowCallback {}
impl ::core::clone::Clone for IWTSProtocolShadowCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSProtocolShadowConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSProtocolShadowConnection {}
impl ::core::clone::Clone for IWTSProtocolShadowConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSSBPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSSBPlugin {}
impl ::core::clone::Clone for IWTSSBPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSVirtualChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSVirtualChannel {}
impl ::core::clone::Clone for IWTSVirtualChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSVirtualChannelCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSVirtualChannelCallback {}
impl ::core::clone::Clone for IWTSVirtualChannelCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWTSVirtualChannelManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWTSVirtualChannelManager {}
impl ::core::clone::Clone for IWTSVirtualChannelManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspace {}
impl ::core::clone::Clone for IWorkspace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspace2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspace2 {}
impl ::core::clone::Clone for IWorkspace2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspace3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspace3 {}
impl ::core::clone::Clone for IWorkspace3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceClientExt(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceClientExt {}
impl ::core::clone::Clone for IWorkspaceClientExt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceRegistration {}
impl ::core::clone::Clone for IWorkspaceRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceRegistration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceRegistration2 {}
impl ::core::clone::Clone for IWorkspaceRegistration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceReportMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceReportMessage {}
impl ::core::clone::Clone for IWorkspaceReportMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceResTypeRegistry(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceResTypeRegistry {}
impl ::core::clone::Clone for IWorkspaceResTypeRegistry {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceScriptable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceScriptable {}
impl ::core::clone::Clone for IWorkspaceScriptable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceScriptable2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceScriptable2 {}
impl ::core::clone::Clone for IWorkspaceScriptable2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspaceScriptable3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspaceScriptable3 {}
impl ::core::clone::Clone for IWorkspaceScriptable3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItsPubPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItsPubPlugin {}
impl ::core::clone::Clone for ItsPubPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItsPubPlugin2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItsPubPlugin2 {}
impl ::core::clone::Clone for ItsPubPlugin2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
pub const KeyCombinationHome: i32 = 0i32;
pub const KeyCombinationLeft: i32 = 1i32;
pub const KeyCombinationUp: i32 = 2i32;
pub const KeyCombinationRight: i32 = 3i32;
pub const KeyCombinationDown: i32 = 4i32;
pub const KeyCombinationScroll: i32 = 5i32;
pub const MAX_DATE_TIME_LENGTH: u32 = 56u32;
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15u32;
pub const MAX_POLICY_ATTRIBUTES: u32 = 20u32;
pub const MaxAppName_Len: u32 = 256u32;
pub const MaxDomainName_Len: u32 = 256u32;
pub const MaxFQDN_Len: u32 = 256u32;
pub const MaxFarm_Len: u32 = 256u32;
pub const MaxNetBiosName_Len: u32 = 16u32;
pub const MaxNumOfExposed_IPs: u32 = 12u32;
pub const MaxUserName_Len: u32 = 104u32;
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
pub type PCHANNEL_INIT_EVENT_FN = unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32);
pub type PCHANNEL_OPEN_EVENT_FN = unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32);
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
pub const UNKNOWN_PLUGIN: i32 = 0i32;
pub const POLICY_PLUGIN: i32 = 1i32;
pub const RESOURCE_PLUGIN: i32 = 2i32;
pub const LOAD_BALANCING_PLUGIN: i32 = 4i32;
pub const PLACEMENT_PLUGIN: i32 = 8i32;
pub const ORCHESTRATION_PLUGIN: i32 = 16i32;
pub const PROVISIONING_PLUGIN: i32 = 32i32;
pub const TASK_PLUGIN: i32 = 64i32;
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 215995022,
    data2: 53433,
    data3: 19487,
    data4: [165, 235, 109, 31, 108, 101, 53, 185],
};
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3979100122,
    data2: 13197,
    data3: 19775,
    data4: [129, 163, 231, 103, 49, 13, 144, 142],
};
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1645401943, data2: 67, data3: 18530, data4: [153, 195, 159, 48, 89, 172, 42, 59] };
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 427573882, data2: 309, data3: 19309, data4: [156, 94, 230, 87, 154, 10, 182, 37] };
pub type PVIRTUALCHANNELCLOSE = unsafe extern "system" fn(openhandle: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELENTRY = unsafe extern "system" fn(pentrypoints: *mut CHANNEL_ENTRY_POINTS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELINIT = unsafe extern "system" fn(ppinithandle: *mut *mut ::core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: PCHANNEL_INIT_EVENT_FN) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PVIRTUALCHANNELOPEN = unsafe extern "system" fn(pinithandle: *mut ::core::ffi::c_void, popenhandle: *mut u32, pchannelname: super::super::Foundation::PSTR, pchannelopeneventproc: PCHANNEL_OPEN_EVENT_FN) -> u32;
pub type PVIRTUALCHANNELWRITE = unsafe extern "system" fn(openhandle: u32, pdata: *mut ::core::ffi::c_void, datalength: u32, puserdata: *mut ::core::ffi::c_void) -> u32;
pub const PasswordEncodingUTF8: i32 = 0i32;
pub const PasswordEncodingUTF16LE: i32 = 1i32;
pub const PasswordEncodingUTF16BE: i32 = 2i32;
pub const EnableAllRedirections: i32 = 0i32;
pub const DisableAllRedirections: i32 = 1i32;
pub const DriveRedirectionDisabled: i32 = 2i32;
pub const PrinterRedirectionDisabled: i32 = 3i32;
pub const PortRedirectionDisabled: i32 = 4i32;
pub const ClipboardRedirectionDisabled: i32 = 5i32;
pub const PnpRedirectionDisabled: i32 = 6i32;
pub const AllowOnlySDRServers: i32 = 7i32;
pub const RDCLIENT_BITMAP_RENDER_SERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3838576843, data2: 37934, data3: 19225, data4: [133, 4, 189, 90, 137, 167, 71, 245] };
pub const RDV_TASK_STATUS_UNKNOWN: i32 = 0i32;
pub const RDV_TASK_STATUS_SEARCHING: i32 = 1i32;
pub const RDV_TASK_STATUS_DOWNLOADING: i32 = 2i32;
pub const RDV_TASK_STATUS_APPLYING: i32 = 3i32;
pub const RDV_TASK_STATUS_REBOOTING: i32 = 4i32;
pub const RDV_TASK_STATUS_REBOOTED: i32 = 5i32;
pub const RDV_TASK_STATUS_SUCCESS: i32 = 6i32;
pub const RDV_TASK_STATUS_FAILED: i32 = 7i32;
pub const RDV_TASK_STATUS_TIMEOUT: i32 = 8i32;
pub const RD_FARM_RDSH: i32 = 0i32;
pub const RD_FARM_TEMP_VM: i32 = 1i32;
pub const RD_FARM_MANUAL_PERSONAL_VM: i32 = 2i32;
pub const RD_FARM_AUTO_PERSONAL_VM: i32 = 3i32;
pub const RD_FARM_MANUAL_PERSONAL_RDSH: i32 = 4i32;
pub const RD_FARM_AUTO_PERSONAL_RDSH: i32 = 5i32;
pub const RD_FARM_TYPE_UNKNOWN: i32 = -1i32;
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
pub const RENDER_HINT_CLEAR: u32 = 0u32;
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
pub const RENDER_HINT_VIDEO: u32 = 1u32;
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
pub const RFX_CLIENT_ID_LENGTH: u32 = 32u32;
pub const RFX_GFX_MAX_SUPPORTED_MONITORS: u32 = 16u32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MONITOR_INFO {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub physicalWidth: u32,
    pub physicalHeight: u32,
    pub orientation: u32,
    pub primary: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MONITOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MONITOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub reserved: u32,
    pub monitorCount: u32,
    pub MonitorData: [RFX_GFX_MONITOR_INFO; 16],
    pub clientUniqueId: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulBpp: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_INPUT_RESET {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub RedrawRect: RFX_GFX_RECT,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {}
impl ::core::clone::Clone for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub DisconnectReason: u32,
}
impl ::core::marker::Copy for RFX_GFX_MSG_DISCONNECT_NOTIFY {}
impl ::core::clone::Clone for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_MSG_HEADER {
    pub uMSGType: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for RFX_GFX_MSG_HEADER {}
impl ::core::clone::Clone for RFX_GFX_MSG_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
#[repr(C)]
pub struct RFX_GFX_MSG_RDP_DATA {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub rdpData: [u8; 1],
}
impl ::core::marker::Copy for RFX_GFX_MSG_RDP_DATA {}
impl ::core::clone::Clone for RFX_GFX_MSG_RDP_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct RFX_GFX_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::core::marker::Copy for RFX_GFX_RECT {}
impl ::core::clone::Clone for RFX_GFX_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
pub const RemoteActionCharms: i32 = 0i32;
pub const RemoteActionAppbar: i32 = 1i32;
pub const RemoteActionSnap: i32 = 2i32;
pub const RemoteActionStartScreen: i32 = 3i32;
pub const RemoteActionAppSwitch: i32 = 4i32;
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: i32 = 0i32;
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: i32 = 1i32;
pub const SINGLE_SESSION: u32 = 1u32;
pub const SnapshotEncodingDataUri: i32 = 0i32;
pub const SnapshotFormatPng: i32 = 0i32;
pub const SnapshotFormatJpeg: i32 = 1i32;
pub const SnapshotFormatBmp: i32 = 2i32;
pub const TARGET_CHANGE_UNSPEC: i32 = 1i32;
pub const TARGET_EXTERNALIP_CHANGED: i32 = 2i32;
pub const TARGET_INTERNALIP_CHANGED: i32 = 4i32;
pub const TARGET_JOINED: i32 = 8i32;
pub const TARGET_REMOVED: i32 = 16i32;
pub const TARGET_STATE_CHANGED: i32 = 32i32;
pub const TARGET_IDLE: i32 = 64i32;
pub const TARGET_PENDING: i32 = 128i32;
pub const TARGET_INUSE: i32 = 256i32;
pub const TARGET_PATCH_STATE_CHANGED: i32 = 512i32;
pub const TARGET_FARM_MEMBERSHIP_CHANGED: i32 = 1024i32;
pub const OWNER_UNKNOWN: i32 = 0i32;
pub const OWNER_MS_TS_PLUGIN: i32 = 1i32;
pub const OWNER_MS_VM_PLUGIN: i32 = 2i32;
pub const TARGET_PATCH_UNKNOWN: i32 = 0i32;
pub const TARGET_PATCH_NOT_STARTED: i32 = 1i32;
pub const TARGET_PATCH_IN_PROGRESS: i32 = 2i32;
pub const TARGET_PATCH_COMPLETED: i32 = 3i32;
pub const TARGET_PATCH_FAILED: i32 = 4i32;
pub const TARGET_UNKNOWN: i32 = 1i32;
pub const TARGET_INITIALIZING: i32 = 2i32;
pub const TARGET_RUNNING: i32 = 3i32;
pub const TARGET_DOWN: i32 = 4i32;
pub const TARGET_HIBERNATED: i32 = 5i32;
pub const TARGET_CHECKED_OUT: i32 = 6i32;
pub const TARGET_STOPPED: i32 = 7i32;
pub const TARGET_INVALID: i32 = 8i32;
pub const TARGET_STARTING: i32 = 9i32;
pub const TARGET_STOPPING: i32 = 10i32;
pub const TARGET_MAXSTATE: i32 = 11i32;
pub const UNKNOWN: i32 = 0i32;
pub const FARM: i32 = 1i32;
pub const NONFARM: i32 = 2i32;
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: i32 = 0i32;
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: i32 = 1i32;
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: i32 = 0i32;
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: i32 = 1i32;
pub const TSSB_NOTIFY_INVALID: i32 = 0i32;
pub const TSSB_NOTIFY_TARGET_CHANGE: i32 = 1i32;
pub const TSSB_NOTIFY_SESSION_CHANGE: i32 = 2i32;
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: i32 = 4i32;
pub const TSSD_ADDR_UNDEFINED: i32 = 0i32;
pub const TSSD_ADDR_IPv4: i32 = 4i32;
pub const TSSD_ADDR_IPv6: i32 = 6i32;
#[repr(C)]
pub struct TSSD_ConnectionPoint {
    pub ServerAddressB: [u8; 16],
    pub AddressType: TSSD_AddrV46Type,
    pub PortNumber: u16,
    pub AddressScope: u32,
}
impl ::core::marker::Copy for TSSD_ConnectionPoint {}
impl ::core::clone::Clone for TSSD_ConnectionPoint {
    fn clone(&self) -> Self {
        *self
    }
}
pub const STATE_INVALID: i32 = -1i32;
pub const STATE_ACTIVE: i32 = 0i32;
pub const STATE_CONNECTED: i32 = 1i32;
pub const STATE_CONNECTQUERY: i32 = 2i32;
pub const STATE_SHADOW: i32 = 3i32;
pub const STATE_DISCONNECTED: i32 = 4i32;
pub const STATE_IDLE: i32 = 5i32;
pub const STATE_LISTEN: i32 = 6i32;
pub const STATE_RESET: i32 = 7i32;
pub const STATE_DOWN: i32 = 8i32;
pub const STATE_INIT: i32 = 9i32;
pub const STATE_MAX: i32 = 10i32;
pub const TSUserExInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 152100097, data2: 57228, data3: 4561, data4: [174, 39, 0, 192, 79, 163, 88, 19] };
pub const TS_SB_SORT_BY_NONE: i32 = 0i32;
pub const TS_SB_SORT_BY_NAME: i32 = 1i32;
pub const TS_SB_SORT_BY_PROP: i32 = 2i32;
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
pub const USERNAME_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
pub const VM_HOST_STATUS_INIT_PENDING: i32 = 0i32;
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: i32 = 1i32;
pub const VM_HOST_STATUS_INIT_COMPLETE: i32 = 2i32;
pub const VM_HOST_STATUS_INIT_FAILED: i32 = 3i32;
#[repr(C)]
pub struct VM_NOTIFY_ENTRY {
    pub VmName: [u16; 128],
    pub VmHost: [u16; 128],
}
impl ::core::marker::Copy for VM_NOTIFY_ENTRY {}
impl ::core::clone::Clone for VM_NOTIFY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl ::core::marker::Copy for VM_NOTIFY_INFO {}
impl ::core::clone::Clone for VM_NOTIFY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VM_NOTIFY_STATUS_PENDING: i32 = 0i32;
pub const VM_NOTIFY_STATUS_IN_PROGRESS: i32 = 1i32;
pub const VM_NOTIFY_STATUS_COMPLETE: i32 = 2i32;
pub const VM_NOTIFY_STATUS_FAILED: i32 = 3i32;
pub const VM_NOTIFY_STATUS_CANCELED: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VM_PATCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VM_PATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_CONNECTION_SETTING {
    pub WRdsConnectionSettings1: WRDS_CONNECTION_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS {
    pub WRdsConnectionSettingLevel: WRDS_CONNECTION_SETTING_LEVEL,
    pub WRdsConnectionSetting: WRDS_CONNECTION_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_CONNECTION_SETTINGS_1 {
    pub fInheritInitialProgram: super::super::Foundation::BOOLEAN,
    pub fInheritColorDepth: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOLEAN,
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fResetBroken: super::super::Foundation::BOOLEAN,
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub ColorDepth: u16,
    pub ProtocolType: u16,
    pub HRes: u16,
    pub VRes: u16,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub KeyboardLayout: u32,
    pub MaxConnectionTime: u32,
    pub MaxDisconnectionTime: u32,
    pub MaxIdleTime: u32,
    pub PerformanceFlags: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub ActiveInputLocale: u32,
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientBuildNumber: u32,
    pub ClientSessionId: u32,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserName: [u16; 256],
    pub Domain: [u16; 256],
    pub Password: [u16; 256],
    pub ProtocolName: [u16; 9],
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub imeFileName: [u16; 33],
    pub AudioDriverName: [u16; 9],
    pub ClientName: [u16; 21],
    pub ClientAddress: [u16; 31],
    pub ClientDirectory: [u16; 257],
    pub ClientDigProductId: [u16; 33],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub WRdsListenerSettings: WRDS_LISTENER_SETTINGS,
    pub EventLogActivityId: ::windows_sys::core::GUID,
    pub ContextSize: u32,
    pub ContextData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_CONNECTION_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_CONNECTION_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: i32 = 0i32;
pub const WRDS_CONNECTION_SETTING_LEVEL_1: i32 = 1i32;
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
pub struct WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: u16,
}
impl ::core::marker::Copy for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[repr(C)]
pub union WRDS_LISTENER_SETTING {
    pub WRdsListenerSettings1: WRDS_LISTENER_SETTINGS_1,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTING {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WRDS_LISTENER_SETTINGS {
    pub WRdsListenerSettingLevel: WRDS_LISTENER_SETTING_LEVEL,
    pub WRdsListenerSetting: WRDS_LISTENER_SETTING,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WRDS_LISTENER_SETTINGS_1 {
    pub MaxProtocolListenerConnectionCount: u32,
    pub SecurityDescriptorSize: u32,
    pub pSecurityDescriptor: *mut u8,
}
impl ::core::marker::Copy for WRDS_LISTENER_SETTINGS_1 {}
impl ::core::clone::Clone for WRDS_LISTENER_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WRDS_LISTENER_SETTING_LEVEL_INVALID: i32 = 0i32;
pub const WRDS_LISTENER_SETTING_LEVEL_1: i32 = 1i32;
pub const WRDS_MAX_CACHE_RESERVED: u32 = 20u32;
pub const WRDS_MAX_COUNTERS: u32 = 100u32;
pub const WRDS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
pub const WRDS_MAX_PROTOCOL_CACHE: u32 = 4u32;
pub const WRDS_MAX_RESERVED: u32 = 100u32;
pub const WRDS_PASSWORD_LENGTH: u32 = 255u32;
pub const WRDS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
pub const WRDS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
pub const WRDS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
pub const WRDS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
pub const WRDS_PERF_DISABLE_NOTHING: u32 = 0u32;
pub const WRDS_PERF_DISABLE_THEMING: u32 = 8u32;
pub const WRDS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
pub const WRDS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
pub const WRDS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
pub const WRDS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
pub const WRDS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3533258573, data2: 719, data3: 17024, data4: [140, 72, 22, 36, 180, 79, 135, 6] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WRDS_SETTING {
    pub WRdsSettings1: WRDS_SETTINGS_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS {
    pub WRdsSettingType: WRDS_SETTING_TYPE,
    pub WRdsSettingLevel: WRDS_SETTING_LEVEL,
    pub WRdsSetting: WRDS_SETTING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WRDS_SETTINGS_1 {
    pub WRdsDisableClipStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableClipValue: u32,
    pub WRdsDisableLPTStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableLPTValue: u32,
    pub WRdsDisableCcmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCcmValue: u32,
    pub WRdsDisableCdmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCdmValue: u32,
    pub WRdsDisableCpmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCpmValue: u32,
    pub WRdsDisablePnpStatus: WRDS_SETTING_STATUS,
    pub WRdsDisablePnpValue: u32,
    pub WRdsEncryptionLevelStatus: WRDS_SETTING_STATUS,
    pub WRdsEncryptionValue: u32,
    pub WRdsColorDepthStatus: WRDS_SETTING_STATUS,
    pub WRdsColorDepthValue: u32,
    pub WRdsDisableAutoReconnecetStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableAutoReconnecetValue: u32,
    pub WRdsDisableEncryptionStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableEncryptionValue: u32,
    pub WRdsResetBrokenStatus: WRDS_SETTING_STATUS,
    pub WRdsResetBrokenValue: u32,
    pub WRdsMaxIdleTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxIdleTimeValue: u32,
    pub WRdsMaxDisconnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxDisconnectTimeValue: u32,
    pub WRdsMaxConnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxConnectTimeValue: u32,
    pub WRdsKeepAliveStatus: WRDS_SETTING_STATUS,
    pub WRdsKeepAliveStartValue: super::super::Foundation::BOOLEAN,
    pub WRdsKeepAliveIntervalValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WRDS_SETTINGS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WRDS_SETTING_LEVEL_INVALID: i32 = 0i32;
pub const WRDS_SETTING_LEVEL_1: i32 = 1i32;
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: i32 = -1i32;
pub const WRDS_SETTING_STATUS_DISABLED: i32 = 0i32;
pub const WRDS_SETTING_STATUS_ENABLED: i32 = 1i32;
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: i32 = 2i32;
pub const WRDS_SETTING_TYPE_INVALID: i32 = 0i32;
pub const WRDS_SETTING_TYPE_MACHINE: i32 = 1i32;
pub const WRDS_SETTING_TYPE_USER: i32 = 2i32;
pub const WRDS_SETTING_TYPE_SAM: i32 = 3i32;
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
pub const WRdsGraphicsChannelType_GuaranteedDelivery: i32 = 0i32;
pub const WRdsGraphicsChannelType_BestEffortDelivery: i32 = 1i32;
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCLIENTA {
    pub ClientName: [super::super::Foundation::CHAR; 21],
    pub Domain: [super::super::Foundation::CHAR; 18],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [super::super::Foundation::CHAR; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSCLIENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSCLIENTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for WTSCLIENTW {}
impl ::core::clone::Clone for WTSCLIENTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSCONFIGINFOA {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [super::super::Foundation::CHAR; 21],
    pub LogonDomain: [super::super::Foundation::CHAR; 18],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub ApplicationName: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSCONFIGINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSCONFIGINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for WTSCONFIGINFOW {}
impl ::core::clone::Clone for WTSCONFIGINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOA {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBy: u32,
    pub WinStationName: [super::super::Foundation::CHAR; 32],
    pub Domain: [super::super::Foundation::CHAR; 17],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEXA {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_A,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTSINFOEXW {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_W,
}
impl ::core::marker::Copy for WTSINFOEXW {}
impl ::core::clone::Clone for WTSINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSINFOEX_LEVEL1_A {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [super::super::Foundation::CHAR; 33],
    pub UserName: [super::super::Foundation::CHAR; 21],
    pub DomainName: [super::super::Foundation::CHAR; 18],
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for WTSINFOEX_LEVEL1_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSINFOEX_LEVEL_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSINFOEX_LEVEL_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
impl ::core::marker::Copy for WTSINFOEX_LEVEL_W {}
impl ::core::clone::Clone for WTSINFOEX_LEVEL_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for WTSINFOW {}
impl ::core::clone::Clone for WTSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub Comment: [super::super::Foundation::CHAR; 61],
    pub LogonUserName: [super::super::Foundation::CHAR; 21],
    pub LogonDomain: [super::super::Foundation::CHAR; 18],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSLISTENERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSLISTENERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for WTSLISTENERCONFIGW {}
impl ::core::clone::Clone for WTSLISTENERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: i32 = 0i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: i32 = 1i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: i32 = 2i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: i32 = 3i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: i32 = 4i32;
#[repr(C)]
pub struct WTSSBX_IP_ADDRESS {
    pub AddressFamily: WTSSBX_ADDRESS_FAMILY,
    pub Address: [u8; 16],
    pub PortNumber: u16,
    pub dwScope: u32,
}
impl ::core::marker::Copy for WTSSBX_IP_ADDRESS {}
impl ::core::clone::Clone for WTSSBX_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTSSBX_MACHINE_CONNECT_INFO {
    pub wczMachineFQDN: [u16; 257],
    pub wczMachineNetBiosName: [u16; 17],
    pub dwNumOfIPAddr: u32,
    pub IPaddr: [WTSSBX_IP_ADDRESS; 12],
}
impl ::core::marker::Copy for WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_CONNECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: i32 = 0i32;
pub const WTSSBX_MACHINE_DRAIN_OFF: i32 = 1i32;
pub const WTSSBX_MACHINE_DRAIN_ON: i32 = 2i32;
#[repr(C)]
pub struct WTSSBX_MACHINE_INFO {
    pub ClientConnectInfo: WTSSBX_MACHINE_CONNECT_INFO,
    pub wczFarmName: [u16; 257],
    pub InternalIPAddress: WTSSBX_IP_ADDRESS,
    pub dwMaxSessionsLimit: u32,
    pub ServerWeight: u32,
    pub SingleSessionMode: WTSSBX_MACHINE_SESSION_MODE,
    pub InDrain: WTSSBX_MACHINE_DRAIN,
    pub MachineState: WTSSBX_MACHINE_STATE,
}
impl ::core::marker::Copy for WTSSBX_MACHINE_INFO {}
impl ::core::clone::Clone for WTSSBX_MACHINE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: i32 = 0i32;
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: i32 = 1i32;
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: i32 = 2i32;
pub const WTSSBX_MACHINE_STATE_UNSPEC: i32 = 0i32;
pub const WTSSBX_MACHINE_STATE_READY: i32 = 1i32;
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: i32 = 2i32;
pub const WTSSBX_NOTIFICATION_REMOVED: i32 = 1i32;
pub const WTSSBX_NOTIFICATION_CHANGED: i32 = 2i32;
pub const WTSSBX_NOTIFICATION_ADDED: i32 = 4i32;
pub const WTSSBX_NOTIFICATION_RESYNC: i32 = 8i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTSSBX_SESSION_INFO {
    pub wszUserName: [u16; 105],
    pub wszDomainName: [u16; 257],
    pub ApplicationType: [u16; 257],
    pub dwSessionId: u32,
    pub CreateTime: super::super::Foundation::FILETIME,
    pub DisconnectTime: super::super::Foundation::FILETIME,
    pub SessionState: WTSSBX_SESSION_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSSBX_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTSSBX_SESSION_STATE_UNSPEC: i32 = 0i32;
pub const WTSSBX_SESSION_STATE_ACTIVE: i32 = 1i32;
pub const WTSSBX_SESSION_STATE_DISCONNECTED: i32 = 2i32;
#[repr(C)]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
impl ::core::marker::Copy for WTSSESSION_NOTIFICATION {}
impl ::core::clone::Clone for WTSSESSION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub InitialProgram: [super::super::Foundation::CHAR; 261],
    pub WorkDirectory: [super::super::Foundation::CHAR; 261],
    pub TerminalServerProfilePath: [super::super::Foundation::CHAR; 261],
    pub TerminalServerHomeDir: [super::super::Foundation::CHAR; 261],
    pub TerminalServerHomeDirDrive: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTSUSERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTSUSERCONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for WTSUSERCONFIGW {}
impl ::core::clone::Clone for WTSUSERCONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_CACHE_STATS {
    pub Specific: u32,
    pub Data: WTS_CACHE_STATS_UN,
    pub ProtocolType: u16,
    pub Length: u16,
}
impl ::core::marker::Copy for WTS_CACHE_STATS {}
impl ::core::clone::Clone for WTS_CACHE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WTS_CACHE_STATS_UN {
    pub ProtocolCache: [WTS_PROTOCOL_CACHE; 4],
    pub TShareCacheStats: u32,
    pub Reserved: [u32; 20],
}
impl ::core::marker::Copy for WTS_CACHE_STATS_UN {}
impl ::core::clone::Clone for WTS_CACHE_STATS_UN {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_CERT_TYPE_INVALID: i32 = 0i32;
pub const WTS_CERT_TYPE_PROPRIETORY: i32 = 1i32;
pub const WTS_CERT_TYPE_X509: i32 = 2i32;
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_CLIENT_ADDRESS {}
impl ::core::clone::Clone for WTS_CLIENT_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_CLIENT_DATA {
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOL,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub Domain: [u16; 256],
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fInheritInitialProgram: super::super::Foundation::BOOL,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub PerformanceFlags: u32,
    pub ProtocolName: [u16; 9],
    pub ProtocolType: u16,
    pub fInheritColorDepth: super::super::Foundation::BOOL,
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub KeyboardLayout: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub imeFileName: [u16; 33],
    pub ActiveInputLocale: u32,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
    pub AudioDriverName: [u16; 9],
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub ClientName: [u16; 21],
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientDirectory: [u16; 257],
    pub ClientBuildNumber: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub ClientSessionId: u32,
    pub ClientDigProductId: [u16; 33],
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_CLIENT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_CLIENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl ::core::marker::Copy for WTS_CLIENT_DISPLAY {}
impl ::core::clone::Clone for WTS_CLIENT_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
pub const WTSUserConfigInitialProgram: i32 = 0i32;
pub const WTSUserConfigWorkingDirectory: i32 = 1i32;
pub const WTSUserConfigfInheritInitialProgram: i32 = 2i32;
pub const WTSUserConfigfAllowLogonTerminalServer: i32 = 3i32;
pub const WTSUserConfigTimeoutSettingsConnections: i32 = 4i32;
pub const WTSUserConfigTimeoutSettingsDisconnections: i32 = 5i32;
pub const WTSUserConfigTimeoutSettingsIdle: i32 = 6i32;
pub const WTSUserConfigfDeviceClientDrives: i32 = 7i32;
pub const WTSUserConfigfDeviceClientPrinters: i32 = 8i32;
pub const WTSUserConfigfDeviceClientDefaultPrinter: i32 = 9i32;
pub const WTSUserConfigBrokenTimeoutSettings: i32 = 10i32;
pub const WTSUserConfigReconnectSettings: i32 = 11i32;
pub const WTSUserConfigModemCallbackSettings: i32 = 12i32;
pub const WTSUserConfigModemCallbackPhoneNumber: i32 = 13i32;
pub const WTSUserConfigShadowingSettings: i32 = 14i32;
pub const WTSUserConfigTerminalServerProfilePath: i32 = 15i32;
pub const WTSUserConfigTerminalServerHomeDir: i32 = 16i32;
pub const WTSUserConfigTerminalServerHomeDirDrive: i32 = 17i32;
pub const WTSUserConfigfTerminalServerRemoteHomeDir: i32 = 18i32;
pub const WTSUserConfigUser: i32 = 19i32;
pub const WTSUserConfigSourceSAM: i32 = 0i32;
pub const WTSActive: i32 = 0i32;
pub const WTSConnected: i32 = 1i32;
pub const WTSConnectQuery: i32 = 2i32;
pub const WTSShadow: i32 = 3i32;
pub const WTSDisconnected: i32 = 4i32;
pub const WTSIdle: i32 = 5i32;
pub const WTSListen: i32 = 6i32;
pub const WTSReset: i32 = 7i32;
pub const WTSDown: i32 = 8i32;
pub const WTSInit: i32 = 9i32;
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
#[repr(C)]
pub struct WTS_DISPLAY_IOCTL {
    pub pDisplayIOCtlData: [u8; 256],
    pub cbDisplayIOCtlData: u32,
}
impl ::core::marker::Copy for WTS_DISPLAY_IOCTL {}
impl ::core::clone::Clone for WTS_DISPLAY_IOCTL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_DOMAIN_LENGTH: u32 = 255u32;
pub const WTS_DRIVER_NAME_LENGTH: u32 = 8u32;
pub const WTS_DRIVE_LENGTH: u32 = 3u32;
pub const WTS_EVENT_ALL: u32 = 2147483647u32;
pub const WTS_EVENT_CONNECT: u32 = 8u32;
pub const WTS_EVENT_CREATE: u32 = 1u32;
pub const WTS_EVENT_DELETE: u32 = 2u32;
pub const WTS_EVENT_DISCONNECT: u32 = 16u32;
pub const WTS_EVENT_FLUSH: u32 = 2147483648u32;
pub const WTS_EVENT_LICENSE: u32 = 256u32;
pub const WTS_EVENT_LOGOFF: u32 = 64u32;
pub const WTS_EVENT_LOGON: u32 = 32u32;
pub const WTS_EVENT_NONE: u32 = 0u32;
pub const WTS_EVENT_RENAME: u32 = 4u32;
pub const WTS_EVENT_STATECHANGE: u32 = 128u32;
pub const WTS_IMEFILENAME_LENGTH: u32 = 32u32;
pub const WTSInitialProgram: i32 = 0i32;
pub const WTSApplicationName: i32 = 1i32;
pub const WTSWorkingDirectory: i32 = 2i32;
pub const WTSOEMId: i32 = 3i32;
pub const WTSSessionId: i32 = 4i32;
pub const WTSUserName: i32 = 5i32;
pub const WTSWinStationName: i32 = 6i32;
pub const WTSDomainName: i32 = 7i32;
pub const WTSConnectState: i32 = 8i32;
pub const WTSClientBuildNumber: i32 = 9i32;
pub const WTSClientName: i32 = 10i32;
pub const WTSClientDirectory: i32 = 11i32;
pub const WTSClientProductId: i32 = 12i32;
pub const WTSClientHardwareId: i32 = 13i32;
pub const WTSClientAddress: i32 = 14i32;
pub const WTSClientDisplay: i32 = 15i32;
pub const WTSClientProtocolType: i32 = 16i32;
pub const WTSIdleTime: i32 = 17i32;
pub const WTSLogonTime: i32 = 18i32;
pub const WTSIncomingBytes: i32 = 19i32;
pub const WTSOutgoingBytes: i32 = 20i32;
pub const WTSIncomingFrames: i32 = 21i32;
pub const WTSOutgoingFrames: i32 = 22i32;
pub const WTSClientInfo: i32 = 23i32;
pub const WTSSessionInfo: i32 = 24i32;
pub const WTSSessionInfoEx: i32 = 25i32;
pub const WTSConfigInfo: i32 = 26i32;
pub const WTSValidationInfo: i32 = 27i32;
pub const WTSSessionAddressV4: i32 = 28i32;
pub const WTSIsRemoteSession: i32 = 29i32;
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: super::super::Foundation::BOOL,
    pub CertType: WTS_CERT_TYPE,
    pub cbClientName: u32,
    pub rgbClientName: [u8; 42],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_LICENSE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
pub const WTS_LISTENER_CREATE: u32 = 1u32;
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
pub const WTS_LOGON_ERR_INVALID: i32 = 0i32;
pub const WTS_LOGON_ERR_NOT_HANDLED: i32 = 1i32;
pub const WTS_LOGON_ERR_HANDLED_SHOW: i32 = 2i32;
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW: i32 = 3i32;
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER: i32 = 4i32;
pub const WTS_MAX_CACHE_RESERVED: u32 = 20u32;
pub const WTS_MAX_COUNTERS: u32 = 100u32;
pub const WTS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
pub const WTS_MAX_PROTOCOL_CACHE: u32 = 4u32;
pub const WTS_MAX_RESERVED: u32 = 100u32;
pub const WTS_PASSWORD_LENGTH: u32 = 255u32;
pub const WTS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
pub const WTS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
pub const WTS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
pub const WTS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
pub const WTS_PERF_DISABLE_NOTHING: u32 = 0u32;
pub const WTS_PERF_DISABLE_THEMING: u32 = 8u32;
pub const WTS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
pub const WTS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
pub const WTS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
pub const WTS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_POLICY_DATA {
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub ColorDepth: u32,
    pub MinEncryptionLevel: u8,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNPRedir: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_POLICY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: super::super::Foundation::PWSTR,
    pub pUserSid: super::super::Foundation::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROCESS_INFO_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROPERTY_VALUE {
    pub Type: u16,
    pub u: WTS_PROPERTY_VALUE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WTS_PROPERTY_VALUE_0 {
    pub ulVal: u32,
    pub strVal: WTS_PROPERTY_VALUE_0_1,
    pub bVal: WTS_PROPERTY_VALUE_0_0,
    pub guidVal: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pbVal: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pstrVal: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_PROPERTY_VALUE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_PROPERTY_VALUE_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
impl ::core::marker::Copy for WTS_PROTOCOL_CACHE {}
impl ::core::clone::Clone for WTS_PROTOCOL_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_PROTOCOL_COUNTERS {
    pub WdBytes: u32,
    pub WdFrames: u32,
    pub WaitForOutBuf: u32,
    pub Frames: u32,
    pub Bytes: u32,
    pub CompressedBytes: u32,
    pub CompressFlushes: u32,
    pub Errors: u32,
    pub Timeouts: u32,
    pub AsyncFramingError: u32,
    pub AsyncOverrunError: u32,
    pub AsyncOverflowError: u32,
    pub AsyncParityError: u32,
    pub TdErrors: u32,
    pub ProtocolType: u16,
    pub Length: u16,
    pub Specific: u16,
    pub Reserved: [u32; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_COUNTERS {}
impl ::core::clone::Clone for WTS_PROTOCOL_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
pub struct WTS_PROTOCOL_STATUS {
    pub Output: WTS_PROTOCOL_COUNTERS,
    pub Input: WTS_PROTOCOL_COUNTERS,
    pub Cache: WTS_CACHE_STATS,
    pub AsyncSignal: u32,
    pub AsyncSignalMask: u32,
    pub Counters: [i64; 100],
}
impl ::core::marker::Copy for WTS_PROTOCOL_STATUS {}
impl ::core::clone::Clone for WTS_PROTOCOL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3346864944,
    data2: 23521,
    data3: 19563,
    data4: [160, 225, 189, 109, 46, 92, 159, 204],
};
pub const WTS_QUERY_AUDIOENUM_DLL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2616523415, data2: 51331, data3: 19498, data4: [128, 171, 90, 57, 201, 175, 0, 219] };
pub const WTS_QUERY_LOGON_SCREEN_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2341343207, data2: 2052, data3: 18958, data4: [178, 121, 134, 96, 177, 223, 0, 73] };
pub const WTS_QUERY_MF_FORMAT_SUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1099340496,
    data2: 25394,
    data3: 19912,
    data4: [149, 213, 219, 116, 158, 47, 29, 148],
};
pub const WTS_DRAIN_STATE_NONE: i32 = 0i32;
pub const WTS_DRAIN_IN_DRAIN: i32 = 1i32;
pub const WTS_DRAIN_NOT_IN_DRAIN: i32 = 2i32;
pub const WTS_SERVICE_NONE: i32 = 0i32;
pub const WTS_SERVICE_START: i32 = 1i32;
pub const WTS_SERVICE_STOP: i32 = 2i32;
pub const WTS_SECURITY_CONNECT: u32 = 256u32;
pub const WTS_SECURITY_DISCONNECT: u32 = 512u32;
pub const WTS_SECURITY_GUEST_ACCESS: u32 = 32u32;
pub const WTS_SECURITY_LOGOFF: u32 = 64u32;
pub const WTS_SECURITY_LOGON: u32 = 32u32;
pub const WTS_SECURITY_MESSAGE: u32 = 128u32;
pub const WTS_SECURITY_QUERY_INFORMATION: u32 = 1u32;
pub const WTS_SECURITY_REMOTE_CONTROL: u32 = 16u32;
pub const WTS_SECURITY_RESET: u32 = 4u32;
pub const WTS_SECURITY_SET_INFORMATION: u32 = 2u32;
pub const WTS_SECURITY_VIRTUAL_CHANNELS: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SERVER_INFOA {
    pub pServerName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_SERVER_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_SERVER_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SERVER_INFOW {
    pub pServerName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_SERVER_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_SERVER_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
impl ::core::marker::Copy for WTS_SERVICE_STATE {}
impl ::core::clone::Clone for WTS_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
#[repr(C)]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for WTS_SESSION_ADDRESS {}
impl ::core::clone::Clone for WTS_SESSION_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: ::windows_sys::core::GUID,
    pub SessionId: u32,
}
impl ::core::marker::Copy for WTS_SESSION_ID {}
impl ::core::clone::Clone for WTS_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: super::super::Foundation::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_SESSION_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_SESSION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: super::super::Foundation::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_SESSION_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_SESSION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: super::super::Foundation::PSTR,
    pub pHostName: super::super::Foundation::PSTR,
    pub pUserName: super::super::Foundation::PSTR,
    pub pDomainName: super::super::Foundation::PSTR,
    pub pFarmName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_SESSION_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_SESSION_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: super::super::Foundation::PWSTR,
    pub pHostName: super::super::Foundation::PWSTR,
    pub pUserName: super::super::Foundation::PWSTR,
    pub pDomainName: super::super::Foundation::PWSTR,
    pub pFarmName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_SESSION_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_SESSION_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for WTS_SMALL_RECT {}
impl ::core::clone::Clone for WTS_SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_SOCKADDR {
    pub sin_family: u16,
    pub u: WTS_SOCKADDR_0,
}
impl ::core::marker::Copy for WTS_SOCKADDR {}
impl ::core::clone::Clone for WTS_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WTS_SOCKADDR_0 {
    pub ipv4: WTS_SOCKADDR_0_0,
    pub ipv6: WTS_SOCKADDR_0_1,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_SOCKADDR_0_0 {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_0 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_SOCKADDR_0_1 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for WTS_SOCKADDR_0_1 {}
impl ::core::clone::Clone for WTS_SOCKADDR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for WTS_SYSTEMTIME {}
impl ::core::clone::Clone for WTS_SYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
}
impl ::core::marker::Copy for WTS_TIME_ZONE_INFORMATION {}
impl ::core::clone::Clone for WTS_TIME_ZONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTSTypeProcessInfoLevel0: i32 = 0i32;
pub const WTSTypeProcessInfoLevel1: i32 = 1i32;
pub const WTSTypeSessionInfoLevel1: i32 = 2i32;
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
#[repr(C)]
pub struct WTS_USER_CREDENTIAL {
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub Domain: [u16; 256],
}
impl ::core::marker::Copy for WTS_USER_CREDENTIAL {}
impl ::core::clone::Clone for WTS_USER_CREDENTIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_USER_DATA {
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserTimeZone: WTS_TIME_ZONE_INFORMATION,
}
impl ::core::marker::Copy for WTS_USER_DATA {}
impl ::core::clone::Clone for WTS_USER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: _WTS_PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: _WTS_PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl ::core::marker::Copy for WTS_VALIDATION_INFORMATIONW {}
impl ::core::clone::Clone for WTS_VALIDATION_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
pub const WTSVirtualClientData: i32 = 0i32;
pub const WTSVirtualFileHandle: i32 = 1i32;
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
pub const WTS_WSD_LOGOFF: u32 = 1u32;
pub const WTS_WSD_POWEROFF: u32 = 8u32;
pub const WTS_WSD_REBOOT: u32 = 4u32;
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub const Workspace: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1327365286, data2: 15021, data3: 18657, data4: [132, 6, 75, 194, 26, 80, 29, 124] };
#[repr(transparent)]
pub struct _ITSWkspEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _ITSWkspEvents {}
impl ::core::clone::Clone for _ITSWkspEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct _WTS_PRODUCT_INFOA {
    pub CompanyName: [super::super::Foundation::CHAR; 256],
    pub ProductID: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _WTS_PRODUCT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _WTS_PRODUCT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _WTS_PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl ::core::marker::Copy for _WTS_PRODUCT_INFOW {}
impl ::core::clone::Clone for _WTS_PRODUCT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct pluginResource {
    pub alias: [u16; 256],
    pub name: [u16; 256],
    pub resourceFileContents: super::super::Foundation::PWSTR,
    pub fileExtension: [u16; 256],
    pub resourcePluginType: [u16; 256],
    pub isDiscoverable: u8,
    pub resourceType: i32,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
    pub pcePluginBlobSize: u32,
    pub blobContents: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for pluginResource {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for pluginResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: super::super::Foundation::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for pluginResource2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for pluginResource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct pluginResource2FileAssociation {
    pub extName: [u16; 256],
    pub primaryHandler: u8,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
}
impl ::core::marker::Copy for pluginResource2FileAssociation {}
impl ::core::clone::Clone for pluginResource2FileAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
