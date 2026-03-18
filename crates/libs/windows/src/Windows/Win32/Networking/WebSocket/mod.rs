#[inline]
pub unsafe fn WebSocketAbortHandle(hwebsocket: WEB_SOCKET_HANDLE) {
    windows_core::link!("websocket.dll" "system" fn WebSocketAbortHandle(hwebsocket : WEB_SOCKET_HANDLE));
    unsafe { WebSocketAbortHandle(hwebsocket) }
}
#[inline]
pub unsafe fn WebSocketBeginClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocols: *const windows_core::PCSTR, ulsubprotocolcount: u32, pszextensions: *const windows_core::PCSTR, ulextensioncount: u32, pinitialheaders: *const WEB_SOCKET_HTTP_HEADER, ulinitialheadercount: u32, padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("websocket.dll" "system" fn WebSocketBeginClientHandshake(hwebsocket : WEB_SOCKET_HANDLE, pszsubprotocols : *const windows_core::PCSTR, ulsubprotocolcount : u32, pszextensions : *const windows_core::PCSTR, ulextensioncount : u32, pinitialheaders : *const WEB_SOCKET_HTTP_HEADER, ulinitialheadercount : u32, padditionalheaders : *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount : *mut u32) -> windows_core::HRESULT);
    unsafe { WebSocketBeginClientHandshake(hwebsocket, pszsubprotocols, ulsubprotocolcount, pszextensions, ulextensioncount, pinitialheaders, ulinitialheadercount, padditionalheaders as _, puladditionalheadercount as _).ok() }
}
#[inline]
pub unsafe fn WebSocketBeginServerHandshake<P1>(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocolselected: P1, pszextensionselected: *const windows_core::PCSTR, ulextensionselectedcount: u32, prequestheaders: *const WEB_SOCKET_HTTP_HEADER, ulrequestheadercount: u32, presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("websocket.dll" "system" fn WebSocketBeginServerHandshake(hwebsocket : WEB_SOCKET_HANDLE, pszsubprotocolselected : windows_core::PCSTR, pszextensionselected : *const windows_core::PCSTR, ulextensionselectedcount : u32, prequestheaders : *const WEB_SOCKET_HTTP_HEADER, ulrequestheadercount : u32, presponseheaders : *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount : *mut u32) -> windows_core::HRESULT);
    unsafe { WebSocketBeginServerHandshake(hwebsocket, pszsubprotocolselected.param().abi(), pszextensionselected, ulextensionselectedcount, prequestheaders, ulrequestheadercount, presponseheaders as _, pulresponseheadercount as _).ok() }
}
#[inline]
pub unsafe fn WebSocketCompleteAction(hwebsocket: WEB_SOCKET_HANDLE, pvactioncontext: *const core::ffi::c_void, ulbytestransferred: u32) {
    windows_core::link!("websocket.dll" "system" fn WebSocketCompleteAction(hwebsocket : WEB_SOCKET_HANDLE, pvactioncontext : *const core::ffi::c_void, ulbytestransferred : u32));
    unsafe { WebSocketCompleteAction(hwebsocket, pvactioncontext, ulbytestransferred) }
}
#[inline]
pub unsafe fn WebSocketCreateClientHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32) -> windows_core::Result<WEB_SOCKET_HANDLE> {
    windows_core::link!("websocket.dll" "system" fn WebSocketCreateClientHandle(pproperties : *const WEB_SOCKET_PROPERTY, ulpropertycount : u32, phwebsocket : *mut WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebSocketCreateClientHandle(pproperties, ulpropertycount, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebSocketCreateServerHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32) -> windows_core::Result<WEB_SOCKET_HANDLE> {
    windows_core::link!("websocket.dll" "system" fn WebSocketCreateServerHandle(pproperties : *const WEB_SOCKET_PROPERTY, ulpropertycount : u32, phwebsocket : *mut WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebSocketCreateServerHandle(pproperties, ulpropertycount, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebSocketDeleteHandle(hwebsocket: WEB_SOCKET_HANDLE) {
    windows_core::link!("websocket.dll" "system" fn WebSocketDeleteHandle(hwebsocket : WEB_SOCKET_HANDLE));
    unsafe { WebSocketDeleteHandle(hwebsocket) }
}
#[inline]
pub unsafe fn WebSocketEndClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, presponseheaders: *const WEB_SOCKET_HTTP_HEADER, ulreponseheadercount: u32, pulselectedextensions: *mut u32, pulselectedextensioncount: *mut u32, pulselectedsubprotocol: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("websocket.dll" "system" fn WebSocketEndClientHandshake(hwebsocket : WEB_SOCKET_HANDLE, presponseheaders : *const WEB_SOCKET_HTTP_HEADER, ulreponseheadercount : u32, pulselectedextensions : *mut u32, pulselectedextensioncount : *mut u32, pulselectedsubprotocol : *mut u32) -> windows_core::HRESULT);
    unsafe { WebSocketEndClientHandshake(hwebsocket, presponseheaders, ulreponseheadercount, pulselectedextensions as _, pulselectedextensioncount as _, pulselectedsubprotocol as _).ok() }
}
#[inline]
pub unsafe fn WebSocketEndServerHandshake(hwebsocket: WEB_SOCKET_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("websocket.dll" "system" fn WebSocketEndServerHandshake(hwebsocket : WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    unsafe { WebSocketEndServerHandshake(hwebsocket).ok() }
}
#[inline]
pub unsafe fn WebSocketGetAction(hwebsocket: WEB_SOCKET_HANDLE, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: *mut *mut core::ffi::c_void, pvactioncontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("websocket.dll" "system" fn WebSocketGetAction(hwebsocket : WEB_SOCKET_HANDLE, eactionqueue : WEB_SOCKET_ACTION_QUEUE, pdatabuffers : *mut WEB_SOCKET_BUFFER, puldatabuffercount : *mut u32, paction : *mut WEB_SOCKET_ACTION, pbuffertype : *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext : *mut *mut core::ffi::c_void, pvactioncontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WebSocketGetAction(hwebsocket, eactionqueue, pdatabuffers as _, puldatabuffercount as _, paction as _, pbuffertype as _, pvapplicationcontext as _, pvactioncontext as _).ok() }
}
#[inline]
pub unsafe fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut core::ffi::c_void, ulsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("websocket.dll" "system" fn WebSocketGetGlobalProperty(etype : WEB_SOCKET_PROPERTY_TYPE, pvvalue : *mut core::ffi::c_void, ulsize : *mut u32) -> windows_core::HRESULT);
    unsafe { WebSocketGetGlobalProperty(etype, pvvalue as _, ulsize as _).ok() }
}
#[inline]
pub unsafe fn WebSocketReceive(hwebsocket: WEB_SOCKET_HANDLE, pbuffer: *const WEB_SOCKET_BUFFER, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("websocket.dll" "system" fn WebSocketReceive(hwebsocket : WEB_SOCKET_HANDLE, pbuffer : *const WEB_SOCKET_BUFFER, pvcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WebSocketReceive(hwebsocket, pbuffer, pvcontext).ok() }
}
#[inline]
pub unsafe fn WebSocketSend(hwebsocket: WEB_SOCKET_HANDLE, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: *const WEB_SOCKET_BUFFER, context: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("websocket.dll" "system" fn WebSocketSend(hwebsocket : WEB_SOCKET_HANDLE, buffertype : WEB_SOCKET_BUFFER_TYPE, pbuffer : *const WEB_SOCKET_BUFFER, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WebSocketSend(hwebsocket, buffertype, pbuffer, context).ok() }
}
pub const WEB_SOCKET_ABORTED_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1006i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_ACTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_ACTION_QUEUE(pub i32);
pub const WEB_SOCKET_ALLOCATED_BUFFER_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(3i32);
pub const WEB_SOCKET_ALL_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(3i32);
pub const WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483645i32);
pub const WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483646i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union WEB_SOCKET_BUFFER {
    pub Data: WEB_SOCKET_BUFFER_0,
    pub CloseStatus: WEB_SOCKET_BUFFER_1,
}
impl Default for WEB_SOCKET_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEB_SOCKET_BUFFER_0 {
    pub pbBuffer: *mut u8,
    pub ulBufferLength: u32,
}
impl Default for WEB_SOCKET_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEB_SOCKET_BUFFER_1 {
    pub pbReason: *mut u8,
    pub ulReasonLength: u32,
    pub usStatus: u16,
}
impl Default for WEB_SOCKET_BUFFER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_BUFFER_TYPE(pub i32);
pub const WEB_SOCKET_CLOSE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483644i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_CLOSE_STATUS(pub i32);
pub const WEB_SOCKET_DISABLE_MASKING_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(2i32);
pub const WEB_SOCKET_DISABLE_UTF8_VERIFICATION_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(4i32);
pub const WEB_SOCKET_EMPTY_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1005i32);
pub const WEB_SOCKET_ENDPOINT_UNAVAILABLE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1001i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WEB_SOCKET_HANDLE(pub *mut core::ffi::c_void);
impl WEB_SOCKET_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for WEB_SOCKET_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("websocket.dll" "system" fn WebSocketDeleteHandle(hwebsocket : *mut core::ffi::c_void));
            unsafe {
                WebSocketDeleteHandle(self.0);
            }
        }
    }
}
impl Default for WEB_SOCKET_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEB_SOCKET_HTTP_HEADER {
    pub pcName: windows_core::PSTR,
    pub ulNameLength: u32,
    pub pcValue: windows_core::PSTR,
    pub ulValueLength: u32,
}
pub const WEB_SOCKET_INDICATE_RECEIVE_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(4i32);
pub const WEB_SOCKET_INDICATE_SEND_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(2i32);
pub const WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1003i32);
pub const WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1007i32);
pub const WEB_SOCKET_KEEPALIVE_INTERVAL_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(5i32);
pub const WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
pub const WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1009i32);
pub const WEB_SOCKET_NO_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(0i32);
pub const WEB_SOCKET_PING_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483643i32);
pub const WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1008i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEB_SOCKET_PROPERTY {
    pub Type: WEB_SOCKET_PROPERTY_TYPE,
    pub pvValue: *mut core::ffi::c_void,
    pub ulValueSize: u32,
}
impl Default for WEB_SOCKET_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_PROPERTY_TYPE(pub i32);
pub const WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1002i32);
pub const WEB_SOCKET_RECEIVE_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(2i32);
pub const WEB_SOCKET_RECEIVE_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(0i32);
pub const WEB_SOCKET_RECEIVE_FROM_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(3i32);
pub const WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1015i32);
pub const WEB_SOCKET_SEND_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(1i32);
pub const WEB_SOCKET_SEND_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(1i32);
pub const WEB_SOCKET_SEND_TO_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(1i32);
pub const WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1011i32);
pub const WEB_SOCKET_SUCCESS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1000i32);
pub const WEB_SOCKET_SUPPORTED_VERSIONS_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(6i32);
pub const WEB_SOCKET_UNSOLICITED_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483642i32);
pub const WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1010i32);
pub const WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483647i32);
pub const WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483648i32);
