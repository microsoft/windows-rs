#[cfg(feature = "implement_exclusive")]
pub trait IAccessoryManagerImpl: Sized {
    fn RegisterAccessoryApp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNextTriggerDetails(&self) -> ::windows::core::Result<IAccessoryNotificationTriggerDetails>;
    fn ProcessTriggerDetails(&self, pdetails: &::core::option::Option<IAccessoryNotificationTriggerDetails>) -> ::windows::core::Result<()>;
    fn PhoneLineDetails(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>>;
    fn GetPhoneLineDetails(&self, phoneline: &::windows::core::GUID) -> ::windows::core::Result<PhoneLineDetails>;
    fn AcceptPhoneCall(&self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn AcceptPhoneCallOnEndpoint(&self, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn AcceptPhoneCallWithVideo(&self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn AcceptPhoneCallWithVideoOnAudioEndpoint(&self, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn RejectPhoneCall(&self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn RejectPhoneCallWithText(&self, phonecallid: u32, textresponseid: u32) -> ::windows::core::Result<()>;
    fn MakePhoneCall(&self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MakePhoneCallOnAudioEndpoint(&self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn MakePhoneCallWithVideo(&self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MakePhoneCallWithVideoOnAudioEndpoint(&self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn SwapPhoneCalls(&self, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::core::Result<()>;
    fn HoldPhoneCall(&self, phonecallid: u32, holdcall: bool) -> ::windows::core::Result<()>;
    fn EndPhoneCall(&self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn SetPhoneMute(&self, value: bool) -> ::windows::core::Result<()>;
    fn PhoneMute(&self) -> ::windows::core::Result<bool>;
    fn SetPhoneCallAudioEndpoint(&self, value: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn PhoneCallAudioEndpoint(&self) -> ::windows::core::Result<PhoneCallAudioEndpoint>;
    fn SnoozeAlarm(&self, alarmid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SnoozeAlarmForSpecifiedTime(&self, alarmid: &::windows::core::GUID, timespan: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DismissAlarm(&self, alarmid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SnoozeReminder(&self, reminderid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SnoozeReminderForSpecifiedTime(&self, reminderid: &::windows::core::GUID, timespan: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DismissReminder(&self, reminderid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMediaMetadata(&self) -> ::windows::core::Result<MediaMetadata>;
    fn MediaPlaybackCapabilities(&self) -> ::windows::core::Result<PlaybackCapability>;
    fn MediaPlaybackStatus(&self) -> ::windows::core::Result<PlaybackStatus>;
    fn PerformMediaPlaybackCommand(&self, command: PlaybackCommand) -> ::windows::core::Result<()>;
    fn DoNotDisturbEnabled(&self) -> ::windows::core::Result<bool>;
    fn DrivingModeEnabled(&self) -> ::windows::core::Result<bool>;
    fn BatterySaverState(&self) -> ::windows::core::Result<bool>;
    fn GetApps(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppNotificationInfo>>;
    fn EnableNotificationsForApplication(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisableNotificationsForApplication(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsNotificationEnabledForApplication(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetEnabledAccessoryNotificationTypes(&self) -> ::windows::core::Result<i32>;
    fn EnableAccessoryNotificationTypes(&self, accessorynotificationtypes: i32) -> ::windows::core::Result<()>;
    fn DisableAllAccessoryNotificationTypes(&self) -> ::windows::core::Result<()>;
    fn GetUserConsent(&self) -> ::windows::core::Result<bool>;
    fn GetAppIcon(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessoryManager2Impl: Sized {
    fn RingDevice(&self) -> ::windows::core::Result<()>;
    fn SpeedDialList(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>>;
    fn ClearToast(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsPhonePinLocked(&self) -> ::windows::core::Result<bool>;
    fn IncreaseVolume(&self, step: i32) -> ::windows::core::Result<()>;
    fn DecreaseVolume(&self, step: i32) -> ::windows::core::Result<()>;
    fn SetMute(&self, mute: bool) -> ::windows::core::Result<()>;
    fn SetRingerVibrate(&self, ringer: bool, vibrate: bool) -> ::windows::core::Result<()>;
    fn VolumeInfo(&self) -> ::windows::core::Result<VolumeInfo>;
    fn GetAllEmailAccounts(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>>;
    fn GetFolders(&self, emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>>;
    fn EnableEmailNotificationEmailAccount(&self, emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisableEmailNotificationEmailAccount(&self, emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EnableEmailNotificationFolderFilter(&self, emailaccount: &::windows::core::HSTRING, folders: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn UpdateEmailReadStatus(&self, messageentryid: &::core::option::Option<BinaryId>, isread: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessoryManager3Impl: Sized {
    fn SnoozeAlarmByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissAlarmByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SnoozeReminderByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissReminderByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IAccessoryNotificationTriggerDetailsImpl: Sized {
    fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType>;
    fn StartedProcessing(&self) -> ::windows::core::Result<bool>;
    fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAlarmNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AlarmId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ReminderState(&self) -> ::windows::core::Result<ReminderState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAlarmNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppNotificationInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBinaryIdImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u8>;
    fn Length(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarChangedNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn EventType(&self) -> ::windows::core::Result<CalendarChangedEvent>;
    fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICortanaTileNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LargeContent1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LargeContent2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmphasizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent4(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailAccountInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailFolderInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SenderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SenderAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessage(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Email::EmailMessage>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailNotificationTriggerDetails2Impl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailReadNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId>;
    fn IsRead(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaControlsTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn PlaybackStatus(&self) -> ::windows::core::Result<PlaybackStatus>;
    fn MediaMetadata(&self) -> ::windows::core::Result<MediaMetadata>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaMetadataImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Track(&self) -> ::windows::core::Result<u32>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallDetailsImpl: Sized {
    fn PhoneLine(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CallId(&self) -> ::windows::core::Result<u32>;
    fn CallTransport(&self) -> ::windows::core::Result<PhoneCallTransport>;
    fn CallMediaType(&self) -> ::windows::core::Result<PhoneMediaType>;
    fn CallDirection(&self) -> ::windows::core::Result<PhoneCallDirection>;
    fn State(&self) -> ::windows::core::Result<PhoneCallState>;
    fn ConferenceCallId(&self) -> ::windows::core::Result<u32>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresetTextResponses(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineDetailsImpl: Sized {
    fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LineNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultOutgoingLine(&self) -> ::windows::core::Result<bool>;
    fn VoicemailCount(&self) -> ::windows::core::Result<u32>;
    fn RegistrationState(&self) -> ::windows::core::Result<PhoneLineRegistrationState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineDetails2Impl: Sized {
    fn MissedCallCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn PhoneNotificationType(&self) -> ::windows::core::Result<PhoneNotificationType>;
    fn CallDetails(&self) -> ::windows::core::Result<PhoneCallDetails>;
    fn PhoneLineChangedId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReminderNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn ReminderId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Appointment(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Appointments::Appointment>;
    fn ReminderState(&self) -> ::windows::core::Result<ReminderState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReminderNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeedDialEntryImpl: Sized {
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumberType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextResponseImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn Text1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text4(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SuppressPopup(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVolumeInfoImpl: Sized {
    fn SystemVolume(&self) -> ::windows::core::Result<u32>;
    fn CallVolume(&self) -> ::windows::core::Result<u32>;
    fn MediaVolume(&self) -> ::windows::core::Result<u32>;
    fn IsMuted(&self) -> ::windows::core::Result<bool>;
    fn IsVibrateEnabled(&self) -> ::windows::core::Result<VibrateState>;
}
