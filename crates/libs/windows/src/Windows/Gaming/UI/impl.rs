#[cfg(feature = "implement_exclusive")]
pub trait IGameBarStaticsImpl: Sized {
    fn VisibilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsInputRedirectedChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsInputRedirectedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn IsInputRedirected(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatMessageReceivedEventArgsImpl: Sized {
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SenderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Origin(&self) -> ::windows::core::Result<GameChatMessageOrigin>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatOverlayImpl: Sized {
    fn DesiredPosition(&self) -> ::windows::core::Result<GameChatOverlayPosition>;
    fn SetDesiredPosition(&self, value: GameChatOverlayPosition) -> ::windows::core::Result<()>;
    fn AddMessage(&self, sender: &::windows::core::HSTRING, message: &::windows::core::HSTRING, origin: GameChatMessageOrigin) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatOverlayMessageSourceImpl: Sized {
    fn MessageReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDelayBeforeClosingAfterMessageReceived(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatOverlayStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GameChatOverlay>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IGameUIProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn GameUIArgs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn ReportCompleted(&self, results: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
