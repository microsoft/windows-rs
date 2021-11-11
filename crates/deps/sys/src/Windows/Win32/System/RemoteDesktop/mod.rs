#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessIdToSessionId();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCloseServer();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSConnectSessionW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSCreateListenerW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSDisconnectSession();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnableChildSessions();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateListenersW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesExW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateProcessesW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateServersW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsExW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSEnumerateSessionsW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub fn WTSFreeMemory();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSFreeMemoryExW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`*"]
    pub fn WTSGetActiveConsoleSessionId();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSGetChildSessionId();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSGetListenerSecurityW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSIsChildSessionsEnabled();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSLogoffSession();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerExW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSOpenServerW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryListenerConfigW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQuerySessionInformationW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserConfigW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSQueryUserToken();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotification();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSRegisterSessionNotificationEx();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn WTSSendMessageW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WTSSetListenerSecurityW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetRenderHint();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSSetUserConfigW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSShutdownSystem();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionA();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStartRemoteControlSessionW();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSStopRemoteControlSession();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSTerminateProcess();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotification();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSUnRegisterSessionNotificationEx();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelClose();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelOpen();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelOpenEx();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeInput();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelPurgeOutput();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelQuery();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelRead();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSVirtualChannelWrite();
    #[doc = "*Required features: `Win32_System_RemoteDesktop`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WTSWaitSystemEvent();
}
