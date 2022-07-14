#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const APPCLASS_MASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const APPCMD_MASK: i32 = 4080i32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn AddAtomA<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddAtomA(lpstring: ::windows::core::PCSTR) -> u16;
    }
    AddAtomA(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn AddAtomW<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddAtomW(lpstring: ::windows::core::PCWSTR) -> u16;
    }
    AddAtomW(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddClipboardFormatListener<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    AddClipboardFormatListener(hwnd.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CADV_LATEACK: u32 = 65535u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CONVCONTEXT {
    pub cb: u32,
    pub wFlags: u32,
    pub wCountryID: u32,
    pub iCodePage: i32,
    pub dwLangID: u32,
    pub dwSecurity: u32,
    pub qos: super::super::Security::SECURITY_QUALITY_OF_SERVICE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for CONVCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for CONVCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CONVCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONVCONTEXT").field("cb", &self.cb).field("wFlags", &self.wFlags).field("wCountryID", &self.wCountryID).field("iCodePage", &self.iCodePage).field("dwLangID", &self.dwLangID).field("dwSecurity", &self.dwSecurity).field("qos", &self.qos).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for CONVCONTEXT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CONVCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVCONTEXT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CONVCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CONVCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CONVINFO {
    pub cb: u32,
    pub hUser: usize,
    pub hConvPartner: HCONV,
    pub hszSvcPartner: HSZ,
    pub hszServiceReq: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub wType: DDE_CLIENT_TRANSACTION_TYPE,
    pub wStatus: CONVINFO_STATUS,
    pub wConvst: CONVINFO_CONVERSATION_STATE,
    pub wLastError: u32,
    pub hConvList: HCONVLIST,
    pub ConvCtxt: CONVCONTEXT,
    pub hwnd: super::super::Foundation::HWND,
    pub hwndPartner: super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for CONVINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for CONVINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CONVINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONVINFO")
            .field("cb", &self.cb)
            .field("hUser", &self.hUser)
            .field("hConvPartner", &self.hConvPartner)
            .field("hszSvcPartner", &self.hszSvcPartner)
            .field("hszServiceReq", &self.hszServiceReq)
            .field("hszTopic", &self.hszTopic)
            .field("hszItem", &self.hszItem)
            .field("wFmt", &self.wFmt)
            .field("wType", &self.wType)
            .field("wStatus", &self.wStatus)
            .field("wConvst", &self.wConvst)
            .field("wLastError", &self.wLastError)
            .field("hConvList", &self.hConvList)
            .field("ConvCtxt", &self.ConvCtxt)
            .field("hwnd", &self.hwnd)
            .field("hwndPartner", &self.hwndPartner)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for CONVINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CONVINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVINFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CONVINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CONVINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONVINFO_CONVERSATION_STATE(pub u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_ADVACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(13u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_ADVDATAACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(16u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_ADVDATASENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(15u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_ADVSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(11u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_CONNECTED: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(2u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_DATARCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(6u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_EXECACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(10u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_EXECSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(9u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_INCOMPLETE: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(1u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_INIT1: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(3u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_INIT2: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(4u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_NULL: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(0u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_POKEACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(8u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_POKESENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(7u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_REQSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(5u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_UNADVACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(14u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XST_UNADVSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(12u32);
impl ::core::marker::Copy for CONVINFO_CONVERSATION_STATE {}
impl ::core::clone::Clone for CONVINFO_CONVERSATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONVINFO_CONVERSATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONVINFO_CONVERSATION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONVINFO_CONVERSATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONVINFO_CONVERSATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONVINFO_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_ADVISE: CONVINFO_STATUS = CONVINFO_STATUS(2u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_BLOCKED: CONVINFO_STATUS = CONVINFO_STATUS(8u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_BLOCKNEXT: CONVINFO_STATUS = CONVINFO_STATUS(128u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_CLIENT: CONVINFO_STATUS = CONVINFO_STATUS(16u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_CONNECTED: CONVINFO_STATUS = CONVINFO_STATUS(1u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_INLIST: CONVINFO_STATUS = CONVINFO_STATUS(64u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_ISLOCAL: CONVINFO_STATUS = CONVINFO_STATUS(4u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_ISSELF: CONVINFO_STATUS = CONVINFO_STATUS(256u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const ST_TERMINATED: CONVINFO_STATUS = CONVINFO_STATUS(32u32);
impl ::core::marker::Copy for CONVINFO_STATUS {}
impl ::core::clone::Clone for CONVINFO_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONVINFO_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONVINFO_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONVINFO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONVINFO_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONVINFO_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONVINFO_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONVINFO_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONVINFO_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONVINFO_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct COPYDATASTRUCT {
    pub dwData: usize,
    pub cbData: u32,
    pub lpData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for COPYDATASTRUCT {}
impl ::core::clone::Clone for COPYDATASTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COPYDATASTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYDATASTRUCT").field("dwData", &self.dwData).field("cbData", &self.cbData).field("lpData", &self.lpData).finish()
    }
}
unsafe impl ::windows::core::Abi for COPYDATASTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COPYDATASTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COPYDATASTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for COPYDATASTRUCT {}
impl ::core::default::Default for COPYDATASTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CP_WINANSI: i32 = 1004i32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CP_WINNEUTRAL: i32 = 1200i32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CP_WINUNICODE: i32 = 1200i32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeClipboardChain<'a, P0, P1>(hwndremove: P0, hwndnewnext: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeClipboardChain(hwndremove: super::super::Foundation::HWND, hwndnewnext: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    ChangeClipboardChain(hwndremove.into(), hwndnewnext.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClipboard() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseClipboard() -> super::super::Foundation::BOOL;
    }
    CloseClipboard()
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn CountClipboardFormats() -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CountClipboardFormats() -> i32;
    }
    CountClipboardFormats()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct DDEACK {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DDEACK {}
impl ::core::clone::Clone for DDEACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDEACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEACK").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for DDEACK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDEACK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDEACK>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDEACK {}
impl ::core::default::Default for DDEACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl ::core::marker::Copy for DDEADVISE {}
impl ::core::clone::Clone for DDEADVISE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDEADVISE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEADVISE").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for DDEADVISE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDEADVISE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDEADVISE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDEADVISE {}
impl ::core::default::Default for DDEADVISE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl ::core::marker::Copy for DDEDATA {}
impl ::core::clone::Clone for DDEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEDATA").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for DDEDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDEDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDEDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDEDATA {}
impl ::core::default::Default for DDEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl ::core::marker::Copy for DDELN {}
impl ::core::clone::Clone for DDELN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDELN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDELN").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for DDELN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDELN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDELN>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDELN {}
impl ::core::default::Default for DDELN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct DDEML_MSG_HOOK_DATA {
    pub uiLo: usize,
    pub uiHi: usize,
    pub cbData: u32,
    pub Data: [u32; 8],
}
impl ::core::marker::Copy for DDEML_MSG_HOOK_DATA {}
impl ::core::clone::Clone for DDEML_MSG_HOOK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDEML_MSG_HOOK_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEML_MSG_HOOK_DATA").field("uiLo", &self.uiLo).field("uiHi", &self.uiHi).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for DDEML_MSG_HOOK_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDEML_MSG_HOOK_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDEML_MSG_HOOK_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDEML_MSG_HOOK_DATA {}
impl ::core::default::Default for DDEML_MSG_HOOK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl ::core::marker::Copy for DDEPOKE {}
impl ::core::clone::Clone for DDEPOKE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDEPOKE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEPOKE").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for DDEPOKE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDEPOKE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDEPOKE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDEPOKE {}
impl ::core::default::Default for DDEPOKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for DDEUP {}
impl ::core::clone::Clone for DDEUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDEUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEUP").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("rgb", &self.rgb).finish()
    }
}
unsafe impl ::windows::core::Abi for DDEUP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDEUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDEUP>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDEUP {}
impl ::core::default::Default for DDEUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DDE_CLIENT_TRANSACTION_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_ADVSTART: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(4144u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_ADVSTOP: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32832u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_EXECUTE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16464u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_POKE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16528u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_REQUEST: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8368u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_ADVDATA: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16400u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_ADVREQ: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8226u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_CONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(4194u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_CONNECT_CONFIRM: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32882u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_DISCONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32962u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_MONITOR: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(33010u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_REGISTER: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32930u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_UNREGISTER: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32978u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_WILDCONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8418u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_XACT_COMPLETE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32896u32);
impl ::core::marker::Copy for DDE_CLIENT_TRANSACTION_TYPE {}
impl ::core::clone::Clone for DDE_CLIENT_TRANSACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DDE_CLIENT_TRANSACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DDE_CLIENT_TRANSACTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DDE_CLIENT_TRANSACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_CLIENT_TRANSACTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DDE_ENABLE_CALLBACK_CMD(pub u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const EC_ENABLEALL: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(0u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const EC_ENABLEONE: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(128u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const EC_DISABLE: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(8u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const EC_QUERYWAITING: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(2u32);
impl ::core::marker::Copy for DDE_ENABLE_CALLBACK_CMD {}
impl ::core::clone::Clone for DDE_ENABLE_CALLBACK_CMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DDE_ENABLE_CALLBACK_CMD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DDE_ENABLE_CALLBACK_CMD {
    type Abi = Self;
}
impl ::core::fmt::Debug for DDE_ENABLE_CALLBACK_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_ENABLE_CALLBACK_CMD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FACK: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FACKREQ: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FAPPSTATUS: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FBUSY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FDEFERUPD: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FNOTPROCESSED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FRELEASE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DDE_FREQUESTED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DDE_INITIALIZE_COMMAND(pub u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const APPCLASS_MONITOR: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const APPCLASS_STANDARD: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(0u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const APPCMD_CLIENTONLY: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const APPCMD_FILTERINITS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(32u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_FAIL_ALLSVRXACTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(258048u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_FAIL_ADVISES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16384u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_FAIL_CONNECTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(8192u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_FAIL_EXECUTES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(32768u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_FAIL_POKES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(65536u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_FAIL_REQUESTS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(131072u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_FAIL_SELFCONNECTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(4096u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_SKIP_ALLNOTIFICATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(3932160u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_SKIP_CONNECT_CONFIRMS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(262144u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_SKIP_DISCONNECTS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(2097152u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_SKIP_REGISTRATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(524288u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const CBF_SKIP_UNREGISTRATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1048576u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_CALLBACKS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(134217728u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_CONV: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1073741824u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_ERRORS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(268435456u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_HSZ_INFO: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16777216u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_LINKS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(536870912u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_POSTMSGS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(67108864u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_SENDMSGS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(33554432u32);
impl ::core::marker::Copy for DDE_INITIALIZE_COMMAND {}
impl ::core::clone::Clone for DDE_INITIALIZE_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DDE_INITIALIZE_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DDE_INITIALIZE_COMMAND {
    type Abi = Self;
}
impl ::core::fmt::Debug for DDE_INITIALIZE_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_INITIALIZE_COMMAND").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DDE_INITIALIZE_COMMAND {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DDE_INITIALIZE_COMMAND {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DDE_NAME_SERVICE_CMD(pub u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DNS_REGISTER: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(1u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DNS_UNREGISTER: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(2u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DNS_FILTERON: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(4u32);
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DNS_FILTEROFF: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(8u32);
impl ::core::marker::Copy for DDE_NAME_SERVICE_CMD {}
impl ::core::clone::Clone for DDE_NAME_SERVICE_CMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DDE_NAME_SERVICE_CMD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DDE_NAME_SERVICE_CMD {
    type Abi = Self;
}
impl ::core::fmt::Debug for DDE_NAME_SERVICE_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_NAME_SERVICE_CMD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_ADVACKTIMEOUT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_BUSY: u32 = 16385u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_DATAACKTIMEOUT: u32 = 16386u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_DLL_NOT_INITIALIZED: u32 = 16387u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_DLL_USAGE: u32 = 16388u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_EXECACKTIMEOUT: u32 = 16389u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_FIRST: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_INVALIDPARAMETER: u32 = 16390u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_LAST: u32 = 16401u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_LOW_MEMORY: u32 = 16391u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_MEMORY_ERROR: u32 = 16392u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_NOTPROCESSED: u32 = 16393u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_NO_CONV_ESTABLISHED: u32 = 16394u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_NO_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_POKEACKTIMEOUT: u32 = 16395u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_POSTMSG_FAILED: u32 = 16396u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_REENTRANCY: u32 = 16397u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_SERVER_DIED: u32 = 16398u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_SYS_ERROR: u32 = 16399u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_UNADVACKTIMEOUT: u32 = 16400u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const DMLERR_UNFOUND_QUEUE_ID: u32 = 16401u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeAbandonTransaction<'a, P0>(idinst: u32, hconv: P0, idtransaction: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeAbandonTransaction(idinst: u32, hconv: HCONV, idtransaction: u32) -> super::super::Foundation::BOOL;
    }
    DdeAbandonTransaction(idinst, hconv.into(), idtransaction)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeAccessData<'a, P0>(hdata: P0, pcbdatasize: *mut u32) -> *mut u8
where
    P0: ::std::convert::Into<HDDEDATA>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeAccessData(hdata: HDDEDATA, pcbdatasize: *mut u32) -> *mut u8;
    }
    DdeAccessData(hdata.into(), ::core::mem::transmute(pcbdatasize))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeAddData<'a, P0>(hdata: P0, psrc: *const u8, cb: u32, cboff: u32) -> HDDEDATA
where
    P0: ::std::convert::Into<HDDEDATA>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeAddData(hdata: HDDEDATA, psrc: *const u8, cb: u32, cboff: u32) -> HDDEDATA;
    }
    DdeAddData(hdata.into(), ::core::mem::transmute(psrc), cb, cboff)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeClientTransaction<'a, P0, P1>(pdata: *const u8, cbdata: u32, hconv: P0, hszitem: P1, wfmt: u32, wtype: DDE_CLIENT_TRANSACTION_TYPE, dwtimeout: u32, pdwresult: *mut u32) -> HDDEDATA
where
    P0: ::std::convert::Into<HCONV>,
    P1: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeClientTransaction(pdata: *const u8, cbdata: u32, hconv: HCONV, hszitem: HSZ, wfmt: u32, wtype: DDE_CLIENT_TRANSACTION_TYPE, dwtimeout: u32, pdwresult: *mut u32) -> HDDEDATA;
    }
    DdeClientTransaction(::core::mem::transmute(pdata), cbdata, hconv.into(), hszitem.into(), wfmt, wtype, dwtimeout, ::core::mem::transmute(pdwresult))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeCmpStringHandles<'a, P0, P1>(hsz1: P0, hsz2: P1) -> i32
where
    P0: ::std::convert::Into<HSZ>,
    P1: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeCmpStringHandles(hsz1: HSZ, hsz2: HSZ) -> i32;
    }
    DdeCmpStringHandles(hsz1.into(), hsz2.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeConnect<'a, P0, P1>(idinst: u32, hszservice: P0, hsztopic: P1, pcc: *const CONVCONTEXT) -> HCONV
where
    P0: ::std::convert::Into<HSZ>,
    P1: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeConnect(idinst: u32, hszservice: HSZ, hsztopic: HSZ, pcc: *const CONVCONTEXT) -> HCONV;
    }
    DdeConnect(idinst, hszservice.into(), hsztopic.into(), ::core::mem::transmute(pcc))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeConnectList<'a, P0, P1, P2>(idinst: u32, hszservice: P0, hsztopic: P1, hconvlist: P2, pcc: *const CONVCONTEXT) -> HCONVLIST
where
    P0: ::std::convert::Into<HSZ>,
    P1: ::std::convert::Into<HSZ>,
    P2: ::std::convert::Into<HCONVLIST>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeConnectList(idinst: u32, hszservice: HSZ, hsztopic: HSZ, hconvlist: HCONVLIST, pcc: *const CONVCONTEXT) -> HCONVLIST;
    }
    DdeConnectList(idinst, hszservice.into(), hsztopic.into(), hconvlist.into(), ::core::mem::transmute(pcc))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeCreateDataHandle<'a, P0>(idinst: u32, psrc: *const u8, cb: u32, cboff: u32, hszitem: P0, wfmt: u32, afcmd: u32) -> HDDEDATA
where
    P0: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeCreateDataHandle(idinst: u32, psrc: *const u8, cb: u32, cboff: u32, hszitem: HSZ, wfmt: u32, afcmd: u32) -> HDDEDATA;
    }
    DdeCreateDataHandle(idinst, ::core::mem::transmute(psrc), cb, cboff, hszitem.into(), wfmt, afcmd)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeCreateStringHandleA<'a, P0>(idinst: u32, psz: P0, icodepage: i32) -> HSZ
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeCreateStringHandleA(idinst: u32, psz: ::windows::core::PCSTR, icodepage: i32) -> HSZ;
    }
    DdeCreateStringHandleA(idinst, psz.into(), icodepage)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeCreateStringHandleW<'a, P0>(idinst: u32, psz: P0, icodepage: i32) -> HSZ
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeCreateStringHandleW(idinst: u32, psz: ::windows::core::PCWSTR, icodepage: i32) -> HSZ;
    }
    DdeCreateStringHandleW(idinst, psz.into(), icodepage)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeDisconnect<'a, P0>(hconv: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeDisconnect(hconv: HCONV) -> super::super::Foundation::BOOL;
    }
    DdeDisconnect(hconv.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeDisconnectList<'a, P0>(hconvlist: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCONVLIST>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeDisconnectList(hconvlist: HCONVLIST) -> super::super::Foundation::BOOL;
    }
    DdeDisconnectList(hconvlist.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeEnableCallback<'a, P0>(idinst: u32, hconv: P0, wcmd: DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeEnableCallback(idinst: u32, hconv: HCONV, wcmd: DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation::BOOL;
    }
    DdeEnableCallback(idinst, hconv.into(), wcmd)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeFreeDataHandle<'a, P0>(hdata: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDDEDATA>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeFreeDataHandle(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
    }
    DdeFreeDataHandle(hdata.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeFreeStringHandle<'a, P0>(idinst: u32, hsz: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeFreeStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
    }
    DdeFreeStringHandle(idinst, hsz.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeGetData<'a, P0>(hdata: P0, pdst: *mut u8, cbmax: u32, cboff: u32) -> u32
where
    P0: ::std::convert::Into<HDDEDATA>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeGetData(hdata: HDDEDATA, pdst: *mut u8, cbmax: u32, cboff: u32) -> u32;
    }
    DdeGetData(hdata.into(), ::core::mem::transmute(pdst), cbmax, cboff)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeGetLastError(idinst: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeGetLastError(idinst: u32) -> u32;
    }
    DdeGetLastError(idinst)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeImpersonateClient<'a, P0>(hconv: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeImpersonateClient(hconv: HCONV) -> super::super::Foundation::BOOL;
    }
    DdeImpersonateClient(hconv.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeInitializeA(pidinst: *mut u32, pfncallback: PFNCALLBACK, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeInitializeA(pidinst: *mut u32, pfncallback: *mut ::core::ffi::c_void, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
    }
    DdeInitializeA(::core::mem::transmute(pidinst), ::core::mem::transmute(pfncallback), afcmd, ulres)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeInitializeW(pidinst: *mut u32, pfncallback: PFNCALLBACK, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeInitializeW(pidinst: *mut u32, pfncallback: *mut ::core::ffi::c_void, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
    }
    DdeInitializeW(::core::mem::transmute(pidinst), ::core::mem::transmute(pfncallback), afcmd, ulres)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeKeepStringHandle<'a, P0>(idinst: u32, hsz: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeKeepStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
    }
    DdeKeepStringHandle(idinst, hsz.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeNameService<'a, P0, P1>(idinst: u32, hsz1: P0, hsz2: P1, afcmd: DDE_NAME_SERVICE_CMD) -> HDDEDATA
where
    P0: ::std::convert::Into<HSZ>,
    P1: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeNameService(idinst: u32, hsz1: HSZ, hsz2: HSZ, afcmd: DDE_NAME_SERVICE_CMD) -> HDDEDATA;
    }
    DdeNameService(idinst, hsz1.into(), hsz2.into(), afcmd)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdePostAdvise<'a, P0, P1>(idinst: u32, hsztopic: P0, hszitem: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HSZ>,
    P1: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdePostAdvise(idinst: u32, hsztopic: HSZ, hszitem: HSZ) -> super::super::Foundation::BOOL;
    }
    DdePostAdvise(idinst, hsztopic.into(), hszitem.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeQueryConvInfo<'a, P0>(hconv: P0, idtransaction: u32, pconvinfo: *mut CONVINFO) -> u32
where
    P0: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeQueryConvInfo(hconv: HCONV, idtransaction: u32, pconvinfo: *mut CONVINFO) -> u32;
    }
    DdeQueryConvInfo(hconv.into(), idtransaction, ::core::mem::transmute(pconvinfo))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeQueryNextServer<'a, P0, P1>(hconvlist: P0, hconvprev: P1) -> HCONV
where
    P0: ::std::convert::Into<HCONVLIST>,
    P1: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeQueryNextServer(hconvlist: HCONVLIST, hconvprev: HCONV) -> HCONV;
    }
    DdeQueryNextServer(hconvlist.into(), hconvprev.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeQueryStringA<'a, P0>(idinst: u32, hsz: P0, psz: &mut [u8], icodepage: i32) -> u32
where
    P0: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeQueryStringA(idinst: u32, hsz: HSZ, psz: ::windows::core::PSTR, cchmax: u32, icodepage: i32) -> u32;
    }
    DdeQueryStringA(idinst, hsz.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psz)), psz.len() as _, icodepage)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeQueryStringW<'a, P0>(idinst: u32, hsz: P0, psz: &mut [u16], icodepage: i32) -> u32
where
    P0: ::std::convert::Into<HSZ>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeQueryStringW(idinst: u32, hsz: HSZ, psz: ::windows::core::PWSTR, cchmax: u32, icodepage: i32) -> u32;
    }
    DdeQueryStringW(idinst, hsz.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psz)), psz.len() as _, icodepage)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DdeReconnect<'a, P0>(hconv: P0) -> HCONV
where
    P0: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeReconnect(hconv: HCONV) -> HCONV;
    }
    DdeReconnect(hconv.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeSetQualityOfService<'a, P0>(hwndclient: P0, pqosnew: *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeSetQualityOfService(hwndclient: super::super::Foundation::HWND, pqosnew: *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> super::super::Foundation::BOOL;
    }
    DdeSetQualityOfService(hwndclient.into(), ::core::mem::transmute(pqosnew), ::core::mem::transmute(pqosprev))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeSetUserHandle<'a, P0>(hconv: P0, id: u32, huser: usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HCONV>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeSetUserHandle(hconv: HCONV, id: u32, huser: usize) -> super::super::Foundation::BOOL;
    }
    DdeSetUserHandle(hconv.into(), id, huser)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeUnaccessData<'a, P0>(hdata: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDDEDATA>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeUnaccessData(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
    }
    DdeUnaccessData(hdata.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeUninitialize(idinst: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdeUninitialize(idinst: u32) -> super::super::Foundation::BOOL;
    }
    DdeUninitialize(idinst)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn DeleteAtom(natom: u16) -> u16 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteAtom(natom: u16) -> u16;
    }
    DeleteAtom(natom)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EmptyClipboard() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EmptyClipboard() -> super::super::Foundation::BOOL;
    }
    EmptyClipboard()
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn EnumClipboardFormats(format: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumClipboardFormats(format: u32) -> u32;
    }
    EnumClipboardFormats(format)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn FindAtomA<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindAtomA(lpstring: ::windows::core::PCSTR) -> u16;
    }
    FindAtomA(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn FindAtomW<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindAtomW(lpstring: ::windows::core::PCWSTR) -> u16;
    }
    FindAtomW(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeDDElParam<'a, P0>(msg: u32, lparam: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreeDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    }
    FreeDDElParam(msg, lparam.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GetAtomNameA(natom: u16, lpbuffer: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAtomNameA(natom: u16, lpbuffer: ::windows::core::PSTR, nsize: i32) -> u32;
    }
    GetAtomNameA(natom, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GetAtomNameW(natom: u16, lpbuffer: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAtomNameW(natom: u16, lpbuffer: ::windows::core::PWSTR, nsize: i32) -> u32;
    }
    GetAtomNameW(natom, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardData(uformat: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipboardData(uformat: u32) -> super::super::Foundation::HANDLE;
    }
    let result__ = GetClipboardData(uformat);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GetClipboardFormatNameA(format: u32, lpszformatname: &mut [u8]) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipboardFormatNameA(format: u32, lpszformatname: ::windows::core::PSTR, cchmaxcount: i32) -> i32;
    }
    GetClipboardFormatNameA(format, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszformatname)), lpszformatname.len() as _)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GetClipboardFormatNameW(format: u32, lpszformatname: &mut [u16]) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipboardFormatNameW(format: u32, lpszformatname: ::windows::core::PWSTR, cchmaxcount: i32) -> i32;
    }
    GetClipboardFormatNameW(format, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszformatname)), lpszformatname.len() as _)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardOwner() -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipboardOwner() -> super::super::Foundation::HWND;
    }
    GetClipboardOwner()
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GetClipboardSequenceNumber() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipboardSequenceNumber() -> u32;
    }
    GetClipboardSequenceNumber()
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardViewer() -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetClipboardViewer() -> super::super::Foundation::HWND;
    }
    GetClipboardViewer()
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenClipboardWindow() -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetOpenClipboardWindow() -> super::super::Foundation::HWND;
    }
    GetOpenClipboardWindow()
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GetPriorityClipboardFormat(paformatprioritylist: &[u32]) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetPriorityClipboardFormat(paformatprioritylist: *const u32, cformats: i32) -> i32;
    }
    GetPriorityClipboardFormat(::core::mem::transmute(::windows::core::as_ptr_or_null(paformatprioritylist)), paformatprioritylist.len() as _)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUpdatedClipboardFormats(lpuiformats: &mut [u32], pcformatsout: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetUpdatedClipboardFormats(lpuiformats: *mut u32, cformats: u32, pcformatsout: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetUpdatedClipboardFormats(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpuiformats)), lpuiformats.len() as _, ::core::mem::transmute(pcformatsout))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalAddAtomA<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalAddAtomA(lpstring: ::windows::core::PCSTR) -> u16;
    }
    GlobalAddAtomA(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalAddAtomExA<'a, P0>(lpstring: P0, flags: u32) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalAddAtomExA(lpstring: ::windows::core::PCSTR, flags: u32) -> u16;
    }
    GlobalAddAtomExA(lpstring.into(), flags)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalAddAtomExW<'a, P0>(lpstring: P0, flags: u32) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalAddAtomExW(lpstring: ::windows::core::PCWSTR, flags: u32) -> u16;
    }
    GlobalAddAtomExW(lpstring.into(), flags)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalAddAtomW<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalAddAtomW(lpstring: ::windows::core::PCWSTR) -> u16;
    }
    GlobalAddAtomW(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalDeleteAtom(natom: u16) -> u16 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalDeleteAtom(natom: u16) -> u16;
    }
    GlobalDeleteAtom(natom)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalFindAtomA<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalFindAtomA(lpstring: ::windows::core::PCSTR) -> u16;
    }
    GlobalFindAtomA(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalFindAtomW<'a, P0>(lpstring: P0) -> u16
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalFindAtomW(lpstring: ::windows::core::PCWSTR) -> u16;
    }
    GlobalFindAtomW(lpstring.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalGetAtomNameA(natom: u16, lpbuffer: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalGetAtomNameA(natom: u16, lpbuffer: ::windows::core::PSTR, nsize: i32) -> u32;
    }
    GlobalGetAtomNameA(natom, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn GlobalGetAtomNameW(natom: u16, lpbuffer: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GlobalGetAtomNameW(natom: u16, lpbuffer: ::windows::core::PWSTR, nsize: i32) -> u32;
    }
    GlobalGetAtomNameW(natom, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCONV(pub isize);
impl HCONV {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCONV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCONV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCONV {}
impl ::core::fmt::Debug for HCONV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCONV").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HCONV>> for HCONV {
    fn from(optional: ::core::option::Option<HCONV>) -> HCONV {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HCONV {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCONVLIST(pub isize);
impl HCONVLIST {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCONVLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCONVLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCONVLIST {}
impl ::core::fmt::Debug for HCONVLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCONVLIST").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HCONVLIST>> for HCONVLIST {
    fn from(optional: ::core::option::Option<HCONVLIST>) -> HCONVLIST {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HCONVLIST {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const HDATA_APPOWNED: u32 = 1u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDDEDATA(pub isize);
impl HDDEDATA {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for HDDEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDDEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDDEDATA {}
impl ::core::fmt::Debug for HDDEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDDEDATA").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HDDEDATA>> for HDDEDATA {
    fn from(optional: ::core::option::Option<HDDEDATA>) -> HDDEDATA {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HDDEDATA {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HSZ(pub isize);
impl HSZ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSZ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSZ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSZ {}
impl ::core::fmt::Debug for HSZ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSZ").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HSZ>> for HSZ {
    fn from(optional: ::core::option::Option<HSZ>) -> HSZ {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HSZ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub struct HSZPAIR {
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
}
impl ::core::marker::Copy for HSZPAIR {}
impl ::core::clone::Clone for HSZPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSZPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSZPAIR").field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).finish()
    }
}
unsafe impl ::windows::core::Abi for HSZPAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSZPAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSZPAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSZPAIR {}
impl ::core::default::Default for HSZPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateDdeClientWindow<'a, P0, P1>(hwndclient: P0, hwndserver: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ImpersonateDdeClientWindow(hwndclient: super::super::Foundation::HWND, hwndserver: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    ImpersonateDdeClientWindow(hwndclient.into(), hwndserver.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitAtomTable(nsize: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitAtomTable(nsize: u32) -> super::super::Foundation::BOOL;
    }
    InitAtomTable(nsize)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsClipboardFormatAvailable(format: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsClipboardFormatAvailable(format: u32) -> super::super::Foundation::BOOL;
    }
    IsClipboardFormatAvailable(format)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MAX_MONITORS: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct METAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: super::super::Graphics::Gdi::HMETAFILE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for METAFILEPICT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for METAFILEPICT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for METAFILEPICT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METAFILEPICT").field("mm", &self.mm).field("xExt", &self.xExt).field("yExt", &self.yExt).field("hMF", &self.hMF).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for METAFILEPICT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for METAFILEPICT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METAFILEPICT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for METAFILEPICT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for METAFILEPICT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MF_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MH_CLEANUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MH_CREATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MH_DELETE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MH_KEEP: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct MONCBSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub dwRet: u32,
    pub wType: u32,
    pub wFmt: u32,
    pub hConv: HCONV,
    pub hsz1: HSZ,
    pub hsz2: HSZ,
    pub hData: HDDEDATA,
    pub dwData1: usize,
    pub dwData2: usize,
    pub cc: CONVCONTEXT,
    pub cbData: u32,
    pub Data: [u32; 8],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for MONCBSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for MONCBSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for MONCBSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONCBSTRUCT").field("cb", &self.cb).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("dwRet", &self.dwRet).field("wType", &self.wType).field("wFmt", &self.wFmt).field("hConv", &self.hConv).field("hsz1", &self.hsz1).field("hsz2", &self.hsz2).field("hData", &self.hData).field("dwData1", &self.dwData1).field("dwData2", &self.dwData2).field("cc", &self.cc).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for MONCBSTRUCT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for MONCBSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONCBSTRUCT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for MONCBSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for MONCBSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONCONVSTRUCT {
    pub cb: u32,
    pub fConnect: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hConvClient: HCONV,
    pub hConvServer: HCONV,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONCONVSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONCONVSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONCONVSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONCONVSTRUCT").field("cb", &self.cb).field("fConnect", &self.fConnect).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).field("hConvClient", &self.hConvClient).field("hConvServer", &self.hConvServer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONCONVSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONCONVSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONCONVSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONCONVSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONCONVSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONERRSTRUCT {
    pub cb: u32,
    pub wLastError: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONERRSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONERRSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONERRSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONERRSTRUCT").field("cb", &self.cb).field("wLastError", &self.wLastError).field("dwTime", &self.dwTime).field("hTask", &self.hTask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONERRSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONERRSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONERRSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONERRSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONERRSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONHSZSTRUCTA {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONHSZSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONHSZSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONHSZSTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONHSZSTRUCTA").field("cb", &self.cb).field("fsAction", &self.fsAction).field("dwTime", &self.dwTime).field("hsz", &self.hsz).field("hTask", &self.hTask).field("str", &self.str).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONHSZSTRUCTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONHSZSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONHSZSTRUCTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONHSZSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONHSZSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONHSZSTRUCTW {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONHSZSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONHSZSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONHSZSTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONHSZSTRUCTW").field("cb", &self.cb).field("fsAction", &self.fsAction).field("dwTime", &self.dwTime).field("hsz", &self.hsz).field("hTask", &self.hTask).field("str", &self.str).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONHSZSTRUCTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONHSZSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONHSZSTRUCTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONHSZSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONHSZSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONLINKSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub fEstablished: super::super::Foundation::BOOL,
    pub fNoData: super::super::Foundation::BOOL,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub fServer: super::super::Foundation::BOOL,
    pub hConvServer: HCONV,
    pub hConvClient: HCONV,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONLINKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONLINKSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONLINKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONLINKSTRUCT").field("cb", &self.cb).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("fEstablished", &self.fEstablished).field("fNoData", &self.fNoData).field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).field("hszItem", &self.hszItem).field("wFmt", &self.wFmt).field("fServer", &self.fServer).field("hConvServer", &self.hConvServer).field("hConvClient", &self.hConvClient).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONLINKSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONLINKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONLINKSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONLINKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONLINKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONMSGSTRUCT {
    pub cb: u32,
    pub hwndTo: super::super::Foundation::HWND,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub wMsg: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub dmhd: DDEML_MSG_HOOK_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONMSGSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONMSGSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONMSGSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONMSGSTRUCT").field("cb", &self.cb).field("hwndTo", &self.hwndTo).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("wMsg", &self.wMsg).field("wParam", &self.wParam).field("lParam", &self.lParam).field("dmhd", &self.dmhd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONMSGSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONMSGSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONMSGSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONMSGSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONMSGSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const MSGF_DDEMGR: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenClipboard<'a, P0>(hwndnewowner: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenClipboard(hwndnewowner: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    OpenClipboard(hwndnewowner.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub type PFNCALLBACK = ::core::option::Option<unsafe extern "system" fn(wtype: u32, wfmt: u32, hconv: HCONV, hsz1: HSZ, hsz2: HSZ, hdata: HDDEDATA, dwdata1: usize, dwdata2: usize) -> HDDEDATA>;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
    }
    PackDDElParam(msg, uilo, uihi)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const QID_SYNC: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn RegisterClipboardFormatA<'a, P0>(lpszformat: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterClipboardFormatA(lpszformat: ::windows::core::PCSTR) -> u32;
    }
    RegisterClipboardFormatA(lpszformat.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
#[inline]
pub unsafe fn RegisterClipboardFormatW<'a, P0>(lpszformat: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterClipboardFormatW(lpszformat: ::windows::core::PCWSTR) -> u32;
    }
    RegisterClipboardFormatW(lpszformat.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveClipboardFormatListener<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    RemoveClipboardFormatListener(hwnd.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReuseDDElParam<'a, P0>(lparam: P0, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReuseDDElParam(lparam: super::super::Foundation::LPARAM, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
    }
    ReuseDDElParam(lparam.into(), msgin, msgout, uilo, uihi)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDESYS_ITEM_FORMATS: &str = "Formats";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDESYS_ITEM_HELP: &str = "Help";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDESYS_ITEM_RTNMSG: &str = "ReturnMessage";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDESYS_ITEM_STATUS: &str = "Status";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDESYS_ITEM_SYSITEMS: &str = "SysItems";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDESYS_ITEM_TOPICS: &str = "Topics";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDESYS_TOPIC: &str = "System";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const SZDDE_ITEM_ITEMLIST: &str = "TopicItemList";
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClipboardData<'a, P0>(uformat: u32, hmem: P0) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetClipboardData(uformat: u32, hmem: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    }
    let result__ = SetClipboardData(uformat, hmem.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClipboardViewer<'a, P0>(hwndnewviewer: P0) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetClipboardViewer(hwndnewviewer: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    }
    SetClipboardViewer(hwndnewviewer.into())
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetWinMetaFileBits<'a, P0>(nsize: u32, lpmeta16data: *const u8, hdcref: P0, lpmfp: *const METAFILEPICT) -> super::super::Graphics::Gdi::HENHMETAFILE
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetWinMetaFileBits(nsize: u32, lpmeta16data: *const u8, hdcref: super::super::Graphics::Gdi::HDC, lpmfp: *const METAFILEPICT) -> super::super::Graphics::Gdi::HENHMETAFILE;
    }
    SetWinMetaFileBits(nsize, ::core::mem::transmute(lpmeta16data), hdcref.into(), ::core::mem::transmute(lpmfp))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const TIMEOUT_ASYNC: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnpackDDElParam<'a, P0>(msg: u32, lparam: P0, puilo: *mut usize, puihi: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnpackDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM, puilo: *mut usize, puihi: *mut usize) -> super::super::Foundation::BOOL;
    }
    UnpackDDElParam(msg, lparam.into(), ::core::mem::transmute(puilo), ::core::mem::transmute(puihi))
}
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_ACK: u32 = 996u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_ADVISE: u32 = 994u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_DATA: u32 = 997u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_EXECUTE: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_FIRST: u32 = 992u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_INITIATE: u32 = 992u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_LAST: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_POKE: u32 = 999u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_REQUEST: u32 = 998u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_TERMINATE: u32 = 993u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const WM_DDE_UNADVISE: u32 = 995u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XCLASS_BOOL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XCLASS_DATA: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XCLASS_FLAGS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XCLASS_MASK: u32 = 64512u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XCLASS_NOTIFICATION: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYPF_ACKREQ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYPF_NOBLOCK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYPF_NODATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_MASK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_DataExchange\"`*"]
pub const XTYP_SHIFT: u32 = 4u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
