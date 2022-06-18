#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
pub struct AccessoryManager;
impl AccessoryManager {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn RegisterAccessoryApp() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).RegisterAccessoryApp)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn GetNextTriggerDetails() -> ::windows::core::Result<IAccessoryNotificationTriggerDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetNextTriggerDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IAccessoryNotificationTriggerDetails>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ProcessTriggerDetails<'a, Param0: ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails>>(pdetails: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).ProcessTriggerDetails)(::windows::core::Interface::as_raw(this), pdetails.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PhoneLineDetails() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneLineDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn GetPhoneLineDetails<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(phoneline: Param0) -> ::windows::core::Result<PhoneLineDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetPhoneLineDetails)(::windows::core::Interface::as_raw(this), phoneline.into_param().abi(), result__.as_mut_ptr()).from_abi::<PhoneLineDetails>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AcceptPhoneCall(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).AcceptPhoneCall)(::windows::core::Interface::as_raw(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AcceptPhoneCallOnEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).AcceptPhoneCallOnEndpoint)(::windows::core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AcceptPhoneCallWithVideo(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).AcceptPhoneCallWithVideo)(::windows::core::Interface::as_raw(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).AcceptPhoneCallWithVideoOnAudioEndpoint)(::windows::core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn RejectPhoneCall(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).RejectPhoneCall)(::windows::core::Interface::as_raw(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn RejectPhoneCallWithText(phonecallid: u32, textresponseid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).RejectPhoneCallWithText)(::windows::core::Interface::as_raw(this), phonecallid, textresponseid).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MakePhoneCall<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).MakePhoneCall)(::windows::core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MakePhoneCallOnAudioEndpoint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).MakePhoneCallOnAudioEndpoint)(::windows::core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MakePhoneCallWithVideo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phoneline: Param0, phonenumber: Param1) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).MakePhoneCallWithVideo)(::windows::core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MakePhoneCallWithVideoOnAudioEndpoint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(phoneline: Param0, phonenumber: Param1, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).MakePhoneCallWithVideoOnAudioEndpoint)(::windows::core::Interface::as_raw(this), phoneline.into_param().abi(), phonenumber.into_param().abi(), endpoint).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SwapPhoneCalls(phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).SwapPhoneCalls)(::windows::core::Interface::as_raw(this), phonecallidtohold, phonecallidonhold).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn HoldPhoneCall(phonecallid: u32, holdcall: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).HoldPhoneCall)(::windows::core::Interface::as_raw(this), phonecallid, holdcall).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn EndPhoneCall(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).EndPhoneCall)(::windows::core::Interface::as_raw(this), phonecallid).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetPhoneMute(value: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).SetPhoneMute)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PhoneMute() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneMute)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetPhoneCallAudioEndpoint(value: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).SetPhoneCallAudioEndpoint)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PhoneCallAudioEndpoint() -> ::windows::core::Result<PhoneCallAudioEndpoint> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallAudioEndpoint>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneCallAudioEndpoint)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallAudioEndpoint>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SnoozeAlarm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(alarmid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).SnoozeAlarm)(::windows::core::Interface::as_raw(this), alarmid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnoozeAlarmForSpecifiedTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(alarmid: Param0, timespan: Param1) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).SnoozeAlarmForSpecifiedTime)(::windows::core::Interface::as_raw(this), alarmid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DismissAlarm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(alarmid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).DismissAlarm)(::windows::core::Interface::as_raw(this), alarmid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SnoozeReminder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(reminderid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).SnoozeReminder)(::windows::core::Interface::as_raw(this), reminderid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnoozeReminderForSpecifiedTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(reminderid: Param0, timespan: Param1) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).SnoozeReminderForSpecifiedTime)(::windows::core::Interface::as_raw(this), reminderid.into_param().abi(), timespan.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DismissReminder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(reminderid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).DismissReminder)(::windows::core::Interface::as_raw(this), reminderid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn GetMediaMetadata() -> ::windows::core::Result<MediaMetadata> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetMediaMetadata)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaMetadata>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MediaPlaybackCapabilities() -> ::windows::core::Result<PlaybackCapability> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlaybackCapability>::zeroed();
            (::windows::core::Interface::vtable(this).MediaPlaybackCapabilities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackCapability>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MediaPlaybackStatus() -> ::windows::core::Result<PlaybackStatus> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlaybackStatus>::zeroed();
            (::windows::core::Interface::vtable(this).MediaPlaybackStatus)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PerformMediaPlaybackCommand(command: PlaybackCommand) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).PerformMediaPlaybackCommand)(::windows::core::Interface::as_raw(this), command).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DoNotDisturbEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DoNotDisturbEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DrivingModeEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DrivingModeEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn BatterySaverState() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).BatterySaverState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetApps() -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppNotificationInfo>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetApps)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppNotificationInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn EnableNotificationsForApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).EnableNotificationsForApplication)(::windows::core::Interface::as_raw(this), appid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DisableNotificationsForApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).DisableNotificationsForApplication)(::windows::core::Interface::as_raw(this), appid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IsNotificationEnabledForApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appid: Param0) -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsNotificationEnabledForApplication)(::windows::core::Interface::as_raw(this), appid.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn GetEnabledAccessoryNotificationTypes() -> ::windows::core::Result<i32> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).GetEnabledAccessoryNotificationTypes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn EnableAccessoryNotificationTypes(accessorynotificationtypes: i32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).EnableAccessoryNotificationTypes)(::windows::core::Interface::as_raw(this), accessorynotificationtypes).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DisableAllAccessoryNotificationTypes() -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Interface::vtable(this).DisableAllAccessoryNotificationTypes)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn GetUserConsent() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).GetUserConsent)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetAppIcon<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appid: Param0) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAppIcon)(::windows::core::Interface::as_raw(this), appid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn RingDevice() -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).RingDevice)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SpeedDialList() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).SpeedDialList)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ClearToast<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(instanceid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).ClearToast)(::windows::core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IsPhonePinLocked() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsPhonePinLocked)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IncreaseVolume(step: i32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).IncreaseVolume)(::windows::core::Interface::as_raw(this), step).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DecreaseVolume(step: i32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).DecreaseVolume)(::windows::core::Interface::as_raw(this), step).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetMute(mute: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).SetMute)(::windows::core::Interface::as_raw(this), mute).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetRingerVibrate(ringer: bool, vibrate: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).SetRingerVibrate)(::windows::core::Interface::as_raw(this), ringer, vibrate).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn VolumeInfo() -> ::windows::core::Result<VolumeInfo> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).VolumeInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VolumeInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllEmailAccounts() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAllEmailAccounts)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFolders<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(emailaccount: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetFolders)(::windows::core::Interface::as_raw(this), emailaccount.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn EnableEmailNotificationEmailAccount<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(emailaccount: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).EnableEmailNotificationEmailAccount)(::windows::core::Interface::as_raw(this), emailaccount.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DisableEmailNotificationEmailAccount<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(emailaccount: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).DisableEmailNotificationEmailAccount)(::windows::core::Interface::as_raw(this), emailaccount.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnableEmailNotificationFolderFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(emailaccount: Param0, folders: Param1) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).EnableEmailNotificationFolderFilter)(::windows::core::Interface::as_raw(this), emailaccount.into_param().abi(), folders.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn UpdateEmailReadStatus<'a, Param0: ::windows::core::IntoParam<'a, BinaryId>>(messageentryid: Param0, isread: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Interface::vtable(this).UpdateEmailReadStatus)(::windows::core::Interface::as_raw(this), messageentryid.into_param().abi(), isread).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SnoozeAlarmByInstanceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(instanceid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Interface::vtable(this).SnoozeAlarmByInstanceId)(::windows::core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DismissAlarmByInstanceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(instanceid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Interface::vtable(this).DismissAlarmByInstanceId)(::windows::core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SnoozeReminderByInstanceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(instanceid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Interface::vtable(this).SnoozeReminderByInstanceId)(::windows::core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DismissReminderByInstanceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(instanceid: Param0) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Interface::vtable(this).DismissReminderByInstanceId)(::windows::core::Interface::as_raw(this), instanceid.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAccessoryManager<R, F: FnOnce(&IAccessoryManager) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AccessoryManager, IAccessoryManager> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccessoryManager2<R, F: FnOnce(&IAccessoryManager2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AccessoryManager, IAccessoryManager2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccessoryManager3<R, F: FnOnce(&IAccessoryManager3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AccessoryManager, IAccessoryManager3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AccessoryManager {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AccessoryManager";
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AccessoryNotificationType(pub u32);
impl AccessoryNotificationType {
    pub const None: Self = Self(0u32);
    pub const Phone: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Reminder: Self = Self(4u32);
    pub const Alarm: Self = Self(8u32);
    pub const Toast: Self = Self(16u32);
    pub const AppUninstalled: Self = Self(32u32);
    pub const Dnd: Self = Self(64u32);
    pub const DrivingMode: Self = Self(128u32);
    pub const BatterySaver: Self = Self(256u32);
    pub const Media: Self = Self(512u32);
    pub const CortanaTile: Self = Self(1024u32);
    pub const ToastCleared: Self = Self(2048u32);
    pub const CalendarChanged: Self = Self(4096u32);
    pub const VolumeChanged: Self = Self(8192u32);
    pub const EmailReadStatusChanged: Self = Self(16384u32);
}
impl ::core::marker::Copy for AccessoryNotificationType {}
impl ::core::clone::Clone for AccessoryNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AccessoryNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AccessoryNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AccessoryNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessoryNotificationType").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AccessoryNotificationType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AccessoryNotificationType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AccessoryNotificationType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AccessoryNotificationType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AccessoryNotificationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for AccessoryNotificationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.AccessoryNotificationType;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct AlarmNotificationTriggerDetails(::windows::core::IUnknown);
impl AlarmNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AlarmId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
            (::windows::core::Interface::vtable(this).AlarmId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ReminderState(&self) -> ::windows::core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ReminderState>::zeroed();
            (::windows::core::Interface::vtable(this).ReminderState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ReminderState>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAlarmNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AlarmNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AlarmNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AlarmNotificationTriggerDetails {}
impl ::core::fmt::Debug for AlarmNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlarmNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AlarmNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails;{38f5fa30-c738-4da2-908c-775d83c36abb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IAlarmNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails";
}
impl ::core::convert::From<AlarmNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AlarmNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &AlarmNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AlarmNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: AlarmNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AlarmNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &AlarmNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: AlarmNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AlarmNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &AlarmNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &AlarmNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct AppNotificationInfo(::windows::core::IUnknown);
impl AppNotificationInfo {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppNotificationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationInfo {}
impl ::core::fmt::Debug for AppNotificationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AppNotificationInfo;{2157bea5-e286-45d3-9bea-f790fc216e0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppNotificationInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AppNotificationInfo";
}
impl ::core::convert::From<AppNotificationInfo> for ::windows::core::IUnknown {
    fn from(value: AppNotificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationInfo> for ::windows::core::IUnknown {
    fn from(value: &AppNotificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppNotificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppNotificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppNotificationInfo> for ::windows::core::IInspectable {
    fn from(value: AppNotificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppNotificationInfo> for ::windows::core::IInspectable {
    fn from(value: &AppNotificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppNotificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppNotificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct BinaryId(::windows::core::IUnknown);
impl BinaryId {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for BinaryId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BinaryId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BinaryId {}
impl ::core::fmt::Debug for BinaryId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BinaryId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BinaryId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.BinaryId;{4f0da531-5595-44b4-9181-ce4efa3fc168})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BinaryId {
    type Vtable = IBinaryId_Vtbl;
    const IID: ::windows::core::GUID = <IBinaryId as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.BinaryId";
}
impl ::core::convert::From<BinaryId> for ::windows::core::IUnknown {
    fn from(value: BinaryId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BinaryId> for ::windows::core::IUnknown {
    fn from(value: &BinaryId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BinaryId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BinaryId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BinaryId> for ::windows::core::IInspectable {
    fn from(value: BinaryId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BinaryId> for ::windows::core::IInspectable {
    fn from(value: &BinaryId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BinaryId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BinaryId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CalendarChangedEvent(pub i32);
impl CalendarChangedEvent {
    pub const LostEvents: Self = Self(0i32);
    pub const AppointmentAdded: Self = Self(1i32);
    pub const AppointmentChanged: Self = Self(2i32);
    pub const AppointmentDeleted: Self = Self(3i32);
    pub const CalendarAdded: Self = Self(4i32);
    pub const CalendarChanged: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for CalendarChangedEvent {}
impl ::core::clone::Clone for CalendarChangedEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CalendarChangedEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CalendarChangedEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for CalendarChangedEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarChangedEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CalendarChangedEvent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.CalendarChangedEvent;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct CalendarChangedNotificationTriggerDetails(::windows::core::IUnknown);
impl CalendarChangedNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn EventType(&self) -> ::windows::core::Result<CalendarChangedEvent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CalendarChangedEvent>::zeroed();
            (::windows::core::Interface::vtable(this).EventType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CalendarChangedEvent>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ItemId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CalendarChangedNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CalendarChangedNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CalendarChangedNotificationTriggerDetails {}
impl ::core::fmt::Debug for CalendarChangedNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarChangedNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CalendarChangedNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails;{4b8a3bfc-279d-42ab-9c68-3e87977bf216})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <ICalendarChangedNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails";
}
impl ::core::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &CalendarChangedNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CalendarChangedNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: CalendarChangedNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarChangedNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &CalendarChangedNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: CalendarChangedNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CalendarChangedNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &CalendarChangedNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &CalendarChangedNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct CortanaTileNotificationTriggerDetails(::windows::core::IUnknown);
impl CortanaTileNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn LargeContent1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).LargeContent1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn LargeContent2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).LargeContent2)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn EmphasizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).EmphasizedText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn NonWrappedSmallContent1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).NonWrappedSmallContent1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn NonWrappedSmallContent2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).NonWrappedSmallContent2)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn NonWrappedSmallContent3(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).NonWrappedSmallContent3)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn NonWrappedSmallContent4(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).NonWrappedSmallContent4)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CortanaTileNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CortanaTileNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CortanaTileNotificationTriggerDetails {}
impl ::core::fmt::Debug for CortanaTileNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CortanaTileNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CortanaTileNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails;{dc0f01d5-1489-46bb-b73b-7f90067ecf27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <ICortanaTileNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails";
}
impl ::core::convert::From<CortanaTileNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &CortanaTileNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CortanaTileNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: CortanaTileNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CortanaTileNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &CortanaTileNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: CortanaTileNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CortanaTileNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &CortanaTileNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &CortanaTileNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailAccountInfo(::windows::core::IUnknown);
impl EmailAccountInfo {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsNotificationEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailAccountInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailAccountInfo {}
impl ::core::fmt::Debug for EmailAccountInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailAccountInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailAccountInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailAccountInfo;{dfbc02ab-bda0-4568-927e-b2ede35818a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
    const IID: ::windows::core::GUID = <IEmailAccountInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailAccountInfo";
}
impl ::core::convert::From<EmailAccountInfo> for ::windows::core::IUnknown {
    fn from(value: EmailAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailAccountInfo> for ::windows::core::IUnknown {
    fn from(value: &EmailAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailAccountInfo> for ::windows::core::IInspectable {
    fn from(value: EmailAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailAccountInfo> for ::windows::core::IInspectable {
    fn from(value: &EmailAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailFolderInfo(::windows::core::IUnknown);
impl EmailFolderInfo {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsNotificationEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailFolderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailFolderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailFolderInfo {}
impl ::core::fmt::Debug for EmailFolderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailFolderInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailFolderInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailFolderInfo;{c207150e-e237-46d6-90e6-4f529eeac1e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
    const IID: ::windows::core::GUID = <IEmailFolderInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailFolderInfo";
}
impl ::core::convert::From<EmailFolderInfo> for ::windows::core::IUnknown {
    fn from(value: EmailFolderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailFolderInfo> for ::windows::core::IUnknown {
    fn from(value: &EmailFolderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailFolderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailFolderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailFolderInfo> for ::windows::core::IInspectable {
    fn from(value: EmailFolderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailFolderInfo> for ::windows::core::IInspectable {
    fn from(value: &EmailFolderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailFolderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailFolderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailNotificationTriggerDetails(::windows::core::IUnknown);
impl EmailNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AccountName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ParentFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ParentFolderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SenderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).SenderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SenderAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).SenderAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"ApplicationModel_Email\"`*"]
    #[cfg(feature = "ApplicationModel_Email")]
    pub fn EmailMessage(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Email::EmailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).EmailMessage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::ApplicationModel::Email::EmailMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId> {
        let this = &::windows::core::Interface::cast::<IEmailNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).MessageEntryId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BinaryId>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailNotificationTriggerDetails;{f3b82612-46cf-4e70-8e0d-7b2e04ab492b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IEmailNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailNotificationTriggerDetails";
}
impl ::core::convert::From<EmailNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &EmailNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: EmailNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &EmailNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: EmailNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EmailNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &EmailNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &EmailNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailReadNotificationTriggerDetails(::windows::core::IUnknown);
impl EmailReadNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AccountName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ParentFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ParentFolderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).MessageEntryId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BinaryId>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IsRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsRead)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for EmailReadNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailReadNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailReadNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailReadNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailReadNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailReadNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails;{f5b7a087-06f3-4e3e-8c42-325e67010413})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IEmailReadNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails";
}
impl ::core::convert::From<EmailReadNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailReadNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &EmailReadNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EmailReadNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: EmailReadNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EmailReadNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &EmailReadNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: EmailReadNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EmailReadNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &EmailReadNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &EmailReadNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessoryManager {
    type Vtable = IAccessoryManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d04a12c_883d_4aa7_bca7_fa4bb8bffee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RegisterAccessoryApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetNextTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetails: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PhoneLineDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PhoneLineDetails: usize,
    pub GetPhoneLineDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AcceptPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT,
    pub AcceptPhoneCallOnEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
    pub AcceptPhoneCallWithVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT,
    pub AcceptPhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
    pub RejectPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT,
    pub RejectPhoneCallWithText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, textresponseid: u32) -> ::windows::core::HRESULT,
    pub MakePhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MakePhoneCallOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
    pub MakePhoneCallWithVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MakePhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
    pub SwapPhoneCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::core::HRESULT,
    pub HoldPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, holdcall: bool) -> ::windows::core::HRESULT,
    pub EndPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT,
    pub SetPhoneMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PhoneMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
    pub PhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
    pub SnoozeAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SnoozeAlarmForSpecifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeAlarmForSpecifiedTime: usize,
    pub DismissAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SnoozeReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SnoozeReminderForSpecifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeReminderForSpecifiedTime: usize,
    pub DismissReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMediaMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MediaPlaybackCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackCapability) -> ::windows::core::HRESULT,
    pub MediaPlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows::core::HRESULT,
    pub PerformMediaPlaybackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: PlaybackCommand) -> ::windows::core::HRESULT,
    pub DoNotDisturbEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DrivingModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub BatterySaverState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetApps: usize,
    pub EnableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsNotificationEnabledForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetEnabledAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub EnableAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessorynotificationtypes: i32) -> ::windows::core::HRESULT,
    pub DisableAllAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserConsent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAppIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAppIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessoryManager2 {
    type Vtable = IAccessoryManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbacad44d_d393_46c6_b80c_15fdf44d5386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SpeedDialList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SpeedDialList: usize,
    pub ClearToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsPhonePinLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IncreaseVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, step: i32) -> ::windows::core::HRESULT,
    pub DecreaseVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, step: i32) -> ::windows::core::HRESULT,
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mute: bool) -> ::windows::core::HRESULT,
    pub SetRingerVibrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ringer: bool, vibrate: bool) -> ::windows::core::HRESULT,
    pub VolumeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllEmailAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllEmailAccounts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFolders: usize,
    pub EnableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EnableEmailNotificationFolderFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, folders: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnableEmailNotificationFolderFilter: usize,
    pub UpdateEmailReadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageentryid: *mut ::core::ffi::c_void, isread: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessoryManager3 {
    type Vtable = IAccessoryManager3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81f75137_edc7_47e0_b2f7_7e577c833f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SnoozeAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SnoozeReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct IAccessoryNotificationTriggerDetails(::windows::core::IUnknown);
impl IAccessoryNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<IAccessoryNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: IAccessoryNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccessoryNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &IAccessoryNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAccessoryNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: IAccessoryNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccessoryNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &IAccessoryNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAccessoryNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAccessoryNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccessoryNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessoryNotificationTriggerDetails {}
impl ::core::fmt::Debug for IAccessoryNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessoryNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAccessoryNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6968a7d4-e3ca-49cb-8c87-2c11cdff9646}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAccessoryNotificationTriggerDetails {
    type Vtable = IAccessoryNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6968a7d4_e3ca_49cb_8c87_2c11cdff9646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TimeCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeCreated: usize,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AccessoryNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows::core::HRESULT,
    pub StartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38f5fa30_c738_4da2_908c_775d83c36abb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AlarmId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAlarmNotificationTriggerDetails2 {
    type Vtable = IAlarmNotificationTriggerDetails2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf16e06a_7155_40fe_a9c2_7bd2127ef853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2157bea5_e286_45d3_9bea_f790fc216e0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBinaryId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBinaryId {
    type Vtable = IBinaryId_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f0da531_5595_44b4_9181_ce4efa3fc168);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinaryId_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarChangedNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b8a3bfc_279d_42ab_9c68_3e87977bf216);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarChangedNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CalendarChangedEvent) -> ::windows::core::HRESULT,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICortanaTileNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc0f01d5_1489_46bb_b73b_7f90067ecf27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaTileNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LargeContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LargeContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EmphasizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAccountInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfbc02ab_bda0_4568_927e_b2ede35818a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAccountInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailFolderInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc207150e_e237_46d6_90e6_4f529eeac1e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailFolderInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b82612_46cf_4e70_8e0d_7b2e04ab492b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SenderAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Email")]
    pub EmailMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Email"))]
    EmailMessage: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailNotificationTriggerDetails2 {
    type Vtable = IEmailNotificationTriggerDetails2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x168067e3_c56f_4ec7_bed1_f734e08de5b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailReadNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b7a087_06f3_4e3e_8c42_325e67010413);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailReadNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaControlsTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfab4648b_ae45_4548_91ca_4ab0548e33b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControlsTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows::core::HRESULT,
    pub MediaMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaMetadata(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b50ddf7_bb6c_4330_b3cd_0704a54cdb80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMetadata_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Track: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c1b6f53_f071_483e_bf33_ebd44b724447);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PhoneLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CallTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallTransport) -> ::windows::core::HRESULT,
    pub CallMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneMediaType) -> ::windows::core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallDirection) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallState) -> ::windows::core::HRESULT,
    pub ConferenceCallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresetTextResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresetTextResponses: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47eb32dc_33ed_49b9_995c_a296bac82b77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DefaultOutgoingLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub RegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineRegistrationState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneLineDetails2 {
    type Vtable = IPhoneLineDetails2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb30cd77d_0147_498c_8241_bf0cabc60a25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub MissedCallCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccc2fdf7_09c3_4118_91bc_ca6323a8d383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PhoneNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneNotificationType) -> ::windows::core::HRESULT,
    pub CallDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhoneLineChangedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bddaa5d_9f61_4bf0_9feb_10502bc0b0c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ReminderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Appointment: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReminderNotificationTriggerDetails2 {
    type Vtable = IReminderNotificationTriggerDetails2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe715f9c0_504d_4c0f_a6b3_bcb9722c6cdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeedDialEntry(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9240b6db_872c_46dc_b62a_be4541b166f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeedDialEntry_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NumberType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextResponse {
    type Vtable = ITextResponse_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9cb74c3_2457_4cdb_8110_72f5e8e883e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextResponse_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9314895_4e6d_4e9d_afec_9e921b875ae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Text1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Text2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Text3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Text4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IToastNotificationTriggerDetails2 {
    type Vtable = IToastNotificationTriggerDetails2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e0479dd_cac4_4f60_afa3_b925d9d83c93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVolumeInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x944dd118_7704_4481_b92e_d3ed3ece6322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SystemVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CallVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MediaVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVibrateEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VibrateState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct MediaControlsTriggerDetails(::windows::core::IUnknown);
impl MediaControlsTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PlaybackStatus(&self) -> ::windows::core::Result<PlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PlaybackStatus>::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackStatus)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PlaybackStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MediaMetadata(&self) -> ::windows::core::Result<MediaMetadata> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).MediaMetadata)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaMetadata>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaControlsTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaControlsTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaControlsTriggerDetails {}
impl ::core::fmt::Debug for MediaControlsTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaControlsTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaControlsTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaControlsTriggerDetails;{fab4648b-ae45-4548-91ca-4ab0548e33b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IMediaControlsTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaControlsTriggerDetails";
}
impl ::core::convert::From<MediaControlsTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaControlsTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &MediaControlsTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaControlsTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: MediaControlsTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaControlsTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &MediaControlsTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaControlsTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaControlsTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaControlsTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &MediaControlsTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct MediaMetadata(::windows::core::IUnknown);
impl MediaMetadata {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Subtitle)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Artist)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Album)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Track(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).Track)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::TimeSpan>::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaMetadata {}
impl ::core::fmt::Debug for MediaMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaMetadata {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaMetadata;{9b50ddf7-bb6c-4330-b3cd-0704a54cdb80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
    const IID: ::windows::core::GUID = <IMediaMetadata as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaMetadata";
}
impl ::core::convert::From<MediaMetadata> for ::windows::core::IUnknown {
    fn from(value: MediaMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaMetadata> for ::windows::core::IUnknown {
    fn from(value: &MediaMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaMetadata> for ::windows::core::IInspectable {
    fn from(value: MediaMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaMetadata> for ::windows::core::IInspectable {
    fn from(value: &MediaMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallAudioEndpoint(pub i32);
impl PhoneCallAudioEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Speaker: Self = Self(1i32);
    pub const Handsfree: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallAudioEndpoint {}
impl ::core::clone::Clone for PhoneCallAudioEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallAudioEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallAudioEndpoint {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallAudioEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallAudioEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallAudioEndpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallAudioEndpoint;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneCallDetails(::windows::core::IUnknown);
impl PhoneCallDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PhoneLine(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneLine)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn CallId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).CallId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn CallTransport(&self) -> ::windows::core::Result<PhoneCallTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallTransport>::zeroed();
            (::windows::core::Interface::vtable(this).CallTransport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallTransport>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn CallMediaType(&self) -> ::windows::core::Result<PhoneMediaType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneMediaType>::zeroed();
            (::windows::core::Interface::vtable(this).CallMediaType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneMediaType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn CallDirection(&self) -> ::windows::core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallDirection>::zeroed();
            (::windows::core::Interface::vtable(this).CallDirection)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn State(&self) -> ::windows::core::Result<PhoneCallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneCallState>::zeroed();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallState>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ConferenceCallId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).ConferenceCallId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).EndTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ContactName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresetTextResponses(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).PresetTextResponses)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<TextResponse>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallDetails {}
impl ::core::fmt::Debug for PhoneCallDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneCallDetails;{0c1b6f53-f071-483e-bf33-ebd44b724447})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneCallDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneCallDetails";
}
impl ::core::convert::From<PhoneCallDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneCallDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneCallDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Incoming: Self = Self(0i32);
    pub const Outgoing: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallDirection {}
impl ::core::clone::Clone for PhoneCallDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallState(pub i32);
impl PhoneCallState {
    pub const Unknown: Self = Self(0i32);
    pub const Ringing: Self = Self(1i32);
    pub const Talking: Self = Self(2i32);
    pub const Held: Self = Self(3i32);
    pub const Ended: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneCallState {}
impl ::core::clone::Clone for PhoneCallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneCallTransport(pub i32);
impl PhoneCallTransport {
    pub const Cellular: Self = Self(0i32);
    pub const Voip: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallTransport {}
impl ::core::clone::Clone for PhoneCallTransport {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneCallTransport {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneCallTransport {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneCallTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallTransport;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneLineDetails(::windows::core::IUnknown);
impl PhoneLineDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn LineNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).LineNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn DefaultOutgoingLine(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DefaultOutgoingLine)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn VoicemailCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).VoicemailCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn RegistrationState(&self) -> ::windows::core::Result<PhoneLineRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneLineRegistrationState>::zeroed();
            (::windows::core::Interface::vtable(this).RegistrationState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneLineRegistrationState>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MissedCallCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPhoneLineDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).MissedCallCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneLineDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneLineDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineDetails {}
impl ::core::fmt::Debug for PhoneLineDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneLineDetails;{47eb32dc-33ed-49b9-995c-a296bac82b77})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneLineDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneLineDetails";
}
impl ::core::convert::From<PhoneLineDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneLineDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneLineDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneLineDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneLineDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneLineDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneLineDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneLineDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneLineDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneLineDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneLineDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneLineRegistrationState(pub i32);
impl PhoneLineRegistrationState {
    pub const Disconnected: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Roaming: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineRegistrationState {}
impl ::core::clone::Clone for PhoneLineRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneLineRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneLineRegistrationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneLineRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineRegistrationState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneLineRegistrationState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneLineRegistrationState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneMediaType(pub i32);
impl PhoneMediaType {
    pub const AudioOnly: Self = Self(0i32);
    pub const AudioVideo: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneMediaType {}
impl ::core::clone::Clone for PhoneMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneMediaType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneMediaType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneMediaType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneMediaType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneMediaType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneNotificationTriggerDetails(::windows::core::IUnknown);
impl PhoneNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PhoneNotificationType(&self) -> ::windows::core::Result<PhoneNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PhoneNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn CallDetails(&self) -> ::windows::core::Result<PhoneCallDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CallDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhoneCallDetails>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PhoneLineChangedId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneLineChangedId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNotificationTriggerDetails {}
impl ::core::fmt::Debug for PhoneNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails;{ccc2fdf7-09c3-4118-91bc-ca6323a8d383})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPhoneNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails";
}
impl ::core::convert::From<PhoneNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PhoneNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PhoneNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PhoneNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &PhoneNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PhoneNotificationType(pub i32);
impl PhoneNotificationType {
    pub const NewCall: Self = Self(0i32);
    pub const CallChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const PhoneCallAudioEndpointChanged: Self = Self(3i32);
    pub const PhoneMuteChanged: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneNotificationType {}
impl ::core::clone::Clone for PhoneNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhoneNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhoneNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNotificationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneNotificationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneNotificationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlaybackCapability(pub u32);
impl PlaybackCapability {
    pub const None: Self = Self(0u32);
    pub const Play: Self = Self(1u32);
    pub const Pause: Self = Self(2u32);
    pub const Stop: Self = Self(4u32);
    pub const Record: Self = Self(8u32);
    pub const FastForward: Self = Self(16u32);
    pub const Rewind: Self = Self(32u32);
    pub const Next: Self = Self(64u32);
    pub const Previous: Self = Self(128u32);
    pub const ChannelUp: Self = Self(256u32);
    pub const ChannelDown: Self = Self(512u32);
}
impl ::core::marker::Copy for PlaybackCapability {}
impl ::core::clone::Clone for PlaybackCapability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlaybackCapability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlaybackCapability {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaybackCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCapability").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PlaybackCapability {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PlaybackCapability {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PlaybackCapability {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PlaybackCapability {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PlaybackCapability {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCapability;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlaybackCommand(pub i32);
impl PlaybackCommand {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl ::core::marker::Copy for PlaybackCommand {}
impl ::core::clone::Clone for PlaybackCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlaybackCommand {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlaybackCommand {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaybackCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCommand;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlaybackStatus(pub i32);
impl PlaybackStatus {
    pub const None: Self = Self(0i32);
    pub const TrackChanged: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for PlaybackStatus {}
impl ::core::clone::Clone for PlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlaybackStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlaybackStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct ReminderNotificationTriggerDetails(::windows::core::IUnknown);
impl ReminderNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ReminderId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
            (::windows::core::Interface::vtable(this).ReminderId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Details)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"ApplicationModel_Appointments\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn Appointment(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Appointment)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::ApplicationModel::Appointments::Appointment>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ReminderState(&self) -> ::windows::core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ReminderState>::zeroed();
            (::windows::core::Interface::vtable(this).ReminderState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ReminderState>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IReminderNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ReminderNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReminderNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReminderNotificationTriggerDetails {}
impl ::core::fmt::Debug for ReminderNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReminderNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReminderNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails;{5bddaa5d-9f61-4bf0-9feb-10502bc0b0c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IReminderNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails";
}
impl ::core::convert::From<ReminderNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReminderNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &ReminderNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReminderNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: ReminderNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReminderNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &ReminderNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: ReminderNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReminderNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReminderNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &ReminderNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ReminderState(pub i32);
impl ReminderState {
    pub const Active: Self = Self(0i32);
    pub const Snoozed: Self = Self(1i32);
    pub const Dismissed: Self = Self(2i32);
}
impl ::core::marker::Copy for ReminderState {}
impl ::core::clone::Clone for ReminderState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReminderState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ReminderState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ReminderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReminderState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReminderState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.ReminderState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct SpeedDialEntry(::windows::core::IUnknown);
impl SpeedDialEntry {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn NumberType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).NumberType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ContactName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeedDialEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeedDialEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeedDialEntry {}
impl ::core::fmt::Debug for SpeedDialEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeedDialEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeedDialEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.SpeedDialEntry;{9240b6db-872c-46dc-b62a-be4541b166f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
    const IID: ::windows::core::GUID = <ISpeedDialEntry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.SpeedDialEntry";
}
impl ::core::convert::From<SpeedDialEntry> for ::windows::core::IUnknown {
    fn from(value: SpeedDialEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeedDialEntry> for ::windows::core::IUnknown {
    fn from(value: &SpeedDialEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeedDialEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpeedDialEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeedDialEntry> for ::windows::core::IInspectable {
    fn from(value: SpeedDialEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeedDialEntry> for ::windows::core::IInspectable {
    fn from(value: &SpeedDialEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeedDialEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpeedDialEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct TextResponse(::windows::core::IUnknown);
impl TextResponse {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for TextResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextResponse {}
impl ::core::fmt::Debug for TextResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.TextResponse;{e9cb74c3-2457-4cdb-8110-72f5e8e883e8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TextResponse {
    type Vtable = ITextResponse_Vtbl;
    const IID: ::windows::core::GUID = <ITextResponse as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.TextResponse";
}
impl ::core::convert::From<TextResponse> for ::windows::core::IUnknown {
    fn from(value: TextResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextResponse> for ::windows::core::IUnknown {
    fn from(value: &TextResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextResponse> for ::windows::core::IInspectable {
    fn from(value: TextResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextResponse> for ::windows::core::IInspectable {
    fn from(value: &TextResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct ToastNotificationTriggerDetails(::windows::core::IUnknown);
impl ToastNotificationTriggerDetails {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).TimeCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppDisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AccessoryNotificationType>::zeroed();
            (::windows::core::Interface::vtable(this).AccessoryNotificationType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessoryNotificationType>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).StartedProcessing)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartedProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Text1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Text1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Text2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Text2)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Text3(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Text3)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn Text4(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Text4)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SuppressPopup(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).SuppressPopup)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationTriggerDetails {}
impl ::core::fmt::Debug for ToastNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ToastNotificationTriggerDetails;{c9314895-4e6d-4e9d-afec-9e921b875ae8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IToastNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ToastNotificationTriggerDetails";
}
impl ::core::convert::From<ToastNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationTriggerDetails> for IAccessoryNotificationTriggerDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAccessoryNotificationTriggerDetails> for &ToastNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IAccessoryNotificationTriggerDetails> {
        ::core::convert::TryInto::<IAccessoryNotificationTriggerDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VibrateState(pub i32);
impl VibrateState {
    pub const RingerOffVibrateOff: Self = Self(0i32);
    pub const RingerOffVibrateOn: Self = Self(1i32);
    pub const RingerOnVibrateOff: Self = Self(2i32);
    pub const RingerOnVibrateOn: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrateState {}
impl ::core::clone::Clone for VibrateState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VibrateState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VibrateState {
    type Abi = Self;
}
impl ::core::fmt::Debug for VibrateState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrateState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VibrateState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.VibrateState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct VolumeInfo(::windows::core::IUnknown);
impl VolumeInfo {
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn SystemVolume(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).SystemVolume)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn CallVolume(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).CallVolume)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn MediaVolume(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).MediaVolume)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IsMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsMuted)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
    pub fn IsVibrateEnabled(&self) -> ::windows::core::Result<VibrateState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VibrateState>::zeroed();
            (::windows::core::Interface::vtable(this).IsVibrateEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VibrateState>(result__)
        }
    }
}
impl ::core::clone::Clone for VolumeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VolumeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VolumeInfo {}
impl ::core::fmt::Debug for VolumeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VolumeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VolumeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.VolumeInfo;{944dd118-7704-4481-b92e-d3ed3ece6322})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
    const IID: ::windows::core::GUID = <IVolumeInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.VolumeInfo";
}
impl ::core::convert::From<VolumeInfo> for ::windows::core::IUnknown {
    fn from(value: VolumeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VolumeInfo> for ::windows::core::IUnknown {
    fn from(value: &VolumeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VolumeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VolumeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VolumeInfo> for ::windows::core::IInspectable {
    fn from(value: VolumeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VolumeInfo> for ::windows::core::IInspectable {
    fn from(value: &VolumeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VolumeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VolumeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
