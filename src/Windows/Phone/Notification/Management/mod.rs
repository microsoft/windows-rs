#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_Notification_Management`*"]
pub struct AccessoryManager {}
impl AccessoryManager {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RegisterAccessoryApp() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetNextTriggerDetails() -> ::windows::runtime::Result<IAccessoryNotificationTriggerDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IAccessoryNotificationTriggerDetails>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ProcessTriggerDetails<'a, Param0: ::windows::runtime::IntoParam<'a, IAccessoryNotificationTriggerDetails>>(pdetails: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pdetails.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn PhoneLineDetails() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetPhoneLineDetails<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(phoneline: Param0) -> ::windows::runtime::Result<PhoneLineDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), phoneline.into_param().abi(), &mut result__).from_abi::<PhoneLineDetails>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCall(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCallOnEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), phonecallid, endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCallWithVideo(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), phonecallid, endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RejectPhoneCall(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RejectPhoneCallWithText(phonecallid: u32, textresponseid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), phonecallid, textresponseid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCall<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCallOnAudioEndpoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCallWithVideo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MakePhoneCallWithVideoOnAudioEndpoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SwapPhoneCalls(phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), phonecallidtohold, phonecallidonhold).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn HoldPhoneCall(phonecallid: u32, holdcall: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), phonecallid, holdcall).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EndPhoneCall(phonecallid: u32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetPhoneMute(value: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneMute() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetPhoneCallAudioEndpoint(value: PhoneCallAudioEndpoint) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneCallAudioEndpoint() -> ::windows::runtime::Result<PhoneCallAudioEndpoint> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: PhoneCallAudioEndpoint = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallAudioEndpoint>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeAlarm<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(alarmid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), alarmid.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn SnoozeAlarmForSpecifiedTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(alarmid: Param0, timespan: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), alarmid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissAlarm<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(alarmid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), alarmid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeReminder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(reminderid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), reminderid.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn SnoozeReminderForSpecifiedTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(reminderid: Param0, timespan: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), reminderid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissReminder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(reminderid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), reminderid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetMediaMetadata() -> ::windows::runtime::Result<MediaMetadata> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaMetadata>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaPlaybackCapabilities() -> ::windows::runtime::Result<PlaybackCapability> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: PlaybackCapability = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackCapability>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaPlaybackStatus() -> ::windows::runtime::Result<PlaybackStatus> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: PlaybackStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackStatus>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PerformMediaPlaybackCommand(command: PlaybackCommand) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), command).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DoNotDisturbEnabled() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DrivingModeEnabled() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn BatterySaverState() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn GetApps() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, AppNotificationInfo>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, AppNotificationInfo>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EnableNotificationsForApplication<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).42)(::core::mem::transmute_copy(this), appid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisableNotificationsForApplication<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).43)(::core::mem::transmute_copy(this), appid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsNotificationEnabledForApplication<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::core::mem::transmute_copy(this), appid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetEnabledAccessoryNotificationTypes() -> ::windows::runtime::Result<i32> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EnableAccessoryNotificationTypes(accessorynotificationtypes: i32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).46)(::core::mem::transmute_copy(this), accessorynotificationtypes).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisableAllAccessoryNotificationTypes() -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::runtime::Interface::vtable(this).47)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn GetUserConsent() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Storage_Streams`*"]
    pub fn GetAppIcon<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(appid: Param0) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(::core::mem::transmute_copy(this), appid.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RingDevice() -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn SpeedDialList() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ClearToast<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsPhonePinLocked() -> ::windows::runtime::Result<bool> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IncreaseVolume(step: i32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), step).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DecreaseVolume(step: i32) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), step).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetMute(mute: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), mute).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetRingerVibrate(ringer: bool, vibrate: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), ringer, vibrate).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn VolumeInfo() -> ::windows::runtime::Result<VolumeInfo> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VolumeInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn GetAllEmailAccounts() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn GetFolders<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(emailaccount: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), emailaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>>(result__)
        })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EnableEmailNotificationEmailAccount<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(emailaccount: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), emailaccount.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisableEmailNotificationEmailAccount<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(emailaccount: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), emailaccount.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn EnableEmailNotificationFolderFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(emailaccount: Param0, folders: Param1) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), emailaccount.into_param().abi(), folders.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn UpdateEmailReadStatus<'a, Param0: ::windows::runtime::IntoParam<'a, BinaryId>>(messageentryid: Param0, isread: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), messageentryid.into_param().abi(), isread).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeAlarmByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissAlarmByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SnoozeReminderByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DismissReminderByInstanceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(instanceid: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), instanceid.into_param().abi()).ok() })
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<u32> for AccessoryNotificationType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AccessoryNotificationType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AccessoryNotificationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.AccessoryNotificationType;u4)");
}
impl ::windows::runtime::DefaultType for AccessoryNotificationType {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for AccessoryNotificationType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AccessoryNotificationType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AccessoryNotificationType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AccessoryNotificationType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AccessoryNotificationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AlarmNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl AlarmNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AlarmId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ReminderState(&self) -> ::windows::runtime::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__: ReminderState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ReminderState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAlarmNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AlarmNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails;{38f5fa30-c738-4da2-908c-775d83c36abb})");
}
unsafe impl ::windows::runtime::Interface for AlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38f5fa30_c738_4da2_908c_775d83c36abb);
}
impl ::windows::runtime::RuntimeName for AlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails";
}
impl ::core::convert::From<AlarmNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AlarmNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &AlarmNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AlarmNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AlarmNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AlarmNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppNotificationInfo(pub ::windows::runtime::IInspectable);
impl AppNotificationInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppNotificationInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AppNotificationInfo;{2157bea5-e286-45d3-9bea-f790fc216e0e})");
}
unsafe impl ::windows::runtime::Interface for AppNotificationInfo {
    type Vtable = IAppNotificationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2157bea5_e286_45d3_9bea_f790fc216e0e);
}
impl ::windows::runtime::RuntimeName for AppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AppNotificationInfo";
}
impl ::core::convert::From<AppNotificationInfo> for ::windows::runtime::IUnknown {
    fn from(value: AppNotificationInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppNotificationInfo> for ::windows::runtime::IUnknown {
    fn from(value: &AppNotificationInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppNotificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppNotificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppNotificationInfo> for ::windows::runtime::IInspectable {
    fn from(value: AppNotificationInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppNotificationInfo> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BinaryId(pub ::windows::runtime::IInspectable);
impl BinaryId {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BinaryId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.BinaryId;{4f0da531-5595-44b4-9181-ce4efa3fc168})");
}
unsafe impl ::windows::runtime::Interface for BinaryId {
    type Vtable = IBinaryId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4f0da531_5595_44b4_9181_ce4efa3fc168);
}
impl ::windows::runtime::RuntimeName for BinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.BinaryId";
}
impl ::core::convert::From<BinaryId> for ::windows::runtime::IUnknown {
    fn from(value: BinaryId) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BinaryId> for ::windows::runtime::IUnknown {
    fn from(value: &BinaryId) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BinaryId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BinaryId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BinaryId> for ::windows::runtime::IInspectable {
    fn from(value: BinaryId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BinaryId> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for CalendarChangedEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CalendarChangedEvent {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CalendarChangedEvent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.CalendarChangedEvent;i4)");
}
impl ::windows::runtime::DefaultType for CalendarChangedEvent {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CalendarChangedNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl CalendarChangedNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EventType(&self) -> ::windows::runtime::Result<CalendarChangedEvent> {
        let this = self;
        unsafe {
            let mut result__: CalendarChangedEvent = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CalendarChangedEvent>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ItemId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CalendarChangedNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails;{4b8a3bfc-279d-42ab-9c68-3e87977bf216})");
}
unsafe impl ::windows::runtime::Interface for CalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4b8a3bfc_279d_42ab_9c68_3e87977bf216);
}
impl ::windows::runtime::RuntimeName for CalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails";
}
impl ::core::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &CalendarChangedNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CalendarChangedNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CortanaTileNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl CortanaTileNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LargeContent1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LargeContent2(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn EmphasizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent2(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent3(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NonWrappedSmallContent4(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CortanaTileNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails;{dc0f01d5-1489-46bb-b73b-7f90067ecf27})");
}
unsafe impl ::windows::runtime::Interface for CortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdc0f01d5_1489_46bb_b73b_7f90067ecf27);
}
impl ::windows::runtime::RuntimeName for CortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails";
}
impl ::core::convert::From<CortanaTileNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &CortanaTileNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CortanaTileNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CortanaTileNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailAccountInfo(pub ::windows::runtime::IInspectable);
impl EmailAccountInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsNotificationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailAccountInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailAccountInfo;{dfbc02ab-bda0-4568-927e-b2ede35818a1})");
}
unsafe impl ::windows::runtime::Interface for EmailAccountInfo {
    type Vtable = IEmailAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdfbc02ab_bda0_4568_927e_b2ede35818a1);
}
impl ::windows::runtime::RuntimeName for EmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailAccountInfo";
}
impl ::core::convert::From<EmailAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: EmailAccountInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: &EmailAccountInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EmailAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailAccountInfo> for ::windows::runtime::IInspectable {
    fn from(value: EmailAccountInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailAccountInfo> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailFolderInfo(pub ::windows::runtime::IInspectable);
impl EmailFolderInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsNotificationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailFolderInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailFolderInfo;{c207150e-e237-46d6-90e6-4f529eeac1e2})");
}
unsafe impl ::windows::runtime::Interface for EmailFolderInfo {
    type Vtable = IEmailFolderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc207150e_e237_46d6_90e6_4f529eeac1e2);
}
impl ::windows::runtime::RuntimeName for EmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailFolderInfo";
}
impl ::core::convert::From<EmailFolderInfo> for ::windows::runtime::IUnknown {
    fn from(value: EmailFolderInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailFolderInfo> for ::windows::runtime::IUnknown {
    fn from(value: &EmailFolderInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailFolderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EmailFolderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailFolderInfo> for ::windows::runtime::IInspectable {
    fn from(value: EmailFolderInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailFolderInfo> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl EmailNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccountName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ParentFolderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SenderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SenderAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Email")]
    #[doc = "*Required features: `Phone_Notification_Management`, `ApplicationModel_Email`*"]
    pub fn EmailMessage(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::Email::EmailMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Email::EmailMessage>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MessageEntryId(&self) -> ::windows::runtime::Result<BinaryId> {
        let this = &::windows::runtime::Interface::cast::<IEmailNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BinaryId>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailNotificationTriggerDetails;{f3b82612-46cf-4e70-8e0d-7b2e04ab492b})");
}
unsafe impl ::windows::runtime::Interface for EmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf3b82612_46cf_4e70_8e0d_7b2e04ab492b);
}
impl ::windows::runtime::RuntimeName for EmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailNotificationTriggerDetails";
}
impl ::core::convert::From<EmailNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &EmailNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: EmailNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailReadNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl EmailReadNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccountName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ParentFolderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MessageEntryId(&self) -> ::windows::runtime::Result<BinaryId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BinaryId>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsRead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailReadNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails;{f5b7a087-06f3-4e3e-8c42-325e67010413})");
}
unsafe impl ::windows::runtime::Interface for EmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf5b7a087_06f3_4e3e_8c42_325e67010413);
}
impl ::windows::runtime::RuntimeName for EmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails";
}
impl ::core::convert::From<EmailReadNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailReadNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &EmailReadNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailReadNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailReadNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: EmailReadNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessoryManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryManager {
    type Vtable = IAccessoryManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0d04a12c_883d_4aa7_bca7_fa4bb8bffee6);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phoneline: ::windows::runtime::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::runtime::HRESULT,
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accessorynotificationtypes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessoryManager2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryManager2 {
    type Vtable = IAccessoryManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbacad44d_d393_46c6_b80c_15fdf44d5386);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, step: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, step: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mute: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ringer: bool, vibrate: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaccount: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, folders: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messageentryid: ::windows::runtime::RawPtr, isread: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessoryManager3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryManager3 {
    type Vtable = IAccessoryManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x81f75137_edc7_47e0_b2f7_7e577c833f7d);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instanceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Phone_Notification_Management`*"]
pub struct IAccessoryNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessoryNotificationTriggerDetails {
    type Vtable = IAccessoryNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6968a7d4_e3ca_49cb_8c87_2c11cdff9646);
}
impl IAccessoryNotificationTriggerDetails {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = self;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAccessoryNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6968a7d4-e3ca-49cb-8c87-2c11cdff9646}");
}
impl ::core::convert::From<IAccessoryNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: IAccessoryNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAccessoryNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &IAccessoryNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAccessoryNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: IAccessoryNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAccessoryNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &IAccessoryNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AccessoryNotificationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38f5fa30_c738_4da2_908c_775d83c36abb);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ReminderState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAlarmNotificationTriggerDetails2 {
    type Vtable = IAlarmNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcf16e06a_7155_40fe_a9c2_7bd2127ef853);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppNotificationInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppNotificationInfo {
    type Vtable = IAppNotificationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2157bea5_e286_45d3_9bea_f790fc216e0e);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBinaryId(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBinaryId {
    type Vtable = IBinaryId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4f0da531_5595_44b4_9181_ce4efa3fc168);
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
#[doc(hidden)]
pub struct ICalendarChangedNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4b8a3bfc_279d_42ab_9c68_3e87977bf216);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICortanaTileNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdc0f01d5_1489_46bb_b73b_7f90067ecf27);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailAccountInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailAccountInfo {
    type Vtable = IEmailAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdfbc02ab_bda0_4568_927e_b2ede35818a1);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailFolderInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailFolderInfo {
    type Vtable = IEmailFolderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc207150e_e237_46d6_90e6_4f529eeac1e2);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf3b82612_46cf_4e70_8e0d_7b2e04ab492b);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Email")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Email"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailNotificationTriggerDetails2 {
    type Vtable = IEmailNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x168067e3_c56f_4ec7_bed1_f734e08de5b2);
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
#[doc(hidden)]
pub struct IEmailReadNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf5b7a087_06f3_4e3e_8c42_325e67010413);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaControlsTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfab4648b_ae45_4548_91ca_4ab0548e33b5);
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
#[doc(hidden)]
pub struct IMediaMetadata(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaMetadata {
    type Vtable = IMediaMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b50ddf7_bb6c_4330_b3cd_0704a54cdb80);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneCallDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallDetails {
    type Vtable = IPhoneCallDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c1b6f53_f071_483e_bf33_ebd44b724447);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineDetails {
    type Vtable = IPhoneLineDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47eb32dc_33ed_49b9_995c_a296bac82b77);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhoneLineRegistrationState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneLineDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneLineDetails2 {
    type Vtable = IPhoneLineDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb30cd77d_0147_498c_8241_bf0cabc60a25);
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
#[doc(hidden)]
pub struct IPhoneNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xccc2fdf7_09c3_4118_91bc_ca6323a8d383);
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
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5bddaa5d_9f61_4bf0_9feb_10502bc0b0c2);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "ApplicationModel_Appointments")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ReminderState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReminderNotificationTriggerDetails2 {
    type Vtable = IReminderNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe715f9c0_504d_4c0f_a6b3_bcb9722c6cdd);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeedDialEntry(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeedDialEntry {
    type Vtable = ISpeedDialEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9240b6db_872c_46dc_b62a_be4541b166f8);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextResponse(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextResponse {
    type Vtable = ITextResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe9cb74c3_2457_4cdb_8110_72f5e8e883e8);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9314895_4e6d_4e9d_afec_9e921b875ae8);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationTriggerDetails2 {
    type Vtable = IToastNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3e0479dd_cac4_4f60_afa3_b925d9d83c93);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVolumeInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVolumeInfo {
    type Vtable = IVolumeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x944dd118_7704_4481_b92e_d3ed3ece6322);
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaControlsTriggerDetails(pub ::windows::runtime::IInspectable);
impl MediaControlsTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PlaybackStatus(&self) -> ::windows::runtime::Result<PlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__: PlaybackStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlaybackStatus>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaMetadata(&self) -> ::windows::runtime::Result<MediaMetadata> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaMetadata>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaControlsTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaControlsTriggerDetails;{fab4648b-ae45-4548-91ca-4ab0548e33b5})");
}
unsafe impl ::windows::runtime::Interface for MediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfab4648b_ae45_4548_91ca_4ab0548e33b5);
}
impl ::windows::runtime::RuntimeName for MediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaControlsTriggerDetails";
}
impl ::core::convert::From<MediaControlsTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaControlsTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &MediaControlsTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaControlsTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaControlsTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaControlsTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaMetadata(pub ::windows::runtime::IInspectable);
impl MediaMetadata {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Subtitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Artist(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Album(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Track(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaMetadata {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaMetadata;{9b50ddf7-bb6c-4330-b3cd-0704a54cdb80})");
}
unsafe impl ::windows::runtime::Interface for MediaMetadata {
    type Vtable = IMediaMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b50ddf7_bb6c_4330_b3cd_0704a54cdb80);
}
impl ::windows::runtime::RuntimeName for MediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaMetadata";
}
impl ::core::convert::From<MediaMetadata> for ::windows::runtime::IUnknown {
    fn from(value: MediaMetadata) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaMetadata> for ::windows::runtime::IUnknown {
    fn from(value: &MediaMetadata) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaMetadata> for ::windows::runtime::IInspectable {
    fn from(value: MediaMetadata) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaMetadata> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallAudioEndpoint(pub i32);
impl PhoneCallAudioEndpoint {
    pub const Default: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(0i32);
    pub const Speaker: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(1i32);
    pub const Handsfree: PhoneCallAudioEndpoint = PhoneCallAudioEndpoint(2i32);
}
impl ::core::convert::From<i32> for PhoneCallAudioEndpoint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallAudioEndpoint {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallAudioEndpoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallAudioEndpoint;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallAudioEndpoint {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallDetails(pub ::windows::runtime::IInspectable);
impl PhoneCallDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneLine(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallTransport(&self) -> ::windows::runtime::Result<PhoneCallTransport> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallTransport = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallTransport>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallMediaType(&self) -> ::windows::runtime::Result<PhoneMediaType> {
        let this = self;
        unsafe {
            let mut result__: PhoneMediaType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneMediaType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallDirection(&self) -> ::windows::runtime::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallDirection = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallDirection>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn State(&self) -> ::windows::runtime::Result<PhoneCallState> {
        let this = self;
        unsafe {
            let mut result__: PhoneCallState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallState>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ConferenceCallId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn EndTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ContactName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation_Collections`*"]
    pub fn PresetTextResponses(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<TextResponse>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneCallDetails;{0c1b6f53-f071-483e-bf33-ebd44b724447})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallDetails {
    type Vtable = IPhoneCallDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c1b6f53_f071_483e_bf33_ebd44b724447);
}
impl ::windows::runtime::RuntimeName for PhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneCallDetails";
}
impl ::core::convert::From<PhoneCallDetails> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallDetails> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallDetails> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Incoming: PhoneCallDirection = PhoneCallDirection(0i32);
    pub const Outgoing: PhoneCallDirection = PhoneCallDirection(1i32);
}
impl ::core::convert::From<i32> for PhoneCallDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallDirection {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallDirection;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallState(pub i32);
impl PhoneCallState {
    pub const Unknown: PhoneCallState = PhoneCallState(0i32);
    pub const Ringing: PhoneCallState = PhoneCallState(1i32);
    pub const Talking: PhoneCallState = PhoneCallState(2i32);
    pub const Held: PhoneCallState = PhoneCallState(3i32);
    pub const Ended: PhoneCallState = PhoneCallState(4i32);
}
impl ::core::convert::From<i32> for PhoneCallState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallState;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneCallTransport(pub i32);
impl PhoneCallTransport {
    pub const Cellular: PhoneCallTransport = PhoneCallTransport(0i32);
    pub const Voip: PhoneCallTransport = PhoneCallTransport(1i32);
}
impl ::core::convert::From<i32> for PhoneCallTransport {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneCallTransport {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallTransport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallTransport;i4)");
}
impl ::windows::runtime::DefaultType for PhoneCallTransport {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneLineDetails(pub ::windows::runtime::IInspectable);
impl PhoneLineDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LineId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn LineNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn DefaultOutgoingLine(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn VoicemailCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn RegistrationState(&self) -> ::windows::runtime::Result<PhoneLineRegistrationState> {
        let this = self;
        unsafe {
            let mut result__: PhoneLineRegistrationState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneLineRegistrationState>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MissedCallCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPhoneLineDetails2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneLineDetails;{47eb32dc-33ed-49b9-995c-a296bac82b77})");
}
unsafe impl ::windows::runtime::Interface for PhoneLineDetails {
    type Vtable = IPhoneLineDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47eb32dc_33ed_49b9_995c_a296bac82b77);
}
impl ::windows::runtime::RuntimeName for PhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneLineDetails";
}
impl ::core::convert::From<PhoneLineDetails> for ::windows::runtime::IUnknown {
    fn from(value: PhoneLineDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneLineDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneLineDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneLineDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneLineDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneLineDetails> for ::windows::runtime::IInspectable {
    fn from(value: PhoneLineDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneLineDetails> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneLineRegistrationState(pub i32);
impl PhoneLineRegistrationState {
    pub const Disconnected: PhoneLineRegistrationState = PhoneLineRegistrationState(0i32);
    pub const Home: PhoneLineRegistrationState = PhoneLineRegistrationState(1i32);
    pub const Roaming: PhoneLineRegistrationState = PhoneLineRegistrationState(2i32);
}
impl ::core::convert::From<i32> for PhoneLineRegistrationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneLineRegistrationState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneLineRegistrationState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneLineRegistrationState;i4)");
}
impl ::windows::runtime::DefaultType for PhoneLineRegistrationState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneMediaType(pub i32);
impl PhoneMediaType {
    pub const AudioOnly: PhoneMediaType = PhoneMediaType(0i32);
    pub const AudioVideo: PhoneMediaType = PhoneMediaType(1i32);
}
impl ::core::convert::From<i32> for PhoneMediaType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneMediaType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneMediaType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneMediaType;i4)");
}
impl ::windows::runtime::DefaultType for PhoneMediaType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl PhoneNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneNotificationType(&self) -> ::windows::runtime::Result<PhoneNotificationType> {
        let this = self;
        unsafe {
            let mut result__: PhoneNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallDetails(&self) -> ::windows::runtime::Result<PhoneCallDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhoneCallDetails>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneLineChangedId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails;{ccc2fdf7-09c3-4118-91bc-ca6323a8d383})");
}
unsafe impl ::windows::runtime::Interface for PhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xccc2fdf7_09c3_4118_91bc_ca6323a8d383);
}
impl ::windows::runtime::RuntimeName for PhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails";
}
impl ::core::convert::From<PhoneNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PhoneNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneNotificationType(pub i32);
impl PhoneNotificationType {
    pub const NewCall: PhoneNotificationType = PhoneNotificationType(0i32);
    pub const CallChanged: PhoneNotificationType = PhoneNotificationType(1i32);
    pub const LineChanged: PhoneNotificationType = PhoneNotificationType(2i32);
    pub const PhoneCallAudioEndpointChanged: PhoneNotificationType = PhoneNotificationType(3i32);
    pub const PhoneMuteChanged: PhoneNotificationType = PhoneNotificationType(4i32);
}
impl ::core::convert::From<i32> for PhoneNotificationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneNotificationType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNotificationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneNotificationType;i4)");
}
impl ::windows::runtime::DefaultType for PhoneNotificationType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<u32> for PlaybackCapability {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlaybackCapability {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackCapability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCapability;u4)");
}
impl ::windows::runtime::DefaultType for PlaybackCapability {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PlaybackCapability {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PlaybackCapability {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PlaybackCapability {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PlaybackCapability {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PlaybackCapability {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for PlaybackCommand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlaybackCommand {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCommand;i4)");
}
impl ::windows::runtime::DefaultType for PlaybackCommand {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlaybackStatus(pub i32);
impl PlaybackStatus {
    pub const None: PlaybackStatus = PlaybackStatus(0i32);
    pub const TrackChanged: PlaybackStatus = PlaybackStatus(1i32);
    pub const Stopped: PlaybackStatus = PlaybackStatus(2i32);
    pub const Playing: PlaybackStatus = PlaybackStatus(3i32);
    pub const Paused: PlaybackStatus = PlaybackStatus(4i32);
}
impl ::core::convert::From<i32> for PlaybackStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlaybackStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackStatus;i4)");
}
impl ::windows::runtime::DefaultType for PlaybackStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ReminderNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl ReminderNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ReminderId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Details(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments")]
    #[doc = "*Required features: `Phone_Notification_Management`, `ApplicationModel_Appointments`*"]
    pub fn Appointment(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Appointments::Appointment>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ReminderState(&self) -> ::windows::runtime::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__: ReminderState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ReminderState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IReminderNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ReminderNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails;{5bddaa5d-9f61-4bf0-9feb-10502bc0b0c2})");
}
unsafe impl ::windows::runtime::Interface for ReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5bddaa5d_9f61_4bf0_9feb_10502bc0b0c2);
}
impl ::windows::runtime::RuntimeName for ReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails";
}
impl ::core::convert::From<ReminderNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ReminderNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ReminderNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ReminderNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ReminderNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ReminderNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ReminderState(pub i32);
impl ReminderState {
    pub const Active: ReminderState = ReminderState(0i32);
    pub const Snoozed: ReminderState = ReminderState(1i32);
    pub const Dismissed: ReminderState = ReminderState(2i32);
}
impl ::core::convert::From<i32> for ReminderState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ReminderState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ReminderState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.ReminderState;i4)");
}
impl ::windows::runtime::DefaultType for ReminderState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpeedDialEntry(pub ::windows::runtime::IInspectable);
impl SpeedDialEntry {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn PhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn NumberType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn ContactName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeedDialEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.SpeedDialEntry;{9240b6db-872c-46dc-b62a-be4541b166f8})");
}
unsafe impl ::windows::runtime::Interface for SpeedDialEntry {
    type Vtable = ISpeedDialEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9240b6db_872c_46dc_b62a_be4541b166f8);
}
impl ::windows::runtime::RuntimeName for SpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.SpeedDialEntry";
}
impl ::core::convert::From<SpeedDialEntry> for ::windows::runtime::IUnknown {
    fn from(value: SpeedDialEntry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpeedDialEntry> for ::windows::runtime::IUnknown {
    fn from(value: &SpeedDialEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeedDialEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeedDialEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpeedDialEntry> for ::windows::runtime::IInspectable {
    fn from(value: SpeedDialEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpeedDialEntry> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TextResponse(pub ::windows::runtime::IInspectable);
impl TextResponse {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.TextResponse;{e9cb74c3-2457-4cdb-8110-72f5e8e883e8})");
}
unsafe impl ::windows::runtime::Interface for TextResponse {
    type Vtable = ITextResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe9cb74c3_2457_4cdb_8110_72f5e8e883e8);
}
impl ::windows::runtime::RuntimeName for TextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.TextResponse";
}
impl ::core::convert::From<TextResponse> for ::windows::runtime::IUnknown {
    fn from(value: TextResponse) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextResponse> for ::windows::runtime::IUnknown {
    fn from(value: &TextResponse) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TextResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextResponse> for ::windows::runtime::IInspectable {
    fn from(value: TextResponse) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextResponse> for ::windows::runtime::IInspectable {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl ToastNotificationTriggerDetails {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text2(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text3(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn Text4(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SuppressPopup(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Notification_Management`, `Foundation`*"]
    pub fn TimeCreated(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::runtime::Result<AccessoryNotificationType> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: AccessoryNotificationType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn StartedProcessing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IToastNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ToastNotificationTriggerDetails;{c9314895-4e6d-4e9d-afec-9e921b875ae8})");
}
unsafe impl ::windows::runtime::Interface for ToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9314895_4e6d_4e9d_afec_9e921b875ae8);
}
impl ::windows::runtime::RuntimeName for ToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ToastNotificationTriggerDetails";
}
impl ::core::convert::From<ToastNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotificationTriggerDetails> for ::windows::runtime::IInspectable {
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
impl ::core::convert::TryFrom<ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ToastNotificationTriggerDetails) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
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
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VibrateState(pub i32);
impl VibrateState {
    pub const RingerOffVibrateOff: VibrateState = VibrateState(0i32);
    pub const RingerOffVibrateOn: VibrateState = VibrateState(1i32);
    pub const RingerOnVibrateOff: VibrateState = VibrateState(2i32);
    pub const RingerOnVibrateOn: VibrateState = VibrateState(3i32);
}
impl ::core::convert::From<i32> for VibrateState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VibrateState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VibrateState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.VibrateState;i4)");
}
impl ::windows::runtime::DefaultType for VibrateState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Phone_Notification_Management`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VolumeInfo(pub ::windows::runtime::IInspectable);
impl VolumeInfo {
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn SystemVolume(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn CallVolume(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn MediaVolume(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsMuted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Notification_Management`*"]
    pub fn IsVibrateEnabled(&self) -> ::windows::runtime::Result<VibrateState> {
        let this = self;
        unsafe {
            let mut result__: VibrateState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VibrateState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VolumeInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.VolumeInfo;{944dd118-7704-4481-b92e-d3ed3ece6322})");
}
unsafe impl ::windows::runtime::Interface for VolumeInfo {
    type Vtable = IVolumeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x944dd118_7704_4481_b92e_d3ed3ece6322);
}
impl ::windows::runtime::RuntimeName for VolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.VolumeInfo";
}
impl ::core::convert::From<VolumeInfo> for ::windows::runtime::IUnknown {
    fn from(value: VolumeInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VolumeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &VolumeInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VolumeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VolumeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VolumeInfo> for ::windows::runtime::IInspectable {
    fn from(value: VolumeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VolumeInfo> for ::windows::runtime::IInspectable {
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
