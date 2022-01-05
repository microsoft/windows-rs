#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAccessManagerStaticsImpl: Sized {
    fn IsBlockingActive(&self) -> ::windows::core::Result<bool>;
    fn IsBlockedNumberAsync(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowBlockNumbersUI(&self, phonenumbers: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn ShowUnblockNumbersUI(&self, phonenumbers: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn ShowBlockedCallsUI(&self) -> ::windows::core::Result<()>;
    fn ShowBlockedMessagesUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAppManagerStaticsImpl: Sized {
    fn IsCurrentAppActiveBlockingApp(&self) -> ::windows::core::Result<bool>;
    fn ShowCommunicationBlockingSettingsUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAppManagerStatics2Impl: Sized + ICommunicationBlockingAppManagerStaticsImpl {
    fn RequestSetAsActiveBlockingAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
