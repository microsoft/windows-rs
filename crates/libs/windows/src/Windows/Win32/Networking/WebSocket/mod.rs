#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketAbortHandle<P0>(hwebsocket: P0)
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketAbortHandle ( hwebsocket : WEB_SOCKET_HANDLE ) -> ( ) );
    WebSocketAbortHandle(hwebsocket.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketBeginClientHandshake<P0>(hwebsocket: P0, pszsubprotocols: ::core::option::Option<&[::windows::core::PCSTR]>, pszextensions: ::core::option::Option<&[::windows::core::PCSTR]>, pinitialheaders: ::core::option::Option<&[WEB_SOCKET_HTTP_HEADER]>, padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketBeginClientHandshake ( hwebsocket : WEB_SOCKET_HANDLE , pszsubprotocols : *const :: windows::core::PCSTR , ulsubprotocolcount : u32 , pszextensions : *const :: windows::core::PCSTR , ulextensioncount : u32 , pinitialheaders : *const WEB_SOCKET_HTTP_HEADER , ulinitialheadercount : u32 , padditionalheaders : *mut *mut WEB_SOCKET_HTTP_HEADER , puladditionalheadercount : *mut u32 ) -> :: windows::core::HRESULT );
    WebSocketBeginClientHandshake(
        hwebsocket.into_param().abi(),
        ::core::mem::transmute(pszsubprotocols.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        pszsubprotocols.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(pszextensions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        pszextensions.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(pinitialheaders.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        pinitialheaders.as_deref().map_or(0, |slice| slice.len() as _),
        padditionalheaders,
        puladditionalheadercount,
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketBeginServerHandshake<P0, P1>(hwebsocket: P0, pszsubprotocolselected: P1, pszextensionselected: ::core::option::Option<&[::windows::core::PCSTR]>, prequestheaders: &[WEB_SOCKET_HTTP_HEADER], presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketBeginServerHandshake ( hwebsocket : WEB_SOCKET_HANDLE , pszsubprotocolselected : :: windows::core::PCSTR , pszextensionselected : *const :: windows::core::PCSTR , ulextensionselectedcount : u32 , prequestheaders : *const WEB_SOCKET_HTTP_HEADER , ulrequestheadercount : u32 , presponseheaders : *mut *mut WEB_SOCKET_HTTP_HEADER , pulresponseheadercount : *mut u32 ) -> :: windows::core::HRESULT );
    WebSocketBeginServerHandshake(hwebsocket.into_param().abi(), pszsubprotocolselected.into_param().abi(), ::core::mem::transmute(pszextensionselected.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszextensionselected.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prequestheaders.as_ptr()), prequestheaders.len() as _, presponseheaders, pulresponseheadercount).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketCompleteAction<P0>(hwebsocket: P0, pvactioncontext: *const ::core::ffi::c_void, ulbytestransferred: u32)
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketCompleteAction ( hwebsocket : WEB_SOCKET_HANDLE , pvactioncontext : *const ::core::ffi::c_void , ulbytestransferred : u32 ) -> ( ) );
    WebSocketCompleteAction(hwebsocket.into_param().abi(), pvactioncontext, ulbytestransferred)
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketCreateClientHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> ::windows::core::Result<WEB_SOCKET_HANDLE> {
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketCreateClientHandle ( pproperties : *const WEB_SOCKET_PROPERTY , ulpropertycount : u32 , phwebsocket : *mut WEB_SOCKET_HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<WEB_SOCKET_HANDLE>();
    WebSocketCreateClientHandle(::core::mem::transmute(pproperties.as_ptr()), pproperties.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketCreateServerHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> ::windows::core::Result<WEB_SOCKET_HANDLE> {
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketCreateServerHandle ( pproperties : *const WEB_SOCKET_PROPERTY , ulpropertycount : u32 , phwebsocket : *mut WEB_SOCKET_HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<WEB_SOCKET_HANDLE>();
    WebSocketCreateServerHandle(::core::mem::transmute(pproperties.as_ptr()), pproperties.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketDeleteHandle<P0>(hwebsocket: P0)
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketDeleteHandle ( hwebsocket : WEB_SOCKET_HANDLE ) -> ( ) );
    WebSocketDeleteHandle(hwebsocket.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketEndClientHandshake<P0>(hwebsocket: P0, presponseheaders: &[WEB_SOCKET_HTTP_HEADER], pulselectedextensions: ::core::option::Option<*mut u32>, pulselectedextensioncount: ::core::option::Option<*mut u32>, pulselectedsubprotocol: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketEndClientHandshake ( hwebsocket : WEB_SOCKET_HANDLE , presponseheaders : *const WEB_SOCKET_HTTP_HEADER , ulreponseheadercount : u32 , pulselectedextensions : *mut u32 , pulselectedextensioncount : *mut u32 , pulselectedsubprotocol : *mut u32 ) -> :: windows::core::HRESULT );
    WebSocketEndClientHandshake(hwebsocket.into_param().abi(), ::core::mem::transmute(presponseheaders.as_ptr()), presponseheaders.len() as _, ::core::mem::transmute(pulselectedextensions.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pulselectedextensioncount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pulselectedsubprotocol.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketEndServerHandshake<P0>(hwebsocket: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketEndServerHandshake ( hwebsocket : WEB_SOCKET_HANDLE ) -> :: windows::core::HRESULT );
    WebSocketEndServerHandshake(hwebsocket.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketGetAction<P0>(hwebsocket: P0, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: ::core::option::Option<*mut *mut ::core::ffi::c_void>, pvactioncontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketGetAction ( hwebsocket : WEB_SOCKET_HANDLE , eactionqueue : WEB_SOCKET_ACTION_QUEUE , pdatabuffers : *mut WEB_SOCKET_BUFFER , puldatabuffercount : *mut u32 , paction : *mut WEB_SOCKET_ACTION , pbuffertype : *mut WEB_SOCKET_BUFFER_TYPE , pvapplicationcontext : *mut *mut ::core::ffi::c_void , pvactioncontext : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WebSocketGetAction(hwebsocket.into_param().abi(), eactionqueue, pdatabuffers, puldatabuffercount, paction, pbuffertype, ::core::mem::transmute(pvapplicationcontext.unwrap_or(::std::ptr::null_mut())), pvactioncontext).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut ::core::ffi::c_void, ulsize: *mut u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketGetGlobalProperty ( etype : WEB_SOCKET_PROPERTY_TYPE , pvvalue : *mut ::core::ffi::c_void , ulsize : *mut u32 ) -> :: windows::core::HRESULT );
    WebSocketGetGlobalProperty(etype, pvvalue, ulsize).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketReceive<P0>(hwebsocket: P0, pbuffer: ::core::option::Option<*const WEB_SOCKET_BUFFER>, pvcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketReceive ( hwebsocket : WEB_SOCKET_HANDLE , pbuffer : *const WEB_SOCKET_BUFFER , pvcontext : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WebSocketReceive(hwebsocket.into_param().abi(), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvcontext.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
#[inline]
pub unsafe fn WebSocketSend<P0>(hwebsocket: P0, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: ::core::option::Option<*const WEB_SOCKET_BUFFER>, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<WEB_SOCKET_HANDLE>,
{
    ::windows::imp::link ! ( "websocket.dll""system" fn WebSocketSend ( hwebsocket : WEB_SOCKET_HANDLE , buffertype : WEB_SOCKET_BUFFER_TYPE , pbuffer : *const WEB_SOCKET_BUFFER , context : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WebSocketSend(hwebsocket.into_param().abi(), buffertype, ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null())), ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Networking_WebSocket\"`*"]
pub const WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
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
impl ::windows::core::TypeKind for WEB_SOCKET_ACTION {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for WEB_SOCKET_ACTION_QUEUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WEB_SOCKET_ACTION_QUEUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_ACTION_QUEUE").field(&self.0).finish()
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
impl ::windows::core::TypeKind for WEB_SOCKET_BUFFER_TYPE {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for WEB_SOCKET_CLOSE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
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
impl ::windows::core::TypeKind for WEB_SOCKET_PROPERTY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WEB_SOCKET_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_PROPERTY_TYPE").field(&self.0).finish()
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
impl ::windows::core::TypeKind for WEB_SOCKET_BUFFER {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for WEB_SOCKET_BUFFER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pbReason == other.pbReason && self.ulReasonLength == other.ulReasonLength && self.usStatus == other.usStatus
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
impl ::windows::core::TypeKind for WEB_SOCKET_BUFFER_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_BUFFER_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pbBuffer == other.pbBuffer && self.ulBufferLength == other.ulBufferLength
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_BUFFER_1 {}
impl ::core::default::Default for WEB_SOCKET_BUFFER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for WEB_SOCKET_HANDLE {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for WEB_SOCKET_HTTP_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_HTTP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.pcName == other.pcName && self.ulNameLength == other.ulNameLength && self.pcValue == other.pcValue && self.ulValueLength == other.ulValueLength
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_HTTP_HEADER {}
impl ::core::default::Default for WEB_SOCKET_HTTP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for WEB_SOCKET_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WEB_SOCKET_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pvValue == other.pvValue && self.ulValueSize == other.ulValueSize
    }
}
impl ::core::cmp::Eq for WEB_SOCKET_PROPERTY {}
impl ::core::default::Default for WEB_SOCKET_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
