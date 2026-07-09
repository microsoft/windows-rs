#[cfg(feature = "Win32_windef")]
pub type CURLINVOKECOMMANDINFOA = URLINVOKECOMMANDINFOA;
#[cfg(feature = "Win32_windef")]
pub type CURLINVOKECOMMANDINFOW = URLINVOKECOMMANDINFOW;
pub const E_FLAGS: i32 = -2147217408;
pub const IS_E_EXEC_FAILED: i32 = -2147213310;
pub type IURL_INVOKECOMMAND_FLAGS = i32;
pub const IURL_INVOKECOMMAND_FL_ALLOW_UI: IURL_INVOKECOMMAND_FLAGS = 1;
pub const IURL_INVOKECOMMAND_FL_ASYNCOK: IURL_INVOKECOMMAND_FLAGS = 8;
pub const IURL_INVOKECOMMAND_FL_DDEWAIT: IURL_INVOKECOMMAND_FLAGS = 4;
pub const IURL_INVOKECOMMAND_FL_LOG_USAGE: IURL_INVOKECOMMAND_FLAGS = 16;
pub const IURL_INVOKECOMMAND_FL_USE_DEFAULT_VERB: IURL_INVOKECOMMAND_FLAGS = 2;
pub type IURL_SETURL_FLAGS = i32;
pub const IURL_SETURL_FL_GUESS_PROTOCOL: IURL_SETURL_FLAGS = 1;
pub const IURL_SETURL_FL_USE_DEFAULT_PROTOCOL: IURL_SETURL_FLAGS = 2;
pub const MIMEASSOCDLG_FL_REGISTER_ASSOC: MIMEASSOCIATIONDIALOG_IN_FLAGS = 1;
pub type MIMEASSOCIATIONDIALOG_IN_FLAGS = i32;
#[cfg(feature = "Win32_windef")]
pub type PCURLINVOKECOMMANDINFOA = *const URLINVOKECOMMANDINFOA;
#[cfg(feature = "Win32_windef")]
pub type PCURLINVOKECOMMANDINFOW = *const URLINVOKECOMMANDINFOW;
#[cfg(feature = "Win32_windef")]
pub type PURLINVOKECOMMANDINFOA = *mut URLINVOKECOMMANDINFOA;
#[cfg(feature = "Win32_windef")]
pub type PURLINVOKECOMMANDINFOW = *mut URLINVOKECOMMANDINFOW;
pub const TRANSLATEURL_FL_GUESS_PROTOCOL: TRANSLATEURL_IN_FLAGS = 1;
pub const TRANSLATEURL_FL_USE_DEFAULT_PROTOCOL: TRANSLATEURL_IN_FLAGS = 2;
pub type TRANSLATEURL_IN_FLAGS = i32;
pub const URLASSOCDLG_FL_REGISTER_ASSOC: URLASSOCIATIONDIALOG_IN_FLAGS = 2;
pub const URLASSOCDLG_FL_USE_DEFAULT_NAME: URLASSOCIATIONDIALOG_IN_FLAGS = 1;
pub type URLASSOCIATIONDIALOG_IN_FLAGS = i32;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct URLINVOKECOMMANDINFOA {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::windef::HWND,
    pub pcszVerb: windows_sys::core::PCSTR,
}
#[cfg(feature = "Win32_windef")]
impl Default for URLINVOKECOMMANDINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct URLINVOKECOMMANDINFOW {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::windef::HWND,
    pub pcszVerb: windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_windef")]
impl Default for URLINVOKECOMMANDINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const URL_E_INVALID_SYNTAX: i32 = -2147217407;
pub const URL_E_UNREGISTERED_PROTOCOL: i32 = -2147217406;
