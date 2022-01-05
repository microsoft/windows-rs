pub trait IAdaptiveNotificationContentImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind>;
    fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveNotificationTextImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeNotificationFactoryImpl: Sized {
    fn CreateBadgeNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<BadgeNotification>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdateManagerForUserImpl: Sized {
    fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdateManagerStaticsImpl: Sized {
    fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn GetTemplateContent(&self, r#type: BadgeTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdateManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<BadgeUpdateManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<BadgeNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdate(&self, badgecontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, badgecontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownAdaptiveNotificationHintsStaticsImpl: Sized {
    fn Style(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wrap(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxLines(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinLines(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TextStacking(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Align(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownAdaptiveNotificationTextStylesStaticsImpl: Sized {
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Base(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subheader(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Header(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TitleNumeral(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubheaderNumeral(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HeaderNumeral(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CaptionSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BodySubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BaseSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubtitleSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TitleSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubheaderSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubheaderNumeralSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HeaderSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HeaderNumeralSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownNotificationBindingsStaticsImpl: Sized {
    fn ToastGeneric(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationImpl: Sized {
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Visual(&self) -> ::windows::core::Result<NotificationVisual>;
    fn SetVisual(&self, value: &::core::option::Option<NotificationVisual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationBindingImpl: Sized {
    fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTemplate(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn GetTextElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationDataImpl: Sized {
    fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SequenceNumber(&self) -> ::windows::core::Result<u32>;
    fn SetSequenceNumber(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationDataFactoryImpl: Sized {
    fn CreateNotificationDataWithValuesAndSequenceNumber(&self, initialvalues: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, sequencenumber: u32) -> ::windows::core::Result<NotificationData>;
    fn CreateNotificationDataWithValues(&self, initialvalues: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<NotificationData>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationVisualImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Bindings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<NotificationBinding>>;
    fn GetBinding(&self, templatename: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationBinding>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledTileNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledTileNotificationFactoryImpl: Sized {
    fn CreateScheduledTileNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledTileNotification>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SnoozeInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn MaximumSnoozeCount(&self) -> ::windows::core::Result<u32>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotification2Impl: Sized {
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()>;
    fn SuppressPopup(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotification3Impl: Sized {
    fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring>;
    fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotification4Impl: Sized {
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotificationFactoryImpl: Sized {
    fn CreateScheduledToastNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledToastNotification>;
    fn CreateScheduledToastNotificationRecurring(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime, snoozeinterval: &super::super::Foundation::TimeSpan, maximumsnoozecount: u32) -> ::windows::core::Result<ScheduledToastNotification>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotificationShowingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn ScheduledToastNotification(&self) -> ::windows::core::Result<ScheduledToastNotification>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShownTileNotificationImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutNotificationFactoryImpl: Sized {
    fn CreateTileFlyoutNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<TileFlyoutNotification>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutUpdateManagerStaticsImpl: Sized {
    fn CreateTileFlyoutUpdaterForApplication(&self) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn CreateTileFlyoutUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn CreateTileFlyoutUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn GetTemplateContent(&self, r#type: TileFlyoutTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<TileFlyoutNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdate(&self, tileflyoutcontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, tileflyoutcontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileNotificationFactoryImpl: Sized {
    fn CreateTileNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<TileNotification>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdateManagerForUserImpl: Sized {
    fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdateManagerStaticsImpl: Sized {
    fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn GetTemplateContent(&self, r#type: TileTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdateManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<TileUpdateManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<TileNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn EnableNotificationQueue(&self, enable: bool) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
    fn AddToSchedule(&self, scheduledtile: &::core::option::Option<ScheduledTileNotification>) -> ::windows::core::Result<()>;
    fn RemoveFromSchedule(&self, scheduledtile: &::core::option::Option<ScheduledTileNotification>) -> ::windows::core::Result<()>;
    fn GetScheduledTileNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledTileNotification>>;
    fn StartPeriodicUpdate(&self, tilecontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, tilecontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateBatch(&self, tilecontents: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateBatchAtTime(&self, tilecontents: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdater2Impl: Sized {
    fn EnableNotificationQueueForSquare150x150(&self, enable: bool) -> ::windows::core::Result<()>;
    fn EnableNotificationQueueForWide310x150(&self, enable: bool) -> ::windows::core::Result<()>;
    fn EnableNotificationQueueForSquare310x310(&self, enable: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastActivatedEventArgsImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastActivatedEventArgs2Impl: Sized {
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastCollectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LaunchArgs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLaunchArgs(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetIcon(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastCollectionFactoryImpl: Sized {
    fn CreateInstance(&self, collectionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, launchargs: &::windows::core::HSTRING, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<ToastCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastCollectionManagerImpl: Sized {
    fn SaveToastCollectionAsync(&self, collection: &::core::option::Option<ToastCollection>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FindAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ToastCollection>>>;
    fn GetToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastCollection>>;
    fn RemoveToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RemoveAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastDismissedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<ToastDismissalReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastFailedEventArgsImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn Dismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Failed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification2Impl: Sized {
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()>;
    fn SuppressPopup(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification3Impl: Sized {
    fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring>;
    fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification4Impl: Sized {
    fn Data(&self) -> ::windows::core::Result<NotificationData>;
    fn SetData(&self, value: &::core::option::Option<NotificationData>) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<ToastNotificationPriority>;
    fn SetPriority(&self, value: ToastNotificationPriority) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification6Impl: Sized {
    fn ExpiresOnReboot(&self) -> ::windows::core::Result<bool>;
    fn SetExpiresOnReboot(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationActionTriggerDetailImpl: Sized {
    fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationFactoryImpl: Sized {
    fn CreateToastNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<ToastNotification>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryImpl: Sized {
    fn RemoveGroup(&self, group: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveGroupWithId(&self, group: &::windows::core::HSTRING, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveGroupedTagWithId(&self, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveGroupedTag(&self, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Remove(&self, tag: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn ClearWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistory2Impl: Sized {
    fn GetHistory(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>>;
    fn GetHistoryWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryChangedTriggerDetailImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<ToastHistoryChangedType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryChangedTriggerDetail2Impl: Sized {
    fn CollectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerForUserImpl: Sized {
    fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier>;
    fn CreateToastNotifierWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier>;
    fn History(&self) -> ::windows::core::Result<ToastNotificationHistory>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerForUser2Impl: Sized {
    fn GetToastNotifierForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotifier>>;
    fn GetHistoryForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>>;
    fn GetToastCollectionManager(&self) -> ::windows::core::Result<ToastCollectionManager>;
    fn GetToastCollectionManagerWithAppId(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastCollectionManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStaticsImpl: Sized {
    fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier>;
    fn CreateToastNotifierWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier>;
    fn GetTemplateContent(&self, r#type: ToastTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStatics2Impl: Sized {
    fn History(&self) -> ::windows::core::Result<ToastNotificationHistory>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStatics4Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<ToastNotificationManagerForUser>;
    fn ConfigureNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStatics5Impl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<ToastNotificationManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotifierImpl: Sized {
    fn Show(&self, notification: &::core::option::Option<ToastNotification>) -> ::windows::core::Result<()>;
    fn Hide(&self, notification: &::core::option::Option<ToastNotification>) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
    fn AddToSchedule(&self, scheduledtoast: &::core::option::Option<ScheduledToastNotification>) -> ::windows::core::Result<()>;
    fn RemoveFromSchedule(&self, scheduledtoast: &::core::option::Option<ScheduledToastNotification>) -> ::windows::core::Result<()>;
    fn GetScheduledToastNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotifier2Impl: Sized {
    fn UpdateWithTagAndGroup(&self, data: &::core::option::Option<NotificationData>, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationUpdateResult>;
    fn UpdateWithTag(&self, data: &::core::option::Option<NotificationData>, tag: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationUpdateResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotifier3Impl: Sized {
    fn ScheduledToastNotificationShowing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScheduledToastNotificationShowing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationImpl: Sized {
    fn Notification(&self) -> ::windows::core::Result<Notification>;
    fn AppInfo(&self) -> ::windows::core::Result<super::super::ApplicationModel::AppInfo>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn CreationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationChangedEventArgsImpl: Sized {
    fn ChangeKind(&self) -> ::windows::core::Result<UserNotificationChangedKind>;
    fn UserNotificationId(&self) -> ::windows::core::Result<u32>;
}
