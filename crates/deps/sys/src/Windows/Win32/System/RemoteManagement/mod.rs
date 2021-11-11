#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCloseCommand();
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManCloseOperation();
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManCloseSession();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCloseShell();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManConnectShell();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManConnectShellCommand();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateSession();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateShell();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateShellEx();
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManDeinitialize();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManDisconnectShell();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManGetErrorMessage();
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManGetSessionOptionAsDword();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManGetSessionOptionAsString();
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManInitialize();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzOperationComplete();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzQueryQuotaComplete();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzUserComplete();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginFreeRequestDetails();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginGetConfiguration();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginGetOperationParameters();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginOperationComplete();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginReceiveResult();
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManPluginReportCompletion();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginReportContext();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManReceiveShellOutput();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManReconnectShell();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManReconnectShellCommand();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManRunShellCommand();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManRunShellCommandEx();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManSendShellInput();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManSetSessionOption();
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManSignalShell();
}
