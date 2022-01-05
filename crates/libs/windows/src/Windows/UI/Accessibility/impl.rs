#[cfg(feature = "implement_exclusive")]
pub trait IScreenReaderPositionChangedEventArgsImpl: Sized {
    fn ScreenPositionInRawPixels(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn IsReadingText(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScreenReaderServiceImpl: Sized {
    fn CurrentScreenReaderPosition(&self) -> ::windows::core::Result<ScreenReaderPositionChangedEventArgs>;
    fn ScreenReaderPositionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenReaderPositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
