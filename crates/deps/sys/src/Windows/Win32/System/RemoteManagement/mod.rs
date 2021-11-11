#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCloseCommand(commandhandle: *mut WSMAN_COMMAND, flags: u32, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>);
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManCloseOperation(operationhandle: *mut WSMAN_OPERATION, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManCloseSession(session: *mut WSMAN_SESSION, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCloseShell(shellhandle: *mut WSMAN_SHELL, flags: u32, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManConnectShell(session: *mut WSMAN_SESSION, flags: u32, resourceuri: super::super::Foundation::PWSTR, shellid: super::super::Foundation::PWSTR, options: *const WSMAN_OPTION_SET, connectxml: *const WSMAN_DATA, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, shell: *mut *mut WSMAN_SHELL);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManConnectShellCommand(shell: *mut WSMAN_SHELL, flags: u32, commandid: super::super::Foundation::PWSTR, options: *const WSMAN_OPTION_SET, connectxml: *const WSMAN_DATA, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, command: *mut *mut WSMAN_COMMAND);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateSession(apihandle: *const WSMAN_API, connection: super::super::Foundation::PWSTR, flags: u32, serverauthenticationcredentials: *const WSMAN_AUTHENTICATION_CREDENTIALS, proxyinfo: *const WSMAN_PROXY_INFO, session: *mut *mut WSMAN_SESSION) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateShell(session: *mut WSMAN_SESSION, flags: u32, resourceuri: super::super::Foundation::PWSTR, startupinfo: *const WSMAN_SHELL_STARTUP_INFO_V11, options: *const WSMAN_OPTION_SET, createxml: *const WSMAN_DATA, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, shell: *mut *mut WSMAN_SHELL);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateShellEx(session: *mut WSMAN_SESSION, flags: u32, resourceuri: super::super::Foundation::PWSTR, shellid: super::super::Foundation::PWSTR, startupinfo: *const WSMAN_SHELL_STARTUP_INFO_V11, options: *const WSMAN_OPTION_SET, createxml: *const WSMAN_DATA, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, shell: *mut *mut WSMAN_SHELL);
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManDeinitialize(apihandle: *mut WSMAN_API, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManDisconnectShell(shell: *mut WSMAN_SHELL, flags: u32, disconnectinfo: *const WSMAN_SHELL_DISCONNECT_INFO, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManGetErrorMessage(apihandle: *const WSMAN_API, flags: u32, languagecode: super::super::Foundation::PWSTR, errorcode: u32, messagelength: u32, message: super::super::Foundation::PWSTR, messagelengthused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManGetSessionOptionAsDword(session: *const WSMAN_SESSION, option: WSManSessionOption, value: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManGetSessionOptionAsString(session: *const WSMAN_SESSION, option: WSManSessionOption, stringlength: u32, string: super::super::Foundation::PWSTR, stringlengthused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManInitialize(flags: u32, apihandle: *mut *mut WSMAN_API) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzOperationComplete(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: *const ::core::ffi::c_void, errorcode: u32, extendederrorinformation: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzQueryQuotaComplete(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, quota: *const WSMAN_AUTHZ_QUOTA, errorcode: u32, extendederrorinformation: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzUserComplete(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: *const ::core::ffi::c_void, impersonationtoken: super::super::Foundation::HANDLE, userisadministrator: super::super::Foundation::BOOL, errorcode: u32, extendederrorinformation: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginFreeRequestDetails(requestdetails: *const WSMAN_PLUGIN_REQUEST) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginGetConfiguration(plugincontext: *const ::core::ffi::c_void, flags: u32, data: *mut WSMAN_DATA) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginGetOperationParameters(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, data: *mut WSMAN_DATA) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginOperationComplete(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, errorcode: u32, extendedinformation: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginReceiveResult(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, stream: super::super::Foundation::PWSTR, streamresult: *const WSMAN_DATA, commandstate: super::super::Foundation::PWSTR, exitcode: u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`*"]
    pub fn WSManPluginReportCompletion(plugincontext: *const ::core::ffi::c_void, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginReportContext(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, context: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManReceiveShellOutput(shell: *mut WSMAN_SHELL, command: *const WSMAN_COMMAND, flags: u32, desiredstreamset: *const WSMAN_STREAM_ID_SET, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, receiveoperation: *mut *mut WSMAN_OPERATION);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManReconnectShell(shell: *mut WSMAN_SHELL, flags: u32, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManReconnectShellCommand(commandhandle: *mut WSMAN_COMMAND, flags: u32, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManRunShellCommand(shell: *mut WSMAN_SHELL, flags: u32, commandline: super::super::Foundation::PWSTR, args: *const WSMAN_COMMAND_ARG_SET, options: *const WSMAN_OPTION_SET, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, command: *mut *mut WSMAN_COMMAND);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManRunShellCommandEx(shell: *mut WSMAN_SHELL, flags: u32, commandid: super::super::Foundation::PWSTR, commandline: super::super::Foundation::PWSTR, args: *const WSMAN_COMMAND_ARG_SET, options: *const WSMAN_OPTION_SET, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, command: *mut *mut WSMAN_COMMAND);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManSendShellInput(shell: *const WSMAN_SHELL, command: *const WSMAN_COMMAND, flags: u32, streamid: super::super::Foundation::PWSTR, streamdata: *const WSMAN_DATA, endofstream: super::super::Foundation::BOOL, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, sendoperation: *mut *mut WSMAN_OPERATION);
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManSetSessionOption(session: *const WSMAN_SESSION, option: WSManSessionOption, data: *const WSMAN_DATA) -> u32;
    #[doc = "*Required features: `Win32_System_RemoteManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManSignalShell(shell: *const WSMAN_SHELL, command: *const WSMAN_COMMAND, flags: u32, code: super::super::Foundation::PWSTR, r#async: *const ::core::mem::ManuallyDrop<WSMAN_SHELL_ASYNC>, signaloperation: *mut *mut WSMAN_OPERATION);
}
