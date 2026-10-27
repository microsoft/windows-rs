#[inline]
pub unsafe fn WebSocketAbortHandle(hwebsocket: WEB_SOCKET_HANDLE) {
    windows_core::link!("websocket.dll" "system" fn WebSocketAbortHandle(hwebsocket : WEB_SOCKET_HANDLE));
    unsafe { WebSocketAbortHandle(hwebsocket) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketBeginClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocols: Option<&[windows_core::PCSTR]>, pszextensions: Option<&[windows_core::PCSTR]>, pinitialheaders: Option<&[WEB_SOCKET_HTTP_HEADER]>, padditionalheaders: *mut PWEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("websocket.dll" "system" fn WebSocketBeginClientHandshake(hwebsocket : WEB_SOCKET_HANDLE, pszsubprotocols : *const windows_core::PCSTR, ulsubprotocolcount : super::ULONG, pszextensions : *const windows_core::PCSTR, ulextensioncount : super::ULONG, pinitialheaders : PWEB_SOCKET_HTTP_HEADER, ulinitialheadercount : super::ULONG, padditionalheaders : *mut PWEB_SOCKET_HTTP_HEADER, puladditionalheadercount : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe {
        WebSocketBeginClientHandshake(
            hwebsocket,
            pszsubprotocols.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            pszsubprotocols.map_or(0, |slice| slice.len().try_into().unwrap()),
            pszextensions.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            pszextensions.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pinitialheaders.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pinitialheaders.map_or(0, |slice| slice.len().try_into().unwrap()),
            padditionalheaders as _,
            puladditionalheadercount as _,
        )
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketBeginServerHandshake<P1>(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocolselected: P1, pszextensionselected: Option<&[windows_core::PCSTR]>, prequestheaders: &[WEB_SOCKET_HTTP_HEADER], presponseheaders: *mut PWEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut super::ULONG) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("websocket.dll" "system" fn WebSocketBeginServerHandshake(hwebsocket : WEB_SOCKET_HANDLE, pszsubprotocolselected : windows_core::PCSTR, pszextensionselected : *const windows_core::PCSTR, ulextensionselectedcount : super::ULONG, prequestheaders : PWEB_SOCKET_HTTP_HEADER, ulrequestheadercount : super::ULONG, presponseheaders : *mut PWEB_SOCKET_HTTP_HEADER, pulresponseheadercount : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { WebSocketBeginServerHandshake(hwebsocket, pszsubprotocolselected.param().abi(), pszextensionselected.map_or(core::ptr::null(), |slice| slice.as_ptr()), pszextensionselected.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prequestheaders.as_ptr()), prequestheaders.len().try_into().unwrap(), presponseheaders as _, pulresponseheadercount as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketCompleteAction(hwebsocket: WEB_SOCKET_HANDLE, pvactioncontext: super::PVOID, ulbytestransferred: super::ULONG) {
    windows_core::link!("websocket.dll" "system" fn WebSocketCompleteAction(hwebsocket : WEB_SOCKET_HANDLE, pvactioncontext : super::PVOID, ulbytestransferred : super::ULONG));
    unsafe { WebSocketCompleteAction(hwebsocket, pvactioncontext, ulbytestransferred) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketCreateClientHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> windows_core::Result<WEB_SOCKET_HANDLE> {
    windows_core::link!("websocket.dll" "system" fn WebSocketCreateClientHandle(pproperties : PWEB_SOCKET_PROPERTY, ulpropertycount : super::ULONG, phwebsocket : *mut WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebSocketCreateClientHandle(core::mem::transmute(pproperties.as_ptr()), pproperties.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketCreateServerHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> windows_core::Result<WEB_SOCKET_HANDLE> {
    windows_core::link!("websocket.dll" "system" fn WebSocketCreateServerHandle(pproperties : PWEB_SOCKET_PROPERTY, ulpropertycount : super::ULONG, phwebsocket : *mut WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebSocketCreateServerHandle(core::mem::transmute(pproperties.as_ptr()), pproperties.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebSocketDeleteHandle(hwebsocket: WEB_SOCKET_HANDLE) {
    windows_core::link!("websocket.dll" "system" fn WebSocketDeleteHandle(hwebsocket : WEB_SOCKET_HANDLE));
    unsafe { WebSocketDeleteHandle(hwebsocket) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketEndClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, presponseheaders: &[WEB_SOCKET_HTTP_HEADER], pulselectedextensions: Option<*mut super::ULONG>, pulselectedextensioncount: Option<*mut super::ULONG>, pulselectedsubprotocol: Option<*mut super::ULONG>) -> windows_core::HRESULT {
    windows_core::link!("websocket.dll" "system" fn WebSocketEndClientHandshake(hwebsocket : WEB_SOCKET_HANDLE, presponseheaders : PWEB_SOCKET_HTTP_HEADER, ulreponseheadercount : super::ULONG, pulselectedextensions : *mut super::ULONG, pulselectedextensioncount : *mut super::ULONG, pulselectedsubprotocol : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { WebSocketEndClientHandshake(hwebsocket, core::mem::transmute(presponseheaders.as_ptr()), presponseheaders.len().try_into().unwrap(), pulselectedextensions.unwrap_or(core::mem::zeroed()) as _, pulselectedextensioncount.unwrap_or(core::mem::zeroed()) as _, pulselectedsubprotocol.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WebSocketEndServerHandshake(hwebsocket: WEB_SOCKET_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("websocket.dll" "system" fn WebSocketEndServerHandshake(hwebsocket : WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    unsafe { WebSocketEndServerHandshake(hwebsocket) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketGetAction(hwebsocket: WEB_SOCKET_HANDLE, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut super::ULONG, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: Option<*mut super::PVOID>, pvactioncontext: *mut super::PVOID) -> windows_core::HRESULT {
    windows_core::link!("websocket.dll" "system" fn WebSocketGetAction(hwebsocket : WEB_SOCKET_HANDLE, eactionqueue : WEB_SOCKET_ACTION_QUEUE, pdatabuffers : *mut WEB_SOCKET_BUFFER, puldatabuffercount : *mut super::ULONG, paction : *mut WEB_SOCKET_ACTION, pbuffertype : *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext : *mut super::PVOID, pvactioncontext : *mut super::PVOID) -> windows_core::HRESULT);
    unsafe { WebSocketGetAction(hwebsocket, eactionqueue, pdatabuffers as _, puldatabuffercount as _, paction as _, pbuffertype as _, pvapplicationcontext.unwrap_or(core::mem::zeroed()) as _, pvactioncontext as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: super::PVOID, ulsize: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("websocket.dll" "system" fn WebSocketGetGlobalProperty(etype : WEB_SOCKET_PROPERTY_TYPE, pvvalue : super::PVOID, ulsize : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { WebSocketGetGlobalProperty(etype, pvvalue as _, ulsize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketReceive(hwebsocket: WEB_SOCKET_HANDLE, pbuffer: Option<*const WEB_SOCKET_BUFFER>, pvcontext: Option<super::PVOID>) -> windows_core::HRESULT {
    windows_core::link!("websocket.dll" "system" fn WebSocketReceive(hwebsocket : WEB_SOCKET_HANDLE, pbuffer : *const WEB_SOCKET_BUFFER, pvcontext : super::PVOID) -> windows_core::HRESULT);
    unsafe { WebSocketReceive(hwebsocket, pbuffer.unwrap_or(core::mem::zeroed()) as _, pvcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebSocketSend(hwebsocket: WEB_SOCKET_HANDLE, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: Option<*const WEB_SOCKET_BUFFER>, context: Option<super::PVOID>) -> windows_core::HRESULT {
    windows_core::link!("websocket.dll" "system" fn WebSocketSend(hwebsocket : WEB_SOCKET_HANDLE, buffertype : WEB_SOCKET_BUFFER_TYPE, pbuffer : *const WEB_SOCKET_BUFFER, context : super::PVOID) -> windows_core::HRESULT);
    unsafe { WebSocketSend(hwebsocket, buffertype, pbuffer.unwrap_or(core::mem::zeroed()) as _, context.unwrap_or(core::mem::zeroed()) as _) }
}
pub type PWEB_SOCKET_ACTION = *mut WEB_SOCKET_ACTION;
#[cfg(feature = "shared")]
pub type PWEB_SOCKET_BUFFER = *mut WEB_SOCKET_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEB_SOCKET_HTTP_HEADER = *mut WEB_SOCKET_HTTP_HEADER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEB_SOCKET_PROPERTY = *mut WEB_SOCKET_PROPERTY;
pub const WEB_SOCKET_ABORTED_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1006;
pub type WEB_SOCKET_ACTION = i32;
pub type WEB_SOCKET_ACTION_QUEUE = i32;
pub const WEB_SOCKET_ALLOCATED_BUFFER_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = 3;
pub const WEB_SOCKET_ALL_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = 3;
pub const WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = -2147483645;
pub const WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = -2147483646;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WEB_SOCKET_BUFFER {
    pub Data: WEB_SOCKET_BUFFER_0,
    pub CloseStatus: WEB_SOCKET_BUFFER_1,
}
#[cfg(feature = "shared")]
impl Default for WEB_SOCKET_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_BUFFER_0 {
    pub pbBuffer: super::PBYTE,
    pub ulBufferLength: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_BUFFER_1 {
    pub pbReason: super::PBYTE,
    pub ulReasonLength: super::ULONG,
    pub usStatus: super::USHORT,
}
pub type WEB_SOCKET_BUFFER_TYPE = i32;
pub const WEB_SOCKET_CLOSE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = -2147483644;
pub type WEB_SOCKET_CLOSE_STATUS = i32;
pub const WEB_SOCKET_DISABLE_MASKING_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = 2;
pub const WEB_SOCKET_DISABLE_UTF8_VERIFICATION_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = 4;
pub const WEB_SOCKET_EMPTY_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1005;
pub const WEB_SOCKET_ENDPOINT_UNAVAILABLE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1001;
pub type WEB_SOCKET_HANDLE = *mut WEB_SOCKET_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_HANDLE__ {
    pub unused: i32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_HTTP_HEADER {
    pub pcName: super::PCHAR,
    pub ulNameLength: super::ULONG,
    pub pcValue: super::PCHAR,
    pub ulValueLength: super::ULONG,
}
pub const WEB_SOCKET_INDICATE_RECEIVE_COMPLETE_ACTION: WEB_SOCKET_ACTION = 4;
pub const WEB_SOCKET_INDICATE_SEND_COMPLETE_ACTION: WEB_SOCKET_ACTION = 2;
pub const WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1003;
pub const WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1007;
pub const WEB_SOCKET_KEEPALIVE_INTERVAL_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = 5;
pub const WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: i32 = 123;
pub const WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1009;
pub const WEB_SOCKET_NO_ACTION: WEB_SOCKET_ACTION = 0;
pub const WEB_SOCKET_PING_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = -2147483643;
pub const WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1008;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEB_SOCKET_PROPERTY {
    pub Type: WEB_SOCKET_PROPERTY_TYPE,
    pub pvValue: super::PVOID,
    pub ulValueSize: super::ULONG,
}
pub type WEB_SOCKET_PROPERTY_TYPE = i32;
pub const WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1002;
pub const WEB_SOCKET_RECEIVE_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = 2;
pub const WEB_SOCKET_RECEIVE_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = 0;
pub const WEB_SOCKET_RECEIVE_FROM_NETWORK_ACTION: WEB_SOCKET_ACTION = 3;
pub const WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1015;
pub const WEB_SOCKET_SEND_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = 1;
pub const WEB_SOCKET_SEND_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = 1;
pub const WEB_SOCKET_SEND_TO_NETWORK_ACTION: WEB_SOCKET_ACTION = 1;
pub const WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1011;
pub const WEB_SOCKET_SUCCESS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1000;
pub const WEB_SOCKET_SUPPORTED_VERSIONS_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = 6;
pub const WEB_SOCKET_UNSOLICITED_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = -2147483642;
pub const WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = 1010;
pub const WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = -2147483647;
pub const WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = -2147483648;
