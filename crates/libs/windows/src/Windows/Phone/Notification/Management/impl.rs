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
impl ::windows::core::RuntimeName for IAccessoryManager {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessoryManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryManagerImpl, const OFFSET: isize>() -> IAccessoryManagerVtbl {
        unsafe extern "system" fn RegisterAccessoryApp<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAccessoryApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextTriggerDetails<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextTriggerDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTriggerDetails<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetails: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessTriggerDetails(&*(&pdetails as *const <IAccessoryNotificationTriggerDetails as ::windows::core::Abi>::Abi as *const <IAccessoryNotificationTriggerDetails as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PhoneLineDetails<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneLineDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneLineDetails<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhoneLineDetails(&*(&phoneline as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptPhoneCall<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptPhoneCall(phonecallid).into()
        }
        unsafe extern "system" fn AcceptPhoneCallOnEndpoint<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptPhoneCallOnEndpoint(phonecallid, endpoint).into()
        }
        unsafe extern "system" fn AcceptPhoneCallWithVideo<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptPhoneCallWithVideo(phonecallid).into()
        }
        unsafe extern "system" fn AcceptPhoneCallWithVideoOnAudioEndpoint<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid, endpoint).into()
        }
        unsafe extern "system" fn RejectPhoneCall<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RejectPhoneCall(phonecallid).into()
        }
        unsafe extern "system" fn RejectPhoneCallWithText<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32, textresponseid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RejectPhoneCallWithText(phonecallid, textresponseid).into()
        }
        unsafe extern "system" fn MakePhoneCall<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakePhoneCall(&*(&phoneline as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&phonenumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MakePhoneCallOnAudioEndpoint<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakePhoneCallOnAudioEndpoint(&*(&phoneline as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&phonenumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), endpoint).into()
        }
        unsafe extern "system" fn MakePhoneCallWithVideo<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakePhoneCallWithVideo(&*(&phoneline as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&phonenumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MakePhoneCallWithVideoOnAudioEndpoint<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneline: ::windows::core::GUID, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakePhoneCallWithVideoOnAudioEndpoint(&*(&phoneline as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&phonenumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), endpoint).into()
        }
        unsafe extern "system" fn SwapPhoneCalls<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwapPhoneCalls(phonecallidtohold, phonecallidonhold).into()
        }
        unsafe extern "system" fn HoldPhoneCall<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32, holdcall: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HoldPhoneCall(phonecallid, holdcall).into()
        }
        unsafe extern "system" fn EndPhoneCall<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonecallid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndPhoneCall(phonecallid).into()
        }
        unsafe extern "system" fn SetPhoneMute<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhoneMute(value).into()
        }
        unsafe extern "system" fn PhoneMute<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneMute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneCallAudioEndpoint<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneCallAudioEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhoneCallAudioEndpoint(value).into()
        }
        unsafe extern "system" fn PhoneCallAudioEndpoint<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallAudioEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneCallAudioEndpoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnoozeAlarm<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alarmid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SnoozeAlarm(&*(&alarmid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SnoozeAlarmForSpecifiedTime<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alarmid: ::windows::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SnoozeAlarmForSpecifiedTime(&*(&alarmid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&timespan as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DismissAlarm<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alarmid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissAlarm(&*(&alarmid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SnoozeReminder<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reminderid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SnoozeReminder(&*(&reminderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SnoozeReminderForSpecifiedTime<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reminderid: ::windows::core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SnoozeReminderForSpecifiedTime(&*(&reminderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&timespan as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DismissReminder<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reminderid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissReminder(&*(&reminderid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetMediaMetadata<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMediaMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPlaybackCapabilities<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlaybackCapability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlaybackCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPlaybackStatus<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlaybackStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PerformMediaPlaybackCommand<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: PlaybackCommand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PerformMediaPlaybackCommand(command).into()
        }
        unsafe extern "system" fn DoNotDisturbEnabled<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoNotDisturbEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrivingModeEnabled<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrivingModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatterySaverState<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatterySaverState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApps<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotificationsForApplication<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotificationsForApplication(&*(&appid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisableNotificationsForApplication<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableNotificationsForApplication(&*(&appid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsNotificationEnabledForApplication<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNotificationEnabledForApplication(&*(&appid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnabledAccessoryNotificationTypes<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnabledAccessoryNotificationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAccessoryNotificationTypes<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessorynotificationtypes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableAccessoryNotificationTypes(accessorynotificationtypes).into()
        }
        unsafe extern "system" fn DisableAllAccessoryNotificationTypes<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableAllAccessoryNotificationTypes().into()
        }
        unsafe extern "system" fn GetUserConsent<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserConsent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppIcon<Impl: IAccessoryManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppIcon(&*(&appid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAccessoryManager>,
            ::windows::core::GetTrustLevel,
            RegisterAccessoryApp::<Impl, OFFSET>,
            GetNextTriggerDetails::<Impl, OFFSET>,
            ProcessTriggerDetails::<Impl, OFFSET>,
            PhoneLineDetails::<Impl, OFFSET>,
            GetPhoneLineDetails::<Impl, OFFSET>,
            AcceptPhoneCall::<Impl, OFFSET>,
            AcceptPhoneCallOnEndpoint::<Impl, OFFSET>,
            AcceptPhoneCallWithVideo::<Impl, OFFSET>,
            AcceptPhoneCallWithVideoOnAudioEndpoint::<Impl, OFFSET>,
            RejectPhoneCall::<Impl, OFFSET>,
            RejectPhoneCallWithText::<Impl, OFFSET>,
            MakePhoneCall::<Impl, OFFSET>,
            MakePhoneCallOnAudioEndpoint::<Impl, OFFSET>,
            MakePhoneCallWithVideo::<Impl, OFFSET>,
            MakePhoneCallWithVideoOnAudioEndpoint::<Impl, OFFSET>,
            SwapPhoneCalls::<Impl, OFFSET>,
            HoldPhoneCall::<Impl, OFFSET>,
            EndPhoneCall::<Impl, OFFSET>,
            SetPhoneMute::<Impl, OFFSET>,
            PhoneMute::<Impl, OFFSET>,
            SetPhoneCallAudioEndpoint::<Impl, OFFSET>,
            PhoneCallAudioEndpoint::<Impl, OFFSET>,
            SnoozeAlarm::<Impl, OFFSET>,
            SnoozeAlarmForSpecifiedTime::<Impl, OFFSET>,
            DismissAlarm::<Impl, OFFSET>,
            SnoozeReminder::<Impl, OFFSET>,
            SnoozeReminderForSpecifiedTime::<Impl, OFFSET>,
            DismissReminder::<Impl, OFFSET>,
            GetMediaMetadata::<Impl, OFFSET>,
            MediaPlaybackCapabilities::<Impl, OFFSET>,
            MediaPlaybackStatus::<Impl, OFFSET>,
            PerformMediaPlaybackCommand::<Impl, OFFSET>,
            DoNotDisturbEnabled::<Impl, OFFSET>,
            DrivingModeEnabled::<Impl, OFFSET>,
            BatterySaverState::<Impl, OFFSET>,
            GetApps::<Impl, OFFSET>,
            EnableNotificationsForApplication::<Impl, OFFSET>,
            DisableNotificationsForApplication::<Impl, OFFSET>,
            IsNotificationEnabledForApplication::<Impl, OFFSET>,
            GetEnabledAccessoryNotificationTypes::<Impl, OFFSET>,
            EnableAccessoryNotificationTypes::<Impl, OFFSET>,
            DisableAllAccessoryNotificationTypes::<Impl, OFFSET>,
            GetUserConsent::<Impl, OFFSET>,
            GetAppIcon::<Impl, OFFSET>,
        )
    }
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
impl ::windows::core::RuntimeName for IAccessoryManager2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessoryManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryManager2Impl, const OFFSET: isize>() -> IAccessoryManager2Vtbl {
        unsafe extern "system" fn RingDevice<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RingDevice().into()
        }
        unsafe extern "system" fn SpeedDialList<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeedDialList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearToast<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearToast(&*(&instanceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsPhonePinLocked<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPhonePinLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncreaseVolume<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, step: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IncreaseVolume(step).into()
        }
        unsafe extern "system" fn DecreaseVolume<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, step: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecreaseVolume(step).into()
        }
        unsafe extern "system" fn SetMute<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mute: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMute(mute).into()
        }
        unsafe extern "system" fn SetRingerVibrate<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ringer: bool, vibrate: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRingerVibrate(ringer, vibrate).into()
        }
        unsafe extern "system" fn VolumeInfo<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllEmailAccounts<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllEmailAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolders<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolders(&*(&emailaccount as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableEmailNotificationEmailAccount<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableEmailNotificationEmailAccount(&*(&emailaccount as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisableEmailNotificationEmailAccount<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableEmailNotificationEmailAccount(&*(&emailaccount as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableEmailNotificationFolderFilter<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emailaccount: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, folders: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .EnableEmailNotificationFolderFilter(&*(&emailaccount as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&folders as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn UpdateEmailReadStatus<Impl: IAccessoryManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageentryid: ::windows::core::RawPtr, isread: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateEmailReadStatus(&*(&messageentryid as *const <BinaryId as ::windows::core::Abi>::Abi as *const <BinaryId as ::windows::core::DefaultType>::DefaultType), isread).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAccessoryManager2>,
            ::windows::core::GetTrustLevel,
            RingDevice::<Impl, OFFSET>,
            SpeedDialList::<Impl, OFFSET>,
            ClearToast::<Impl, OFFSET>,
            IsPhonePinLocked::<Impl, OFFSET>,
            IncreaseVolume::<Impl, OFFSET>,
            DecreaseVolume::<Impl, OFFSET>,
            SetMute::<Impl, OFFSET>,
            SetRingerVibrate::<Impl, OFFSET>,
            VolumeInfo::<Impl, OFFSET>,
            GetAllEmailAccounts::<Impl, OFFSET>,
            GetFolders::<Impl, OFFSET>,
            EnableEmailNotificationEmailAccount::<Impl, OFFSET>,
            DisableEmailNotificationEmailAccount::<Impl, OFFSET>,
            EnableEmailNotificationFolderFilter::<Impl, OFFSET>,
            UpdateEmailReadStatus::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessoryManager3Impl: Sized {
    fn SnoozeAlarmByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissAlarmByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SnoozeReminderByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissReminderByInstanceId(&self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessoryManager3 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryManager3";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessoryManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryManager3Impl, const OFFSET: isize>() -> IAccessoryManager3Vtbl {
        unsafe extern "system" fn SnoozeAlarmByInstanceId<Impl: IAccessoryManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SnoozeAlarmByInstanceId(&*(&instanceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DismissAlarmByInstanceId<Impl: IAccessoryManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissAlarmByInstanceId(&*(&instanceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SnoozeReminderByInstanceId<Impl: IAccessoryManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SnoozeReminderByInstanceId(&*(&instanceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DismissReminderByInstanceId<Impl: IAccessoryManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instanceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DismissReminderByInstanceId(&*(&instanceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccessoryManager3>, ::windows::core::GetTrustLevel, SnoozeAlarmByInstanceId::<Impl, OFFSET>, DismissAlarmByInstanceId::<Impl, OFFSET>, SnoozeReminderByInstanceId::<Impl, OFFSET>, DismissReminderByInstanceId::<Impl, OFFSET>)
    }
}
pub trait IAccessoryNotificationTriggerDetailsImpl: Sized {
    fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType>;
    fn StartedProcessing(&self) -> ::windows::core::Result<bool>;
    fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAccessoryNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryNotificationTriggerDetails";
}
impl IAccessoryNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IAccessoryNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn TimeCreated<Impl: IAccessoryNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppDisplayName<Impl: IAccessoryNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IAccessoryNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessoryNotificationType<Impl: IAccessoryNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessoryNotificationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartedProcessing<Impl: IAccessoryNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartedProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartedProcessing<Impl: IAccessoryNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartedProcessing(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccessoryNotificationTriggerDetails>, ::windows::core::GetTrustLevel, TimeCreated::<Impl, OFFSET>, AppDisplayName::<Impl, OFFSET>, AppId::<Impl, OFFSET>, AccessoryNotificationType::<Impl, OFFSET>, StartedProcessing::<Impl, OFFSET>, SetStartedProcessing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAlarmNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AlarmId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ReminderState(&self) -> ::windows::core::Result<ReminderState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAlarmNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IAlarmNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlarmNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IAlarmNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn AlarmId<Impl: IAlarmNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlarmId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IAlarmNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IAlarmNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReminderState<Impl: IAlarmNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReminderState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAlarmNotificationTriggerDetails>, ::windows::core::GetTrustLevel, AlarmId::<Impl, OFFSET>, Title::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>, ReminderState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAlarmNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAlarmNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAlarmNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IAlarmNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlarmNotificationTriggerDetails2Impl, const OFFSET: isize>() -> IAlarmNotificationTriggerDetails2Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IAlarmNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAlarmNotificationTriggerDetails2>, ::windows::core::GetTrustLevel, InstanceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppNotificationInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAppNotificationInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppNotificationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppNotificationInfoImpl, const OFFSET: isize>() -> IAppNotificationInfoVtbl {
        unsafe extern "system" fn Id<Impl: IAppNotificationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IAppNotificationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppNotificationInfo>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Name::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBinaryIdImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u8>;
    fn Length(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IBinaryId";
}
#[cfg(feature = "implement_exclusive")]
impl IBinaryIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBinaryIdImpl, const OFFSET: isize>() -> IBinaryIdVtbl {
        unsafe extern "system" fn Id<Impl: IBinaryIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IBinaryIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBinaryId>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Length::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarChangedNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn EventType(&self) -> ::windows::core::Result<CalendarChangedEvent>;
    fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ICalendarChangedNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarChangedNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarChangedNotificationTriggerDetailsImpl, const OFFSET: isize>() -> ICalendarChangedNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn EventType<Impl: ICalendarChangedNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CalendarChangedEvent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemId<Impl: ICalendarChangedNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICalendarChangedNotificationTriggerDetails>, ::windows::core::GetTrustLevel, EventType::<Impl, OFFSET>, ItemId::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for ICortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ICortanaTileNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ICortanaTileNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>() -> ICortanaTileNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn TileId<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeContent1<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeContent1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeContent2<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeContent2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmphasizedText<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmphasizedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonWrappedSmallContent1<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonWrappedSmallContent1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonWrappedSmallContent2<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonWrappedSmallContent2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonWrappedSmallContent3<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonWrappedSmallContent3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonWrappedSmallContent4<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonWrappedSmallContent4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Impl: ICortanaTileNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICortanaTileNotificationTriggerDetails>,
            ::windows::core::GetTrustLevel,
            TileId::<Impl, OFFSET>,
            Content::<Impl, OFFSET>,
            LargeContent1::<Impl, OFFSET>,
            LargeContent2::<Impl, OFFSET>,
            EmphasizedText::<Impl, OFFSET>,
            NonWrappedSmallContent1::<Impl, OFFSET>,
            NonWrappedSmallContent2::<Impl, OFFSET>,
            NonWrappedSmallContent3::<Impl, OFFSET>,
            NonWrappedSmallContent4::<Impl, OFFSET>,
            Source::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailAccountInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailAccountInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailAccountInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAccountInfoImpl, const OFFSET: isize>() -> IEmailAccountInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IEmailAccountInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotificationEnabled<Impl: IEmailAccountInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNotificationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailAccountInfo>, ::windows::core::GetTrustLevel, DisplayName::<Impl, OFFSET>, IsNotificationEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailFolderInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsNotificationEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailFolderInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailFolderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailFolderInfoImpl, const OFFSET: isize>() -> IEmailFolderInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IEmailFolderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotificationEnabled<Impl: IEmailFolderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNotificationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailFolderInfo>, ::windows::core::GetTrustLevel, DisplayName::<Impl, OFFSET>, IsNotificationEnabled::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IEmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IEmailNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn AccountName<Impl: IEmailNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentFolderName<Impl: IEmailNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentFolderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderName<Impl: IEmailNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderAddress<Impl: IEmailNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailMessage<Impl: IEmailNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IEmailNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailNotificationTriggerDetails>, ::windows::core::GetTrustLevel, AccountName::<Impl, OFFSET>, ParentFolderName::<Impl, OFFSET>, SenderName::<Impl, OFFSET>, SenderAddress::<Impl, OFFSET>, EmailMessage::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailNotificationTriggerDetails2Impl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailNotificationTriggerDetails2Impl, const OFFSET: isize>() -> IEmailNotificationTriggerDetails2Vtbl {
        unsafe extern "system" fn MessageEntryId<Impl: IEmailNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageEntryId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailNotificationTriggerDetails2>, ::windows::core::GetTrustLevel, MessageEntryId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailReadNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MessageEntryId(&self) -> ::windows::core::Result<BinaryId>;
    fn IsRead(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailReadNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailReadNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailReadNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IEmailReadNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn AccountName<Impl: IEmailReadNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentFolderName<Impl: IEmailReadNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentFolderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageEntryId<Impl: IEmailReadNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageEntryId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Impl: IEmailReadNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailReadNotificationTriggerDetails>, ::windows::core::GetTrustLevel, AccountName::<Impl, OFFSET>, ParentFolderName::<Impl, OFFSET>, MessageEntryId::<Impl, OFFSET>, IsRead::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaControlsTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn PlaybackStatus(&self) -> ::windows::core::Result<PlaybackStatus>;
    fn MediaMetadata(&self) -> ::windows::core::Result<MediaMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IMediaControlsTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaControlsTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaControlsTriggerDetailsImpl, const OFFSET: isize>() -> IMediaControlsTriggerDetailsVtbl {
        unsafe extern "system" fn PlaybackStatus<Impl: IMediaControlsTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlaybackStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaMetadata<Impl: IMediaControlsTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaControlsTriggerDetails>, ::windows::core::GetTrustLevel, PlaybackStatus::<Impl, OFFSET>, MediaMetadata::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IMediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IMediaMetadata";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMetadataImpl, const OFFSET: isize>() -> IMediaMetadataVtbl {
        unsafe extern "system" fn Title<Impl: IMediaMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtitle<Impl: IMediaMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Artist<Impl: IMediaMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Artist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Album<Impl: IMediaMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Album() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track<Impl: IMediaMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IMediaMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IMediaMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaMetadata>, ::windows::core::GetTrustLevel, Title::<Impl, OFFSET>, Subtitle::<Impl, OFFSET>, Artist::<Impl, OFFSET>, Album::<Impl, OFFSET>, Track::<Impl, OFFSET>, Duration::<Impl, OFFSET>, Thumbnail::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IPhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneCallDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallDetailsImpl, const OFFSET: isize>() -> IPhoneCallDetailsVtbl {
        unsafe extern "system" fn PhoneLine<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallId<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallTransport<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallTransport) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallTransport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallMediaType<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneMediaType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallDirection<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConferenceCallId<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConferenceCallId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndTime<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneNumber<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactName<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresetTextResponses<Impl: IPhoneCallDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresetTextResponses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPhoneCallDetails>,
            ::windows::core::GetTrustLevel,
            PhoneLine::<Impl, OFFSET>,
            CallId::<Impl, OFFSET>,
            CallTransport::<Impl, OFFSET>,
            CallMediaType::<Impl, OFFSET>,
            CallDirection::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            ConferenceCallId::<Impl, OFFSET>,
            StartTime::<Impl, OFFSET>,
            EndTime::<Impl, OFFSET>,
            PhoneNumber::<Impl, OFFSET>,
            ContactName::<Impl, OFFSET>,
            PresetTextResponses::<Impl, OFFSET>,
        )
    }
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
impl ::windows::core::RuntimeName for IPhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneLineDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineDetailsImpl, const OFFSET: isize>() -> IPhoneLineDetailsVtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneLineDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IPhoneLineDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineNumber<Impl: IPhoneLineDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultOutgoingLine<Impl: IPhoneLineDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultOutgoingLine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VoicemailCount<Impl: IPhoneLineDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VoicemailCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistrationState<Impl: IPhoneLineDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineRegistrationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhoneLineDetails>, ::windows::core::GetTrustLevel, LineId::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, LineNumber::<Impl, OFFSET>, DefaultOutgoingLine::<Impl, OFFSET>, VoicemailCount::<Impl, OFFSET>, RegistrationState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineDetails2Impl: Sized {
    fn MissedCallCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneLineDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineDetails2Impl, const OFFSET: isize>() -> IPhoneLineDetails2Vtbl {
        unsafe extern "system" fn MissedCallCount<Impl: IPhoneLineDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MissedCallCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhoneLineDetails2>, ::windows::core::GetTrustLevel, MissedCallCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn PhoneNotificationType(&self) -> ::windows::core::Result<PhoneNotificationType>;
    fn CallDetails(&self) -> ::windows::core::Result<PhoneCallDetails>;
    fn PhoneLineChangedId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IPhoneNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn PhoneNotificationType<Impl: IPhoneNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneNotificationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNotificationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallDetails<Impl: IPhoneNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneLineChangedId<Impl: IPhoneNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneLineChangedId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhoneNotificationTriggerDetails>, ::windows::core::GetTrustLevel, PhoneNotificationType::<Impl, OFFSET>, CallDetails::<Impl, OFFSET>, PhoneLineChangedId::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IReminderNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IReminderNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IReminderNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn ReminderId<Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReminderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Details<Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Details() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appointment<Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appointment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReminderState<Impl: IReminderNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ReminderState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReminderState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IReminderNotificationTriggerDetails>, ::windows::core::GetTrustLevel, ReminderId::<Impl, OFFSET>, Title::<Impl, OFFSET>, Description::<Impl, OFFSET>, Details::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>, Appointment::<Impl, OFFSET>, ReminderState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReminderNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReminderNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IReminderNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IReminderNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReminderNotificationTriggerDetails2Impl, const OFFSET: isize>() -> IReminderNotificationTriggerDetails2Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IReminderNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IReminderNotificationTriggerDetails2>, ::windows::core::GetTrustLevel, InstanceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeedDialEntryImpl: Sized {
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumberType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ISpeedDialEntry";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeedDialEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeedDialEntryImpl, const OFFSET: isize>() -> ISpeedDialEntryVtbl {
        unsafe extern "system" fn PhoneNumber<Impl: ISpeedDialEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberType<Impl: ISpeedDialEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactName<Impl: ISpeedDialEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeedDialEntry>, ::windows::core::GetTrustLevel, PhoneNumber::<Impl, OFFSET>, NumberType::<Impl, OFFSET>, ContactName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextResponseImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ITextResponse";
}
#[cfg(feature = "implement_exclusive")]
impl ITextResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextResponseImpl, const OFFSET: isize>() -> ITextResponseVtbl {
        unsafe extern "system" fn Id<Impl: ITextResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: ITextResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextResponse>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Content::<Impl, OFFSET>)
    }
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
impl ::windows::core::RuntimeName for IToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IToastNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IToastNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn Text1<Impl: IToastNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text2<Impl: IToastNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text3<Impl: IToastNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text4<Impl: IToastNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressPopup<Impl: IToastNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressPopup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToastNotificationTriggerDetails>, ::windows::core::GetTrustLevel, Text1::<Impl, OFFSET>, Text2::<Impl, OFFSET>, Text3::<Impl, OFFSET>, Text4::<Impl, OFFSET>, SuppressPopup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IToastNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationTriggerDetails2Impl, const OFFSET: isize>() -> IToastNotificationTriggerDetails2Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IToastNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToastNotificationTriggerDetails2>, ::windows::core::GetTrustLevel, InstanceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVolumeInfoImpl: Sized {
    fn SystemVolume(&self) -> ::windows::core::Result<u32>;
    fn CallVolume(&self) -> ::windows::core::Result<u32>;
    fn MediaVolume(&self) -> ::windows::core::Result<u32>;
    fn IsMuted(&self) -> ::windows::core::Result<bool>;
    fn IsVibrateEnabled(&self) -> ::windows::core::Result<VibrateState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IVolumeInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IVolumeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVolumeInfoImpl, const OFFSET: isize>() -> IVolumeInfoVtbl {
        unsafe extern "system" fn SystemVolume<Impl: IVolumeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallVolume<Impl: IVolumeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaVolume<Impl: IVolumeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMuted<Impl: IVolumeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMuted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVibrateEnabled<Impl: IVolumeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VibrateState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVibrateEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVolumeInfo>, ::windows::core::GetTrustLevel, SystemVolume::<Impl, OFFSET>, CallVolume::<Impl, OFFSET>, MediaVolume::<Impl, OFFSET>, IsMuted::<Impl, OFFSET>, IsVibrateEnabled::<Impl, OFFSET>)
    }
}
