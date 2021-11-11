#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
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
