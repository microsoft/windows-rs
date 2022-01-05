#[cfg(feature = "implement_exclusive")]
pub trait IAppCapabilityImpl: Sized {
    fn CapabilityName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&self) -> ::windows::core::Result<super::super::super::System::User>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AppCapabilityAccessStatus>>;
    fn CheckAccess(&self) -> ::windows::core::Result<AppCapabilityAccessStatus>;
    fn AccessChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AppCapability, AppCapabilityAccessChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCapabilityAccessChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCapabilityStaticsImpl: Sized {
    fn RequestAccessForCapabilitiesAsync(&self, capabilitynames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>;
    fn RequestAccessForCapabilitiesForUserAsync(&self, user: &::core::option::Option<super::super::super::System::User>, capabilitynames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>;
    fn Create(&self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<AppCapability>;
    fn CreateWithProcessIdForUser(&self, user: &::core::option::Option<super::super::super::System::User>, capabilityname: &::windows::core::HSTRING, pid: u32) -> ::windows::core::Result<AppCapability>;
}
