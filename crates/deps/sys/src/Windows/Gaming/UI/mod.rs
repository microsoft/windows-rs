#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GameBar(i32);
pub struct GameChatMessageOrigin(i32);
pub struct GameChatMessageReceivedEventArgs(i32);
pub struct GameChatOverlay(i32);
pub struct GameChatOverlayContract(i32);
pub struct GameChatOverlayMessageSource(i32);
pub struct GameChatOverlayPosition(i32);
pub struct GameUIProviderActivatedEventArgs(i32);
pub struct GamingUIProviderContract(i32);
pub struct IGameBarStatics(pub *mut ::core::ffi::c_void);
pub struct IGameChatMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGameChatOverlay(pub *mut ::core::ffi::c_void);
pub struct IGameChatOverlayMessageSource(pub *mut ::core::ffi::c_void);
pub struct IGameChatOverlayStatics(pub *mut ::core::ffi::c_void);
pub struct IGameUIProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
