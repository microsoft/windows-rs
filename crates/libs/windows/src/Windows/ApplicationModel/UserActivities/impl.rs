#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityImpl: Sized {
    fn State(&self) -> ::windows::core::Result<UserActivityState>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VisualElements(&self) -> ::windows::core::Result<UserActivityVisualElements>;
    fn ContentUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetFallbackUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ActivationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetActivationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentInfo(&self) -> ::windows::core::Result<IUserActivityContentInfo>;
    fn SetContentInfo(&self, value: &::core::option::Option<IUserActivityContentInfo>) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateSession(&self) -> ::windows::core::Result<UserActivitySession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivity2Impl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivity3Impl: Sized {
    fn IsRoamable(&self) -> ::windows::core::Result<bool>;
    fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityAttributionImpl: Sized {
    fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetIconUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlternateText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddImageQuery(&self) -> ::windows::core::Result<bool>;
    fn SetAddImageQuery(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityAttributionFactoryImpl: Sized {
    fn CreateWithUri(&self, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<UserActivityAttribution>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityChannelImpl: Sized {
    fn GetOrCreateUserActivityAsync(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserActivity>>;
    fn DeleteActivityAsync(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAllActivitiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityChannel2Impl: Sized {
    fn GetRecentUserActivitiesAsync(&self, maxuniqueactivities: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>;
    fn GetSessionHistoryItemsForUserActivityAsync(&self, activityid: &::windows::core::HSTRING, starttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityChannelStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityChannelStatics2Impl: Sized {
    fn DisableAutoSessionCreation(&self) -> ::windows::core::Result<()>;
    fn TryGetForWebAccount(&self, account: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityChannelStatics3Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserActivityChannel>;
}
pub trait IUserActivityContentInfoImpl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityContentInfoStaticsImpl: Sized {
    fn FromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivityContentInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityFactoryImpl: Sized {
    fn CreateWithActivityId(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequestImpl: Sized {
    fn SetUserActivity(&self, activity: &::core::option::Option<UserActivity>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequestManagerImpl: Sized {
    fn UserActivityRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserActivityRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequestManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<UserActivityRequestManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<UserActivityRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivitySessionImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivitySessionHistoryItemImpl: Sized {
    fn UserActivity(&self) -> ::windows::core::Result<UserActivity>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityStaticsImpl: Sized {
    fn TryParseFromJson(&self, json: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity>;
    fn TryParseFromJsonArray(&self, json: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<UserActivity>>;
    fn ToJsonArray(&self, activities: &::core::option::Option<super::super::Foundation::Collections::IIterable<UserActivity>>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityVisualElementsImpl: Sized {
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Attribution(&self) -> ::windows::core::Result<UserActivityAttribution>;
    fn SetAttribution(&self, value: &::core::option::Option<UserActivityAttribution>) -> ::windows::core::Result<()>;
    fn SetContent(&self, value: &::core::option::Option<super::super::UI::Shell::IAdaptiveCard>) -> ::windows::core::Result<()>;
    fn Content(&self) -> ::windows::core::Result<super::super::UI::Shell::IAdaptiveCard>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityVisualElements2Impl: Sized {
    fn AttributionDisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAttributionDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
