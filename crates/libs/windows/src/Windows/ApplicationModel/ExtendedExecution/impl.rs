#[cfg(feature = "implement_exclusive")]
pub trait IExtendedExecutionRevokedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionRevokedReason>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IExtendedExecutionSessionImpl: Sized + IClosableImpl {
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionReason>;
    fn SetReason(&self, value: ExtendedExecutionReason) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PercentProgress(&self) -> ::windows::core::Result<u32>;
    fn SetPercentProgress(&self, value: u32) -> ::windows::core::Result<()>;
    fn Revoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRevoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestExtensionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>>;
}
