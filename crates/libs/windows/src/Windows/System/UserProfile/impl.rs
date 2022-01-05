#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerForUserImpl: Sized {
    fn AdvertisingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerStaticsImpl: Sized {
    fn AdvertisingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvertisingManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<AdvertisingManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAssignedAccessSettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsSingleAppKioskMode(&self) -> ::windows::core::Result<bool>;
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAssignedAccessSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AssignedAccessSettings>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<AssignedAccessSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticsSettingsImpl: Sized {
    fn CanUseDiagnosticsToTailorExperiences(&self) -> ::windows::core::Result<bool>;
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticsSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<DiagnosticsSettings>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<DiagnosticsSettings>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFirstSignInSettingsImpl: Sized + IIterableImpl<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapViewImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {}
#[cfg(feature = "implement_exclusive")]
pub trait IFirstSignInSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<FirstSignInSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesForUserImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
    fn Calendars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesStaticsImpl: Sized {
    fn Calendars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Clocks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Currencies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn HomeGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekStartsOn(&self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesStatics2Impl: Sized {
    fn TrySetHomeGeographicRegion(&self, region: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TrySetLanguages(&self, languagetags: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGlobalizationPreferencesStatics3Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<GlobalizationPreferencesForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenImageFeedStaticsImpl: Sized {
    fn RequestSetImageFeedAsync(&self, syndicationfeeduri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetImageFeedResult>>;
    fn TryRemoveImageFeed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenStaticsImpl: Sized {
    fn OriginalImageFile(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetImageStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetImageFileAsync(&self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetImageStreamAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IUserInformationStaticsImpl: Sized {
    fn AccountPictureChangeEnabled(&self) -> ::windows::core::Result<bool>;
    fn NameAccessAllowed(&self) -> ::windows::core::Result<bool>;
    fn GetAccountPicture(&self, kind: AccountPictureKind) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn SetAccountPictureAsync(&self, image: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPicturesAsync(&self, smallimage: &::core::option::Option<super::super::Storage::IStorageFile>, largeimage: &::core::option::Option<super::super::Storage::IStorageFile>, video: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPictureFromStreamAsync(&self, image: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn SetAccountPicturesFromStreamsAsync(&self, smallimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, largeimage: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, video: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>;
    fn AccountPictureChanged(&self, changehandler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountPictureChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetDisplayNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetFirstNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetLastNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetPrincipalNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetSessionInitiationProtocolUriAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetDomainNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserProfilePersonalizationSettingsImpl: Sized {
    fn TrySetLockScreenImageAsync(&self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetWallpaperImageAsync(&self, imagefile: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserProfilePersonalizationSettingsStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<UserProfilePersonalizationSettings>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
