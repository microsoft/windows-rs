#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManCloseCommand(commandhandle: *mut WSMAN_COMMAND, flags: u32, r#async: *const WSMAN_SHELL_ASYNC);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManCloseOperation(operationhandle: *mut WSMAN_OPERATION, flags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManCloseSession(session: *mut WSMAN_SESSION, flags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManCloseShell(shellhandle: *mut WSMAN_SHELL, flags: u32, r#async: *const WSMAN_SHELL_ASYNC);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManConnectShell(session: *mut WSMAN_SESSION, flags: u32, resourceuri: ::windows_sys::core::PCWSTR, shellid: ::windows_sys::core::PCWSTR, options: *const WSMAN_OPTION_SET, connectxml: *const WSMAN_DATA, r#async: *const WSMAN_SHELL_ASYNC, shell: *mut *mut WSMAN_SHELL);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManConnectShellCommand(shell: *mut WSMAN_SHELL, flags: u32, commandid: ::windows_sys::core::PCWSTR, options: *const WSMAN_OPTION_SET, connectxml: *const WSMAN_DATA, r#async: *const WSMAN_SHELL_ASYNC, command: *mut *mut WSMAN_COMMAND);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManCreateSession(apihandle: *const WSMAN_API, connection: ::windows_sys::core::PCWSTR, flags: u32, serverauthenticationcredentials: *const WSMAN_AUTHENTICATION_CREDENTIALS, proxyinfo: *const WSMAN_PROXY_INFO, session: *mut *mut WSMAN_SESSION) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateShell(session: *mut WSMAN_SESSION, flags: u32, resourceuri: ::windows_sys::core::PCWSTR, startupinfo: *const WSMAN_SHELL_STARTUP_INFO_V11, options: *const WSMAN_OPTION_SET, createxml: *const WSMAN_DATA, r#async: *const WSMAN_SHELL_ASYNC, shell: *mut *mut WSMAN_SHELL);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManCreateShellEx(session: *mut WSMAN_SESSION, flags: u32, resourceuri: ::windows_sys::core::PCWSTR, shellid: ::windows_sys::core::PCWSTR, startupinfo: *const WSMAN_SHELL_STARTUP_INFO_V11, options: *const WSMAN_OPTION_SET, createxml: *const WSMAN_DATA, r#async: *const WSMAN_SHELL_ASYNC, shell: *mut *mut WSMAN_SHELL);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManDeinitialize(apihandle: *mut WSMAN_API, flags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManDisconnectShell(shell: *mut WSMAN_SHELL, flags: u32, disconnectinfo: *const WSMAN_SHELL_DISCONNECT_INFO, r#async: *const WSMAN_SHELL_ASYNC);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManGetErrorMessage(apihandle: *const WSMAN_API, flags: u32, languagecode: ::windows_sys::core::PCWSTR, errorcode: u32, messagelength: u32, message: ::windows_sys::core::PWSTR, messagelengthused: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManGetSessionOptionAsDword(session: *const WSMAN_SESSION, option: WSManSessionOption, value: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManGetSessionOptionAsString(session: *const WSMAN_SESSION, option: WSManSessionOption, stringlength: u32, string: ::windows_sys::core::PWSTR, stringlengthused: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManInitialize(flags: u32, apihandle: *mut *mut WSMAN_API) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzOperationComplete(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: *const ::core::ffi::c_void, errorcode: u32, extendederrorinformation: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzQueryQuotaComplete(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, quota: *const WSMAN_AUTHZ_QUOTA, errorcode: u32, extendederrorinformation: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginAuthzUserComplete(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: *const ::core::ffi::c_void, impersonationtoken: super::super::Foundation::HANDLE, userisadministrator: super::super::Foundation::BOOL, errorcode: u32, extendederrorinformation: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginFreeRequestDetails(requestdetails: *const WSMAN_PLUGIN_REQUEST) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManPluginGetConfiguration(plugincontext: *const ::core::ffi::c_void, flags: u32, data: *mut WSMAN_DATA) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginGetOperationParameters(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, data: *mut WSMAN_DATA) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginOperationComplete(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, errorcode: u32, extendedinformation: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginReceiveResult(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, stream: ::windows_sys::core::PCWSTR, streamresult: *const WSMAN_DATA, commandstate: ::windows_sys::core::PCWSTR, exitcode: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManPluginReportCompletion(plugincontext: *const ::core::ffi::c_void, flags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManPluginReportContext(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, context: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManReceiveShellOutput(shell: *mut WSMAN_SHELL, command: *const WSMAN_COMMAND, flags: u32, desiredstreamset: *const WSMAN_STREAM_ID_SET, r#async: *const WSMAN_SHELL_ASYNC, receiveoperation: *mut *mut WSMAN_OPERATION);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManReconnectShell(shell: *mut WSMAN_SHELL, flags: u32, r#async: *const WSMAN_SHELL_ASYNC);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManReconnectShellCommand(commandhandle: *mut WSMAN_COMMAND, flags: u32, r#async: *const WSMAN_SHELL_ASYNC);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManRunShellCommand(shell: *mut WSMAN_SHELL, flags: u32, commandline: ::windows_sys::core::PCWSTR, args: *const WSMAN_COMMAND_ARG_SET, options: *const WSMAN_OPTION_SET, r#async: *const WSMAN_SHELL_ASYNC, command: *mut *mut WSMAN_COMMAND);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManRunShellCommandEx(shell: *mut WSMAN_SHELL, flags: u32, commandid: ::windows_sys::core::PCWSTR, commandline: ::windows_sys::core::PCWSTR, args: *const WSMAN_COMMAND_ARG_SET, options: *const WSMAN_OPTION_SET, r#async: *const WSMAN_SHELL_ASYNC, command: *mut *mut WSMAN_COMMAND);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSManSendShellInput(shell: *const WSMAN_SHELL, command: *const WSMAN_COMMAND, flags: u32, streamid: ::windows_sys::core::PCWSTR, streamdata: *const WSMAN_DATA, endofstream: super::super::Foundation::BOOL, r#async: *const WSMAN_SHELL_ASYNC, sendoperation: *mut *mut WSMAN_OPERATION);
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManSetSessionOption(session: *const WSMAN_SESSION, option: WSManSessionOption, data: *const WSMAN_DATA) -> u32;
    #[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
    pub fn WSManSignalShell(shell: *const WSMAN_SHELL, command: *const WSMAN_COMMAND, flags: u32, code: ::windows_sys::core::PCWSTR, r#async: *const WSMAN_SHELL_ASYNC, signaloperation: *mut *mut WSMAN_OPERATION);
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_REDIRECT_LOCATION_INVALID: u32 = 2150859191u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_REDIRECT_LOCATION_TOO_LONG: u32 = 2150859190u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_SERVICE_CBT_HARDENING_INVALID: u32 = 2150859192u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_CLOSERECEIVEHANDLE_NULL_PARAM: u32 = 2150859058u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_CLOSESENDHANDLE_NULL_PARAM: u32 = 2150859061u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_CLOSESHELL_NULL_PARAM: u32 = 2150859050u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_CREATESHELL_NULL_PARAM: u32 = 2150859049u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_FREECREATESHELLRESULT_NULL_PARAM: u32 = 2150859051u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_FREEPULLRESULT_NULL_PARAM: u32 = 2150859056u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_FREERUNCOMMANDRESULT_NULL_PARAM: u32 = 2150859053u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_GET_NULL_PARAM: u32 = 2150859062u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_INVALID_FLAG: u32 = 2150859040u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_NULL_PARAM: u32 = 2150859041u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_PULL_NULL_PARAM: u32 = 2150859057u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_PUSH_NULL_PARAM: u32 = 2150859060u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_RECEIVE_NULL_PARAM: u32 = 2150859055u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_RUNCOMMAND_NULL_PARAM: u32 = 2150859052u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_SEND_NULL_PARAM: u32 = 2150859059u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CLIENT_SIGNAL_NULL_PARAM: u32 = 2150859054u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CODE_PAGE_NOT_SUPPORTED: u32 = 2150859072u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_CONNECT_RESPONSE_BAD_BODY: u32 = 2150859211u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_IDLETIMEOUT_OUTOFBOUNDS: u32 = 2150859250u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_RECEIVE_IN_PROGRESS: u32 = 2150859047u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_RECEIVE_NO_RESPONSE_DATA: u32 = 2150859048u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELLCOMMAND_CLIENTID_NOT_VALID: u32 = 2150859220u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELLCOMMAND_CLIENTID_RESOURCE_CONFLICT: u32 = 2150859222u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELLCOMMAND_DISCONNECT_OPERATION_NOT_VALID: u32 = 2150859224u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELLCOMMAND_RECONNECT_OPERATION_NOT_VALID: u32 = 2150859219u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_CLIENTID_NOT_VALID: u32 = 2150859221u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_CLIENTID_RESOURCE_CONFLICT: u32 = 2150859223u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_CLIENTSESSIONID_MISMATCH: u32 = 2150859206u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_CONNECTED_TO_DIFFERENT_CLIENT: u32 = 2150859213u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_DISCONNECTED: u32 = 2150859204u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_DISCONNECT_NOT_SUPPORTED: u32 = 2150859205u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_GRACEFUL: u32 = 2150859214u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_VALID: u32 = 2150859215u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_RECONNECT_OPERATION_NOT_VALID: u32 = 2150859216u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WINRS_SHELL_URI_INVALID: u32 = 2150859099u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ACK_NOT_SUPPORTED: u32 = 2150858853u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ACTION_MISMATCH: u32 = 2150858801u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ACTION_NOT_SUPPORTED: u32 = 2150858771u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ADDOBJECT_MISSING_EPR: u32 = 2150859045u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ADDOBJECT_MISSING_OBJECT: u32 = 2150859044u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ALREADY_EXISTS: u32 = 2150858803u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_AMBIGUOUS_SELECTORS: u32 = 2150858846u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_AUTHENTICATION_INVALID_FLAG: u32 = 2150859077u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_AUTHORIZATION_MODE_NOT_SUPPORTED: u32 = 2150858852u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_BAD_METHOD: u32 = 2150858868u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_BATCHSIZE_TOO_SMALL: u32 = 2150858919u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_BATCH_COMPLETE: u32 = 2150858756u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_BOOKMARKS_NOT_SUPPORTED: u32 = 2150858859u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_BOOKMARK_EXPIRED: u32 = 2150858832u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_CHANGE_KEYS: u32 = 2150858989u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_DECRYPT: u32 = 2150859001u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_PROCESS_FILTER: u32 = 2150859042u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_USE_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS_FOR_HTTP: u32 = 2150859184u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_USE_CERTIFICATES_FOR_HTTP: u32 = 2150858968u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_CREDSSP: u32 = 2150859187u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_HTTP: u32 = 2150859185u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_KERBEROS: u32 = 2150859186u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_CONFIGLIMIT_EXCEEDED: u32 = 2150859091u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_CREDENTIAL_MANAGEMENT_FAILIED: u32 = 2150859262u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_INVALIDISSUERKEY: u32 = 2150859106u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_INVALIDSUBJECTKEY: u32 = 2150859105u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_INVALIDUSERCREDENTIALS: u32 = 2150859092u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDBLANK: u32 = 2150859115u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDTOOLONG: u32 = 2150859114u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDUSERTUPLE: u32 = 2150859116u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERT_INVALID_USAGE: u32 = 2150858990u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERT_INVALID_USAGE_CLIENT: u32 = 2150859093u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERT_MISSING_AUTH_FLAG: u32 = 2150859094u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERT_MULTIPLE_CREDENTIALS_FLAG: u32 = 2150859095u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERT_NOT_FOUND: u32 = 2150858882u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERT_THUMBPRINT_BLANK: u32 = 2150858983u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CERT_THUMBPRINT_NOT_BLANK: u32 = 2150858982u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CHARACTER_SET: u32 = 2150858828u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS: u32 = 2150859171u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS_NTLMONLY: u32 = 2150859172u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_BASIC_AUTHENTICATION_DISABLED: u32 = 2150858975u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_BATCH_ITEMS_TOO_SMALL: u32 = 2150858946u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_BLANK_ACTION_URI: u32 = 2150858948u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_BLANK_INPUT_XML: u32 = 2150858945u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_BLANK_URI: u32 = 2150858943u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CERTIFICATES_AUTHENTICATION_DISABLED: u32 = 2150858979u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CERT_NEEDED: u32 = 2150858932u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CERT_UNKNOWN_LOCATION: u32 = 2150858934u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CERT_UNKNOWN_TYPE: u32 = 2150858933u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CERT_UNNEEDED_CREDS: u32 = 2150858927u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CERT_UNNEEDED_USERNAME: u32 = 2150858929u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CLOSECOMMAND_NULL_PARAM: u32 = 2150859135u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CLOSESHELL_NULL_PARAM: u32 = 2150859134u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_COMPRESSION_INVALID_OPTION: u32 = 2150858957u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CONNECTCOMMAND_NULL_PARAM: u32 = 2150859210u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CONNECTSHELL_NULL_PARAM: u32 = 2150859209u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CONSTRUCTERROR_NULL_PARAM: u32 = 2150858965u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREATESESSION_NULL_PARAM: u32 = 2150858938u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREATESHELL_NAME_INVALID: u32 = 2150859202u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREATESHELL_NULL_PARAM: u32 = 2150859130u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FLAG_NEEDED: u32 = 2150858931u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_DEFAULT_AUTHENTICATION: u32 = 2150859078u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_PROXY_AUTHENTICATION: u32 = 2150859163u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_NEEDED: u32 = 2150858930u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_CREDSSP_AUTHENTICATION_DISABLED: u32 = 2150859170u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_DECODEOBJECT_NULL_PARAM: u32 = 2150858961u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_DELIVERENDSUBSCRIPTION_NULL_PARAM: u32 = 2150858958u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_DELIVEREVENTS_NULL_PARAM: u32 = 2150858959u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_DIGEST_AUTHENTICATION_DISABLED: u32 = 2150858976u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_DISABLE_LOOPBACK_WITH_EXPLICIT_CREDENTIALS: u32 = 2150859073u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_DISCONNECTSHELL_NULL_PARAM: u32 = 2150859207u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ENCODEOBJECT_NULL_PARAM: u32 = 2150858962u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ENUMERATE_NULL_PARAM: u32 = 2150858939u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ENUMERATORADDEVENT_NULL_PARAM: u32 = 2150859043u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ENUMERATORADDOBJECT_NULL_PARAM: u32 = 2150858963u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ENUMERATORNEXTOBJECT_NULL_PARAM: u32 = 2150858964u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ENUM_RECEIVED_TOO_MANY_ITEMS: u32 = 2150859075u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_GETBOOKMARK_NULL_PARAM: u32 = 2150858960u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_GETERRORMESSAGE_NULL_PARAM: u32 = 2150859158u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_INVALID_PARAM: u32 = 2150859167u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_NULL_PARAM: u32 = 2150859166u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_INVALID_PARAM: u32 = 2150859129u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_STRING_INVALID_PARAM: u32 = 2150859168u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INITIALIZE_NULL_PARAM: u32 = 2150859124u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_CERT: u32 = 2150858935u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_CERT_DNS_OR_UPN: u32 = 2150859080u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_CLOSE_COMMAND_FLAG: u32 = 2150859133u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_CLOSE_SHELL_FLAG: u32 = 2150859132u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_CREATE_SHELL_FLAG: u32 = 2150859131u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_DEINIT_APPLICATION_FLAG: u32 = 2150859126u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_DELIVERY_RETRY: u32 = 2150859108u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_DISABLE_LOOPBACK: u32 = 2150859074u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_DISCONNECT_SHELL_FLAG: u32 = 2150859226u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_FLAG: u32 = 2150858924u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_GETERRORMESSAGE_FLAG: u32 = 2150859160u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_INIT_APPLICATION_FLAG: u32 = 2150859125u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_LANGUAGE_CODE: u32 = 2150859159u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_LOCALE: u32 = 2150859156u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_RECEIVE_SHELL_FLAG: u32 = 2150859150u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_RESOURCE_LOCATOR: u32 = 2150858944u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_RUNCOMMAND_FLAG: u32 = 2150859137u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_FLAG: u32 = 2150859145u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_PARAMETER: u32 = 2150859146u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_SHELL_COMMAND_PAIR: u32 = 2150859227u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_SIGNAL_SHELL_FLAG: u32 = 2150859143u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_INVALID_UI_LANGUAGE: u32 = 2150859157u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_KERBEROS_AUTHENTICATION_DISABLED: u32 = 2150858978u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_LOCAL_INVALID_CONNECTION_OPTIONS: u32 = 2150858937u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_LOCAL_INVALID_CREDS: u32 = 2150858936u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_MAX_CHARS_TOO_SMALL: u32 = 2150858947u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_MISSING_EXPIRATION: u32 = 2150858953u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_MULTIPLE_AUTH_FLAGS: u32 = 2150858925u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_MULTIPLE_DELIVERY_MODES: u32 = 2150858950u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_MULTIPLE_ENUM_MODE_FLAGS: u32 = 2150859039u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_MULTIPLE_ENVELOPE_POLICIES: u32 = 2150858951u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_MULTIPLE_PROXY_AUTH_FLAGS: u32 = 2150859188u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_NEGOTIATE_AUTHENTICATION_DISABLED: u32 = 2150858977u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_NO_HANDLE: u32 = 2150858942u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_NO_SOURCES: u32 = 2150859111u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_NULL_ISSUERS: u32 = 2150859110u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_NULL_PUBLISHERS: u32 = 2150859109u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_NULL_RESULT_PARAM: u32 = 2150858941u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_PULL_INVALID_FLAGS: u32 = 2150858954u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_PUSH_HOST_TOO_LONG: u32 = 2150858956u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_PUSH_UNSUPPORTED_TRANSPORT: u32 = 2150858955u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_RECEIVE_NULL_PARAM: u32 = 2150859148u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_RECONNECTSHELLCOMMAND_NULL_PARAM: u32 = 2150859218u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_RECONNECTSHELL_NULL_PARAM: u32 = 2150859208u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_RUNCOMMAND_NOTCOMPLETED: u32 = 2150859138u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_RUNCOMMAND_NULL_PARAM: u32 = 2150859136u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_SEND_NULL_PARAM: u32 = 2150859144u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_SESSION_UNUSABLE: u32 = 2150859258u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_SETSESSIONOPTION_INVALID_PARAM: u32 = 2150859128u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_SETSESSIONOPTION_NULL_PARAM: u32 = 2150859127u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_SIGNAL_NULL_PARAM: u32 = 2150859142u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_SPN_WRONG_AUTH: u32 = 2150858926u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_SUBSCRIBE_NULL_PARAM: u32 = 2150858940u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_UNENCRYPTED_DISABLED: u32 = 2150858974u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_UNENCRYPTED_HTTP_ONLY: u32 = 2150858967u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_UNKNOWN_EXPIRATION_TYPE: u32 = 2150858952u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_USERNAME_AND_PASSWORD_NEEDED: u32 = 2150859079u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_USERNAME_PASSWORD_NEEDED: u32 = 2150858928u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_WORKGROUP_NO_KERBEROS: u32 = 2150859020u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CLIENT_ZERO_HEARTBEAT: u32 = 2150858949u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_COMMAND_ALREADY_CLOSED: u32 = 2150859087u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_COMMAND_TERMINATED: u32 = 2150859212u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONCURRENCY: u32 = 2150858802u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_CERTMAPPING_KEYS: u32 = 2150859122u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_GPO_CONTROLLED_SETTING: u32 = 2150858890u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_MUTUAL: u32 = 2150858885u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_CANNOT_SHARE_SSL_CONFIG: u32 = 2150858984u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_CERT_CN_DOES_NOT_MATCH_HOSTNAME: u32 = 2150858985u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_CORRUPTED: u32 = 2150858757u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_GROUP_POLICY_CHANGE_NOTIFICATION_SUBSCRIPTION_FAILED: u32 = 2150859217u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_HOSTNAME_CHANGE_WITHOUT_CERT: u32 = 2150858986u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_PORT_INVALID: u32 = 2150858972u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_READONLY_PROPERTY: u32 = 2150859071u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_SHELLURI_INVALID_OPERATION_ON_KEY: u32 = 2150859119u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_SHELLURI_INVALID_PROCESSPATH: u32 = 2150859098u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_SHELL_URI_CMDSHELLURI_NOTPERMITTED: u32 = 2150859097u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_SHELL_URI_INVALID: u32 = 2150859096u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONFIG_THUMBPRINT_SHOULD_BE_EMPTY: u32 = 2150858987u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONNECTIONSTR_INVALID: u32 = 2150858969u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CONNECTOR_GET: u32 = 2150858873u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CREATESHELL_NULL_ENVIRONMENT_VARIABLE_NAME: u32 = 2150859081u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CREATESHELL_NULL_STREAMID: u32 = 2150859083u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CREATESHELL_RUNAS_FAILED: u32 = 2150859231u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CREATE_RESPONSE_NO_EPR: u32 = 2150858992u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CREDSSP_USERNAME_PASSWORD_NEEDED: u32 = 2150859169u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CREDS_PASSED_WITH_NO_AUTH_FLAG: u32 = 2150858923u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_CUSTOMREMOTESHELL_DEPRECATED: u32 = 2150859196u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DEFAULTAUTH_IPADDRESS: u32 = 2150859195u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DELIVERY_REFUSED: u32 = 2150858804u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DELIVERY_RETRIES_NOT_SUPPORTED: u32 = 2150858857u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DELIVER_IN_PROGRESS: u32 = 2150858821u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DEPRECATED_CONFIG_SETTING: u32 = 2150859182u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DESERIALIZE_CLASS: u32 = 2150859244u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DESTINATION_INVALID: u32 = 2150859256u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DESTINATION_UNREACHABLE: u32 = 2150858770u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DIFFERENT_AUTHZ_TOKEN: u32 = 2150859177u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DIFFERENT_CIM_SELECTOR: u32 = 2150859067u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_DUPLICATE_SELECTORS: u32 = 2150858847u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENCODING_LIMIT: u32 = 2150858805u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENCODING_TYPE: u32 = 2150859033u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENDPOINT_UNAVAILABLE: u32 = 2150858772u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENDPOINT_UNAVAILABLE_INVALID_VALUE: u32 = 2150859034u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_CANNOT_PROCESS_FILTER: u32 = 2150858778u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_FILTERING_NOT_SUPPORTED: u32 = 2150858776u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_FILTER_DIALECT_REQUESTED_UNAVAILABLE: u32 = 2150858777u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_INVALID_ENUMERATION_CONTEXT: u32 = 2150858779u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_INVALID_EXPIRATION_TIME: u32 = 2150858774u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_SHELLCOMAMNDS_FILTER_EXPECTED: u32 = 2150859200u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_SHELLCOMMANDS_EPRS_NOTSUPPORTED: u32 = 2150859201u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_TIMED_OUT: u32 = 2150858780u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_UNABLE_TO_RENEW: u32 = 2150858781u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TIME: u32 = 2150858775u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TYPE: u32 = 2150859036u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATE_WMI_INVALID_KEY: u32 = 2150859016u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATION_CLOSED: u32 = 2150858759u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATION_INITIALIZING: u32 = 2150858872u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATION_INVALID: u32 = 2150858884u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENUMERATION_MODE_UNSUPPORTED: u32 = 2150858886u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_ENVELOPE_TOO_LARGE: u32 = 2150858790u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EPR_NESTING_EXCEEDED: u32 = 2150858879u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_CONCURRENT_CLIENT_RECEIVE: u32 = 2150858891u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_DELIVERYFAILED_FROMSOURCE: u32 = 2150858908u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_INVALID: u32 = 2150858920u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_UNAVAILABLE: u32 = 2150858782u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_FAST_SENDER: u32 = 2150858892u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_FILTERING_NOT_SUPPORTED: u32 = 2150858785u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_FILTERING_REQUESTED_UNAVAILABLE: u32 = 2150858786u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INCOMPATIBLE_BATCHPARAMS_AND_DELIVERYMODE: u32 = 2150858900u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INSECURE_PUSHSUBSCRIPTION_CONNECTION: u32 = 2150858893u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_ENCODING_IN_DELIVERY: u32 = 2150859255u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_ENDTO_ADDRESSS: u32 = 2150858902u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_EVENTSOURCE: u32 = 2150858894u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_EXPIRATION_TIME: u32 = 2150858783u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_HEARTBEAT: u32 = 2150858916u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_INCOMING_EVENT_PACKET_HEADER: u32 = 2150858903u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_LOCALE_IN_DELIVERY: u32 = 2150858915u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_MESSAGE: u32 = 2150858789u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_INVALID_NOTIFYTO_ADDRESSS: u32 = 2150858914u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_LOOPBACK_TESTFAILED: u32 = 2150858901u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_MISSING_LOCALE_IN_DELIVERY: u32 = 2150859028u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO: u32 = 2150858912u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO_ADDRESSS: u32 = 2150858913u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_NOMATCHING_LISTENER: u32 = 2150858895u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_NONDOMAINJOINED_COLLECTOR: u32 = 2150859070u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_NONDOMAINJOINED_PUBLISHER: u32 = 2150859069u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_SOURCE_UNABLE_TO_PROCESS: u32 = 2150858787u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_SUBSCRIPTIONCLOSED_BYREMOTESERVICE: u32 = 2150858907u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_SUBSCRIPTION_CANCELLED_BYSOURCE: u32 = 2150858910u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_UNABLE_TO_RENEW: u32 = 2150858788u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EVENTING_UNSUPPORTED_EXPIRATION_TYPE: u32 = 2150858784u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EXPIRATION_TIME_NOT_SUPPORTED: u32 = 2150858856u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_EXPLICIT_CREDENTIALS_REQUIRED: u32 = 2150858981u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FAILED_AUTHENTICATION: u32 = 2150858806u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FEATURE_DEPRECATED: u32 = 2150859197u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FILE_NOT_PRESENT: u32 = 2150859154u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FILTERING_REQUIRED: u32 = 2150858831u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FILTERING_REQUIRED_NOT_SUPPORTED: u32 = 2150858864u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FORMAT_MISMATCH_NOT_SUPPORTED: u32 = 2150858866u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FORMAT_SECURITY_TOKEN_NOT_SUPPORTED: u32 = 2150858867u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FRAGMENT_DIALECT_REQUESTED_UNAVAILABLE: u32 = 2150858896u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_FRAGMENT_TRANSFER_NOT_SUPPORTED: u32 = 2150858871u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_GETCLASS: u32 = 2150859245u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HEARTBEATS_NOT_SUPPORTED: u32 = 2150858858u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTML_ERROR: u32 = 2150859123u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_CONTENT_TYPE_MISSMATCH_RESPONSE_DATA: u32 = 2150859000u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_INVALID_CONTENT_TYPE_IN_RESPONSE_DATA: u32 = 2150858999u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_NOT_FOUND_STATUS: u32 = 2150859027u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_NO_RESPONSE_DATA: u32 = 2150858997u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_REQUEST_TOO_LARGE_STATUS: u32 = 2150859025u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_SERVICE_UNAVAILABLE_STATUS: u32 = 2150859026u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_STATUS_BAD_REQUEST: u32 = 2150859121u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_HTTP_STATUS_SERVER_ERROR: u32 = 2150859120u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_IISCONFIGURATION_READ_FAILED: u32 = 2150859155u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INCOMPATIBLE_EPR: u32 = 2150858807u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INEXISTENT_MAC_ADDRESS: u32 = 2150858875u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INSECURE_ADDRESS_NOT_SUPPORTED: u32 = 2150858865u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INSUFFCIENT_SELECTORS: u32 = 2150858842u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INSUFFICIENT_METADATA_FOR_BASIC: u32 = 2150859251u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_ACTIONURI: u32 = 2150858753u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_BATCH_PARAMETER: u32 = 2150858799u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_BATCH_SETTINGS_PARAMETER: u32 = 2150859021u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_BOOKMARK: u32 = 2150858808u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_CHARACTERS_IN_RESPONSE: u32 = 2150859018u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_CONFIGSDDL_URL: u32 = 2150859199u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_CONNECTIONRETRY: u32 = 2150859103u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_FILEPATH: u32 = 2150859153u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_FILTER_XML: u32 = 2150859015u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_FRAGMENT_DIALECT: u32 = 2150858898u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_FRAGMENT_PATH: u32 = 2150858899u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_FRAGMENT_PATH_BLANK: u32 = 2150859017u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_HEADER: u32 = 2150859035u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_HOSTNAME_PATTERN: u32 = 2150858911u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_IPFILTER: u32 = 2150858988u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_KEY: u32 = 2150858820u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_LITERAL_URI: u32 = 2150859252u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_MESSAGE_INFORMATION_HEADER: u32 = 2150858767u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_OPTIONS: u32 = 2150858809u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_OPTIONSET: u32 = 2150859140u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_OPTION_NO_PROXY_SERVER: u32 = 2150859165u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_PARAMETER: u32 = 2150858810u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_PARAMETER_NAME: u32 = 2150858837u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_PROPOSED_ID: u32 = 2150858798u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_PROVIDER_RESPONSE: u32 = 2150859117u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_PUBLISHERS_TYPE: u32 = 2150859107u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_REDIRECT_ERROR: u32 = 2150859189u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_REPRESENTATION: u32 = 2150858773u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_RESOURCE_URI: u32 = 2150858811u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_RESUMPTION_CONTEXT: u32 = 2150858792u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SECURITY_DESCRIPTOR: u32 = 2150859100u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SELECTORS: u32 = 2150858813u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SELECTOR_NAME: u32 = 2150859032u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SELECTOR_VALUE: u32 = 2150858845u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SOAP_BODY: u32 = 2150858791u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SUBSCRIBE_OBJECT: u32 = 2150859112u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SUBSCRIPTION_MANAGER: u32 = 2150859006u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_SYSTEM: u32 = 2150858812u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_TARGET_RESOURCEURI: u32 = 2150858849u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_TARGET_SELECTORS: u32 = 2150858848u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_TARGET_SYSTEM: u32 = 2150858850u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_TIMEOUT_HEADER: u32 = 2150858881u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_URI: u32 = 2150858754u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_URI_WMI_ENUM_WQL: u32 = 2150859003u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_URI_WMI_SINGLETON: u32 = 2150859002u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_USESSL_PARAM: u32 = 2150859198u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_XML: u32 = 2150858819u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_XML_FRAGMENT: u32 = 2150858841u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_XML_MISSING_VALUES: u32 = 2150858839u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_XML_NAMESPACE: u32 = 2150858840u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_XML_RUNAS_DISABLED: u32 = 2150859232u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_INVALID_XML_VALUES: u32 = 2150858838u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_KERBEROS_IPADDRESS: u32 = 2150859019u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_LISTENER_ADDRESS_INVALID: u32 = 2150858889u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_LOCALE_NOT_SUPPORTED: u32 = 2150858855u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MACHINE_OPTION_REQUIRED: u32 = 2150858917u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MAXENVELOPE_POLICY_NOT_SUPPORTED: u32 = 2150858863u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MAXENVELOPE_SIZE_NOT_SUPPORTED: u32 = 2150858862u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MAXITEMS_NOT_SUPPORTED: u32 = 2150858860u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MAXTIME_NOT_SUPPORTED: u32 = 2150858861u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MAX_ELEMENTS_NOT_SUPPORTED: u32 = 2150859037u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MAX_ENVELOPE_SIZE: u32 = 2150858823u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MAX_ENVELOPE_SIZE_EXCEEDED: u32 = 2150858824u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MESSAGE_INFORMATION_HEADER_REQUIRED: u32 = 2150858769u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_METADATA_REDIRECT: u32 = 2150858814u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MIN_ENVELOPE_SIZE: u32 = 2150858878u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MISSING_CLASSNAME: u32 = 2150859254u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MISSING_FRAGMENT_PATH: u32 = 2150858897u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MULTIPLE_CREDENTIALS: u32 = 2150859076u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MUSTUNDERSTAND_ON_LOCALE_UNSUPPORTED: u32 = 2150858887u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_MUTUAL_AUTH_FAILED: u32 = 2150859248u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NAME_NOT_RESOLVED: u32 = 2150859193u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NETWORK_TIMEDOUT: u32 = 2150859046u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NEW_DESERIALIZER: u32 = 2150859243u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NEW_SESSION: u32 = 2150859246u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NON_PULL_SUBSCRIPTION_NOT_SUPPORTED: u32 = 2150859007u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_ACK: u32 = 2150858800u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_CERTMAPPING_OPERATION_FOR_LOCAL_SESSION: u32 = 2150859090u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_COMMANDID: u32 = 2150859141u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_COMMAND_RESPONSE: u32 = 2150859139u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_DHCP_ADDRESSES: u32 = 2150858877u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_IDENTIFY_FOR_LOCAL_SESSION: u32 = 2150859004u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_PUSH_SUBSCRIPTION_FOR_LOCAL_SESSION: u32 = 2150859005u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_RECEIVE_RESPONSE: u32 = 2150859151u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NO_UNICAST_ADDRESSES: u32 = 2150858876u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_NULL_KEY: u32 = 2150859247u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_OBJECTONLY_INVALID: u32 = 2150859253u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_OPERATION_TIMEDOUT: u32 = 2150858793u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_OPERATION_TIMEOUT_NOT_SUPPORTED: u32 = 2150858854u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_OPTIONS_INVALID_NAME: u32 = 2150858834u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_OPTIONS_INVALID_VALUE: u32 = 2150858835u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_OPTIONS_NOT_SUPPORTED: u32 = 2150858833u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_OPTION_LIMIT: u32 = 2150858827u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PARAMETER_TYPE_MISMATCH: u32 = 2150858836u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PLUGIN_CONFIGURATION_CORRUPTED: u32 = 2150859152u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PLUGIN_FAILED: u32 = 2150858883u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_POLICY_CANNOT_COMPLY: u32 = 2150859102u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_POLICY_CORRUPTED: u32 = 2150858888u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_POLICY_TOO_COMPLEX: u32 = 2150859101u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_POLYMORPHISM_MODE_UNSUPPORTED: u32 = 2150859063u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PORT_INVALID: u32 = 2150858971u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PROVIDER_FAILURE: u32 = 2150858755u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PROVIDER_LOAD_FAILED: u32 = 2150858906u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PROVSYS_NOT_SUPPORTED: u32 = 2150858921u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PROXY_ACCESS_TYPE: u32 = 2150859164u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PROXY_AUTHENTICATION_INVALID_FLAG: u32 = 2150859162u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PUBLIC_FIREWALL_PROFILE_ACTIVE: u32 = 2150859113u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PULL_IN_PROGRESS: u32 = 2150858758u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PULL_PARAMS_NOT_SAME_AS_ENUM: u32 = 2150859181u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PUSHSUBSCRIPTION_INVALIDUSERACCOUNT: u32 = 2150859068u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_PUSH_SUBSCRIPTION_CONFIG_INVALID: u32 = 2150858922u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUICK_CONFIG_FAILED_CERT_REQUIRED: u32 = 2150859029u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUICK_CONFIG_FIREWALL_EXCEPTIONS_DISALLOWED: u32 = 2150859030u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUICK_CONFIG_LOCAL_POLICY_CHANGE_DISALLOWED: u32 = 2150859031u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_LIMIT: u32 = 2150858815u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_COMMANDS_PER_SHELL_PPQ: u32 = 2150859241u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_OPERATIONS: u32 = 2150859174u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_OPERATIONS_USER_PPQ: u32 = 2150859240u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_PLUGINOPERATIONS_PPQ: u32 = 2150859239u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_PLUGINSHELLS_PPQ: u32 = 2150859238u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_SHELLS: u32 = 2150859173u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_SHELLS_PPQ: u32 = 2150859236u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_SHELLUSERS: u32 = 2150859179u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MAX_USERS_PPQ: u32 = 2150859237u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_MIN_REQUIREMENT_NOT_AVAILABLE_PPQ: u32 = 2150859242u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_SYSTEM: u32 = 2150859176u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_QUOTA_USER: u32 = 2150859175u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_REDIRECT_LOCATION_NOT_AVAILABLE: u32 = 2150859178u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_REDIRECT_REQUESTED: u32 = 2150859161u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_REMOTESHELLS_NOT_ALLOWED: u32 = 2150859180u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_REMOTE_CIMPATH_NOT_SUPPORTED: u32 = 2150859009u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_REMOTE_CONNECTION_NOT_ALLOWED: u32 = 2150859235u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RENAME_FAILURE: u32 = 2150858816u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_REQUEST_INIT_ERROR: u32 = 2150858880u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_REQUEST_NOT_SUPPORTED_AT_SERVICE: u32 = 2150859064u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESOURCE_NOT_FOUND: u32 = 2150858752u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESPONSE_INVALID_ENUMERATION_CONTEXT: u32 = 2150858993u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESPONSE_INVALID_MESSAGE_INFORMATION_HEADER: u32 = 2150858995u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESPONSE_INVALID_SOAP_FAULT: u32 = 2150858998u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESPONSE_NO_RESULTS: u32 = 2150858991u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESPONSE_NO_SOAP_HEADER_BODY: u32 = 2150858996u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESPONSE_NO_XML_FRAGMENT_WRAPPER: u32 = 2150858994u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESUMPTION_NOT_SUPPORTED: u32 = 2150858794u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RESUMPTION_TYPE_NOT_SUPPORTED: u32 = 2150858795u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RUNASUSER_MANAGEDACCOUNT_LOGON_FAILED: u32 = 2150859261u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RUNAS_INVALIDUSERCREDENTIALS: u32 = 2150859203u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_RUNSHELLCOMMAND_NULL_ARGUMENT: u32 = 2150859086u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SCHEMA_VALIDATION_ERROR: u32 = 2150858817u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SECURITY_UNMAPPED: u32 = 2150858909u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SELECTOR_LIMIT: u32 = 2150858826u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SELECTOR_TYPEMISMATCH: u32 = 2150858844u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SEMANTICCALLBACK_TIMEDOUT: u32 = 2150859228u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SENDHEARBEAT_EMPTY_ENUMERATOR: u32 = 2150858973u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SENDSHELLINPUT_INVALID_STREAMID_INDEX: u32 = 2150859088u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SERVER_DESTINATION_LOCALHOST: u32 = 2150859022u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SERVER_ENVELOPE_LIMIT: u32 = 2150858825u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SERVER_NONPULLSUBSCRIBE_NULL_PARAM: u32 = 2150858966u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SERVER_NOT_TRUSTED: u32 = 2150858980u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SERVICE_REMOTE_ACCESS_DISABLED: u32 = 2150859229u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SERVICE_STREAM_DISCONNECTED: u32 = 2150859230u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SESSION_ALREADY_CLOSED: u32 = 2150858904u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SHELL_ALREADY_CLOSED: u32 = 2150859082u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SHELL_INVALID_COMMAND_HANDLE: u32 = 2150859085u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SHELL_INVALID_DESIRED_STREAMS: u32 = 2150859149u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SHELL_INVALID_INPUT_STREAM: u32 = 2150859147u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SHELL_INVALID_SHELL_HANDLE: u32 = 2150859084u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SHELL_NOT_INITIALIZED: u32 = 2150859118u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SHELL_SYNCHRONOUS_NOT_SUPPORTED: u32 = 2150859089u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SOAP_DATA_ENCODING_UNKNOWN: u32 = 2150858766u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SOAP_FAULT_MUST_UNDERSTAND: u32 = 2150858768u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SOAP_VERSION_MISMATCH: u32 = 2150858765u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SSL_CONNECTION_ABORTED: u32 = 2150859194u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SUBSCRIBE_WMI_INVALID_KEY: u32 = 2150859225u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SUBSCRIPTION_CLIENT_DID_NOT_CALL_WITHIN_HEARTBEAT: u32 = 2150858762u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SUBSCRIPTION_CLOSED: u32 = 2150858760u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SUBSCRIPTION_CLOSE_IN_PROGRESS: u32 = 2150858761u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SUBSCRIPTION_LISTENER_NOLONGERVALID: u32 = 2150858905u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SUBSCRIPTION_NO_HEARTBEAT: u32 = 2150858763u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_SYSTEM_NOT_FOUND: u32 = 2150858822u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_TARGET_ALREADY_EXISTS: u32 = 2150858851u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_TRANSPORT_NOT_SUPPORTED: u32 = 2150858970u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNEXPECTED_SELECTORS: u32 = 2150858843u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNKNOWN_HTTP_STATUS_RETURNED: u32 = 2150859023u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNREPORTABLE_SUCCESS: u32 = 2150858829u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_ADDRESSING_MODE: u32 = 2150858870u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_ENCODING: u32 = 2150858796u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE: u32 = 2150858818u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE_IDENTIFY: u32 = 2150859257u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE_OPTIONS: u32 = 2150858918u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_HTTP_STATUS_REDIRECT: u32 = 2150859024u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_MEDIA: u32 = 2150858869u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_OCTETTYPE: u32 = 2150859249u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_TIMEOUT: u32 = 2150858764u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_UNSUPPORTED_TYPE: u32 = 2150859234u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_URISECURITY_INVALIDURIKEY: u32 = 2150859104u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_URI_LIMIT: u32 = 2150858797u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_URI_NON_DMTF_CLASS: u32 = 2150859065u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_URI_QUERY_STRING_SYNTAX_ERROR: u32 = 2150858874u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_URI_SECURITY_URI: u32 = 2150859183u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_URI_WRONG_DMTF_VERSION: u32 = 2150859066u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED: u32 = 2150859259u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED_DOWNLEVEL: u32 = 2150859260u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WHITESPACE: u32 = 2150858830u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WMI_CANNOT_CONNECT_ACCESS_DENIED: u32 = 2150859014u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WMI_INVALID_VALUE: u32 = 2150859011u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WMI_MAX_NESTED: u32 = 2150859008u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WMI_PROVIDER_ACCESS_DENIED: u32 = 2150859013u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WMI_PROVIDER_INVALID_PARAMETER: u32 = 2150859038u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WMI_PROVIDER_NOT_CAPABLE: u32 = 2150859010u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WMI_SVC_ACCESS_DENIED: u32 = 2150859012u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const ERROR_WSMAN_WRONG_METADATA: u32 = 2150859233u32;
pub type IWSMan = *mut ::core::ffi::c_void;
pub type IWSManConnectionOptions = *mut ::core::ffi::c_void;
pub type IWSManConnectionOptionsEx = *mut ::core::ffi::c_void;
pub type IWSManConnectionOptionsEx2 = *mut ::core::ffi::c_void;
pub type IWSManEnumerator = *mut ::core::ffi::c_void;
pub type IWSManEx = *mut ::core::ffi::c_void;
pub type IWSManEx2 = *mut ::core::ffi::c_void;
pub type IWSManEx3 = *mut ::core::ffi::c_void;
pub type IWSManInternal = *mut ::core::ffi::c_void;
pub type IWSManResourceLocator = *mut ::core::ffi::c_void;
pub type IWSManResourceLocatorInternal = *mut ::core::ffi::c_void;
pub type IWSManSession = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct WSMAN_API(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_AUTHENTICATION_CREDENTIALS {
    pub authenticationMechanism: u32,
    pub Anonymous: WSMAN_AUTHENTICATION_CREDENTIALS_0,
}
impl ::core::marker::Copy for WSMAN_AUTHENTICATION_CREDENTIALS {}
impl ::core::clone::Clone for WSMAN_AUTHENTICATION_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub union WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    pub userAccount: WSMAN_USERNAME_PASSWORD_CREDS,
    pub certificateThumbprint: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_AUTHENTICATION_CREDENTIALS_0 {}
impl ::core::clone::Clone for WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_AUTHZ_QUOTA {
    pub maxAllowedConcurrentShells: u32,
    pub maxAllowedConcurrentOperations: u32,
    pub timeslotSize: u32,
    pub maxAllowedOperationsPerTimeslot: u32,
}
impl ::core::marker::Copy for WSMAN_AUTHZ_QUOTA {}
impl ::core::clone::Clone for WSMAN_AUTHZ_QUOTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_CERTIFICATE_DETAILS {
    pub subject: ::windows_sys::core::PCWSTR,
    pub issuerName: ::windows_sys::core::PCWSTR,
    pub issuerThumbprint: ::windows_sys::core::PCWSTR,
    pub subjectName: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_CERTIFICATE_DETAILS {}
impl ::core::clone::Clone for WSMAN_CERTIFICATE_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_CMDSHELL_OPTION_CODEPAGE: &str = "WINRS_CODEPAGE";
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_CMDSHELL_OPTION_CONSOLEMODE_STDIN: &str = "WINRS_CONSOLEMODE_STDIN";
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_CMDSHELL_OPTION_SKIP_CMD_SHELL: &str = "WINRS_SKIP_CMD_SHELL";
#[repr(C)]
pub struct WSMAN_COMMAND(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_COMMAND_ARG_SET {
    pub argsCount: u32,
    pub args: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WSMAN_COMMAND_ARG_SET {}
impl ::core::clone::Clone for WSMAN_COMMAND_ARG_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_CONNECT_DATA {
    pub data: WSMAN_DATA,
}
impl ::core::marker::Copy for WSMAN_CONNECT_DATA {}
impl ::core::clone::Clone for WSMAN_CONNECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_CREATE_SHELL_DATA {
    pub data: WSMAN_DATA,
}
impl ::core::marker::Copy for WSMAN_CREATE_SHELL_DATA {}
impl ::core::clone::Clone for WSMAN_CREATE_SHELL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_DATA {
    pub r#type: WSManDataType,
    pub Anonymous: WSMAN_DATA_0,
}
impl ::core::marker::Copy for WSMAN_DATA {}
impl ::core::clone::Clone for WSMAN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub union WSMAN_DATA_0 {
    pub text: WSMAN_DATA_TEXT,
    pub binaryData: WSMAN_DATA_BINARY,
    pub number: u32,
}
impl ::core::marker::Copy for WSMAN_DATA_0 {}
impl ::core::clone::Clone for WSMAN_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_DATA_BINARY {
    pub dataLength: u32,
    pub data: *mut u8,
}
impl ::core::marker::Copy for WSMAN_DATA_BINARY {}
impl ::core::clone::Clone for WSMAN_DATA_BINARY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_DATA_TEXT {
    pub bufferLength: u32,
    pub buffer: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_DATA_TEXT {}
impl ::core::clone::Clone for WSMAN_DATA_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_DEFAULT_TIMEOUT_MS: u32 = 60000u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_ENVIRONMENT_VARIABLE {
    pub name: ::windows_sys::core::PCWSTR,
    pub value: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_ENVIRONMENT_VARIABLE {}
impl ::core::clone::Clone for WSMAN_ENVIRONMENT_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_ENVIRONMENT_VARIABLE_SET {
    pub varsCount: u32,
    pub vars: *mut WSMAN_ENVIRONMENT_VARIABLE,
}
impl ::core::marker::Copy for WSMAN_ENVIRONMENT_VARIABLE_SET {}
impl ::core::clone::Clone for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_ERROR {
    pub code: u32,
    pub errorDetail: ::windows_sys::core::PCWSTR,
    pub language: ::windows_sys::core::PCWSTR,
    pub machineName: ::windows_sys::core::PCWSTR,
    pub pluginName: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_ERROR {}
impl ::core::clone::Clone for WSMAN_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_FILTER {
    pub filter: ::windows_sys::core::PCWSTR,
    pub dialect: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_FILTER {}
impl ::core::clone::Clone for WSMAN_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_RECEIVE_FLUSH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_RECEIVE_RESULT_DATA_BOUNDARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_RECEIVE_RESULT_NO_MORE_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_REQUESTED_API_VERSION_1_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_REQUESTED_API_VERSION_1_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_SEND_NO_MORE_DATA: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_FRAGMENT {
    pub path: ::windows_sys::core::PCWSTR,
    pub dialect: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_FRAGMENT {}
impl ::core::clone::Clone for WSMAN_FRAGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_KEY {
    pub key: ::windows_sys::core::PCWSTR,
    pub value: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_KEY {}
impl ::core::clone::Clone for WSMAN_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSMAN_OPERATION(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSMAN_OPERATION_INFO {
    pub fragment: WSMAN_FRAGMENT,
    pub filter: WSMAN_FILTER,
    pub selectorSet: WSMAN_SELECTOR_SET,
    pub optionSet: WSMAN_OPTION_SET,
    pub reserved: *mut ::core::ffi::c_void,
    pub version: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSMAN_OPERATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSMAN_OPERATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSMAN_OPERATION_INFOEX {
    pub fragment: WSMAN_FRAGMENT,
    pub filter: WSMAN_FILTER,
    pub selectorSet: WSMAN_SELECTOR_SET,
    pub optionSet: WSMAN_OPTION_SETEX,
    pub version: u32,
    pub uiLocale: ::windows_sys::core::PCWSTR,
    pub dataLocale: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSMAN_OPERATION_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSMAN_OPERATION_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPERATION_INFOV1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPERATION_INFOV2: u32 = 2864434397u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSMAN_OPTION {
    pub name: ::windows_sys::core::PCWSTR,
    pub value: ::windows_sys::core::PCWSTR,
    pub mustComply: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSMAN_OPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSMAN_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSMAN_OPTION_SET {
    pub optionsCount: u32,
    pub options: *mut WSMAN_OPTION,
    pub optionsMustUnderstand: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSMAN_OPTION_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSMAN_OPTION_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSMAN_OPTION_SETEX {
    pub optionsCount: u32,
    pub options: *mut WSMAN_OPTION,
    pub optionsMustUnderstand: super::super::Foundation::BOOL,
    pub optionTypes: *mut ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSMAN_OPTION_SETEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSMAN_OPTION_SETEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_AUTHORIZE_OPERATION = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, operation: u32, action: ::windows_sys::core::PCWSTR, resourceuri: ::windows_sys::core::PCWSTR)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_AUTHORIZE_QUERY_QUOTA = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSMAN_PLUGIN_AUTHORIZE_RELEASE_CONTEXT = ::core::option::Option<unsafe extern "system" fn(userauthorizationcontext: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_AUTHORIZE_USER = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_COMMAND = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandline: ::windows_sys::core::PCWSTR, arguments: *const WSMAN_COMMAND_ARG_SET)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_CONNECT = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, inboundconnectinformation: *const WSMAN_DATA)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_AUTORESTART: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_GET_REQUESTED_DATA_LOCALE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_GET_REQUESTED_LOCALE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_HOSTIDLETIMEOUTSECONDS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_LARGEST_RESULT_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_MAX_ENVELOPE_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_NAME: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_REMAINING_RESULT_SIZE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_RUNAS_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_SHAREDHOST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_PARAMS_TIMEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_RECEIVE = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, streamset: *const WSMAN_STREAM_ID_SET)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSMAN_PLUGIN_RELEASE_COMMAND_CONTEXT = ::core::option::Option<unsafe extern "system" fn(shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSMAN_PLUGIN_RELEASE_SHELL_CONTEXT = ::core::option::Option<unsafe extern "system" fn(shellcontext: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSMAN_PLUGIN_REQUEST {
    pub senderDetails: *mut WSMAN_SENDER_DETAILS,
    pub locale: ::windows_sys::core::PCWSTR,
    pub resourceUri: ::windows_sys::core::PCWSTR,
    pub operationInfo: *mut WSMAN_OPERATION_INFO,
    pub shutdownNotification: i32,
    pub shutdownNotificationHandle: super::super::Foundation::HANDLE,
    pub dataLocale: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSMAN_PLUGIN_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSMAN_PLUGIN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_SEND = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, stream: ::windows_sys::core::PCWSTR, inbounddata: *const WSMAN_DATA)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_SHELL = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, startupinfo: *const WSMAN_SHELL_STARTUP_INFO_V11, inboundshellinformation: *const WSMAN_DATA)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSMAN_PLUGIN_SHUTDOWN = ::core::option::Option<unsafe extern "system" fn(plugincontext: *const ::core::ffi::c_void, flags: u32, reason: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_SHUTDOWN_IDLETIMEOUT_ELAPSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_SHUTDOWN_IISHOST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_SHUTDOWN_SERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_SHUTDOWN_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WSMAN_PLUGIN_SIGNAL = ::core::option::Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const ::core::ffi::c_void, commandcontext: *const ::core::ffi::c_void, code: ::windows_sys::core::PCWSTR)>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSMAN_PLUGIN_STARTUP = ::core::option::Option<unsafe extern "system" fn(flags: u32, applicationidentification: ::windows_sys::core::PCWSTR, extrainfo: ::windows_sys::core::PCWSTR, plugincontext: *mut *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_STARTUP_AUTORESTARTED_CRASH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_STARTUP_AUTORESTARTED_REBOOT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_PLUGIN_STARTUP_REQUEST_RECEIVED: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_PROXY_INFO {
    pub accessType: u32,
    pub authenticationCredentials: WSMAN_AUTHENTICATION_CREDENTIALS,
}
impl ::core::marker::Copy for WSMAN_PROXY_INFO {}
impl ::core::clone::Clone for WSMAN_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_RECEIVE_DATA_RESULT {
    pub streamId: ::windows_sys::core::PCWSTR,
    pub streamData: WSMAN_DATA,
    pub commandState: ::windows_sys::core::PCWSTR,
    pub exitCode: u32,
}
impl ::core::marker::Copy for WSMAN_RECEIVE_DATA_RESULT {}
impl ::core::clone::Clone for WSMAN_RECEIVE_DATA_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub union WSMAN_RESPONSE_DATA {
    pub receiveData: WSMAN_RECEIVE_DATA_RESULT,
    pub connectData: WSMAN_CONNECT_DATA,
    pub createData: WSMAN_CREATE_SHELL_DATA,
}
impl ::core::marker::Copy for WSMAN_RESPONSE_DATA {}
impl ::core::clone::Clone for WSMAN_RESPONSE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_SELECTOR_SET {
    pub numberKeys: u32,
    pub keys: *mut WSMAN_KEY,
}
impl ::core::marker::Copy for WSMAN_SELECTOR_SET {}
impl ::core::clone::Clone for WSMAN_SELECTOR_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WSMAN_SENDER_DETAILS {
    pub senderName: ::windows_sys::core::PCWSTR,
    pub authenticationMechanism: ::windows_sys::core::PCWSTR,
    pub certificateDetails: *mut WSMAN_CERTIFICATE_DETAILS,
    pub clientToken: super::super::Foundation::HANDLE,
    pub httpURL: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WSMAN_SENDER_DETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WSMAN_SENDER_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WSMAN_SESSION(pub u8);
#[repr(C)]
pub struct WSMAN_SHELL(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_SHELL_ASYNC {
    pub operationContext: *mut ::core::ffi::c_void,
    pub completionFunction: WSMAN_SHELL_COMPLETION_FUNCTION,
}
impl ::core::marker::Copy for WSMAN_SHELL_ASYNC {}
impl ::core::clone::Clone for WSMAN_SHELL_ASYNC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSMAN_SHELL_COMPLETION_FUNCTION = ::core::option::Option<unsafe extern "system" fn(operationcontext: *const ::core::ffi::c_void, flags: u32, error: *const WSMAN_ERROR, shell: *const WSMAN_SHELL, command: *const WSMAN_COMMAND, operationhandle: *const WSMAN_OPERATION, data: *const WSMAN_RESPONSE_DATA)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_SHELL_DISCONNECT_INFO {
    pub idleTimeoutMs: u32,
}
impl ::core::marker::Copy for WSMAN_SHELL_DISCONNECT_INFO {}
impl ::core::clone::Clone for WSMAN_SHELL_DISCONNECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_SHELL_NS: &str = "http://schemas.microsoft.com/wbem/wsman/1/windows/shell";
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_SHELL_OPTION_NOPROFILE: &str = "WINRS_NOPROFILE";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_SHELL_STARTUP_INFO_V10 {
    pub inputStreamSet: *mut WSMAN_STREAM_ID_SET,
    pub outputStreamSet: *mut WSMAN_STREAM_ID_SET,
    pub idleTimeoutMs: u32,
    pub workingDirectory: ::windows_sys::core::PCWSTR,
    pub variableSet: *mut WSMAN_ENVIRONMENT_VARIABLE_SET,
}
impl ::core::marker::Copy for WSMAN_SHELL_STARTUP_INFO_V10 {}
impl ::core::clone::Clone for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_SHELL_STARTUP_INFO_V11 {
    pub __AnonymousBase_wsman_L665_C48: WSMAN_SHELL_STARTUP_INFO_V10,
    pub name: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_SHELL_STARTUP_INFO_V11 {}
impl ::core::clone::Clone for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_STREAM_ID_SET {
    pub streamIDsCount: u32,
    pub streamIDs: *mut ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WSMAN_STREAM_ID_SET {}
impl ::core::clone::Clone for WSMAN_STREAM_ID_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_STREAM_ID_STDERR: &str = "stderr";
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_STREAM_ID_STDIN: &str = "stdin";
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_STREAM_ID_STDOUT: &str = "stdout";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub struct WSMAN_USERNAME_PASSWORD_CREDS {
    pub username: ::windows_sys::core::PCWSTR,
    pub password: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for WSMAN_USERNAME_PASSWORD_CREDS {}
impl ::core::clone::Clone for WSMAN_USERNAME_PASSWORD_CREDS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WSMan: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3169673595, data2: 60419, data3: 16907, data4: [133, 8, 151, 125, 199, 166, 134, 189] };
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManAuthenticationFlags = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_DEFAULT_AUTHENTICATION: WSManAuthenticationFlags = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_NO_AUTHENTICATION: WSManAuthenticationFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_AUTH_DIGEST: WSManAuthenticationFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_AUTH_NEGOTIATE: WSManAuthenticationFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_AUTH_BASIC: WSManAuthenticationFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_AUTH_KERBEROS: WSManAuthenticationFlags = 16i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_AUTH_CREDSSP: WSManAuthenticationFlags = 128i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_AUTH_CLIENT_CERTIFICATE: WSManAuthenticationFlags = 32i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManCallbackFlags = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_END_OF_OPERATION: WSManCallbackFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_END_OF_STREAM: WSManCallbackFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_SHELL_SUPPORTS_DISCONNECT: WSManCallbackFlags = 32i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_SHELL_AUTODISCONNECTED: WSManCallbackFlags = 64i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_NETWORK_FAILURE_DETECTED: WSManCallbackFlags = 256i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_RETRYING_AFTER_NETWORK_FAILURE: WSManCallbackFlags = 512i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_RECONNECTED_AFTER_NETWORK_FAILURE: WSManCallbackFlags = 1024i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_SHELL_AUTODISCONNECTING: WSManCallbackFlags = 2048i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_RETRY_ABORTED_DUE_TO_INTERNAL_ERROR: WSManCallbackFlags = 4096i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_CALLBACK_RECEIVE_DELAY_STREAM_REQUEST_PROCESSED: WSManCallbackFlags = 8192i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManDataType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_DATA_NONE: WSManDataType = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_DATA_TYPE_TEXT: WSManDataType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_DATA_TYPE_BINARY: WSManDataType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_DATA_TYPE_DWORD: WSManDataType = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManEnumFlags = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagNonXmlText: WSManEnumFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagReturnObject: WSManEnumFlags = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagReturnEPR: WSManEnumFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagReturnObjectAndEPR: WSManEnumFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagHierarchyDeep: WSManEnumFlags = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagHierarchyShallow: WSManEnumFlags = 32i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagHierarchyDeepBasePropsOnly: WSManEnumFlags = 64i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagAssociatedInstance: WSManEnumFlags = 0i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagAssociationInstance: WSManEnumFlags = 128i32;
pub const WSManInternal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2111866789, data2: 24011, data3: 19959, data4: [187, 18, 9, 36, 173, 143, 189, 154] };
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManProxyAccessType = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_PROXY_IE_PROXY_CONFIG: WSManProxyAccessType = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_PROXY_WINHTTP_PROXY_CONFIG: WSManProxyAccessType = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_PROXY_AUTO_DETECT: WSManProxyAccessType = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_PROXY_NO_PROXY_SERVER: WSManProxyAccessType = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManProxyAccessTypeFlags = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManProxyIEConfig: WSManProxyAccessTypeFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManProxyWinHttpConfig: WSManProxyAccessTypeFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManProxyAutoDetect: WSManProxyAccessTypeFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManProxyNoProxyServer: WSManProxyAccessTypeFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManProxyAuthenticationFlags = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagProxyAuthenticationUseNegotiate: WSManProxyAuthenticationFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagProxyAuthenticationUseBasic: WSManProxyAuthenticationFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagProxyAuthenticationUseDigest: WSManProxyAuthenticationFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManSessionFlags = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUTF8: WSManSessionFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagCredUsernamePassword: WSManSessionFlags = 4096i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagSkipCACheck: WSManSessionFlags = 8192i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagSkipCNCheck: WSManSessionFlags = 16384i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseNoAuthentication: WSManSessionFlags = 32768i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseDigest: WSManSessionFlags = 65536i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseNegotiate: WSManSessionFlags = 131072i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseBasic: WSManSessionFlags = 262144i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseKerberos: WSManSessionFlags = 524288i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagNoEncryption: WSManSessionFlags = 1048576i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseClientCertificate: WSManSessionFlags = 2097152i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagEnableSPNServerPort: WSManSessionFlags = 4194304i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUTF16: WSManSessionFlags = 8388608i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseCredSsp: WSManSessionFlags = 16777216i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagSkipRevocationCheck: WSManSessionFlags = 33554432i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagAllowNegotiateImplicitCredentials: WSManSessionFlags = 67108864i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSManFlagUseSsl: WSManSessionFlags = 134217728i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManSessionOption = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_DEFAULT_OPERATION_TIMEOUTMS: WSManSessionOption = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_MAX_RETRY_TIME: WSManSessionOption = 11i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_TIMEOUTMS_CREATE_SHELL: WSManSessionOption = 12i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_TIMEOUTMS_RUN_SHELL_COMMAND: WSManSessionOption = 13i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_TIMEOUTMS_RECEIVE_SHELL_OUTPUT: WSManSessionOption = 14i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_TIMEOUTMS_SEND_SHELL_INPUT: WSManSessionOption = 15i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_TIMEOUTMS_SIGNAL_SHELL: WSManSessionOption = 16i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_TIMEOUTMS_CLOSE_SHELL: WSManSessionOption = 17i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_SKIP_CA_CHECK: WSManSessionOption = 18i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_SKIP_CN_CHECK: WSManSessionOption = 19i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_UNENCRYPTED_MESSAGES: WSManSessionOption = 20i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_UTF16: WSManSessionOption = 21i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_ENABLE_SPN_SERVER_PORT: WSManSessionOption = 22i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_MACHINE_ID: WSManSessionOption = 23i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_LOCALE: WSManSessionOption = 25i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_UI_LANGUAGE: WSManSessionOption = 26i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_MAX_ENVELOPE_SIZE_KB: WSManSessionOption = 28i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_SHELL_MAX_DATA_SIZE_PER_MESSAGE_KB: WSManSessionOption = 29i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_REDIRECT_LOCATION: WSManSessionOption = 30i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_SKIP_REVOCATION_CHECK: WSManSessionOption = 31i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS: WSManSessionOption = 32i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_USE_SSL: WSManSessionOption = 33i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_OPTION_USE_INTEARACTIVE_TOKEN: WSManSessionOption = 34i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub type WSManShellFlag = i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_NO_COMPRESSION: WSManShellFlag = 1i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_DELETE_SERVER_SESSION: WSManShellFlag = 2i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_SERVER_BUFFERING_MODE_DROP: WSManShellFlag = 4i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_SERVER_BUFFERING_MODE_BLOCK: WSManShellFlag = 8i32;
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`*"]
pub const WSMAN_FLAG_RECEIVE_DELAY_OUTPUT_STREAM: WSManShellFlag = 16i32;
