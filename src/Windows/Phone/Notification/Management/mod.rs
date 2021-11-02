#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_Notification_Management`*"]
pub struct AccessoryManager {}
impl AccessoryManager {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RegisterAccessoryApp() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetNextTriggerDetails() -> ::windows::runtime::Result<IAccessoryNotificationTriggerDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IAccessoryNotificationTriggerDetails>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ProcessTriggerDetails<'a, Param0: ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails>>(pdetails: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), pdetails.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn PhoneLineDetails() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetPhoneLineDetails<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(phoneline: Param0) -> ::windows::runtime::Result<PhoneLineDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), phoneline.into_param().abi(), &mut result__).from_abi::<PhoneLineDetails>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCall(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCallOnEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), phonecallid, endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCallWithVideo(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), phonecallid, endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RejectPhoneCall(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RejectPhoneCallWithText(phonecallid: u32, textresponseid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), phonecallid, textresponseid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCall<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCallOnAudioEndpoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCallWithVideo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCallWithVideoOnAudioEndpoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SwapPhoneCalls(phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), phonecallidtohold, phonecallidonhold).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn HoldPhoneCall(phonecallid: u32, holdcall: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), phonecallid, holdcall).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EndPhoneCall(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetPhoneMute(value: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneMute() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetPhoneCallAudioEndpoint(value: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneCallAudioEndpoint() -> ::windows::runtime::Result<PhoneCallAudioEndpoint> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: PhoneCallAudioEndpoint = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallAudioEndpoint>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeAlarm<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(alarmid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), alarmid.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn SnoozeAlarmForSpecifiedTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(alarmid: Param0, timespan: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), alarmid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissAlarm<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(alarmid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), alarmid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeReminder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(reminderid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), reminderid.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn SnoozeReminderForSpecifiedTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(reminderid: Param0, timespan: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), reminderid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissReminder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(reminderid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), reminderid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetMediaMetadata() -> ::windows::runtime::Result<MediaMetadata> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaMetadata>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaPlaybackCapabilities() -> ::windows::runtime::Result<PlaybackCapability> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: PlaybackCapability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackCapability>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaPlaybackStatus() -> ::windows::runtime::Result<PlaybackStatus> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: PlaybackStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackStatus>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PerformMediaPlaybackCommand(command: PlaybackCommand) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), command).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DoNotDisturbEnabled() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DrivingModeEnabled() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn BatterySaverState() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn GetApps() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, AppNotificationInfo>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, AppNotificationInfo>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EnableNotificationsForApplication<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), appid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisableNotificationsForApplication<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), appid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsNotificationEnabledForApplication<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), appid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetEnabledAccessoryNotificationTypes() -> ::windows::runtime::Result<i32> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EnableAccessoryNotificationTypes(accessorynotificationtypes: i32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), accessorynotificationtypes).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisableAllAccessoryNotificationTypes() -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetUserConsent() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Storage_Streams`*"]
    pub fn GetAppIcon<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), appid.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RingDevice() -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn SpeedDialList() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ClearToast<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsPhonePinLocked() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IncreaseVolume(step: i32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), step).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DecreaseVolume(step: i32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), step).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetMute(mute: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), mute).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetRingerVibrate(ringer: bool, vibrate: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), ringer, vibrate).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn VolumeInfo() -> ::windows::runtime::Result<VolumeInfo> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VolumeInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn GetAllEmailAccounts() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn GetFolders<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(emailaccount: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), emailaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EnableEmailNotificationEmailAccount<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(emailaccount: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), emailaccount.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisableEmailNotificationEmailAccount<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(emailaccount: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), emailaccount.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn EnableEmailNotificationFolderFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(emailaccount: Param0, folders: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), emailaccount.into_param().abi(), folders.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn UpdateEmailReadStatus<'a, Param0: ::windows::runtime::IntoParam<'a, BinaryId>>(messageentryid: Param0, isread: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), messageentryid.into_param().abi(), isread).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeAlarmByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissAlarmByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeReminderByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissReminderByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    pub fn IAccessoryManager<R, F: FnOnce(&IAccessoryManager) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessoryManager, IAccessoryManager> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccessoryManager2<R, F: FnOnce(&IAccessoryManager2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessoryManager, IAccessoryManager2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccessoryManager3<R, F: FnOnce(&IAccessoryManager3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessoryManager, IAccessoryManager3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AccessoryManager {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AccessoryManager";
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AccessoryNotificationType(pub u32);
impl AccessoryNotificationType {
    pub const None: AccessoryNotificationType = AccessoryNotificationType(0u32);
    pub const Phone: AccessoryNotificationType = AccessoryNotificationType(1u32);
    pub const Email: AccessoryNotificationType = AccessoryNotificationType(2u32);
    pub const Reminder: AccessoryNotificationType = AccessoryNotificationType(4u32);
    pub const Alarm: AccessoryNotificationType = AccessoryNotificationType(8u32);
    pub const Toast: AccessoryNotificationType = AccessoryNotificationType(16u32);
    pub const AppUninstalled: AccessoryNotificationType = AccessoryNotificationType(32u32);
    pub const Dnd: AccessoryNotificationType = AccessoryNotificationType(64u32);
    pub const DrivingMode: AccessoryNotificationType = AccessoryNotificationType(128u32);
    pub const BatterySaver: AccessoryNotificationType = AccessoryNotificationType(256u32);
    pub const Media: AccessoryNotificationType = AccessoryNotificationType(512u32);
    pub const CortanaTile: AccessoryNotificationType = AccessoryNotificationType(1024u32);
    pub const ToastCleared: AccessoryNotificationType = AccessoryNotificationType(2048u32);
    pub const CalendarChanged: AccessoryNotificationType = AccessoryNotificationType(4096u32);
    pub const VolumeChanged: AccessoryNotificationType = AccessoryNotificationType(8192u32);
    pub const EmailReadStatusChanged: AccessoryNotificationType = AccessoryNotificationType(16384u32);
}
impl ::std::convert::From<u32> for AccessoryNotificationType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AccessoryNotificationType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AccessoryNotificationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.AccessoryNotificationType;u4)");
}
impl ::std::ops::BitOr for AccessoryNotificationType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for AccessoryNotificationType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for AccessoryNotificationType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for AccessoryNotificationType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for AccessoryNotificationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AlarmNotificationTriggerDetails(::windows::runtime::IInspectable);
impl AlarmNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AlarmId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ReminderState(&self) -> ::windows::runtime::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__: ReminderState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ReminderState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAlarmNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AlarmNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails;{38f5fa30-c738-4da2-908c-775d83c36abb})");
}
unsafe impl ::windows::runtime::Interface for AlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(955644464, 51000, 19874, [144, 140, 119, 93, 131, 195, 106, 187]);
}
impl ::windows::runtime::RuntimeName for AlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails";
}
impl ::std::convert::From<AlarmNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AlarmNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &AlarmNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AlarmNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AlarmNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &AlarmNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AlarmNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AlarmNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppNotificationInfo(::windows::runtime::IInspectable);
impl AppNotificationInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppNotificationInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AppNotificationInfo;{2157bea5-e286-45d3-9bea-f790fc216e0e})");
}
unsafe impl ::windows::runtime::Interface for AppNotificationInfo {
    type Vtable = IAppNotificationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(559398565, 57990, 17875, [155, 234, 247, 144, 252, 33, 110, 14]);
}
impl ::windows::runtime::RuntimeName for AppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AppNotificationInfo";
}
impl ::std::convert::From<AppNotificationInfo> for ::windows::runtime::IUnknown {
    fn from(value: AppNotificationInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AppNotificationInfo> for ::windows::runtime::IUnknown {
    fn from(value: &AppNotificationInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppNotificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AppNotificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AppNotificationInfo> for ::windows::runtime::IInspectable {
    fn from(value: AppNotificationInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppNotificationInfo> for ::windows::runtime::IInspectable {
    fn from(value: &AppNotificationInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppNotificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppNotificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BinaryId(::windows::runtime::IInspectable);
impl BinaryId {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BinaryId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.BinaryId;{4f0da531-5595-44b4-9181-ce4efa3fc168})");
}
unsafe impl ::windows::runtime::Interface for BinaryId {
    type Vtable = IBinaryId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1326294321, 21909, 17588, [145, 129, 206, 78, 250, 63, 193, 104]);
}
impl ::windows::runtime::RuntimeName for BinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.BinaryId";
}
impl ::std::convert::From<BinaryId> for ::windows::runtime::IUnknown {
    fn from(value: BinaryId) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BinaryId> for ::windows::runtime::IUnknown {
    fn from(value: &BinaryId) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BinaryId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BinaryId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BinaryId> for ::windows::runtime::IInspectable {
    fn from(value: BinaryId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BinaryId> for ::windows::runtime::IInspectable {
    fn from(value: &BinaryId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BinaryId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BinaryId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CalendarChangedEvent(pub i32);
impl CalendarChangedEvent {
    pub const LostEvents: CalendarChangedEvent = CalendarChangedEvent(0i32);
    pub const AppointmentAdded: CalendarChangedEvent = CalendarChangedEvent(1i32);
    pub const AppointmentChanged: CalendarChangedEvent = CalendarChangedEvent(2i32);
    pub const AppointmentDeleted: CalendarChangedEvent = CalendarChangedEvent(3i32);
    pub const CalendarAdded: CalendarChangedEvent = CalendarChangedEvent(4i32);
    pub const CalendarChanged: CalendarChangedEvent = CalendarChangedEvent(5i32);
    pub const CalendarDeleted: CalendarChangedEvent = CalendarChangedEvent(6i32);
}
impl ::std::convert::From<i32> for CalendarChangedEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CalendarChangedEvent {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CalendarChangedEvent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.CalendarChangedEvent;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CalendarChangedNotificationTriggerDetails(::windows::runtime::IInspectable);
impl CalendarChangedNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EventType(&self) -> ::windows::runtime::Result<CalendarChangedEvent> {
        let this = self;
        unsafe {
            let mut result__: CalendarChangedEvent = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CalendarChangedEvent>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ItemId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CalendarChangedNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails;{4b8a3bfc-279d-42ab-9c68-3e87977bf216})");
}
unsafe impl ::windows::runtime::Interface for CalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1267350524, 10141, 17067, [156, 104, 62, 135, 151, 123, 242, 22]);
}
impl ::windows::runtime::RuntimeName for CalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails";
}
impl ::std::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &CalendarChangedNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &CalendarChangedNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CalendarChangedNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CalendarChangedNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CortanaTileNotificationTriggerDetails(::windows::runtime::IInspectable);
impl CortanaTileNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LargeContent1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LargeContent2(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EmphasizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent2(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent3(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent4(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CortanaTileNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails;{dc0f01d5-1489-46bb-b73b-7f90067ecf27})");
}
unsafe impl ::windows::runtime::Interface for CortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3691971029, 5257, 18107, [183, 59, 127, 144, 6, 126, 207, 39]);
}
impl ::windows::runtime::RuntimeName for CortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails";
}
impl ::std::convert::From<CortanaTileNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &CortanaTileNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CortanaTileNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &CortanaTileNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CortanaTileNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CortanaTileNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailAccountInfo(::windows::runtime::IInspectable);
impl EmailAccountInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsNotificationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailAccountInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailAccountInfo;{dfbc02ab-bda0-4568-927e-b2ede35818a1})");
}
unsafe impl ::windows::runtime::Interface for EmailAccountInfo {
    type Vtable = IEmailAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3753640619, 48544, 17768, [146, 126, 178, 237, 227, 88, 24, 161]);
}
impl ::windows::runtime::RuntimeName for EmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailAccountInfo";
}
impl ::std::convert::From<EmailAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: EmailAccountInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: &EmailAccountInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailAccountInfo> for ::windows::runtime::IInspectable {
    fn from(value: EmailAccountInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailAccountInfo> for ::windows::runtime::IInspectable {
    fn from(value: &EmailAccountInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailFolderInfo(::windows::runtime::IInspectable);
impl EmailFolderInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsNotificationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailFolderInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailFolderInfo;{c207150e-e237-46d6-90e6-4f529eeac1e2})");
}
unsafe impl ::windows::runtime::Interface for EmailFolderInfo {
    type Vtable = IEmailFolderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3255244046, 57911, 18134, [144, 230, 79, 82, 158, 234, 193, 226]);
}
impl ::windows::runtime::RuntimeName for EmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailFolderInfo";
}
impl ::std::convert::From<EmailFolderInfo> for ::windows::runtime::IUnknown {
    fn from(value: EmailFolderInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailFolderInfo> for ::windows::runtime::IUnknown {
    fn from(value: &EmailFolderInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailFolderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailFolderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailFolderInfo> for ::windows::runtime::IInspectable {
    fn from(value: EmailFolderInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailFolderInfo> for ::windows::runtime::IInspectable {
    fn from(value: &EmailFolderInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailFolderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailFolderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailNotificationTriggerDetails(::windows::runtime::IInspectable);
impl EmailNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccountName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ParentFolderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SenderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SenderAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Email")]
    #[doc = "*Required features: `Phone_Notification_Management`, `ApplicationModel_Email`*"]
    pub fn EmailMessage(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::Email::EmailMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Email::EmailMessage>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MessageEntryId(&self) -> ::windows::runtime::Result<BinaryId> {
        let this = &::windows::runtime::Interface::cast::<IEmailNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BinaryId>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailNotificationTriggerDetails;{f3b82612-46cf-4e70-8e0d-7b2e04ab492b})");
}
unsafe impl ::windows::runtime::Interface for EmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4088931858, 18127, 20080, [142, 13, 123, 46, 4, 171, 73, 43]);
}
impl ::windows::runtime::RuntimeName for EmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailNotificationTriggerDetails";
}
impl ::std::convert::From<EmailNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &EmailNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &EmailNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: EmailNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &EmailNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailReadNotificationTriggerDetails(::windows::runtime::IInspectable);
impl EmailReadNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccountName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ParentFolderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MessageEntryId(&self) -> ::windows::runtime::Result<BinaryId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BinaryId>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsRead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailReadNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails;{f5b7a087-06f3-4e3e-8c42-325e67010413})");
}
unsafe impl ::windows::runtime::Interface for EmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4122452103, 1779, 20030, [140, 66, 50, 94, 103, 1, 4, 19]);
}
impl ::windows::runtime::RuntimeName for EmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails";
}
impl ::std::convert::From<EmailReadNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailReadNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &EmailReadNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailReadNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailReadNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &EmailReadNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: EmailReadNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &EmailReadNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAccessoryManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryManager {
    type Vtable = IAccessoryManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(218407212, 34877, 19111, [188, 167, 250, 75, 184, 191, 254, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdetails: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32, textresponseid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32, holdcall: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonecallid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alarmid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alarmid: ::windows::runtime::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alarmid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reminderid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reminderid: ::windows::runtime::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reminderid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlaybackCapability) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlaybackStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: PlaybackCommand) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accessorynotificationtypes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAccessoryManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryManager2 {
    type Vtable = IAccessoryManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3133854797, 54163, 18118, [184, 12, 21, 253, 244, 77, 83, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, step: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, step: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mute: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ringer: bool, vibrate: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, folders: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messageentryid: ::windows::runtime::RawPtr, isread: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAccessoryManager3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryManager3 {
    type Vtable = IAccessoryManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2180469047, 60871, 18400, [178, 247, 126, 87, 124, 131, 63, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Phone_Notification_Management`*"]
pub struct IAccessoryNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryNotificationTriggerDetails {
    type Vtable = IAccessoryNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1768466388, 58314, 18891, [140, 135, 44, 17, 205, 255, 150, 70]);
}
impl IAccessoryNotificationTriggerDetails {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = self;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAccessoryNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6968a7d4-e3ca-49cb-8c87-2c11cdff9646}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AccessoryNotificationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(955644464, 51000, 19874, [144, 140, 119, 93, 131, 195, 106, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ReminderState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAlarmNotificationTriggerDetails2 {
    type Vtable = IAlarmNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3474382954, 29013, 16638, [169, 194, 123, 210, 18, 126, 248, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAppNotificationInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppNotificationInfo {
    type Vtable = IAppNotificationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(559398565, 57990, 17875, [155, 234, 247, 144, 252, 33, 110, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBinaryId(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBinaryId {
    type Vtable = IBinaryId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1326294321, 21909, 17588, [145, 129, 206, 78, 250, 63, 193, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinaryId_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICalendarChangedNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1267350524, 10141, 17067, [156, 104, 62, 135, 151, 123, 242, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarChangedNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CalendarChangedEvent) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICortanaTileNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3691971029, 5257, 18107, [183, 59, 127, 144, 6, 126, 207, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaTileNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEmailAccountInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailAccountInfo {
    type Vtable = IEmailAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3753640619, 48544, 17768, [146, 126, 178, 237, 227, 88, 24, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAccountInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEmailFolderInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailFolderInfo {
    type Vtable = IEmailFolderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3255244046, 57911, 18134, [144, 230, 79, 82, 158, 234, 193, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailFolderInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4088931858, 18127, 20080, [142, 13, 123, 46, 4, 171, 73, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Email")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Email"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailNotificationTriggerDetails2 {
    type Vtable = IEmailNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(377513955, 50543, 20167, [190, 209, 247, 52, 224, 141, 229, 178]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEmailReadNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4122452103, 1779, 20030, [140, 66, 50, 94, 103, 1, 4, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailReadNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMediaControlsTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4206126219, 44613, 17736, [145, 202, 74, 176, 84, 142, 51, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControlsTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlaybackStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMediaMetadata(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaMetadata {
    type Vtable = IMediaMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2605768183, 47980, 17200, [179, 205, 7, 4, 165, 76, 219, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMetadata_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPhoneCallDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallDetails {
    type Vtable = IPhoneCallDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(203124563, 61553, 18494, [191, 51, 235, 212, 75, 114, 68, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallTransport) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneMediaType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallDirection) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneCallState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPhoneLineDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineDetails {
    type Vtable = IPhoneLineDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1206596316, 13293, 18873, [153, 92, 162, 150, 186, 200, 43, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneLineRegistrationState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPhoneLineDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineDetails2 {
    type Vtable = IPhoneLineDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3003963261, 327, 18828, [130, 65, 191, 12, 171, 198, 10, 37]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPhoneNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3435331063, 2499, 16664, [145, 188, 202, 99, 35, 168, 211, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneNotificationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1541253725, 40801, 19440, [159, 235, 16, 80, 43, 192, 176, 194]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "ApplicationModel_Appointments")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ReminderState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReminderNotificationTriggerDetails2 {
    type Vtable = IReminderNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3876977088, 20557, 19471, [166, 179, 188, 185, 114, 44, 108, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISpeedDialEntry(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeedDialEntry {
    type Vtable = ISpeedDialEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2453714651, 34604, 18140, [182, 42, 190, 69, 65, 177, 102, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeedDialEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextResponse(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextResponse {
    type Vtable = ITextResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3922425027, 9303, 19675, [129, 16, 114, 245, 232, 232, 131, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextResponse_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3375450261, 20077, 20125, [175, 236, 158, 146, 27, 135, 90, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationTriggerDetails2 {
    type Vtable = IToastNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1040480733, 51908, 20320, [175, 163, 185, 37, 217, 216, 60, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IVolumeInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVolumeInfo {
    type Vtable = IVolumeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2488127768, 30468, 17537, [185, 46, 211, 237, 62, 206, 99, 34]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VibrateState) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaControlsTriggerDetails(::windows::runtime::IInspectable);
impl MediaControlsTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PlaybackStatus(&self) -> ::windows::runtime::Result<PlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__: PlaybackStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackStatus>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaMetadata(&self) -> ::windows::runtime::Result<MediaMetadata> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaMetadata>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaControlsTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaControlsTriggerDetails;{fab4648b-ae45-4548-91ca-4ab0548e33b5})");
}
unsafe impl ::windows::runtime::Interface for MediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4206126219, 44613, 17736, [145, 202, 74, 176, 84, 142, 51, 181]);
}
impl ::windows::runtime::RuntimeName for MediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaControlsTriggerDetails";
}
impl ::std::convert::From<MediaControlsTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaControlsTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MediaControlsTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaControlsTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaControlsTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &MediaControlsTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaControlsTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaControlsTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaMetadata(::windows::runtime::IInspectable);
impl MediaMetadata {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Subtitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Artist(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Album(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Track(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaMetadata {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaMetadata;{9b50ddf7-bb6c-4330-b3cd-0704a54cdb80})");
}
unsafe impl ::windows::runtime::Interface for MediaMetadata {
    type Vtable = IMediaMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2605768183, 47980, 17200, [179, 205, 7, 4, 165, 76, 219, 128]);
}
impl ::windows::runtime::RuntimeName for MediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaMetadata";
}
impl ::std::convert::From<MediaMetadata> for ::windows::runtime::IUnknown {
    fn from(value: MediaMetadata) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaMetadata> for ::windows::runtime::IUnknown {
    fn from(value: &MediaMetadata) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaMetadata> for ::windows::runtime::IInspectable {
    fn from(value: MediaMetadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaMetadata> for ::windows::runtime::IInspectable {
    fn from(value: &MediaMetadata) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallAudioEndpoint(pub i32);
impl PhoneCallAudioEndpoint {
    pub const Default: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(0i32);
    pub const Speaker: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(1i32);
    pub const Handsfree: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(2i32);
}
impl ::std::convert::From<i32> for PhoneCallAudioEndpoint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallAudioEndpoint {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallAudioEndpoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallAudioEndpoint;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneCallDetails(::windows::runtime::IInspectable);
impl PhoneCallDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneLine(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallTransport(&self) -> ::windows::runtime::Result<PhoneCallTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallTransport = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallTransport>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallMediaType(&self) -> ::windows::runtime::Result<PhoneMediaType> {
        let this = self;
        unsafe {
            let mut result__: PhoneMediaType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneMediaType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallDirection(&self) -> ::windows::runtime::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallDirection = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallDirection>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn State(&self) -> ::windows::runtime::Result<PhoneCallState> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallState>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ConferenceCallId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn EndTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ContactName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn PresetTextResponses(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<TextResponse>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneCallDetails;{0c1b6f53-f071-483e-bf33-ebd44b724447})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallDetails {
    type Vtable = IPhoneCallDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(203124563, 61553, 18494, [191, 51, 235, 212, 75, 114, 68, 71]);
}
impl ::windows::runtime::RuntimeName for PhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneCallDetails";
}
impl ::std::convert::From<PhoneCallDetails> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PhoneCallDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PhoneCallDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PhoneCallDetails> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneCallDetails> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Incoming: PhoneCallDirection = PhoneCallDirection(0i32);
    pub const Outgoing: PhoneCallDirection = PhoneCallDirection(1i32);
}
impl ::std::convert::From<i32> for PhoneCallDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallDirection {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallDirection;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallState(pub i32);
impl PhoneCallState {
    pub const Unknown: PhoneCallState = PhoneCallState(0i32);
    pub const Ringing: PhoneCallState = PhoneCallState(1i32);
    pub const Talking: PhoneCallState = PhoneCallState(2i32);
    pub const Held: PhoneCallState = PhoneCallState(3i32);
    pub const Ended: PhoneCallState = PhoneCallState(4i32);
}
impl ::std::convert::From<i32> for PhoneCallState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallState;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallTransport(pub i32);
impl PhoneCallTransport {
    pub const Cellular: PhoneCallTransport = PhoneCallTransport(0i32);
    pub const Voip: PhoneCallTransport = PhoneCallTransport(1i32);
}
impl ::std::convert::From<i32> for PhoneCallTransport {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallTransport {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallTransport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallTransport;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneLineDetails(::windows::runtime::IInspectable);
impl PhoneLineDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LineId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LineNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DefaultOutgoingLine(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn VoicemailCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RegistrationState(&self) -> ::windows::runtime::Result<PhoneLineRegistrationState> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineRegistrationState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineRegistrationState>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MissedCallCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineDetails2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneLineDetails;{47eb32dc-33ed-49b9-995c-a296bac82b77})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineDetails {
    type Vtable = IPhoneLineDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1206596316, 13293, 18873, [153, 92, 162, 150, 186, 200, 43, 119]);
}
impl ::windows::runtime::RuntimeName for PhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneLineDetails";
}
impl ::std::convert::From<PhoneLineDetails> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PhoneLineDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PhoneLineDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PhoneLineDetails> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneLineDetails> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneLineDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneLineDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneLineDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineRegistrationState(pub i32);
impl PhoneLineRegistrationState {
    pub const Disconnected: PhoneLineRegistrationState = PhoneLineRegistrationState(0i32);
    pub const Home: PhoneLineRegistrationState = PhoneLineRegistrationState(1i32);
    pub const Roaming: PhoneLineRegistrationState = PhoneLineRegistrationState(2i32);
}
impl ::std::convert::From<i32> for PhoneLineRegistrationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneLineRegistrationState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineRegistrationState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneLineRegistrationState;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneMediaType(pub i32);
impl PhoneMediaType {
    pub const AudioOnly: PhoneMediaType = PhoneMediaType(0i32);
    pub const AudioVideo: PhoneMediaType = PhoneMediaType(1i32);
}
impl ::std::convert::From<i32> for PhoneMediaType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneMediaType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneMediaType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneMediaType;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PhoneNotificationTriggerDetails(::windows::runtime::IInspectable);
impl PhoneNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneNotificationType(&self) -> ::windows::runtime::Result<PhoneNotificationType> {
        let this = self;
        unsafe {
            let mut result__: PhoneNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallDetails(&self) -> ::windows::runtime::Result<PhoneCallDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallDetails>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneLineChangedId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails;{ccc2fdf7-09c3-4118-91bc-ca6323a8d383})");
}
unsafe impl ::windows::runtime::Interface for PhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3435331063, 2499, 16664, [145, 188, 202, 99, 35, 168, 211, 131]);
}
impl ::windows::runtime::RuntimeName for PhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails";
}
impl ::std::convert::From<PhoneNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PhoneNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PhoneNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhoneNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PhoneNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PhoneNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneNotificationType(pub i32);
impl PhoneNotificationType {
    pub const NewCall: PhoneNotificationType = PhoneNotificationType(0i32);
    pub const CallChanged: PhoneNotificationType = PhoneNotificationType(1i32);
    pub const LineChanged: PhoneNotificationType = PhoneNotificationType(2i32);
    pub const PhoneCallAudioEndpointChanged: PhoneNotificationType = PhoneNotificationType(3i32);
    pub const PhoneMuteChanged: PhoneNotificationType = PhoneNotificationType(4i32);
}
impl ::std::convert::From<i32> for PhoneNotificationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneNotificationType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNotificationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneNotificationType;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlaybackCapability(pub u32);
impl PlaybackCapability {
    pub const None: PlaybackCapability = PlaybackCapability(0u32);
    pub const Play: PlaybackCapability = PlaybackCapability(1u32);
    pub const Pause: PlaybackCapability = PlaybackCapability(2u32);
    pub const Stop: PlaybackCapability = PlaybackCapability(4u32);
    pub const Record: PlaybackCapability = PlaybackCapability(8u32);
    pub const FastForward: PlaybackCapability = PlaybackCapability(16u32);
    pub const Rewind: PlaybackCapability = PlaybackCapability(32u32);
    pub const Next: PlaybackCapability = PlaybackCapability(64u32);
    pub const Previous: PlaybackCapability = PlaybackCapability(128u32);
    pub const ChannelUp: PlaybackCapability = PlaybackCapability(256u32);
    pub const ChannelDown: PlaybackCapability = PlaybackCapability(512u32);
}
impl ::std::convert::From<u32> for PlaybackCapability {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlaybackCapability {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackCapability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCapability;u4)");
}
impl ::std::ops::BitOr for PlaybackCapability {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PlaybackCapability {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PlaybackCapability {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PlaybackCapability {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PlaybackCapability {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlaybackCommand(pub i32);
impl PlaybackCommand {
    pub const Play: PlaybackCommand = PlaybackCommand(0i32);
    pub const Pause: PlaybackCommand = PlaybackCommand(1i32);
    pub const Stop: PlaybackCommand = PlaybackCommand(2i32);
    pub const Record: PlaybackCommand = PlaybackCommand(3i32);
    pub const FastForward: PlaybackCommand = PlaybackCommand(4i32);
    pub const Rewind: PlaybackCommand = PlaybackCommand(5i32);
    pub const Next: PlaybackCommand = PlaybackCommand(6i32);
    pub const Previous: PlaybackCommand = PlaybackCommand(7i32);
    pub const ChannelUp: PlaybackCommand = PlaybackCommand(8i32);
    pub const ChannelDown: PlaybackCommand = PlaybackCommand(9i32);
}
impl ::std::convert::From<i32> for PlaybackCommand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlaybackCommand {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCommand;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlaybackStatus(pub i32);
impl PlaybackStatus {
    pub const None: PlaybackStatus = PlaybackStatus(0i32);
    pub const TrackChanged: PlaybackStatus = PlaybackStatus(1i32);
    pub const Stopped: PlaybackStatus = PlaybackStatus(2i32);
    pub const Playing: PlaybackStatus = PlaybackStatus(3i32);
    pub const Paused: PlaybackStatus = PlaybackStatus(4i32);
}
impl ::std::convert::From<i32> for PlaybackStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlaybackStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackStatus;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ReminderNotificationTriggerDetails(::windows::runtime::IInspectable);
impl ReminderNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ReminderId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Details(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments")]
    #[doc = "*Required features: `Phone_Notification_Management`, `ApplicationModel_Appointments`*"]
    pub fn Appointment(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Appointments::Appointment>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ReminderState(&self) -> ::windows::runtime::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__: ReminderState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ReminderState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IReminderNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ReminderNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails;{5bddaa5d-9f61-4bf0-9feb-10502bc0b0c2})");
}
unsafe impl ::windows::runtime::Interface for ReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1541253725, 40801, 19440, [159, 235, 16, 80, 43, 192, 176, 194]);
}
impl ::windows::runtime::RuntimeName for ReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails";
}
impl ::std::convert::From<ReminderNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ReminderNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ReminderNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ReminderNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ReminderNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &ReminderNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ReminderNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ReminderNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ReminderState(pub i32);
impl ReminderState {
    pub const Active: ReminderState = ReminderState(0i32);
    pub const Snoozed: ReminderState = ReminderState(1i32);
    pub const Dismissed: ReminderState = ReminderState(2i32);
}
impl ::std::convert::From<i32> for ReminderState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ReminderState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ReminderState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.ReminderState;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeedDialEntry(::windows::runtime::IInspectable);
impl SpeedDialEntry {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NumberType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ContactName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeedDialEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.SpeedDialEntry;{9240b6db-872c-46dc-b62a-be4541b166f8})");
}
unsafe impl ::windows::runtime::Interface for SpeedDialEntry {
    type Vtable = ISpeedDialEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2453714651, 34604, 18140, [182, 42, 190, 69, 65, 177, 102, 248]);
}
impl ::windows::runtime::RuntimeName for SpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.SpeedDialEntry";
}
impl ::std::convert::From<SpeedDialEntry> for ::windows::runtime::IUnknown {
    fn from(value: SpeedDialEntry) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpeedDialEntry> for ::windows::runtime::IUnknown {
    fn from(value: &SpeedDialEntry) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeedDialEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpeedDialEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SpeedDialEntry> for ::windows::runtime::IInspectable {
    fn from(value: SpeedDialEntry) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeedDialEntry> for ::windows::runtime::IInspectable {
    fn from(value: &SpeedDialEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeedDialEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeedDialEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TextResponse(::windows::runtime::IInspectable);
impl TextResponse {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.TextResponse;{e9cb74c3-2457-4cdb-8110-72f5e8e883e8})");
}
unsafe impl ::windows::runtime::Interface for TextResponse {
    type Vtable = ITextResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3922425027, 9303, 19675, [129, 16, 114, 245, 232, 232, 131, 232]);
}
impl ::windows::runtime::RuntimeName for TextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.TextResponse";
}
impl ::std::convert::From<TextResponse> for ::windows::runtime::IUnknown {
    fn from(value: TextResponse) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextResponse> for ::windows::runtime::IUnknown {
    fn from(value: &TextResponse) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TextResponse> for ::windows::runtime::IInspectable {
    fn from(value: TextResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextResponse> for ::windows::runtime::IInspectable {
    fn from(value: &TextResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ToastNotificationTriggerDetails(::windows::runtime::IInspectable);
impl ToastNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text2(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text3(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text4(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SuppressPopup(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ToastNotificationTriggerDetails;{c9314895-4e6d-4e9d-afec-9e921b875ae8})");
}
unsafe impl ::windows::runtime::Interface for ToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3375450261, 20077, 20125, [175, 236, 158, 146, 27, 135, 90, 232]);
}
impl ::windows::runtime::RuntimeName for ToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ToastNotificationTriggerDetails";
}
impl ::std::convert::From<ToastNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ToastNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ToastNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ToastNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &ToastNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ToastNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ToastNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::std::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VibrateState(pub i32);
impl VibrateState {
    pub const RingerOffVibrateOff: VibrateState = VibrateState(0i32);
    pub const RingerOffVibrateOn: VibrateState = VibrateState(1i32);
    pub const RingerOnVibrateOff: VibrateState = VibrateState(2i32);
    pub const RingerOnVibrateOn: VibrateState = VibrateState(3i32);
}
impl ::std::convert::From<i32> for VibrateState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VibrateState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VibrateState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.VibrateState;i4)");
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VolumeInfo(::windows::runtime::IInspectable);
impl VolumeInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SystemVolume(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallVolume(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaVolume(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsMuted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsVibrateEnabled(&self) -> ::windows::runtime::Result<VibrateState> {
        let this = self;
        unsafe {
            let mut result__: VibrateState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VibrateState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VolumeInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.VolumeInfo;{944dd118-7704-4481-b92e-d3ed3ece6322})");
}
unsafe impl ::windows::runtime::Interface for VolumeInfo {
    type Vtable = IVolumeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2488127768, 30468, 17537, [185, 46, 211, 237, 62, 206, 99, 34]);
}
impl ::windows::runtime::RuntimeName for VolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.VolumeInfo";
}
impl ::std::convert::From<VolumeInfo> for ::windows::runtime::IUnknown {
    fn from(value: VolumeInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VolumeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &VolumeInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VolumeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VolumeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VolumeInfo> for ::windows::runtime::IInspectable {
    fn from(value: VolumeInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VolumeInfo> for ::windows::runtime::IInspectable {
    fn from(value: &VolumeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VolumeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VolumeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
