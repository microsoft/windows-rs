#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPIADDRESS = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszcaption: ::windows::core::PCSTR, neditfields: u32, lpszlabels: ::windows::core::PCSTR, nrecips: u32, lprecips: *mut MapiRecipDesc, flflags: u32, ulreserved: u32, lpnnewrecips: *mut u32, lppnewrecips: *mut *mut MapiRecipDesc) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPIDELETEMAIL = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: ::windows::core::PCSTR, flflags: u32, ulreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPIDETAILS = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lprecip: *mut MapiRecipDesc, flflags: u32, ulreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPIFINDNEXT = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessagetype: ::windows::core::PCSTR, lpszseedmessageid: ::windows::core::PCSTR, flflags: u32, ulreserved: u32, lpszmessageid: ::windows::core::PCSTR) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPIFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(pv: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPILOGOFF = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, flflags: u32, ulreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPILOGON = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpszprofilename: ::windows::core::PCSTR, lpszpassword: ::windows::core::PCSTR, flflags: u32, ulreserved: u32, lplhsession: *mut usize) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPIREADMAIL = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszmessageid: ::windows::core::PCSTR, flflags: u32, ulreserved: u32, lppmessage: *mut *mut MapiMessage) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPIRESOLVENAME = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpszname: ::windows::core::PCSTR, flflags: u32, ulreserved: u32, lpprecip: *mut *mut MapiRecipDesc) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPISAVEMAIL = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32, lpszmessageid: ::windows::core::PCSTR) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPISENDDOCUMENTS = ::core::option::Option<unsafe extern "system" fn(uluiparam: usize, lpszdelimchar: ::windows::core::PCSTR, lpszfilepaths: ::windows::core::PCSTR, lpszfilenames: ::windows::core::PCSTR, ulreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPISENDMAIL = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *mut MapiMessage, flflags: u32, ulreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub type LPMAPISENDMAILW = ::core::option::Option<unsafe extern "system" fn(lhsession: usize, uluiparam: usize, lpmessage: *const MapiMessageW, flflags: u32, ulreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
#[inline]
pub unsafe fn MAPIFreeBuffer(pv: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MAPIFreeBuffer(pv: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(MAPIFreeBuffer(::core::mem::transmute(pv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_AB_NOMODIFY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_BCC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_BODY_AS_FILE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_CC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_DIALOG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_ENVELOPE_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_EXTENDED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_ACCESS_DENIED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_AMBIGUOUS_RECIPIENT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_AMBIG_RECIP: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_ATTACHMENT_NOT_FOUND: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_ATTACHMENT_OPEN_FAILURE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_ATTACHMENT_TOO_LARGE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_ATTACHMENT_WRITE_FAILURE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_BAD_RECIPTYPE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_DISK_FULL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_FAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_INSUFFICIENT_MEMORY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_INVALID_EDITFIELDS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_INVALID_MESSAGE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_INVALID_RECIPS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_INVALID_SESSION: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_LOGIN_FAILURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_LOGON_FAILURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_MESSAGE_IN_USE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_NETWORK_FAILURE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_NOT_SUPPORTED: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_NO_MESSAGES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_TEXT_TOO_LARGE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_TOO_MANY_FILES: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_TOO_MANY_RECIPIENTS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_TOO_MANY_SESSIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_TYPE_NOT_SUPPORTED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_UNICODE_NOT_SUPPORTED: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_UNKNOWN_RECIPIENT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_E_USER_ABORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_FORCE_DOWNLOAD: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_FORCE_UNICODE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_GUARANTEE_FIFO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_LOGON_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_LONG_MSGID: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_NEW_SESSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_OLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_OLE_STATIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_ORIG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_PASSWORD_UI: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_PEEK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_RECEIPT_REQUESTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_SENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_SUPPRESS_ATTACH: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_TO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_UNREAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_UNREAD_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const MAPI_USER_ABORT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub struct MapiFileDesc {
    pub ulReserved: u32,
    pub flFlags: u32,
    pub nPosition: u32,
    pub lpszPathName: ::windows::core::PSTR,
    pub lpszFileName: ::windows::core::PSTR,
    pub lpFileType: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MapiFileDesc {}
impl ::core::clone::Clone for MapiFileDesc {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapiFileDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiFileDesc").field("ulReserved", &self.ulReserved).field("flFlags", &self.flFlags).field("nPosition", &self.nPosition).field("lpszPathName", &self.lpszPathName).field("lpszFileName", &self.lpszFileName).field("lpFileType", &self.lpFileType).finish()
    }
}
unsafe impl ::windows::core::Abi for MapiFileDesc {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MapiFileDesc {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapiFileDesc>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapiFileDesc {}
impl ::core::default::Default for MapiFileDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub struct MapiFileDescW {
    pub ulReserved: u32,
    pub flFlags: u32,
    pub nPosition: u32,
    pub lpszPathName: ::windows::core::PWSTR,
    pub lpszFileName: ::windows::core::PWSTR,
    pub lpFileType: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MapiFileDescW {}
impl ::core::clone::Clone for MapiFileDescW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapiFileDescW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiFileDescW").field("ulReserved", &self.ulReserved).field("flFlags", &self.flFlags).field("nPosition", &self.nPosition).field("lpszPathName", &self.lpszPathName).field("lpszFileName", &self.lpszFileName).field("lpFileType", &self.lpFileType).finish()
    }
}
unsafe impl ::windows::core::Abi for MapiFileDescW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MapiFileDescW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapiFileDescW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapiFileDescW {}
impl ::core::default::Default for MapiFileDescW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub struct MapiFileTagExt {
    pub ulReserved: u32,
    pub cbTag: u32,
    pub lpTag: *mut u8,
    pub cbEncoding: u32,
    pub lpEncoding: *mut u8,
}
impl ::core::marker::Copy for MapiFileTagExt {}
impl ::core::clone::Clone for MapiFileTagExt {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapiFileTagExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiFileTagExt").field("ulReserved", &self.ulReserved).field("cbTag", &self.cbTag).field("lpTag", &self.lpTag).field("cbEncoding", &self.cbEncoding).field("lpEncoding", &self.lpEncoding).finish()
    }
}
unsafe impl ::windows::core::Abi for MapiFileTagExt {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MapiFileTagExt {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapiFileTagExt>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapiFileTagExt {}
impl ::core::default::Default for MapiFileTagExt {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub struct MapiMessage {
    pub ulReserved: u32,
    pub lpszSubject: ::windows::core::PSTR,
    pub lpszNoteText: ::windows::core::PSTR,
    pub lpszMessageType: ::windows::core::PSTR,
    pub lpszDateReceived: ::windows::core::PSTR,
    pub lpszConversationID: ::windows::core::PSTR,
    pub flFlags: u32,
    pub lpOriginator: *mut MapiRecipDesc,
    pub nRecipCount: u32,
    pub lpRecips: *mut MapiRecipDesc,
    pub nFileCount: u32,
    pub lpFiles: *mut MapiFileDesc,
}
impl ::core::marker::Copy for MapiMessage {}
impl ::core::clone::Clone for MapiMessage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapiMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiMessage")
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
unsafe impl ::windows::core::Abi for MapiMessage {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MapiMessage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapiMessage>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapiMessage {}
impl ::core::default::Default for MapiMessage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub struct MapiMessageW {
    pub ulReserved: u32,
    pub lpszSubject: ::windows::core::PWSTR,
    pub lpszNoteText: ::windows::core::PWSTR,
    pub lpszMessageType: ::windows::core::PWSTR,
    pub lpszDateReceived: ::windows::core::PWSTR,
    pub lpszConversationID: ::windows::core::PWSTR,
    pub flFlags: u32,
    pub lpOriginator: *mut MapiRecipDescW,
    pub nRecipCount: u32,
    pub lpRecips: *mut MapiRecipDescW,
    pub nFileCount: u32,
    pub lpFiles: *mut MapiFileDescW,
}
impl ::core::marker::Copy for MapiMessageW {}
impl ::core::clone::Clone for MapiMessageW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapiMessageW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiMessageW")
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
unsafe impl ::windows::core::Abi for MapiMessageW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MapiMessageW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapiMessageW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapiMessageW {}
impl ::core::default::Default for MapiMessageW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub struct MapiRecipDesc {
    pub ulReserved: u32,
    pub ulRecipClass: u32,
    pub lpszName: ::windows::core::PSTR,
    pub lpszAddress: ::windows::core::PSTR,
    pub ulEIDSize: u32,
    pub lpEntryID: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MapiRecipDesc {}
impl ::core::clone::Clone for MapiRecipDesc {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapiRecipDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiRecipDesc").field("ulReserved", &self.ulReserved).field("ulRecipClass", &self.ulRecipClass).field("lpszName", &self.lpszName).field("lpszAddress", &self.lpszAddress).field("ulEIDSize", &self.ulEIDSize).field("lpEntryID", &self.lpEntryID).finish()
    }
}
unsafe impl ::windows::core::Abi for MapiRecipDesc {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MapiRecipDesc {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapiRecipDesc>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapiRecipDesc {}
impl ::core::default::Default for MapiRecipDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub struct MapiRecipDescW {
    pub ulReserved: u32,
    pub ulRecipClass: u32,
    pub lpszName: ::windows::core::PWSTR,
    pub lpszAddress: ::windows::core::PWSTR,
    pub ulEIDSize: u32,
    pub lpEntryID: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MapiRecipDescW {}
impl ::core::clone::Clone for MapiRecipDescW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MapiRecipDescW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiRecipDescW").field("ulReserved", &self.ulReserved).field("ulRecipClass", &self.ulRecipClass).field("lpszName", &self.lpszName).field("lpszAddress", &self.lpszAddress).field("ulEIDSize", &self.ulEIDSize).field("lpEntryID", &self.lpEntryID).finish()
    }
}
unsafe impl ::windows::core::Abi for MapiRecipDescW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MapiRecipDescW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MapiRecipDescW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MapiRecipDescW {}
impl ::core::default::Default for MapiRecipDescW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Mapi\"`*"]
pub const SUCCESS_SUCCESS: u32 = 0u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
