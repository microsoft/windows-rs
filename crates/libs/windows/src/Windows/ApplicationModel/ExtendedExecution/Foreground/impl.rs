#[cfg(feature = "implement_exclusive")]
pub trait IExtendedExecutionForegroundRevokedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionForegroundRevokedReason>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IExtendedExecutionForegroundSessionImpl: Sized + IClosableImpl {
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Revoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionForegroundRevokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRevoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestExtensionAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>>;
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionForegroundReason>;
    fn SetReason(&self, value: ExtendedExecutionForegroundReason) -> ::windows::core::Result<()>;
}
