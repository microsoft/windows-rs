#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn MatchEnumTag(hmodule: super::super::Foundation::HANDLE, pwcarg: super::super::Foundation::PWSTR, dwnumarg: u32, penumtable: *const TOKEN_VALUE, pdwvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MatchToken(pwszusertoken: super::super::Foundation::PWSTR, pwszcmdtoken: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PreprocessCommand(hmodule: super::super::Foundation::HANDLE, ppwcarguments: *mut super::super::Foundation::PWSTR, dwcurrentindex: u32, dwargcount: u32, ptttags: *mut TAG_TYPE, dwtagcount: u32, dwminargs: u32, dwmaxargs: u32, pdwtagtype: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintError(hmodule: super::super::Foundation::HANDLE, dwerrid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintMessage(pwszformat: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintMessageFromModule(hmodule: super::super::Foundation::HANDLE, dwmsgid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterContext(pchildcontext: *const NS_CONTEXT_ATTRIBUTES) -> u32;
    pub fn RegisterHelper(pguidparentcontext: *const ::windows_sys::core::GUID, pfnregistersubcontext: *const NS_HELPER_ATTRIBUTES) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
pub struct CMD_ENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CMD_GROUP_ENTRY(i32);
pub const DEFAULT_CONTEXT_PRIORITY: u32 = 100u32;
pub const ERROR_CMD_NOT_FOUND: u32 = 15004u32;
pub const ERROR_CONTEXT_ALREADY_REGISTERED: u32 = 15019u32;
pub const ERROR_CONTINUE_IN_PARENT_CONTEXT: u32 = 15016u32;
pub const ERROR_DLL_LOAD_FAILED: u32 = 15006u32;
pub const ERROR_ENTRY_PT_NOT_FOUND: u32 = 15005u32;
pub const ERROR_HELPER_ALREADY_REGISTERED: u32 = 15018u32;
pub const ERROR_INIT_DISPLAY: u32 = 15007u32;
pub const ERROR_INVALID_OPTION_TAG: u32 = 15009u32;
pub const ERROR_INVALID_OPTION_VALUE: u32 = 15014u32;
pub const ERROR_INVALID_SYNTAX: u32 = 15001u32;
pub const ERROR_MISSING_OPTION: u32 = 15011u32;
pub const ERROR_NO_CHANGE: u32 = 15003u32;
pub const ERROR_NO_ENTRIES: u32 = 15000u32;
pub const ERROR_NO_TAG: u32 = 15010u32;
pub const ERROR_OKAY: u32 = 15015u32;
pub const ERROR_PARSING_FAILURE: u32 = 15020u32;
pub const ERROR_PROTOCOL_NOT_IN_TRANSPORT: u32 = 15002u32;
pub const ERROR_SHOW_USAGE: u32 = 15013u32;
pub const ERROR_SUPPRESS_OUTPUT: u32 = 15017u32;
pub const ERROR_TAG_ALREADY_PRESENT: u32 = 15008u32;
pub const ERROR_TRANSPORT_NOT_PRESENT: u32 = 15012u32;
pub const MAX_NAME_LEN: u32 = 48u32;
pub const NETSH_ERROR_BASE: u32 = 15000u32;
pub const NETSH_ERROR_END: u32 = 15019u32;
pub const NETSH_MAX_CMD_TOKEN_LENGTH: u32 = 128u32;
pub const NETSH_MAX_TOKEN_LENGTH: u32 = 64u32;
pub const NETSH_VERSION_50: u32 = 20480u32;
pub struct NS_CMD_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NS_CONTEXT_ATTRIBUTES(i32);
pub struct NS_EVENTS(i32);
pub struct NS_HELPER_ATTRIBUTES(i32);
pub struct NS_MODE_CHANGE(i32);
pub struct NS_REQS(i32);
pub struct PFN_HANDLE_CMD(i32);
pub struct PGET_RESOURCE_STRING_FN(i32);
pub struct PNS_CONTEXT_COMMIT_FN(i32);
pub struct PNS_CONTEXT_CONNECT_FN(i32);
pub struct PNS_CONTEXT_DUMP_FN(i32);
pub struct PNS_DLL_INIT_FN(i32);
pub struct PNS_DLL_STOP_FN(i32);
pub struct PNS_HELPER_START_FN(i32);
pub struct PNS_HELPER_STOP_FN(i32);
pub struct PNS_OSVERSIONCHECK(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TAG_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_VALUE(i32);
