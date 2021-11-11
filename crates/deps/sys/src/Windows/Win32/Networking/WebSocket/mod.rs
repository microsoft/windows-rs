#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketAbortHandle();
    #[doc = "*Required features: `Win32_Networking_WebSocket`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebSocketBeginClientHandshake();
    #[doc = "*Required features: `Win32_Networking_WebSocket`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebSocketBeginServerHandshake();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketCompleteAction();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketCreateClientHandle();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketCreateServerHandle();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketDeleteHandle();
    #[doc = "*Required features: `Win32_Networking_WebSocket`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WebSocketEndClientHandshake();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketEndServerHandshake();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketGetAction();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketGetGlobalProperty();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketReceive();
    #[doc = "*Required features: `Win32_Networking_WebSocket`*"]
    pub fn WebSocketSend();
}
