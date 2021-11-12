#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameBar(pub *mut ::core::ffi::c_void);
pub struct GameChatMessageOrigin(i32);
#[repr(transparent)]
pub struct GameChatMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameChatOverlay(pub *mut ::core::ffi::c_void);
pub struct GameChatOverlayContract(i32);
#[repr(transparent)]
pub struct GameChatOverlayMessageSource(pub *mut ::core::ffi::c_void);
pub struct GameChatOverlayPosition(i32);
#[repr(transparent)]
pub struct GameUIProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
pub struct GamingUIProviderContract(i32);
#[repr(transparent)]
pub struct IGameBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameChatMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameChatOverlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameChatOverlayMessageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameChatOverlayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameUIProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
