#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GameBar();
    fn GameChatMessageOrigin();
    fn GameChatMessageReceivedEventArgs();
    fn GameChatOverlay();
    fn GameChatOverlayContract();
    fn GameChatOverlayMessageSource();
    fn GameChatOverlayPosition();
    fn GameUIProviderActivatedEventArgs();
    fn GamingUIProviderContract();
    fn IGameBarStatics();
    fn IGameChatMessageReceivedEventArgs();
    fn IGameChatOverlay();
    fn IGameChatOverlayMessageSource();
    fn IGameChatOverlayStatics();
    fn IGameUIProviderActivatedEventArgs();
}
