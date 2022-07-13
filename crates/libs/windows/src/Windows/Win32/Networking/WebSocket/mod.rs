#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WEB_SOCKET_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_NO_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_SEND_TO_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_INDICATE_SEND_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_RECEIVE_FROM_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(3i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_INDICATE_RECEIVE_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(4i32);
impl ::core::marker::Copy for WEB_SOCKET_ACTION {}
impl ::core::clone::Clone for WEB_SOCKET_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEB_SOCKET_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WEB_SOCKET_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WEB_SOCKET_ACTION_QUEUE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_SEND_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_RECEIVE_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_ALL_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(3i32);
impl ::core::marker::Copy for WEB_SOCKET_ACTION_QUEUE {}
impl ::core::clone::Clone for WEB_SOCKET_ACTION_QUEUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEB_SOCKET_ACTION_QUEUE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_ACTION_QUEUE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WEB_SOCKET_ACTION_QUEUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_ACTION_QUEUE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub union WEB_SOCKET_BUFFER {
    pub Data: WEB_SOCKET_BUFFER_1,
    pub CloseStatus: WEB_SOCKET_BUFFER_0,
}
impl ::core::marker::Copy for WEB_SOCKET_BUFFER {}
impl ::core::clone::Clone for WEB_SOCKET_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEB_SOCKET_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_BUFFER {}
impl ::core::default::Default for WEB_SOCKET_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub struct WEB_SOCKET_BUFFER_0 {
    pub pbReason: *mut u8,
    pub ulReasonLength: u32,
    pub usStatus: u16,
}
impl ::core::marker::Copy for WEB_SOCKET_BUFFER_0 {}
impl ::core::clone::Clone for WEB_SOCKET_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEB_SOCKET_BUFFER_0").field("pbReason", &self.pbReason).field("ulReasonLength", &self.ulReasonLength).field("usStatus", &self.usStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEB_SOCKET_BUFFER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_BUFFER_0 {}
impl ::core::default::Default for WEB_SOCKET_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub struct WEB_SOCKET_BUFFER_1 {
    pub pbBuffer: *mut u8,
    pub ulBufferLength: u32,
}
impl ::core::marker::Copy for WEB_SOCKET_BUFFER_1 {}
impl ::core::clone::Clone for WEB_SOCKET_BUFFER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_BUFFER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEB_SOCKET_BUFFER_1").field("pbBuffer", &self.pbBuffer).field("ulBufferLength", &self.ulBufferLength).finish()
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEB_SOCKET_BUFFER_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_BUFFER_1 {}
impl ::core::default::Default for WEB_SOCKET_BUFFER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WEB_SOCKET_BUFFER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483648i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483647i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483646i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483645i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_CLOSE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483644i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_PING_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483643i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_UNSOLICITED_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483642i32);
impl ::core::marker::Copy for WEB_SOCKET_BUFFER_TYPE {}
impl ::core::clone::Clone for WEB_SOCKET_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEB_SOCKET_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_BUFFER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WEB_SOCKET_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WEB_SOCKET_CLOSE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_SUCCESS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1000i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_ENDPOINT_UNAVAILABLE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1001i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1002i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1003i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_EMPTY_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1005i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_ABORTED_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1006i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1007i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1008i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1009i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1010i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1011i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1015i32);
impl ::core::marker::Copy for WEB_SOCKET_CLOSE_STATUS {}
impl ::core::clone::Clone for WEB_SOCKET_CLOSE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEB_SOCKET_CLOSE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_CLOSE_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WEB_SOCKET_HANDLE(pub isize);
impl WEB_SOCKET_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for WEB_SOCKET_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for WEB_SOCKET_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for WEB_SOCKET_HANDLE {}
impl ::core::fmt::Debug for WEB_SOCKET_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<WEB_SOCKET_HANDLE>> for WEB_SOCKET_HANDLE {
    fn from(optional: ::core::option::Option<WEB_SOCKET_HANDLE>) -> WEB_SOCKET_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub struct WEB_SOCKET_HTTP_HEADER {
    pub pcName: ::windows::core::PSTR,
    pub ulNameLength: u32,
    pub pcValue: ::windows::core::PSTR,
    pub ulValueLength: u32,
}
impl ::core::marker::Copy for WEB_SOCKET_HTTP_HEADER {}
impl ::core::clone::Clone for WEB_SOCKET_HTTP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_HTTP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEB_SOCKET_HTTP_HEADER").field("pcName", &self.pcName).field("ulNameLength", &self.ulNameLength).field("pcValue", &self.pcValue).field("ulValueLength", &self.ulValueLength).finish()
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_HTTP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_HTTP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEB_SOCKET_HTTP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_HTTP_HEADER {}
impl ::core::default::Default for WEB_SOCKET_HTTP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub struct WEB_SOCKET_PROPERTY {
    pub Type: WEB_SOCKET_PROPERTY_TYPE,
    pub pvValue: *mut ::core::ffi::c_void,
    pub ulValueSize: u32,
}
impl ::core::marker::Copy for WEB_SOCKET_PROPERTY {}
impl ::core::clone::Clone for WEB_SOCKET_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEB_SOCKET_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEB_SOCKET_PROPERTY").field("Type", &self.Type).field("pvValue", &self.pvValue).field("ulValueSize", &self.ulValueSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEB_SOCKET_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_PROPERTY {}
impl ::core::default::Default for WEB_SOCKET_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WEB_SOCKET_PROPERTY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_RECEIVE_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_SEND_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_DISABLE_MASKING_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_ALLOCATED_BUFFER_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_DISABLE_UTF8_VERIFICATION_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_KEEPALIVE_INTERVAL_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_SUPPORTED_VERSIONS_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(6i32);
impl ::core::marker::Copy for WEB_SOCKET_PROPERTY_TYPE {}
impl ::core::clone::Clone for WEB_SOCKET_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEB_SOCKET_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WEB_SOCKET_PROPERTY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WEB_SOCKET_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketAbortHandle<'a, P0>(hwebsocket: P0)
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketAbortHandle(hwebsocket: WEB_SOCKET_HANDLE);
    }
    WebSocketAbortHandle(hwebsocket.into())
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketBeginClientHandshake<'a, P0>(hwebsocket: P0, pszsubprotocols: &[::windows::core::PSTR], pszextensions: &[::windows::core::PSTR], pinitialheaders: &[WEB_SOCKET_HTTP_HEADER], padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketBeginClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocols: *const ::windows::core::PSTR, ulsubprotocolcount: u32, pszextensions: *const ::windows::core::PSTR, ulextensioncount: u32, pinitialheaders: *const WEB_SOCKET_HTTP_HEADER, ulinitialheadercount: u32, padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> ::windows::core::HRESULT;
    }
    WebSocketBeginClientHandshake(hwebsocket.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pszsubprotocols)), pszsubprotocols.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pszextensions)), pszextensions.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pinitialheaders)), pinitialheaders.len() as _, ::core::mem::transmute(padditionalheaders), ::core::mem::transmute(puladditionalheadercount)).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketBeginServerHandshake<'a, P0, P1>(hwebsocket: P0, pszsubprotocolselected: P1, pszextensionselected: &[::windows::core::PSTR], prequestheaders: &[WEB_SOCKET_HTTP_HEADER], presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketBeginServerHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocolselected: ::windows::core::PCSTR, pszextensionselected: *const ::windows::core::PSTR, ulextensionselectedcount: u32, prequestheaders: *const WEB_SOCKET_HTTP_HEADER, ulrequestheadercount: u32, presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut u32) -> ::windows::core::HRESULT;
    }
    WebSocketBeginServerHandshake(hwebsocket.into(), pszsubprotocolselected.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pszextensionselected)), pszextensionselected.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(prequestheaders)), prequestheaders.len() as _, ::core::mem::transmute(presponseheaders), ::core::mem::transmute(pulresponseheadercount)).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketCompleteAction<'a, P0>(hwebsocket: P0, pvactioncontext: *const ::core::ffi::c_void, ulbytestransferred: u32)
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketCompleteAction(hwebsocket: WEB_SOCKET_HANDLE, pvactioncontext: *const ::core::ffi::c_void, ulbytestransferred: u32);
    }
    WebSocketCompleteAction(hwebsocket.into(), ::core::mem::transmute(pvactioncontext), ulbytestransferred)
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketCreateClientHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> ::windows::core::Result<WEB_SOCKET_HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketCreateClientHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32, phwebsocket: *mut WEB_SOCKET_HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WebSocketCreateClientHandle(::core::mem::transmute(::windows::core::as_ptr_or_null(pproperties)), pproperties.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WEB_SOCKET_HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketCreateServerHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> ::windows::core::Result<WEB_SOCKET_HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketCreateServerHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32, phwebsocket: *mut WEB_SOCKET_HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WebSocketCreateServerHandle(::core::mem::transmute(::windows::core::as_ptr_or_null(pproperties)), pproperties.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WEB_SOCKET_HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketDeleteHandle<'a, P0>(hwebsocket: P0)
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketDeleteHandle(hwebsocket: WEB_SOCKET_HANDLE);
    }
    WebSocketDeleteHandle(hwebsocket.into())
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketEndClientHandshake<'a, P0>(hwebsocket: P0, presponseheaders: &[WEB_SOCKET_HTTP_HEADER], pulselectedextensions: *mut u32, pulselectedextensioncount: *mut u32, pulselectedsubprotocol: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketEndClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, presponseheaders: *const WEB_SOCKET_HTTP_HEADER, ulreponseheadercount: u32, pulselectedextensions: *mut u32, pulselectedextensioncount: *mut u32, pulselectedsubprotocol: *mut u32) -> ::windows::core::HRESULT;
    }
    WebSocketEndClientHandshake(hwebsocket.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(presponseheaders)), presponseheaders.len() as _, ::core::mem::transmute(pulselectedextensions), ::core::mem::transmute(pulselectedextensioncount), ::core::mem::transmute(pulselectedsubprotocol)).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketEndServerHandshake<'a, P0>(hwebsocket: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketEndServerHandshake(hwebsocket: WEB_SOCKET_HANDLE) -> ::windows::core::HRESULT;
    }
    WebSocketEndServerHandshake(hwebsocket.into()).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketGetAction<'a, P0>(hwebsocket: P0, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: *mut *mut ::core::ffi::c_void, pvactioncontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketGetAction(hwebsocket: WEB_SOCKET_HANDLE, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: *mut *mut ::core::ffi::c_void, pvactioncontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    WebSocketGetAction(hwebsocket.into(), eactionqueue, ::core::mem::transmute(pdatabuffers), ::core::mem::transmute(puldatabuffercount), ::core::mem::transmute(paction), ::core::mem::transmute(pbuffertype), ::core::mem::transmute(pvapplicationcontext), ::core::mem::transmute(pvactioncontext)).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut ::core::ffi::c_void, ulsize: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut ::core::ffi::c_void, ulsize: *mut u32) -> ::windows::core::HRESULT;
    }
    WebSocketGetGlobalProperty(etype, ::core::mem::transmute(pvvalue), ::core::mem::transmute(ulsize)).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketReceive<'a, P0>(hwebsocket: P0, pbuffer: *const WEB_SOCKET_BUFFER, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketReceive(hwebsocket: WEB_SOCKET_HANDLE, pbuffer: *const WEB_SOCKET_BUFFER, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    WebSocketReceive(hwebsocket.into(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketSend<'a, P0>(hwebsocket: P0, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: *const WEB_SOCKET_BUFFER, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<WEB_SOCKET_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WebSocketSend(hwebsocket: WEB_SOCKET_HANDLE, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: *const WEB_SOCKET_BUFFER, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    WebSocketSend(hwebsocket.into(), buffertype, ::core::mem::transmute(pbuffer), ::core::mem::transmute(context)).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
