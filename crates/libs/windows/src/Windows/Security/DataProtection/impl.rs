#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAvailabilityStateChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataBufferUnprotectResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<UserDataBufferUnprotectStatus>;
    fn UnprotectedBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataProtectionManagerImpl: Sized {
    fn ProtectStorageItemAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>>;
    fn GetStorageItemProtectionInfoAsync(&self, storageitem: &::core::option::Option<super::super::Storage::IStorageItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>>;
    fn ProtectBufferAsync(&self, unprotectedbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, availability: UserDataAvailability) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn UnprotectBufferAsync(&self, protectedbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>>;
    fn IsContinuedDataAvailabilityExpected(&self, availability: UserDataAvailability) -> ::windows::core::Result<bool>;
    fn DataAvailabilityStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataAvailabilityStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataProtectionManagerStaticsImpl: Sized {
    fn TryGetDefault(&self) -> ::windows::core::Result<UserDataProtectionManager>;
    fn TryGetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserDataProtectionManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataStorageItemProtectionInfoImpl: Sized {
    fn Availability(&self) -> ::windows::core::Result<UserDataAvailability>;
}
