#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameChatMessageOrigin(pub i32);
impl GameChatMessageOrigin {
    pub const Voice: GameChatMessageOrigin = GameChatMessageOrigin(0i32);
    pub const Text: GameChatMessageOrigin = GameChatMessageOrigin(1i32);
}
#[repr(transparent)]
pub struct GameChatMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameChatOverlay(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GameChatOverlayContract(i32);
#[repr(transparent)]
pub struct GameChatOverlayMessageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameChatOverlayPosition(pub i32);
impl GameChatOverlayPosition {
    pub const BottomCenter: GameChatOverlayPosition = GameChatOverlayPosition(0i32);
    pub const BottomLeft: GameChatOverlayPosition = GameChatOverlayPosition(1i32);
    pub const BottomRight: GameChatOverlayPosition = GameChatOverlayPosition(2i32);
    pub const MiddleRight: GameChatOverlayPosition = GameChatOverlayPosition(3i32);
    pub const MiddleLeft: GameChatOverlayPosition = GameChatOverlayPosition(4i32);
    pub const TopCenter: GameChatOverlayPosition = GameChatOverlayPosition(5i32);
    pub const TopLeft: GameChatOverlayPosition = GameChatOverlayPosition(6i32);
    pub const TopRight: GameChatOverlayPosition = GameChatOverlayPosition(7i32);
}
#[repr(transparent)]
pub struct GameUIProviderActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
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
