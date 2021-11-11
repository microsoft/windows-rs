#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn WEB_SOCKET_ACTION();
    fn WEB_SOCKET_ACTION_QUEUE();
    fn WEB_SOCKET_BUFFER();
    fn WEB_SOCKET_BUFFER_TYPE();
    fn WEB_SOCKET_CLOSE_STATUS();
    fn WEB_SOCKET_HANDLE();
    fn WEB_SOCKET_HTTP_HEADER();
    fn WEB_SOCKET_MAX_CLOSE_REASON_LENGTH();
    fn WEB_SOCKET_PROPERTY();
    fn WEB_SOCKET_PROPERTY_TYPE();
    fn WebSocketAbortHandle();
    fn WebSocketBeginClientHandshake();
    fn WebSocketBeginServerHandshake();
    fn WebSocketCompleteAction();
    fn WebSocketCreateClientHandle();
    fn WebSocketCreateServerHandle();
    fn WebSocketDeleteHandle();
    fn WebSocketEndClientHandshake();
    fn WebSocketEndServerHandshake();
    fn WebSocketGetAction();
    fn WebSocketGetGlobalProperty();
    fn WebSocketReceive();
    fn WebSocketSend();
}
