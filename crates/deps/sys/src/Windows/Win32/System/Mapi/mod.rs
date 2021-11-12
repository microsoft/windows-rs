#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn MAPIFreeBuffer(pv: *mut ::core::ffi::c_void) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIADDRESS = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszcaption: super::super::Foundation::PSTR, neditfields: u32, lpszlabels: super::super::Foundation::PSTR, nrecips: u32, lprecips: *mut MapiRecipDesc, flflags: u32, ulreserved: u32, lpnnewrecips: *mut u32, lppnewrecips: *mut *mut MapiRecipDesc) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIDELETEMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIDETAILS = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lprecip: *mut MapiRecipDesc, flflags: u32, ulreserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIFINDNEXT = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessagetype: super::super::Foundation::PSTR, lpszseedmessageid: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lpszmessageid: super::super::Foundation::PSTR) -> u32;
pub type LPMAPIFREEBUFFER = unsafe extern "system" fn(pv: *mut ::core::ffi::c_void) -> u32;
pub type LPMAPILOGOFF = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, flflags: u32, ulreserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPILOGON = unsafe extern "system" fn(uluiparam: usize, lpszprofilename: super::super::Foundation::PSTR, lpszpassword: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lplhsession: *mut usize) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIREADMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lppmessage: *mut *mut MapiMessage) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIRESOLVENAME = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszname: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lpprecip: *mut *mut MapiRecipDesc) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISAVEMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32, lpszmessageid: super::super::Foundation::PSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISENDDOCUMENTS = unsafe extern "system" fn(uluiparam: usize, lpszdelimchar: super::super::Foundation::PSTR, lpszfilepaths: super::super::Foundation::PSTR, lpszfilenames: super::super::Foundation::PSTR, ulreserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISENDMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISENDMAILW = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *const MapiMessageW, flflags: u32, ulreserved: u32) -> u32;
pub const MAPI_AB_NOMODIFY: u32 = 1024u32;
pub const MAPI_BCC: u32 = 3u32;
pub const MAPI_BODY_AS_FILE: u32 = 512u32;
pub const MAPI_CC: u32 = 2u32;
pub const MAPI_DIALOG: u32 = 8u32;
pub const MAPI_ENVELOPE_ONLY: u32 = 64u32;
pub const MAPI_EXTENDED: u32 = 32u32;
pub const MAPI_E_ACCESS_DENIED: u32 = 6u32;
pub const MAPI_E_AMBIGUOUS_RECIPIENT: u32 = 21u32;
pub const MAPI_E_AMBIG_RECIP: u32 = 21u32;
pub const MAPI_E_ATTACHMENT_NOT_FOUND: u32 = 11u32;
pub const MAPI_E_ATTACHMENT_OPEN_FAILURE: u32 = 12u32;
pub const MAPI_E_ATTACHMENT_TOO_LARGE: u32 = 28u32;
pub const MAPI_E_ATTACHMENT_WRITE_FAILURE: u32 = 13u32;
pub const MAPI_E_BAD_RECIPTYPE: u32 = 15u32;
pub const MAPI_E_DISK_FULL: u32 = 4u32;
pub const MAPI_E_FAILURE: u32 = 2u32;
pub const MAPI_E_INSUFFICIENT_MEMORY: u32 = 5u32;
pub const MAPI_E_INVALID_EDITFIELDS: u32 = 24u32;
pub const MAPI_E_INVALID_MESSAGE: u32 = 17u32;
pub const MAPI_E_INVALID_RECIPS: u32 = 25u32;
pub const MAPI_E_INVALID_SESSION: u32 = 19u32;
pub const MAPI_E_LOGIN_FAILURE: u32 = 3u32;
pub const MAPI_E_LOGON_FAILURE: u32 = 3u32;
pub const MAPI_E_MESSAGE_IN_USE: u32 = 22u32;
pub const MAPI_E_NETWORK_FAILURE: u32 = 23u32;
pub const MAPI_E_NOT_SUPPORTED: u32 = 26u32;
pub const MAPI_E_NO_MESSAGES: u32 = 16u32;
pub const MAPI_E_TEXT_TOO_LARGE: u32 = 18u32;
pub const MAPI_E_TOO_MANY_FILES: u32 = 9u32;
pub const MAPI_E_TOO_MANY_RECIPIENTS: u32 = 10u32;
pub const MAPI_E_TOO_MANY_SESSIONS: u32 = 8u32;
pub const MAPI_E_TYPE_NOT_SUPPORTED: u32 = 20u32;
pub const MAPI_E_UNICODE_NOT_SUPPORTED: u32 = 27u32;
pub const MAPI_E_UNKNOWN_RECIPIENT: u32 = 14u32;
pub const MAPI_E_USER_ABORT: u32 = 1u32;
pub const MAPI_FORCE_DOWNLOAD: u32 = 4096u32;
pub const MAPI_FORCE_UNICODE: u32 = 262144u32;
pub const MAPI_GUARANTEE_FIFO: u32 = 256u32;
pub const MAPI_LOGON_UI: u32 = 1u32;
pub const MAPI_LONG_MSGID: u32 = 16384u32;
pub const MAPI_NEW_SESSION: u32 = 2u32;
pub const MAPI_OLE: u32 = 1u32;
pub const MAPI_OLE_STATIC: u32 = 2u32;
pub const MAPI_ORIG: u32 = 0u32;
pub const MAPI_PASSWORD_UI: u32 = 131072u32;
pub const MAPI_PEEK: u32 = 128u32;
pub const MAPI_RECEIPT_REQUESTED: u32 = 2u32;
pub const MAPI_SENT: u32 = 4u32;
pub const MAPI_SUPPRESS_ATTACH: u32 = 2048u32;
pub const MAPI_TO: u32 = 1u32;
pub const MAPI_UNREAD: u32 = 1u32;
pub const MAPI_UNREAD_ONLY: u32 = 32u32;
pub const MAPI_USER_ABORT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MapiFileDesc(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MapiFileDescW(i32);
#[repr(C)]
pub struct MapiFileTagExt(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MapiMessage(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MapiMessageW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MapiRecipDesc(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MapiRecipDescW(i32);
pub const SUCCESS_SUCCESS: u32 = 0u32;
