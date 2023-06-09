#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessoryManager {
    type Vtable = IAccessoryManager_Vtbl;
}
impl ::core::clone::Clone for IAccessoryManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAccessoryManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d04a12c_883d_4aa7_bca7_fa4bb8bffee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RegisterAccessoryApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNextTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetails: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PhoneLineDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PhoneLineDetails: usize,
    pub GetPhoneLineDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub AcceptPhoneCallOnEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub AcceptPhoneCallWithVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub AcceptPhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub RejectPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub RejectPhoneCallWithText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, textresponseid: u32) -> ::windows_core::HRESULT,
    pub MakePhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MakePhoneCallOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub MakePhoneCallWithVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MakePhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneline: ::windows_core::GUID, phonenumber: ::std::mem::MaybeUninit<::windows_core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub SwapPhoneCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows_core::HRESULT,
    pub HoldPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32, holdcall: bool) -> ::windows_core::HRESULT,
    pub EndPhoneCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows_core::HRESULT,
    pub SetPhoneMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PhoneMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub PhoneCallAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallAudioEndpoint) -> ::windows_core::HRESULT,
    pub SnoozeAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SnoozeAlarmForSpecifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows_core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeAlarmForSpecifiedTime: usize,
    pub DismissAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alarmid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SnoozeReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SnoozeReminderForSpecifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows_core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnoozeReminderForSpecifiedTime: usize,
    pub DismissReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reminderid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetMediaMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MediaPlaybackCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackCapability) -> ::windows_core::HRESULT,
    pub MediaPlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows_core::HRESULT,
    pub PerformMediaPlaybackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: PlaybackCommand) -> ::windows_core::HRESULT,
    pub DoNotDisturbEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DrivingModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub BatterySaverState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetApps: usize,
    pub EnableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisableNotificationsForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsNotificationEnabledForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetEnabledAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub EnableAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessorynotificationtypes: i32) -> ::windows_core::HRESULT,
    pub DisableAllAccessoryNotificationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetUserConsent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAppIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAppIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessoryManager2 {
    type Vtable = IAccessoryManager2_Vtbl;
}
impl ::core::clone::Clone for IAccessoryManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAccessoryManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbacad44d_d393_46c6_b80c_15fdf44d5386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SpeedDialList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SpeedDialList: usize,
    pub ClearToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPhonePinLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IncreaseVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, step: i32) -> ::windows_core::HRESULT,
    pub DecreaseVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, step: i32) -> ::windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mute: bool) -> ::windows_core::HRESULT,
    pub SetRingerVibrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ringer: bool, vibrate: bool) -> ::windows_core::HRESULT,
    pub VolumeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllEmailAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllEmailAccounts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFolders: usize,
    pub EnableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisableEmailNotificationEmailAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EnableEmailNotificationFolderFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emailaccount: ::std::mem::MaybeUninit<::windows_core::HSTRING>, folders: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnableEmailNotificationFolderFilter: usize,
    pub UpdateEmailReadStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageentryid: *mut ::core::ffi::c_void, isread: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessoryManager3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessoryManager3 {
    type Vtable = IAccessoryManager3_Vtbl;
}
impl ::core::clone::Clone for IAccessoryManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAccessoryManager3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81f75137_edc7_47e0_b2f7_7e577c833f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryManager3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SnoozeAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DismissAlarmByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SnoozeReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DismissReminderByInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instanceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct IAccessoryNotificationTriggerDetails(::windows_core::IUnknown);
impl IAccessoryNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IAccessoryNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IAccessoryNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6968a7d4-e3ca-49cb-8c87-2c11cdff9646}");
}
unsafe impl ::windows_core::Interface for IAccessoryNotificationTriggerDetails {
    type Vtable = IAccessoryNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IAccessoryNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAccessoryNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6968a7d4_e3ca_49cb_8c87_2c11cdff9646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessoryNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TimeCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeCreated: usize,
    pub AppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccessoryNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows_core::HRESULT,
    pub StartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStartedProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IAlarmNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAlarmNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38f5fa30_c738_4da2_908c_775d83c36abb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlarmId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAlarmNotificationTriggerDetails2 {
    type Vtable = IAlarmNotificationTriggerDetails2_Vtbl;
}
impl ::core::clone::Clone for IAlarmNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAlarmNotificationTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf16e06a_7155_40fe_a9c2_7bd2127ef853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
}
impl ::core::clone::Clone for IAppNotificationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppNotificationInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2157bea5_e286_45d3_9bea_f790fc216e0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBinaryId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBinaryId {
    type Vtable = IBinaryId_Vtbl;
}
impl ::core::clone::Clone for IBinaryId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBinaryId {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f0da531_5595_44b4_9181_ce4efa3fc168);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinaryId_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarChangedNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for ICalendarChangedNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICalendarChangedNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b8a3bfc_279d_42ab_9c68_3e87977bf216);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarChangedNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CalendarChangedEvent) -> ::windows_core::HRESULT,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICortanaTileNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for ICortanaTileNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICortanaTileNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc0f01d5_1489_46bb_b73b_7f90067ecf27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICortanaTileNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LargeContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LargeContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EmphasizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NonWrappedSmallContent4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAccountInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
}
impl ::core::clone::Clone for IEmailAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEmailAccountInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfbc02ab_bda0_4568_927e_b2ede35818a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAccountInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailFolderInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
}
impl ::core::clone::Clone for IEmailFolderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEmailFolderInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc207150e_e237_46d6_90e6_4f529eeac1e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailFolderInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IEmailNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEmailNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3b82612_46cf_4e70_8e0d_7b2e04ab492b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SenderAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Email")]
    pub EmailMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Email"))]
    EmailMessage: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailNotificationTriggerDetails2 {
    type Vtable = IEmailNotificationTriggerDetails2_Vtbl;
}
impl ::core::clone::Clone for IEmailNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEmailNotificationTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x168067e3_c56f_4ec7_bed1_f734e08de5b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailReadNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IEmailReadNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEmailReadNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5b7a087_06f3_4e3e_8c42_325e67010413);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailReadNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MessageEntryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaControlsTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IMediaControlsTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaControlsTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfab4648b_ae45_4548_91ca_4ab0548e33b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControlsTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows_core::HRESULT,
    pub MediaMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
}
impl ::core::clone::Clone for IMediaMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaMetadata {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b50ddf7_bb6c_4330_b3cd_0704a54cdb80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMetadata_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Track: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneCallDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneCallDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c1b6f53_f071_483e_bf33_ebd44b724447);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PhoneLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CallTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallTransport) -> ::windows_core::HRESULT,
    pub CallMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneMediaType) -> ::windows_core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallDirection) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallState) -> ::windows_core::HRESULT,
    pub ConferenceCallId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresetTextResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresetTextResponses: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47eb32dc_33ed_49b9_995c_a296bac82b77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultOutgoingLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub RegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineRegistrationState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneLineDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneLineDetails2 {
    type Vtable = IPhoneLineDetails2_Vtbl;
}
impl ::core::clone::Clone for IPhoneLineDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneLineDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb30cd77d_0147_498c_8241_bf0cabc60a25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneLineDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MissedCallCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPhoneNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhoneNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccc2fdf7_09c3_4118_91bc_ca6323a8d383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PhoneNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhoneNotificationType) -> ::windows_core::HRESULT,
    pub CallDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PhoneLineChangedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IReminderNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IReminderNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bddaa5d_9f61_4bf0_9feb_10502bc0b0c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReminderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Appointment: usize,
    pub ReminderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReminderNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReminderNotificationTriggerDetails2 {
    type Vtable = IReminderNotificationTriggerDetails2_Vtbl;
}
impl ::core::clone::Clone for IReminderNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IReminderNotificationTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe715f9c0_504d_4c0f_a6b3_bcb9722c6cdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReminderNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeedDialEntry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
}
impl ::core::clone::Clone for ISpeedDialEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpeedDialEntry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9240b6db_872c_46dc_b62a_be4541b166f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeedDialEntry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumberType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextResponse {
    type Vtable = ITextResponse_Vtbl;
}
impl ::core::clone::Clone for ITextResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITextResponse {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9cb74c3_2457_4cdb_8110_72f5e8e883e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IToastNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IToastNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9314895_4e6d_4e9d_afec_9e921b875ae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationTriggerDetails2 {
    type Vtable = IToastNotificationTriggerDetails2_Vtbl;
}
impl ::core::clone::Clone for IToastNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IToastNotificationTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e0479dd_cac4_4f60_afa3_b925d9d83c93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVolumeInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
}
impl ::core::clone::Clone for IVolumeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVolumeInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x944dd118_7704_4481_b92e_d3ed3ece6322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SystemVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CallVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MediaVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVibrateEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VibrateState) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
pub struct AccessoryManager;
impl AccessoryManager {
    pub fn RegisterAccessoryApp() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterAccessoryApp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetNextTriggerDetails() -> ::windows_core::Result<IAccessoryNotificationTriggerDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNextTriggerDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProcessTriggerDetails<P0>(pdetails: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IAccessoryNotificationTriggerDetails>,
    {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).ProcessTriggerDetails)(::windows_core::Interface::as_raw(this), pdetails.try_into_param()?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PhoneLineDetails() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneLineDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetPhoneLineDetails(phoneline: ::windows_core::GUID) -> ::windows_core::Result<PhoneLineDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPhoneLineDetails)(::windows_core::Interface::as_raw(this), phoneline, &mut result__).from_abi(result__)
        })
    }
    pub fn AcceptPhoneCall(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallOnEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCallOnEndpoint)(::windows_core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn AcceptPhoneCallWithVideo(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCallWithVideo)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).AcceptPhoneCallWithVideoOnAudioEndpoint)(::windows_core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn RejectPhoneCall(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).RejectPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn RejectPhoneCallWithText(phonecallid: u32, textresponseid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).RejectPhoneCallWithText)(::windows_core::Interface::as_raw(this), phonecallid, textresponseid).ok() })
    }
    pub fn MakePhoneCall(phoneline: ::windows_core::GUID, phonenumber: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCall)(::windows_core::Interface::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber)).ok() })
    }
    pub fn MakePhoneCallOnAudioEndpoint(phoneline: ::windows_core::GUID, phonenumber: &::windows_core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCallOnAudioEndpoint)(::windows_core::Interface::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber), endpoint).ok() })
    }
    pub fn MakePhoneCallWithVideo(phoneline: ::windows_core::GUID, phonenumber: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCallWithVideo)(::windows_core::Interface::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber)).ok() })
    }
    pub fn MakePhoneCallWithVideoOnAudioEndpoint(phoneline: ::windows_core::GUID, phonenumber: &::windows_core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).MakePhoneCallWithVideoOnAudioEndpoint)(::windows_core::Interface::as_raw(this), phoneline, ::core::mem::transmute_copy(phonenumber), endpoint).ok() })
    }
    pub fn SwapPhoneCalls(phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SwapPhoneCalls)(::windows_core::Interface::as_raw(this), phonecallidtohold, phonecallidonhold).ok() })
    }
    pub fn HoldPhoneCall(phonecallid: u32, holdcall: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).HoldPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid, holdcall).ok() })
    }
    pub fn EndPhoneCall(phonecallid: u32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).EndPhoneCall)(::windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn SetPhoneMute(value: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SetPhoneMute)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PhoneMute() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneMute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetPhoneCallAudioEndpoint(value: PhoneCallAudioEndpoint) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SetPhoneCallAudioEndpoint)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PhoneCallAudioEndpoint() -> ::windows_core::Result<PhoneCallAudioEndpoint> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneCallAudioEndpoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SnoozeAlarm(alarmid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeAlarm)(::windows_core::Interface::as_raw(this), alarmid).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnoozeAlarmForSpecifiedTime(alarmid: ::windows_core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeAlarmForSpecifiedTime)(::windows_core::Interface::as_raw(this), alarmid, timespan).ok() })
    }
    pub fn DismissAlarm(alarmid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DismissAlarm)(::windows_core::Interface::as_raw(this), alarmid).ok() })
    }
    pub fn SnoozeReminder(reminderid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeReminder)(::windows_core::Interface::as_raw(this), reminderid).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnoozeReminderForSpecifiedTime(reminderid: ::windows_core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeReminderForSpecifiedTime)(::windows_core::Interface::as_raw(this), reminderid, timespan).ok() })
    }
    pub fn DismissReminder(reminderid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DismissReminder)(::windows_core::Interface::as_raw(this), reminderid).ok() })
    }
    pub fn GetMediaMetadata() -> ::windows_core::Result<MediaMetadata> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMediaMetadata)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MediaPlaybackCapabilities() -> ::windows_core::Result<PlaybackCapability> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlaybackCapabilities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MediaPlaybackStatus() -> ::windows_core::Result<PlaybackStatus> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlaybackStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PerformMediaPlaybackCommand(command: PlaybackCommand) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).PerformMediaPlaybackCommand)(::windows_core::Interface::as_raw(this), command).ok() })
    }
    pub fn DoNotDisturbEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DoNotDisturbEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DrivingModeEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DrivingModeEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn BatterySaverState() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BatterySaverState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetApps() -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, AppNotificationInfo>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetApps)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn EnableNotificationsForApplication(appid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).EnableNotificationsForApplication)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appid)).ok() })
    }
    pub fn DisableNotificationsForApplication(appid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DisableNotificationsForApplication)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appid)).ok() })
    }
    pub fn IsNotificationEnabledForApplication(appid: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNotificationEnabledForApplication)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetEnabledAccessoryNotificationTypes() -> ::windows_core::Result<i32> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnabledAccessoryNotificationTypes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn EnableAccessoryNotificationTypes(accessorynotificationtypes: i32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).EnableAccessoryNotificationTypes)(::windows_core::Interface::as_raw(this), accessorynotificationtypes).ok() })
    }
    pub fn DisableAllAccessoryNotificationTypes() -> ::windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (::windows_core::Interface::vtable(this).DisableAllAccessoryNotificationTypes)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn GetUserConsent() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUserConsent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetAppIcon(appid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppIcon)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appid), &mut result__).from_abi(result__)
        })
    }
    pub fn RingDevice() -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).RingDevice)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SpeedDialList() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpeedDialList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ClearToast(instanceid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).ClearToast)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn IsPhonePinLocked() -> ::windows_core::Result<bool> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPhonePinLocked)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IncreaseVolume(step: i32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).IncreaseVolume)(::windows_core::Interface::as_raw(this), step).ok() })
    }
    pub fn DecreaseVolume(step: i32) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).DecreaseVolume)(::windows_core::Interface::as_raw(this), step).ok() })
    }
    pub fn SetMute(mute: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).SetMute)(::windows_core::Interface::as_raw(this), mute).ok() })
    }
    pub fn SetRingerVibrate(ringer: bool, vibrate: bool) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).SetRingerVibrate)(::windows_core::Interface::as_raw(this), ringer, vibrate).ok() })
    }
    pub fn VolumeInfo() -> ::windows_core::Result<VolumeInfo> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VolumeInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllEmailAccounts() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllEmailAccounts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFolders(emailaccount: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFolders)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(emailaccount), &mut result__).from_abi(result__)
        })
    }
    pub fn EnableEmailNotificationEmailAccount(emailaccount: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).EnableEmailNotificationEmailAccount)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(emailaccount)).ok() })
    }
    pub fn DisableEmailNotificationEmailAccount(emailaccount: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).DisableEmailNotificationEmailAccount)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(emailaccount)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnableEmailNotificationFolderFilter<P0>(emailaccount: &::windows_core::HSTRING, folders: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
    {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).EnableEmailNotificationFolderFilter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(emailaccount), folders.try_into_param()?.abi()).ok() })
    }
    pub fn UpdateEmailReadStatus<P0>(messageentryid: P0, isread: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<BinaryId>,
    {
        Self::IAccessoryManager2(|this| unsafe { (::windows_core::Interface::vtable(this).UpdateEmailReadStatus)(::windows_core::Interface::as_raw(this), messageentryid.into_param().abi(), isread).ok() })
    }
    pub fn SnoozeAlarmByInstanceId(instanceid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeAlarmByInstanceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn DismissAlarmByInstanceId(instanceid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).DismissAlarmByInstanceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn SnoozeReminderByInstanceId(instanceid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).SnoozeReminderByInstanceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn DismissReminderByInstanceId(instanceid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (::windows_core::Interface::vtable(this).DismissReminderByInstanceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(instanceid)).ok() })
    }
    #[doc(hidden)]
    pub fn IAccessoryManager<R, F: FnOnce(&IAccessoryManager) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AccessoryManager, IAccessoryManager> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccessoryManager2<R, F: FnOnce(&IAccessoryManager2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AccessoryManager, IAccessoryManager2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccessoryManager3<R, F: FnOnce(&IAccessoryManager3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AccessoryManager, IAccessoryManager3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AccessoryManager {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AccessoryManager";
}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct AlarmNotificationTriggerDetails(::windows_core::IUnknown);
impl AlarmNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlarmId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlarmId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReminderState(&self) -> ::windows_core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReminderState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAlarmNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AlarmNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails;{38f5fa30-c738-4da2-908c-775d83c36abb})");
}
impl ::core::clone::Clone for AlarmNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AlarmNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IAlarmNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(AlarmNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for AlarmNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct AppNotificationInfo(::windows_core::IUnknown);
impl AppNotificationInfo {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppNotificationInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.AppNotificationInfo;{2157bea5-e286-45d3-9bea-f790fc216e0e})");
}
impl ::core::clone::Clone for AppNotificationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationInfo {
    const IID: ::windows_core::GUID = <IAppNotificationInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AppNotificationInfo";
}
::windows_core::imp::interface_hierarchy!(AppNotificationInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct BinaryId(::windows_core::IUnknown);
impl BinaryId {
    pub fn Id(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for BinaryId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.BinaryId;{4f0da531-5595-44b4-9181-ce4efa3fc168})");
}
impl ::core::clone::Clone for BinaryId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BinaryId {
    type Vtable = IBinaryId_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BinaryId {
    const IID: ::windows_core::GUID = <IBinaryId as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.BinaryId";
}
::windows_core::imp::interface_hierarchy!(BinaryId, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct CalendarChangedNotificationTriggerDetails(::windows_core::IUnknown);
impl CalendarChangedNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EventType(&self) -> ::windows_core::Result<CalendarChangedEvent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EventType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for CalendarChangedNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails;{4b8a3bfc-279d-42ab-9c68-3e87977bf216})");
}
impl ::core::clone::Clone for CalendarChangedNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CalendarChangedNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <ICalendarChangedNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(CalendarChangedNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for CalendarChangedNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct CortanaTileNotificationTriggerDetails(::windows_core::IUnknown);
impl CortanaTileNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LargeContent1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LargeContent1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LargeContent2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LargeContent2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EmphasizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EmphasizedText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent3(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NonWrappedSmallContent4(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NonWrappedSmallContent4)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for CortanaTileNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails;{dc0f01d5-1489-46bb-b73b-7f90067ecf27})");
}
impl ::core::clone::Clone for CortanaTileNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CortanaTileNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <ICortanaTileNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(CortanaTileNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for CortanaTileNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailAccountInfo(::windows_core::IUnknown);
impl EmailAccountInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsNotificationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNotificationEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for EmailAccountInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailAccountInfo;{dfbc02ab-bda0-4568-927e-b2ede35818a1})");
}
impl ::core::clone::Clone for EmailAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EmailAccountInfo {
    const IID: ::windows_core::GUID = <IEmailAccountInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailAccountInfo";
}
::windows_core::imp::interface_hierarchy!(EmailAccountInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailFolderInfo(::windows_core::IUnknown);
impl EmailFolderInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsNotificationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNotificationEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for EmailFolderInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailFolderInfo;{c207150e-e237-46d6-90e6-4f529eeac1e2})");
}
impl ::core::clone::Clone for EmailFolderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EmailFolderInfo {
    const IID: ::windows_core::GUID = <IEmailFolderInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailFolderInfo";
}
::windows_core::imp::interface_hierarchy!(EmailFolderInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailNotificationTriggerDetails(::windows_core::IUnknown);
impl EmailNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentFolderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentFolderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SenderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SenderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SenderAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SenderAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
    #[cfg(feature = "ApplicationModel_Email")]
    pub fn EmailMessage(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Email::EmailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EmailMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageEntryId(&self) -> ::windows_core::Result<BinaryId> {
        let this = &::windows_core::ComInterface::cast::<IEmailNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MessageEntryId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for EmailNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailNotificationTriggerDetails;{f3b82612-46cf-4e70-8e0d-7b2e04ab492b})");
}
impl ::core::clone::Clone for EmailNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EmailNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IEmailNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(EmailNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for EmailNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct EmailReadNotificationTriggerDetails(::windows_core::IUnknown);
impl EmailReadNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentFolderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentFolderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageEntryId(&self) -> ::windows_core::Result<BinaryId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MessageEntryId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for EmailReadNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails;{f5b7a087-06f3-4e3e-8c42-325e67010413})");
}
impl ::core::clone::Clone for EmailReadNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EmailReadNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IEmailReadNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(EmailReadNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for EmailReadNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct MediaControlsTriggerDetails(::windows_core::IUnknown);
impl MediaControlsTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackStatus(&self) -> ::windows_core::Result<PlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaMetadata(&self) -> ::windows_core::Result<MediaMetadata> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaMetadata)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MediaControlsTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaControlsTriggerDetails;{fab4648b-ae45-4548-91ca-4ab0548e33b5})");
}
impl ::core::clone::Clone for MediaControlsTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaControlsTriggerDetails {
    const IID: ::windows_core::GUID = <IMediaControlsTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaControlsTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(MediaControlsTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for MediaControlsTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct MediaMetadata(::windows_core::IUnknown);
impl MediaMetadata {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Artist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Artist)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Album(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Album)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Track(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Track)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MediaMetadata {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.MediaMetadata;{9b50ddf7-bb6c-4330-b3cd-0704a54cdb80})");
}
impl ::core::clone::Clone for MediaMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaMetadata {
    const IID: ::windows_core::GUID = <IMediaMetadata as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaMetadata";
}
::windows_core::imp::interface_hierarchy!(MediaMetadata, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneCallDetails(::windows_core::IUnknown);
impl PhoneCallDetails {
    pub fn PhoneLine(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneLine)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallTransport(&self) -> ::windows_core::Result<PhoneCallTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallTransport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallMediaType(&self) -> ::windows_core::Result<PhoneMediaType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallMediaType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallDirection(&self) -> ::windows_core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallDirection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<PhoneCallState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConferenceCallId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConferenceCallId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContactName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresetTextResponses(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PresetTextResponses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PhoneCallDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneCallDetails;{0c1b6f53-f071-483e-bf33-ebd44b724447})");
}
impl ::core::clone::Clone for PhoneCallDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallDetails {
    const IID: ::windows_core::GUID = <IPhoneCallDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneCallDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneCallDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneLineDetails(::windows_core::IUnknown);
impl PhoneLineDetails {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LineNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultOutgoingLine(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultOutgoingLine)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VoicemailCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VoicemailCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegistrationState(&self) -> ::windows_core::Result<PhoneLineRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegistrationState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MissedCallCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IPhoneLineDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MissedCallCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PhoneLineDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneLineDetails;{47eb32dc-33ed-49b9-995c-a296bac82b77})");
}
impl ::core::clone::Clone for PhoneLineDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneLineDetails {
    const IID: ::windows_core::GUID = <IPhoneLineDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneLineDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneLineDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct PhoneNotificationTriggerDetails(::windows_core::IUnknown);
impl PhoneNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhoneNotificationType(&self) -> ::windows_core::Result<PhoneNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallDetails(&self) -> ::windows_core::Result<PhoneCallDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhoneLineChangedId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneLineChangedId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PhoneNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails;{ccc2fdf7-09c3-4118-91bc-ca6323a8d383})");
}
impl ::core::clone::Clone for PhoneNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IPhoneNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(PhoneNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for PhoneNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct ReminderNotificationTriggerDetails(::windows_core::IUnknown);
impl ReminderNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReminderId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReminderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Details(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn Appointment(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Appointment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReminderState(&self) -> ::windows_core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReminderState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IReminderNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ReminderNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails;{5bddaa5d-9f61-4bf0-9feb-10502bc0b0c2})");
}
impl ::core::clone::Clone for ReminderNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ReminderNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IReminderNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(ReminderNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for ReminderNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct SpeedDialEntry(::windows_core::IUnknown);
impl SpeedDialEntry {
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContactName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpeedDialEntry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.SpeedDialEntry;{9240b6db-872c-46dc-b62a-be4541b166f8})");
}
impl ::core::clone::Clone for SpeedDialEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeedDialEntry {
    const IID: ::windows_core::GUID = <ISpeedDialEntry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.SpeedDialEntry";
}
::windows_core::imp::interface_hierarchy!(SpeedDialEntry, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct TextResponse(::windows_core::IUnknown);
impl TextResponse {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for TextResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.TextResponse;{e9cb74c3-2457-4cdb-8110-72f5e8e883e8})");
}
impl ::core::clone::Clone for TextResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TextResponse {
    type Vtable = ITextResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TextResponse {
    const IID: ::windows_core::GUID = <ITextResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.TextResponse";
}
::windows_core::imp::interface_hierarchy!(TextResponse, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct ToastNotificationTriggerDetails(::windows_core::IUnknown);
impl ToastNotificationTriggerDetails {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeCreated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessoryNotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartedProcessing(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartedProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartedProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Text1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text3(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text4(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text4)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SuppressPopup(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuppressPopup)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IToastNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for ToastNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.ToastNotificationTriggerDetails;{c9314895-4e6d-4e9d-afec-9e921b875ae8})");
}
impl ::core::clone::Clone for ToastNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ToastNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IToastNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ToastNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(ToastNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAccessoryNotificationTriggerDetails> for ToastNotificationTriggerDetails {}
#[doc = "*Required features: `\"Phone_Notification_Management\"`*"]
#[repr(transparent)]
pub struct VolumeInfo(::windows_core::IUnknown);
impl VolumeInfo {
    pub fn SystemVolume(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemVolume)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallVolume(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallVolume)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaVolume(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaVolume)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMuted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsVibrateEnabled(&self) -> ::windows_core::Result<VibrateState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVibrateEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for VolumeInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Phone.Notification.Management.VolumeInfo;{944dd118-7704-4481-b92e-d3ed3ece6322})");
}
impl ::core::clone::Clone for VolumeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VolumeInfo {
    const IID: ::windows_core::GUID = <IVolumeInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.VolumeInfo";
}
::windows_core::imp::interface_hierarchy!(VolumeInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::TypeKind for AccessoryNotificationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AccessoryNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessoryNotificationType").field(&self.0).finish()
    }
}
impl AccessoryNotificationType {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows_core::RuntimeType for AccessoryNotificationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.AccessoryNotificationType;u4)");
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
impl ::windows_core::TypeKind for CalendarChangedEvent {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CalendarChangedEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarChangedEvent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CalendarChangedEvent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.CalendarChangedEvent;i4)");
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
impl ::windows_core::TypeKind for PhoneCallAudioEndpoint {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallAudioEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallAudioEndpoint").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallAudioEndpoint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallAudioEndpoint;i4)");
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
impl ::windows_core::TypeKind for PhoneCallDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallDirection;i4)");
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
impl ::windows_core::TypeKind for PhoneCallState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallState;i4)");
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
impl ::windows_core::TypeKind for PhoneCallTransport {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneCallTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallTransport").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneCallTransport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallTransport;i4)");
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
impl ::windows_core::TypeKind for PhoneLineRegistrationState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneLineRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineRegistrationState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneLineRegistrationState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneLineRegistrationState;i4)");
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
impl ::windows_core::TypeKind for PhoneMediaType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneMediaType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneMediaType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneMediaType;i4)");
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
impl ::windows_core::TypeKind for PhoneNotificationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNotificationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneNotificationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneNotificationType;i4)");
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
impl ::windows_core::TypeKind for PlaybackCapability {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlaybackCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCapability").field(&self.0).finish()
    }
}
impl PlaybackCapability {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows_core::RuntimeType for PlaybackCapability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCapability;u4)");
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
impl ::windows_core::TypeKind for PlaybackCommand {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlaybackCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackCommand").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlaybackCommand {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCommand;i4)");
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
impl ::windows_core::TypeKind for PlaybackStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlaybackStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackStatus;i4)");
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
impl ::windows_core::TypeKind for ReminderState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ReminderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReminderState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ReminderState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.ReminderState;i4)");
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
impl ::windows_core::TypeKind for VibrateState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VibrateState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrateState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VibrateState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.VibrateState;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
