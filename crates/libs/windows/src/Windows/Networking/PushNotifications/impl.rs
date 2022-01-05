#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn PushNotificationReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePushNotificationReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerForUserImpl: Sized {
    fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerForUser2Impl: Sized {
    fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(&self, appserverkey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, channelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(&self, appserverkey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, channelid: &::windows::core::HSTRING, appid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStaticsImpl: Sized {
    fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<PushNotificationChannelManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStatics3Impl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PushNotificationChannelManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStatics4Impl: Sized {
    fn ChannelsRevoked(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChannelsRevoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelsRevokedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationReceivedEventArgsImpl: Sized {
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn NotificationType(&self) -> ::windows::core::Result<PushNotificationType>;
    fn ToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn TileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn BadgeNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeNotification>;
    fn RawNotification(&self) -> ::windows::core::Result<RawNotification>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawNotification2Impl: Sized {
    fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn ChannelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawNotification3Impl: Sized {
    fn ContentBytes(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
