#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAccessoryManager {
    type Vtable = IAccessoryManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessoryManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d04a12c_883d_4aa7_bca7_fa4bb8bffee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RegisterAccessoryApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub MakePhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MakePhoneCallOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
    pub MakePhoneCallWithVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MakePhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT,
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
    pub EnableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsNotificationEnabledForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetEnabledAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub EnableAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessorynotificationtypes: i32) -> ::windows::core::HRESULT,
    pub DisableAllAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserConsent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAppIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAppIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAccessoryManager2 {
    type Vtable = IAccessoryManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessoryManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbacad44d_d393_46c6_b80c_15fdf44d5386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SpeedDialList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SpeedDialList: usize,
    pub ClearToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub GetFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFolders: usize,
    pub EnableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EnableEmailNotificationFolderFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: *mut ::core::ffi::c_void, folders: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnableEmailNotificationFolderFilter: usize,
    pub UpdateEmailReadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageentryid: *mut ::core::ffi::c_void, isread: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAccessoryManager3 {
    type Vtable = IAccessoryManager3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessoryManager3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81f75137_edc7_47e0_b2f7_7e577c833f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SnoozeAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DismissAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SnoozeReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DismissReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct IAccessoryNotificationTriggerDetails(::windows::core::IUnknown);
impl IAccessoryNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
::windows::core::interface_hierarchy!(IAccessoryNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Vtable for IAccessoryNotificationTriggerDetails {
    type Vtable = IAccessoryNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessoryNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6968a7d4_e3ca_49cb_8c87_2c11cdff9646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TimeCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeCreated: usize,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AccessoryNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows::core::HRESULT,
    pub StartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IAlarmNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38f5fa30_c738_4da2_908c_775d83c36abb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AlarmId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAlarmNotificationTriggerDetails2 {
    type Vtable = IAlarmNotificationTriggerDetails2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAlarmNotificationTriggerDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf16e06a_7155_40fe_a9c2_7bd2127ef853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2157bea5_e286_45d3_9bea_f790fc216e0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBinaryId(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBinaryId {
    type Vtable = IBinaryId_Vtbl;
}
unsafe impl ::windows::core::Interface for IBinaryId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f0da531_5595_44b4_9181_ce4efa3fc168);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinaryId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarChangedNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ICalendarChangedNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b8a3bfc_279d_42ab_9c68_3e87977bf216);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarChangedNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CalendarChangedEvent) -> ::windows::core::HRESULT,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICortanaTileNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ICortanaTileNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc0f01d5_1489_46bb_b73b_7f90067ecf27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaTileNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LargeContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LargeContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmphasizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NonWrappedSmallContent4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAccountInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailAccountInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfbc02ab_bda0_4568_927e_b2ede35818a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAccountInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailFolderInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailFolderInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc207150e_e237_46d6_90e6_4f529eeac1e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailFolderInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b82612_46cf_4e70_8e0d_7b2e04ab492b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SenderAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IEmailNotificationTriggerDetails2 {
    type Vtable = IEmailNotificationTriggerDetails2_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailNotificationTriggerDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x168067e3_c56f_4ec7_bed1_f734e08de5b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailReadNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailReadNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5b7a087_06f3_4e3e_8c42_325e67010413);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailReadNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaControlsTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaControlsTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfab4648b_ae45_4548_91ca_4ab0548e33b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControlsTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows::core::HRESULT,
    pub MediaMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaMetadata(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaMetadata {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b50ddf7_bb6c_4330_b3cd_0704a54cdb80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMetadata_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IPhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhoneCallDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c1b6f53_f071_483e_bf33_ebd44b724447);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresetTextResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresetTextResponses: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhoneLineDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47eb32dc_33ed_49b9_995c_a296bac82b77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DefaultOutgoingLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub RegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineRegistrationState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPhoneLineDetails2 {
    type Vtable = IPhoneLineDetails2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhoneLineDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb30cd77d_0147_498c_8241_bf0cabc60a25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MissedCallCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhoneNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccc2fdf7_09c3_4118_91bc_ca6323a8d383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PhoneNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneNotificationType) -> ::windows::core::HRESULT,
    pub CallDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhoneLineChangedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IReminderNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bddaa5d_9f61_4bf0_9feb_10502bc0b0c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ReminderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IReminderNotificationTriggerDetails2 {
    type Vtable = IReminderNotificationTriggerDetails2_Vtbl;
}
unsafe impl ::windows::core::Interface for IReminderNotificationTriggerDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe715f9c0_504d_4c0f_a6b3_bcb9722c6cdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeedDialEntry(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpeedDialEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9240b6db_872c_46dc_b62a_be4541b166f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeedDialEntry_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NumberType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextResponse {
    type Vtable = ITextResponse_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextResponse {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9cb74c3_2457_4cdb_8110_72f5e8e883e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextResponse_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9314895_4e6d_4e9d_afec_9e921b875ae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Text2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Text3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Text4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IToastNotificationTriggerDetails2 {
    type Vtable = IToastNotificationTriggerDetails2_Vtbl;
}
unsafe impl ::windows::core::Interface for IToastNotificationTriggerDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e0479dd_cac4_4f60_afa3_b925d9d83c93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVolumeInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IVolumeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x944dd118_7704_4481_b92e_d3ed3ece6322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SystemVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CallVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MediaVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVibrateEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VibrateState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
pub struct AccessoryManager;
impl AccessoryManager {
    pub fn RegisterAccessoryApp() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterAccessoryApp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetNextTriggerDetails() -> ::windows::core::Result<IAccessoryNotificationTriggerDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNextTriggerDetails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ProcessTriggerDetails<P0, E0>(pdetails: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IAccessoryNotificationTriggerDetails>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).ProcessTriggerDetails)(::windows::core::Vtable::as_raw(this), pdetails.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PhoneLineDetails() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneLineDetails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetPhoneLineDetails(phoneline: ::windows::core::GUID) -> ::windows::core::Result<PhoneLineDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPhoneLineDetails)(::windows::core::Vtable::as_raw(this), phoneline, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AcceptPhoneCall(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).AcceptPhoneCall)(::windows::core::Vtable::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallOnEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).AcceptPhoneCallOnEndpoint)(::windows::core::Vtable::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn AcceptPhoneCallWithVideo(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).AcceptPhoneCallWithVideo)(::windows::core::Vtable::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).AcceptPhoneCallWithVideoOnAudioEndpoint)(::windows::core::Vtable::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn RejectPhoneCall(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).RejectPhoneCall)(::windows::core::Vtable::as_raw(this), phonecallid).ok() })
    }
    pub fn RejectPhoneCallWithText(phonecallid: u32, textresponseid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).RejectPhoneCallWithText)(::windows::core::Vtable::as_raw(this), phonecallid, textresponseid).ok() })
    }
    pub fn MakePhoneCall(phoneline: ::windows::core::GUID, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).MakePhoneCall)(::windows::core::Vtable::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber)).ok() })
    }
    pub fn MakePhoneCallOnAudioEndpoint(phoneline: ::windows::core::GUID, phonenumber: &::windows::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).MakePhoneCallOnAudioEndpoint)(::windows::core::Vtable::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber), endpoint).ok() })
    }
    pub fn MakePhoneCallWithVideo(phoneline: ::windows::core::GUID, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).MakePhoneCallWithVideo)(::windows::core::Vtable::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber)).ok() })
    }
    pub fn MakePhoneCallWithVideoOnAudioEndpoint(phoneline: ::windows::core::GUID, phonenumber: &::windows::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).MakePhoneCallWithVideoOnAudioEndpoint)(::windows::core::Vtable::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber), endpoint).ok() })
    }
    pub fn SwapPhoneCalls(phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).SwapPhoneCalls)(::windows::core::Vtable::as_raw(this), phonecallidtohold, phonecallidonhold).ok() })
    }
    pub fn HoldPhoneCall(phonecallid: u32, holdcall: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).HoldPhoneCall)(::windows::core::Vtable::as_raw(this), phonecallid, holdcall).ok() })
    }
    pub fn EndPhoneCall(phonecallid: u32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).EndPhoneCall)(::windows::core::Vtable::as_raw(this), phonecallid).ok() })
    }
    pub fn SetPhoneMute(value: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).SetPhoneMute)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    pub fn PhoneMute() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneMute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetPhoneCallAudioEndpoint(value: PhoneCallAudioEndpoint) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).SetPhoneCallAudioEndpoint)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    pub fn PhoneCallAudioEndpoint() -> ::windows::core::Result<PhoneCallAudioEndpoint> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneCallAudioEndpoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SnoozeAlarm(alarmid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).SnoozeAlarm)(::windows::core::Vtable::as_raw(this), alarmid).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnoozeAlarmForSpecifiedTime(alarmid: ::windows::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).SnoozeAlarmForSpecifiedTime)(::windows::core::Vtable::as_raw(this), alarmid, timespan).ok() })
    }
    pub fn DismissAlarm(alarmid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).DismissAlarm)(::windows::core::Vtable::as_raw(this), alarmid).ok() })
    }
    pub fn SnoozeReminder(reminderid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).SnoozeReminder)(::windows::core::Vtable::as_raw(this), reminderid).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnoozeReminderForSpecifiedTime(reminderid: ::windows::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).SnoozeReminderForSpecifiedTime)(::windows::core::Vtable::as_raw(this), reminderid, timespan).ok() })
    }
    pub fn DismissReminder(reminderid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).DismissReminder)(::windows::core::Vtable::as_raw(this), reminderid).ok() })
    }
    pub fn GetMediaMetadata() -> ::windows::core::Result<MediaMetadata> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMediaMetadata)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MediaPlaybackCapabilities() -> ::windows::core::Result<PlaybackCapability> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaPlaybackCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MediaPlaybackStatus() -> ::windows::core::Result<PlaybackStatus> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaPlaybackStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PerformMediaPlaybackCommand(command: PlaybackCommand) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).PerformMediaPlaybackCommand)(::windows::core::Vtable::as_raw(this), command).ok() })
    }
    pub fn DoNotDisturbEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DoNotDisturbEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DrivingModeEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DrivingModeEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn BatterySaverState() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BatterySaverState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetApps() -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppNotificationInfo>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetApps)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EnableNotificationsForApplication(appid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).EnableNotificationsForApplication)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appid)).ok() })
    }
    pub fn DisableNotificationsForApplication(appid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).DisableNotificationsForApplication)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appid)).ok() })
    }
    pub fn IsNotificationEnabledForApplication(appid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsNotificationEnabledForApplication)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetEnabledAccessoryNotificationTypes() -> ::windows::core::Result<i32> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEnabledAccessoryNotificationTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EnableAccessoryNotificationTypes(accessorynotificationtypes: i32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).EnableAccessoryNotificationTypes)(::windows::core::Vtable::as_raw(this), accessorynotificationtypes).ok() })
    }
    pub fn DisableAllAccessoryNotificationTypes() -> ::windows::core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows::core::Vtable::vtable(this).DisableAllAccessoryNotificationTypes)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    pub fn GetUserConsent() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUserConsent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetAppIcon(appid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAppIcon)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RingDevice() -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).RingDevice)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SpeedDialList() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpeedDialList)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ClearToast(instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).ClearToast)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn IsPhonePinLocked() -> ::windows::core::Result<bool> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPhonePinLocked)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IncreaseVolume(step: i32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).IncreaseVolume)(::windows::core::Vtable::as_raw(this), step).ok() })
    }
    pub fn DecreaseVolume(step: i32) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).DecreaseVolume)(::windows::core::Vtable::as_raw(this), step).ok() })
    }
    pub fn SetMute(mute: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).SetMute)(::windows::core::Vtable::as_raw(this), mute).ok() })
    }
    pub fn SetRingerVibrate(ringer: bool, vibrate: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).SetRingerVibrate)(::windows::core::Vtable::as_raw(this), ringer, vibrate).ok() })
    }
    pub fn VolumeInfo() -> ::windows::core::Result<VolumeInfo> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VolumeInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllEmailAccounts() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAllEmailAccounts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFolders(emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFolders)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(emailaccount), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EnableEmailNotificationEmailAccount(emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).EnableEmailNotificationEmailAccount)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(emailaccount)).ok() })
    }
    pub fn DisableEmailNotificationEmailAccount(emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).DisableEmailNotificationEmailAccount)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(emailaccount)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnableEmailNotificationFolderFilter<P0, E0>(emailaccount: &::windows::core::HSTRING, folders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).EnableEmailNotificationFolderFilter)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(emailaccount), folders.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    pub fn UpdateEmailReadStatus(messageentryid: &BinaryId, isread: bool) -> ::windows::core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows::core::Vtable::vtable(this).UpdateEmailReadStatus)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messageentryid), isread).ok() })
    }
    pub fn SnoozeAlarmByInstanceId(instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Vtable::vtable(this).SnoozeAlarmByInstanceId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn DismissAlarmByInstanceId(instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Vtable::vtable(this).DismissAlarmByInstanceId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn SnoozeReminderByInstanceId(instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Vtable::vtable(this).SnoozeReminderByInstanceId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn DismissReminderByInstanceId(instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows::core::Vtable::vtable(this).DismissReminderByInstanceId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
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
pub struct AlarmNotificationTriggerDetails(::windows::core::IUnknown);
impl AlarmNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AlarmId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlarmId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReminderState(&self) -> ::windows::core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReminderState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAlarmNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstanceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for AlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for AlarmNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IAlarmNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(AlarmNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&AlarmNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AlarmNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct AppNotificationInfo(::windows::core::IUnknown);
impl AppNotificationInfo {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for AppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationInfo {
    const IID: ::windows::core::GUID = <IAppNotificationInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AppNotificationInfo";
}
::windows::core::interface_hierarchy!(AppNotificationInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct BinaryId(::windows::core::IUnknown);
impl BinaryId {
    pub fn Id(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for BinaryId {
    type Vtable = IBinaryId_Vtbl;
}
unsafe impl ::windows::core::Interface for BinaryId {
    const IID: ::windows::core::GUID = <IBinaryId as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.BinaryId";
}
::windows::core::interface_hierarchy!(BinaryId, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct CalendarChangedNotificationTriggerDetails(::windows::core::IUnknown);
impl CalendarChangedNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn EventType(&self) -> ::windows::core::Result<CalendarChangedEvent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EventType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for CalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for CalendarChangedNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <ICalendarChangedNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(CalendarChangedNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&CalendarChangedNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CalendarChangedNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct CortanaTileNotificationTriggerDetails(::windows::core::IUnknown);
impl CortanaTileNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TileId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LargeContent1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LargeContent1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LargeContent2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LargeContent2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmphasizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmphasizedText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NonWrappedSmallContent1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NonWrappedSmallContent2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent3(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NonWrappedSmallContent3)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent4(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NonWrappedSmallContent4)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Source)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for CortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for CortanaTileNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <ICortanaTileNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(CortanaTileNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&CortanaTileNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CortanaTileNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailAccountInfo(::windows::core::IUnknown);
impl EmailAccountInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsNotificationEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for EmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailAccountInfo {
    const IID: ::windows::core::GUID = <IEmailAccountInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailAccountInfo";
}
::windows::core::interface_hierarchy!(EmailAccountInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailFolderInfo(::windows::core::IUnknown);
impl EmailFolderInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsNotificationEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for EmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailFolderInfo {
    const IID: ::windows::core::GUID = <IEmailFolderInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailFolderInfo";
}
::windows::core::interface_hierarchy!(EmailFolderInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailNotificationTriggerDetails(::windows::core::IUnknown);
impl EmailNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentFolderName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SenderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SenderName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SenderAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SenderAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
    #[cfg(feature = "ApplicationModel_Email")]
    pub fn EmailMessage(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Email::EmailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId> {
        let this = &::windows::core::Interface::cast::<IEmailNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageEntryId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for EmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IEmailNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(EmailNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&EmailNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &EmailNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailReadNotificationTriggerDetails(::windows::core::IUnknown);
impl EmailReadNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentFolderName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageEntryId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for EmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailReadNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IEmailReadNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(EmailReadNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&EmailReadNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &EmailReadNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct MediaControlsTriggerDetails(::windows::core::IUnknown);
impl MediaControlsTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PlaybackStatus(&self) -> ::windows::core::Result<PlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MediaMetadata(&self) -> ::windows::core::Result<MediaMetadata> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaMetadata)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for MediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaControlsTriggerDetails {
    const IID: ::windows::core::GUID = <IMediaControlsTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaControlsTriggerDetails";
}
::windows::core::interface_hierarchy!(MediaControlsTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&MediaControlsTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaControlsTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct MediaMetadata(::windows::core::IUnknown);
impl MediaMetadata {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subtitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Artist)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Album)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Track(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Thumbnail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for MediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaMetadata {
    const IID: ::windows::core::GUID = <IMediaMetadata as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaMetadata";
}
::windows::core::interface_hierarchy!(MediaMetadata, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneCallDetails(::windows::core::IUnknown);
impl PhoneCallDetails {
    pub fn PhoneLine(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CallId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CallId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CallTransport(&self) -> ::windows::core::Result<PhoneCallTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CallTransport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CallMediaType(&self) -> ::windows::core::Result<PhoneMediaType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CallMediaType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CallDirection(&self) -> ::windows::core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CallDirection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<PhoneCallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConferenceCallId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConferenceCallId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EndTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresetTextResponses(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PresetTextResponses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for PhoneCallDetails {
    const IID: ::windows::core::GUID = <IPhoneCallDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneCallDetails";
}
::windows::core::interface_hierarchy!(PhoneCallDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneLineDetails(::windows::core::IUnknown);
impl PhoneLineDetails {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LineNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DefaultOutgoingLine(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultOutgoingLine)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn VoicemailCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VoicemailCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RegistrationState(&self) -> ::windows::core::Result<PhoneLineRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegistrationState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MissedCallCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPhoneLineDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MissedCallCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for PhoneLineDetails {
    const IID: ::windows::core::GUID = <IPhoneLineDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneLineDetails";
}
::windows::core::interface_hierarchy!(PhoneLineDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneNotificationTriggerDetails(::windows::core::IUnknown);
impl PhoneNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PhoneNotificationType(&self) -> ::windows::core::Result<PhoneNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CallDetails(&self) -> ::windows::core::Result<PhoneCallDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CallDetails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhoneLineChangedId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneLineChangedId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for PhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for PhoneNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IPhoneNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(PhoneNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&PhoneNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct ReminderNotificationTriggerDetails(::windows::core::IUnknown);
impl ReminderNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ReminderId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReminderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Details)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn Appointment(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Appointment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReminderState(&self) -> ::windows::core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReminderState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IReminderNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstanceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for ReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ReminderNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IReminderNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(ReminderNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&ReminderNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReminderNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct SpeedDialEntry(::windows::core::IUnknown);
impl SpeedDialEntry {
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NumberType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumberType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for SpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for SpeedDialEntry {
    const IID: ::windows::core::GUID = <ISpeedDialEntry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.SpeedDialEntry";
}
::windows::core::interface_hierarchy!(SpeedDialEntry, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct TextResponse(::windows::core::IUnknown);
impl TextResponse {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for TextResponse {
    type Vtable = ITextResponse_Vtbl;
}
unsafe impl ::windows::core::Interface for TextResponse {
    const IID: ::windows::core::GUID = <ITextResponse as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.TextResponse";
}
::windows::core::interface_hierarchy!(TextResponse, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct ToastNotificationTriggerDetails(::windows::core::IUnknown);
impl ToastNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimeCreated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessoryNotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartedProcessing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartedProcessing)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Text1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text3(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text3)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text4(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text4)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SuppressPopup(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuppressPopup)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IToastNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstanceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for ToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ToastNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IToastNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ToastNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(ToastNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::core::convert::TryFrom<&ToastNotificationTriggerDetails> for ::windows::core::InParam<IAccessoryNotificationTriggerDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationTriggerDetails) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct VolumeInfo(::windows::core::IUnknown);
impl VolumeInfo {
    pub fn SystemVolume(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemVolume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CallVolume(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CallVolume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MediaVolume(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaVolume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMuted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsVibrateEnabled(&self) -> ::windows::core::Result<VibrateState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVibrateEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl ::windows::core::Vtable for VolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for VolumeInfo {
    const IID: ::windows::core::GUID = <IVolumeInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.VolumeInfo";
}
::windows::core::interface_hierarchy!(VolumeInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
