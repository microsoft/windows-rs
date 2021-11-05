#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIADDRESS = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszcaption: super::super::Foundation::PSTR, neditfields: u32, lpszlabels: super::super::Foundation::PSTR, nrecips: u32, lprecips: *mut MapiRecipDesc, flflags: u32, ulreserved: u32, lpnnewrecips: *mut u32, lppnewrecips: *mut *mut MapiRecipDesc) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIDELETEMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIDETAILS = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lprecip: *mut MapiRecipDesc, flflags: u32, ulreserved: u32) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIFINDNEXT = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessagetype: super::super::Foundation::PSTR, lpszseedmessageid: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lpszmessageid: super::super::Foundation::PSTR) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub type LPMAPIFREEBUFFER = unsafe extern "system" fn(pv: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub type LPMAPILOGOFF = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, flflags: u32, ulreserved: u32) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPILOGON = unsafe extern "system" fn(uluiparam: usize, lpszprofilename: super::super::Foundation::PSTR, lpszpassword: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lplhsession: *mut usize) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIREADMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lppmessage: *mut *mut MapiMessage) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPIRESOLVENAME = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszname: super::super::Foundation::PSTR, flflags: u32, ulreserved: u32, lpprecip: *mut *mut MapiRecipDesc) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISAVEMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32, lpszmessageid: super::super::Foundation::PSTR) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISENDDOCUMENTS = unsafe extern "system" fn(uluiparam: usize, lpszdelimchar: super::super::Foundation::PSTR, lpszfilepaths: super::super::Foundation::PSTR, lpszfilenames: super::super::Foundation::PSTR, ulreserved: u32) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISENDMAIL = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMAPISENDMAILW = unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *const MapiMessageW, flflags: u32, ulreserved: u32) -> u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
#[inline]
pub unsafe fn MAPIFreeBuffer(pv: *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MAPIFreeBuffer(pv: *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MAPIFreeBuffer(::std::mem::transmute(pv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_AB_NOMODIFY: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_BCC: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_BODY_AS_FILE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_CC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_DIALOG: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_ENVELOPE_ONLY: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_EXTENDED: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_ACCESS_DENIED: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_AMBIGUOUS_RECIPIENT: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_AMBIG_RECIP: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_ATTACHMENT_NOT_FOUND: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_ATTACHMENT_OPEN_FAILURE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_ATTACHMENT_TOO_LARGE: u32 = 28u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_ATTACHMENT_WRITE_FAILURE: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_BAD_RECIPTYPE: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_DISK_FULL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_FAILURE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_INSUFFICIENT_MEMORY: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_INVALID_EDITFIELDS: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_INVALID_MESSAGE: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_INVALID_RECIPS: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_INVALID_SESSION: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_LOGIN_FAILURE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_LOGON_FAILURE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_MESSAGE_IN_USE: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_NETWORK_FAILURE: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_NOT_SUPPORTED: u32 = 26u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_NO_MESSAGES: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_TEXT_TOO_LARGE: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_TOO_MANY_FILES: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_TOO_MANY_RECIPIENTS: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_TOO_MANY_SESSIONS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_TYPE_NOT_SUPPORTED: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_UNICODE_NOT_SUPPORTED: u32 = 27u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_UNKNOWN_RECIPIENT: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_E_USER_ABORT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_FORCE_DOWNLOAD: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_FORCE_UNICODE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_GUARANTEE_FIFO: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_LOGON_UI: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_LONG_MSGID: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_NEW_SESSION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_OLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_OLE_STATIC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_ORIG: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_PASSWORD_UI: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_PEEK: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_RECEIPT_REQUESTED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_SENT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_SUPPRESS_ATTACH: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_TO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_UNREAD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_UNREAD_ONLY: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const MAPI_USER_ABORT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
pub struct MapiFileDesc {
    pub ulReserved: u32,
    pub flFlags: u32,
    pub nPosition: u32,
    pub lpszPathName: super::super::Foundation::PSTR,
    pub lpszFileName: super::super::Foundation::PSTR,
    pub lpFileType: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl MapiFileDesc {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MapiFileDesc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MapiFileDesc {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MapiFileDesc").field("ulReserved", &self.ulReserved).field("flFlags", &self.flFlags).field("nPosition", &self.nPosition).field("lpszPathName", &self.lpszPathName).field("lpszFileName", &self.lpszFileName).field("lpFileType", &self.lpFileType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MapiFileDesc {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.flFlags == other.flFlags && self.nPosition == other.nPosition && self.lpszPathName == other.lpszPathName && self.lpszFileName == other.lpszFileName && self.lpFileType == other.lpFileType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MapiFileDesc {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MapiFileDesc {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
pub struct MapiFileDescW {
    pub ulReserved: u32,
    pub flFlags: u32,
    pub nPosition: u32,
    pub lpszPathName: super::super::Foundation::PWSTR,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub lpFileType: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl MapiFileDescW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MapiFileDescW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MapiFileDescW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MapiFileDescW").field("ulReserved", &self.ulReserved).field("flFlags", &self.flFlags).field("nPosition", &self.nPosition).field("lpszPathName", &self.lpszPathName).field("lpszFileName", &self.lpszFileName).field("lpFileType", &self.lpFileType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MapiFileDescW {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.flFlags == other.flFlags && self.nPosition == other.nPosition && self.lpszPathName == other.lpszPathName && self.lpszFileName == other.lpszFileName && self.lpFileType == other.lpFileType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MapiFileDescW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MapiFileDescW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub struct MapiFileTagExt {
    pub ulReserved: u32,
    pub cbTag: u32,
    pub lpTag: *mut u8,
    pub cbEncoding: u32,
    pub lpEncoding: *mut u8,
}
impl MapiFileTagExt {}
impl ::std::default::Default for MapiFileTagExt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MapiFileTagExt {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MapiFileTagExt").field("ulReserved", &self.ulReserved).field("cbTag", &self.cbTag).field("lpTag", &self.lpTag).field("cbEncoding", &self.cbEncoding).field("lpEncoding", &self.lpEncoding).finish()
    }
}
impl ::std::cmp::PartialEq for MapiFileTagExt {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.cbTag == other.cbTag && self.lpTag == other.lpTag && self.cbEncoding == other.cbEncoding && self.lpEncoding == other.lpEncoding
    }
}
impl ::std::cmp::Eq for MapiFileTagExt {}
unsafe impl ::windows::runtime::Abi for MapiFileTagExt {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
pub struct MapiMessage {
    pub ulReserved: u32,
    pub lpszSubject: super::super::Foundation::PSTR,
    pub lpszNoteText: super::super::Foundation::PSTR,
    pub lpszMessageType: super::super::Foundation::PSTR,
    pub lpszDateReceived: super::super::Foundation::PSTR,
    pub lpszConversationID: super::super::Foundation::PSTR,
    pub flFlags: u32,
    pub lpOriginator: *mut MapiRecipDesc,
    pub nRecipCount: u32,
    pub lpRecips: *mut MapiRecipDesc,
    pub nFileCount: u32,
    pub lpFiles: *mut MapiFileDesc,
}
#[cfg(feature = "Win32_Foundation")]
impl MapiMessage {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MapiMessage {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MapiMessage {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MapiMessage")
            .field("ulReserved", &self.ulReserved)
            .field("lpszSubject", &self.lpszSubject)
            .field("lpszNoteText", &self.lpszNoteText)
            .field("lpszMessageType", &self.lpszMessageType)
            .field("lpszDateReceived", &self.lpszDateReceived)
            .field("lpszConversationID", &self.lpszConversationID)
            .field("flFlags", &self.flFlags)
            .field("lpOriginator", &self.lpOriginator)
            .field("nRecipCount", &self.nRecipCount)
            .field("lpRecips", &self.lpRecips)
            .field("nFileCount", &self.nFileCount)
            .field("lpFiles", &self.lpFiles)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MapiMessage {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved
            && self.lpszSubject == other.lpszSubject
            && self.lpszNoteText == other.lpszNoteText
            && self.lpszMessageType == other.lpszMessageType
            && self.lpszDateReceived == other.lpszDateReceived
            && self.lpszConversationID == other.lpszConversationID
            && self.flFlags == other.flFlags
            && self.lpOriginator == other.lpOriginator
            && self.nRecipCount == other.nRecipCount
            && self.lpRecips == other.lpRecips
            && self.nFileCount == other.nFileCount
            && self.lpFiles == other.lpFiles
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MapiMessage {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MapiMessage {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
pub struct MapiMessageW {
    pub ulReserved: u32,
    pub lpszSubject: super::super::Foundation::PWSTR,
    pub lpszNoteText: super::super::Foundation::PWSTR,
    pub lpszMessageType: super::super::Foundation::PWSTR,
    pub lpszDateReceived: super::super::Foundation::PWSTR,
    pub lpszConversationID: super::super::Foundation::PWSTR,
    pub flFlags: u32,
    pub lpOriginator: *mut MapiRecipDescW,
    pub nRecipCount: u32,
    pub lpRecips: *mut MapiRecipDescW,
    pub nFileCount: u32,
    pub lpFiles: *mut MapiFileDescW,
}
#[cfg(feature = "Win32_Foundation")]
impl MapiMessageW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MapiMessageW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MapiMessageW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MapiMessageW")
            .field("ulReserved", &self.ulReserved)
            .field("lpszSubject", &self.lpszSubject)
            .field("lpszNoteText", &self.lpszNoteText)
            .field("lpszMessageType", &self.lpszMessageType)
            .field("lpszDateReceived", &self.lpszDateReceived)
            .field("lpszConversationID", &self.lpszConversationID)
            .field("flFlags", &self.flFlags)
            .field("lpOriginator", &self.lpOriginator)
            .field("nRecipCount", &self.nRecipCount)
            .field("lpRecips", &self.lpRecips)
            .field("nFileCount", &self.nFileCount)
            .field("lpFiles", &self.lpFiles)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MapiMessageW {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved
            && self.lpszSubject == other.lpszSubject
            && self.lpszNoteText == other.lpszNoteText
            && self.lpszMessageType == other.lpszMessageType
            && self.lpszDateReceived == other.lpszDateReceived
            && self.lpszConversationID == other.lpszConversationID
            && self.flFlags == other.flFlags
            && self.lpOriginator == other.lpOriginator
            && self.nRecipCount == other.nRecipCount
            && self.lpRecips == other.lpRecips
            && self.nFileCount == other.nFileCount
            && self.lpFiles == other.lpFiles
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MapiMessageW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MapiMessageW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
pub struct MapiRecipDesc {
    pub ulReserved: u32,
    pub ulRecipClass: u32,
    pub lpszName: super::super::Foundation::PSTR,
    pub lpszAddress: super::super::Foundation::PSTR,
    pub ulEIDSize: u32,
    pub lpEntryID: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl MapiRecipDesc {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MapiRecipDesc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MapiRecipDesc {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MapiRecipDesc").field("ulReserved", &self.ulReserved).field("ulRecipClass", &self.ulRecipClass).field("lpszName", &self.lpszName).field("lpszAddress", &self.lpszAddress).field("ulEIDSize", &self.ulEIDSize).field("lpEntryID", &self.lpEntryID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MapiRecipDesc {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.ulRecipClass == other.ulRecipClass && self.lpszName == other.lpszName && self.lpszAddress == other.lpszAddress && self.ulEIDSize == other.ulEIDSize && self.lpEntryID == other.lpEntryID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MapiRecipDesc {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MapiRecipDesc {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Mapi`, `Win32_Foundation`*"]
pub struct MapiRecipDescW {
    pub ulReserved: u32,
    pub ulRecipClass: u32,
    pub lpszName: super::super::Foundation::PWSTR,
    pub lpszAddress: super::super::Foundation::PWSTR,
    pub ulEIDSize: u32,
    pub lpEntryID: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl MapiRecipDescW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MapiRecipDescW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MapiRecipDescW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MapiRecipDescW").field("ulReserved", &self.ulReserved).field("ulRecipClass", &self.ulRecipClass).field("lpszName", &self.lpszName).field("lpszAddress", &self.lpszAddress).field("ulEIDSize", &self.ulEIDSize).field("lpEntryID", &self.lpEntryID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MapiRecipDescW {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.ulRecipClass == other.ulRecipClass && self.lpszName == other.lpszName && self.lpszAddress == other.lpszAddress && self.ulEIDSize == other.ulEIDSize && self.lpEntryID == other.lpEntryID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MapiRecipDescW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MapiRecipDescW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Mapi`*"]
pub const SUCCESS_SUCCESS: u32 = 0u32;
