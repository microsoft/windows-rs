#[cfg(feature = "implement_exclusive")]
pub trait IRadioImpl: Sized {
    fn SetStateAsync(&self, value: RadioState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RadioAccessStatus>>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Radio, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<RadioState>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<RadioKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioStaticsImpl: Sized {
    fn GetRadiosAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Radio>>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Radio>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RadioAccessStatus>>;
}
