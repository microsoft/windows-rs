#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn WebSocketAbortHandle(hwebsocket: WEB_SOCKET_HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebSocketBeginClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocols: *const super::super::Foundation::PSTR, ulsubprotocolcount: u32, pszextensions: *const super::super::Foundation::PSTR, ulextensioncount: u32, pinitialheaders: *const WEB_SOCKET_HTTP_HEADER, ulinitialheadercount: u32, padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebSocketBeginServerHandshake(hwebsocket: WEB_SOCKET_HANDLE, pszsubprotocolselected: super::super::Foundation::PSTR, pszextensionselected: *const super::super::Foundation::PSTR, ulextensionselectedcount: u32, prequestheaders: *const WEB_SOCKET_HTTP_HEADER, ulrequestheadercount: u32, presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketCompleteAction(hwebsocket: WEB_SOCKET_HANDLE, pvactioncontext: *const ::core::ffi::c_void, ulbytestransferred: u32);
    pub fn WebSocketCreateClientHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32, phwebsocket: *mut WEB_SOCKET_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketCreateServerHandle(pproperties: *const WEB_SOCKET_PROPERTY, ulpropertycount: u32, phwebsocket: *mut WEB_SOCKET_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketDeleteHandle(hwebsocket: WEB_SOCKET_HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebSocketEndClientHandshake(hwebsocket: WEB_SOCKET_HANDLE, presponseheaders: *const WEB_SOCKET_HTTP_HEADER, ulreponseheadercount: u32, pulselectedextensions: *mut u32, pulselectedextensioncount: *mut u32, pulselectedsubprotocol: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketEndServerHandshake(hwebsocket: WEB_SOCKET_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketGetAction(hwebsocket: WEB_SOCKET_HANDLE, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: *mut *mut ::core::ffi::c_void, pvactioncontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut ::core::ffi::c_void, ulsize: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketReceive(hwebsocket: WEB_SOCKET_HANDLE, pbuffer: *const WEB_SOCKET_BUFFER, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WebSocketSend(hwebsocket: WEB_SOCKET_HANDLE, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: *const WEB_SOCKET_BUFFER, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct WEB_SOCKET_ACTION(i32);
#[repr(C)]
pub struct WEB_SOCKET_ACTION_QUEUE(i32);
#[repr(C)]
pub struct WEB_SOCKET_BUFFER(i32);
#[repr(C)]
pub struct WEB_SOCKET_BUFFER_TYPE(i32);
#[repr(C)]
pub struct WEB_SOCKET_CLOSE_STATUS(i32);
#[repr(C)]
pub struct WEB_SOCKET_HANDLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WEB_SOCKET_HTTP_HEADER(i32);
pub const WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
#[repr(C)]
pub struct WEB_SOCKET_PROPERTY(i32);
#[repr(C)]
pub struct WEB_SOCKET_PROPERTY_TYPE(i32);
