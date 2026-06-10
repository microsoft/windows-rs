#[inline]
pub unsafe fn MAPIFreeBuffer(pv: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("mapi32.dll" "system" fn MAPIFreeBuffer(pv : *mut core::ffi::c_void) -> u32);
    unsafe { MAPIFreeBuffer(pv as _) }
}
pub type LPMAPIADDRESS = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszcaption: windows_core::PCSTR, neditfields: u32, lpszlabels: windows_core::PCSTR, nrecips: u32, lprecips: *mut MapiRecipDesc, flflags: u32, ulreserved: u32, lpnnewrecips: *mut u32, lppnewrecips: *mut *mut MapiRecipDesc) -> u32>;
pub type LPMAPIDELETEMAIL = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: windows_core::PCSTR, flflags: u32, ulreserved: u32) -> u32>;
pub type LPMAPIDETAILS = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lprecip: *mut MapiRecipDesc, flflags: u32, ulreserved: u32) -> u32>;
pub type LPMAPIFINDNEXT = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessagetype: windows_core::PCSTR, lpszseedmessageid: windows_core::PCSTR, flflags: u32, ulreserved: u32, lpszmessageid: windows_core::PCSTR) -> u32>;
pub type LPMAPIFREEBUFFER = Option<unsafe extern "system" fn(pv: *mut core::ffi::c_void) -> u32>;
pub type LPMAPILOGOFF = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, flflags: u32, ulreserved: u32) -> u32>;
pub type LPMAPILOGON = Option<unsafe extern "system" fn(uluiparam: usize, lpszprofilename: windows_core::PCSTR, lpszpassword: windows_core::PCSTR, flflags: u32, ulreserved: u32, lplhsession: *mut usize) -> u32>;
pub type LPMAPIREADMAIL = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: windows_core::PCSTR, flflags: u32, ulreserved: u32, lppmessage: *mut *mut MapiMessage) -> u32>;
pub type LPMAPIRESOLVENAME = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszname: windows_core::PCSTR, flflags: u32, ulreserved: u32, lpprecip: *mut *mut MapiRecipDesc) -> u32>;
pub type LPMAPISAVEMAIL = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32, lpszmessageid: windows_core::PCSTR) -> u32>;
pub type LPMAPISENDDOCUMENTS = Option<unsafe extern "system" fn(uluiparam: usize, lpszdelimchar: windows_core::PCSTR, lpszfilepaths: windows_core::PCSTR, lpszfilenames: windows_core::PCSTR, ulreserved: u32) -> u32>;
pub type LPMAPISENDMAIL = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32) -> u32>;
pub type LPMAPISENDMAILW = Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *const MapiMessageW, flflags: u32, ulreserved: u32) -> u32>;
pub const MAPI_AB_NOMODIFY: u32 = 1024;
pub const MAPI_BCC: u32 = 3;
pub const MAPI_BODY_AS_FILE: u32 = 512;
pub const MAPI_CC: u32 = 2;
pub const MAPI_DIALOG: u32 = 8;
pub const MAPI_ENVELOPE_ONLY: u32 = 64;
pub const MAPI_EXTENDED: u32 = 32;
pub const MAPI_E_ACCESS_DENIED: u32 = 6;
pub const MAPI_E_AMBIGUOUS_RECIPIENT: u32 = 21;
pub const MAPI_E_AMBIG_RECIP: u32 = 21;
pub const MAPI_E_ATTACHMENT_NOT_FOUND: u32 = 11;
pub const MAPI_E_ATTACHMENT_OPEN_FAILURE: u32 = 12;
pub const MAPI_E_ATTACHMENT_TOO_LARGE: u32 = 28;
pub const MAPI_E_ATTACHMENT_WRITE_FAILURE: u32 = 13;
pub const MAPI_E_BAD_RECIPTYPE: u32 = 15;
pub const MAPI_E_DISK_FULL: u32 = 4;
pub const MAPI_E_FAILURE: u32 = 2;
pub const MAPI_E_INSUFFICIENT_MEMORY: u32 = 5;
pub const MAPI_E_INVALID_EDITFIELDS: u32 = 24;
pub const MAPI_E_INVALID_MESSAGE: u32 = 17;
pub const MAPI_E_INVALID_RECIPS: u32 = 25;
pub const MAPI_E_INVALID_SESSION: u32 = 19;
pub const MAPI_E_LOGIN_FAILURE: u32 = 3;
pub const MAPI_E_LOGON_FAILURE: u32 = 3;
pub const MAPI_E_MESSAGE_IN_USE: u32 = 22;
pub const MAPI_E_NETWORK_FAILURE: u32 = 23;
pub const MAPI_E_NOT_SUPPORTED: u32 = 26;
pub const MAPI_E_NO_MESSAGES: u32 = 16;
pub const MAPI_E_TEXT_TOO_LARGE: u32 = 18;
pub const MAPI_E_TOO_MANY_FILES: u32 = 9;
pub const MAPI_E_TOO_MANY_RECIPIENTS: u32 = 10;
pub const MAPI_E_TOO_MANY_SESSIONS: u32 = 8;
pub const MAPI_E_TYPE_NOT_SUPPORTED: u32 = 20;
pub const MAPI_E_UNICODE_NOT_SUPPORTED: u32 = 27;
pub const MAPI_E_UNKNOWN_RECIPIENT: u32 = 14;
pub const MAPI_E_USER_ABORT: u32 = 1;
pub const MAPI_FORCE_DOWNLOAD: u32 = 4096;
pub const MAPI_FORCE_UNICODE: u32 = 262144;
pub const MAPI_GUARANTEE_FIFO: u32 = 256;
pub const MAPI_LOGON_UI: u32 = 1;
pub const MAPI_LONG_MSGID: u32 = 16384;
pub const MAPI_NEW_SESSION: u32 = 2;
pub const MAPI_OLE: u32 = 1;
pub const MAPI_OLE_STATIC: u32 = 2;
pub const MAPI_ORIG: u32 = 0;
pub const MAPI_PASSWORD_UI: u32 = 131072;
pub const MAPI_PEEK: u32 = 128;
pub const MAPI_RECEIPT_REQUESTED: u32 = 2;
pub const MAPI_SENT: u32 = 4;
pub const MAPI_SUPPRESS_ATTACH: u32 = 2048;
pub const MAPI_TO: u32 = 1;
pub const MAPI_UNREAD: u32 = 1;
pub const MAPI_UNREAD_ONLY: u32 = 32;
pub const MAPI_USER_ABORT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapiFileDesc {
    pub ulReserved: u32,
    pub flFlags: u32,
    pub nPosition: u32,
    pub lpszPathName: windows_core::PSTR,
    pub lpszFileName: windows_core::PSTR,
    pub lpFileType: *mut core::ffi::c_void,
}
impl Default for MapiFileDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapiFileDescW {
    pub ulReserved: u32,
    pub flFlags: u32,
    pub nPosition: u32,
    pub lpszPathName: windows_core::PWSTR,
    pub lpszFileName: windows_core::PWSTR,
    pub lpFileType: *mut core::ffi::c_void,
}
impl Default for MapiFileDescW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapiFileTagExt {
    pub ulReserved: u32,
    pub cbTag: u32,
    pub lpTag: *mut u8,
    pub cbEncoding: u32,
    pub lpEncoding: *mut u8,
}
impl Default for MapiFileTagExt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapiMessage {
    pub ulReserved: u32,
    pub lpszSubject: windows_core::PSTR,
    pub lpszNoteText: windows_core::PSTR,
    pub lpszMessageType: windows_core::PSTR,
    pub lpszDateReceived: windows_core::PSTR,
    pub lpszConversationID: windows_core::PSTR,
    pub flFlags: u32,
    pub lpOriginator: *mut MapiRecipDesc,
    pub nRecipCount: u32,
    pub lpRecips: *mut MapiRecipDesc,
    pub nFileCount: u32,
    pub lpFiles: *mut MapiFileDesc,
}
impl Default for MapiMessage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapiMessageW {
    pub ulReserved: u32,
    pub lpszSubject: windows_core::PWSTR,
    pub lpszNoteText: windows_core::PWSTR,
    pub lpszMessageType: windows_core::PWSTR,
    pub lpszDateReceived: windows_core::PWSTR,
    pub lpszConversationID: windows_core::PWSTR,
    pub flFlags: u32,
    pub lpOriginator: *mut MapiRecipDescW,
    pub nRecipCount: u32,
    pub lpRecips: *mut MapiRecipDescW,
    pub nFileCount: u32,
    pub lpFiles: *mut MapiFileDescW,
}
impl Default for MapiMessageW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapiRecipDesc {
    pub ulReserved: u32,
    pub ulRecipClass: u32,
    pub lpszName: windows_core::PSTR,
    pub lpszAddress: windows_core::PSTR,
    pub ulEIDSize: u32,
    pub lpEntryID: *mut core::ffi::c_void,
}
impl Default for MapiRecipDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapiRecipDescW {
    pub ulReserved: u32,
    pub ulRecipClass: u32,
    pub lpszName: windows_core::PWSTR,
    pub lpszAddress: windows_core::PWSTR,
    pub ulEIDSize: u32,
    pub lpEntryID: *mut core::ffi::c_void,
}
impl Default for MapiRecipDescW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SUCCESS_SUCCESS: u32 = 0;
