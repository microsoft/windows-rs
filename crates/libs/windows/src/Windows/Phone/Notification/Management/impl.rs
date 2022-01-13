#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAccessoryManagerImpl: Sized {
    fn RegisterAccessoryApp(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNextTriggerDetails(&mut self) -> ::windows::core::Result<IAccessoryNotificationTriggerDetails>;
    fn ProcessTriggerDetails(&mut self, pdetails: &::core::option::Option<IAccessoryNotificationTriggerDetails>) -> ::windows::core::Result<()>;
    fn PhoneLineDetails(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>>;
    fn GetPhoneLineDetails(&mut self, phoneline: &::windows::core::GUID) -> ::windows::core::Result<PhoneLineDetails>;
    fn AcceptPhoneCall(&mut self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn AcceptPhoneCallOnEndpoint(&mut self, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn AcceptPhoneCallWithVideo(&mut self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn AcceptPhoneCallWithVideoOnAudioEndpoint(&mut self, phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn RejectPhoneCall(&mut self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn RejectPhoneCallWithText(&mut self, phonecallid: u32, textresponseid: u32) -> ::windows::core::Result<()>;
    fn MakePhoneCall(&mut self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MakePhoneCallOnAudioEndpoint(&mut self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn MakePhoneCallWithVideo(&mut self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MakePhoneCallWithVideoOnAudioEndpoint(&mut self, phoneline: &::windows::core::GUID, phonenumber: &::windows::core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn SwapPhoneCalls(&mut self, phonecallidtohold: u32, phonecallidonhold: u32) -> ::windows::core::Result<()>;
    fn HoldPhoneCall(&mut self, phonecallid: u32, holdcall: bool) -> ::windows::core::Result<()>;
    fn EndPhoneCall(&mut self, phonecallid: u32) -> ::windows::core::Result<()>;
    fn SetPhoneMute(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PhoneMute(&mut self) -> ::windows::core::Result<bool>;
    fn SetPhoneCallAudioEndpoint(&mut self, value: PhoneCallAudioEndpoint) -> ::windows::core::Result<()>;
    fn PhoneCallAudioEndpoint(&mut self) -> ::windows::core::Result<PhoneCallAudioEndpoint>;
    fn SnoozeAlarm(&mut self, alarmid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SnoozeAlarmForSpecifiedTime(&mut self, alarmid: &::windows::core::GUID, timespan: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DismissAlarm(&mut self, alarmid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SnoozeReminder(&mut self, reminderid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SnoozeReminderForSpecifiedTime(&mut self, reminderid: &::windows::core::GUID, timespan: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DismissReminder(&mut self, reminderid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMediaMetadata(&mut self) -> ::windows::core::Result<MediaMetadata>;
    fn MediaPlaybackCapabilities(&mut self) -> ::windows::core::Result<PlaybackCapability>;
    fn MediaPlaybackStatus(&mut self) -> ::windows::core::Result<PlaybackStatus>;
    fn PerformMediaPlaybackCommand(&mut self, command: PlaybackCommand) -> ::windows::core::Result<()>;
    fn DoNotDisturbEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn DrivingModeEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn BatterySaverState(&mut self) -> ::windows::core::Result<bool>;
    fn GetApps(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppNotificationInfo>>;
    fn EnableNotificationsForApplication(&mut self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisableNotificationsForApplication(&mut self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsNotificationEnabledForApplication(&mut self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetEnabledAccessoryNotificationTypes(&mut self) -> ::windows::core::Result<i32>;
    fn EnableAccessoryNotificationTypes(&mut self, accessorynotificationtypes: i32) -> ::windows::core::Result<()>;
    fn DisableAllAccessoryNotificationTypes(&mut self) -> ::windows::core::Result<()>;
    fn GetUserConsent(&mut self) -> ::windows::core::Result<bool>;
    fn GetAppIcon(&mut self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccessoryManager {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAccessoryManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessoryManagerVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessoryManager, BASE_OFFSET>(),
            RegisterAccessoryApp: RegisterAccessoryApp::<Impl, IMPL_OFFSET>,
            GetNextTriggerDetails: GetNextTriggerDetails::<Impl, IMPL_OFFSET>,
            ProcessTriggerDetails: ProcessTriggerDetails::<Impl, IMPL_OFFSET>,
            PhoneLineDetails: PhoneLineDetails::<Impl, IMPL_OFFSET>,
            GetPhoneLineDetails: GetPhoneLineDetails::<Impl, IMPL_OFFSET>,
            AcceptPhoneCall: AcceptPhoneCall::<Impl, IMPL_OFFSET>,
            AcceptPhoneCallOnEndpoint: AcceptPhoneCallOnEndpoint::<Impl, IMPL_OFFSET>,
            AcceptPhoneCallWithVideo: AcceptPhoneCallWithVideo::<Impl, IMPL_OFFSET>,
            AcceptPhoneCallWithVideoOnAudioEndpoint: AcceptPhoneCallWithVideoOnAudioEndpoint::<Impl, IMPL_OFFSET>,
            RejectPhoneCall: RejectPhoneCall::<Impl, IMPL_OFFSET>,
            RejectPhoneCallWithText: RejectPhoneCallWithText::<Impl, IMPL_OFFSET>,
            MakePhoneCall: MakePhoneCall::<Impl, IMPL_OFFSET>,
            MakePhoneCallOnAudioEndpoint: MakePhoneCallOnAudioEndpoint::<Impl, IMPL_OFFSET>,
            MakePhoneCallWithVideo: MakePhoneCallWithVideo::<Impl, IMPL_OFFSET>,
            MakePhoneCallWithVideoOnAudioEndpoint: MakePhoneCallWithVideoOnAudioEndpoint::<Impl, IMPL_OFFSET>,
            SwapPhoneCalls: SwapPhoneCalls::<Impl, IMPL_OFFSET>,
            HoldPhoneCall: HoldPhoneCall::<Impl, IMPL_OFFSET>,
            EndPhoneCall: EndPhoneCall::<Impl, IMPL_OFFSET>,
            SetPhoneMute: SetPhoneMute::<Impl, IMPL_OFFSET>,
            PhoneMute: PhoneMute::<Impl, IMPL_OFFSET>,
            SetPhoneCallAudioEndpoint: SetPhoneCallAudioEndpoint::<Impl, IMPL_OFFSET>,
            PhoneCallAudioEndpoint: PhoneCallAudioEndpoint::<Impl, IMPL_OFFSET>,
            SnoozeAlarm: SnoozeAlarm::<Impl, IMPL_OFFSET>,
            SnoozeAlarmForSpecifiedTime: SnoozeAlarmForSpecifiedTime::<Impl, IMPL_OFFSET>,
            DismissAlarm: DismissAlarm::<Impl, IMPL_OFFSET>,
            SnoozeReminder: SnoozeReminder::<Impl, IMPL_OFFSET>,
            SnoozeReminderForSpecifiedTime: SnoozeReminderForSpecifiedTime::<Impl, IMPL_OFFSET>,
            DismissReminder: DismissReminder::<Impl, IMPL_OFFSET>,
            GetMediaMetadata: GetMediaMetadata::<Impl, IMPL_OFFSET>,
            MediaPlaybackCapabilities: MediaPlaybackCapabilities::<Impl, IMPL_OFFSET>,
            MediaPlaybackStatus: MediaPlaybackStatus::<Impl, IMPL_OFFSET>,
            PerformMediaPlaybackCommand: PerformMediaPlaybackCommand::<Impl, IMPL_OFFSET>,
            DoNotDisturbEnabled: DoNotDisturbEnabled::<Impl, IMPL_OFFSET>,
            DrivingModeEnabled: DrivingModeEnabled::<Impl, IMPL_OFFSET>,
            BatterySaverState: BatterySaverState::<Impl, IMPL_OFFSET>,
            GetApps: GetApps::<Impl, IMPL_OFFSET>,
            EnableNotificationsForApplication: EnableNotificationsForApplication::<Impl, IMPL_OFFSET>,
            DisableNotificationsForApplication: DisableNotificationsForApplication::<Impl, IMPL_OFFSET>,
            IsNotificationEnabledForApplication: IsNotificationEnabledForApplication::<Impl, IMPL_OFFSET>,
            GetEnabledAccessoryNotificationTypes: GetEnabledAccessoryNotificationTypes::<Impl, IMPL_OFFSET>,
            EnableAccessoryNotificationTypes: EnableAccessoryNotificationTypes::<Impl, IMPL_OFFSET>,
            DisableAllAccessoryNotificationTypes: DisableAllAccessoryNotificationTypes::<Impl, IMPL_OFFSET>,
            GetUserConsent: GetUserConsent::<Impl, IMPL_OFFSET>,
            GetAppIcon: GetAppIcon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessoryManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAccessoryManager2Impl: Sized {
    fn RingDevice(&mut self) -> ::windows::core::Result<()>;
    fn SpeedDialList(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>>;
    fn ClearToast(&mut self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsPhonePinLocked(&mut self) -> ::windows::core::Result<bool>;
    fn IncreaseVolume(&mut self, step: i32) -> ::windows::core::Result<()>;
    fn DecreaseVolume(&mut self, step: i32) -> ::windows::core::Result<()>;
    fn SetMute(&mut self, mute: bool) -> ::windows::core::Result<()>;
    fn SetRingerVibrate(&mut self, ringer: bool, vibrate: bool) -> ::windows::core::Result<()>;
    fn VolumeInfo(&mut self) -> ::windows::core::Result<VolumeInfo>;
    fn GetAllEmailAccounts(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>>;
    fn GetFolders(&mut self, emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>>;
    fn EnableEmailNotificationEmailAccount(&mut self, emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisableEmailNotificationEmailAccount(&mut self, emailaccount: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EnableEmailNotificationFolderFilter(&mut self, emailaccount: &::windows::core::HSTRING, folders: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn UpdateEmailReadStatus(&mut self, messageentryid: &::core::option::Option<BinaryId>, isread: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccessoryManager2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryManager2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAccessoryManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessoryManager2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessoryManager2, BASE_OFFSET>(),
            RingDevice: RingDevice::<Impl, IMPL_OFFSET>,
            SpeedDialList: SpeedDialList::<Impl, IMPL_OFFSET>,
            ClearToast: ClearToast::<Impl, IMPL_OFFSET>,
            IsPhonePinLocked: IsPhonePinLocked::<Impl, IMPL_OFFSET>,
            IncreaseVolume: IncreaseVolume::<Impl, IMPL_OFFSET>,
            DecreaseVolume: DecreaseVolume::<Impl, IMPL_OFFSET>,
            SetMute: SetMute::<Impl, IMPL_OFFSET>,
            SetRingerVibrate: SetRingerVibrate::<Impl, IMPL_OFFSET>,
            VolumeInfo: VolumeInfo::<Impl, IMPL_OFFSET>,
            GetAllEmailAccounts: GetAllEmailAccounts::<Impl, IMPL_OFFSET>,
            GetFolders: GetFolders::<Impl, IMPL_OFFSET>,
            EnableEmailNotificationEmailAccount: EnableEmailNotificationEmailAccount::<Impl, IMPL_OFFSET>,
            DisableEmailNotificationEmailAccount: DisableEmailNotificationEmailAccount::<Impl, IMPL_OFFSET>,
            EnableEmailNotificationFolderFilter: EnableEmailNotificationFolderFilter::<Impl, IMPL_OFFSET>,
            UpdateEmailReadStatus: UpdateEmailReadStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessoryManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessoryManager3Impl: Sized {
    fn SnoozeAlarmByInstanceId(&mut self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissAlarmByInstanceId(&mut self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SnoozeReminderByInstanceId(&mut self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DismissReminderByInstanceId(&mut self, instanceid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessoryManager3 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryManager3";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessoryManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessoryManager3Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessoryManager3, BASE_OFFSET>(),
            SnoozeAlarmByInstanceId: SnoozeAlarmByInstanceId::<Impl, IMPL_OFFSET>,
            DismissAlarmByInstanceId: DismissAlarmByInstanceId::<Impl, IMPL_OFFSET>,
            SnoozeReminderByInstanceId: SnoozeReminderByInstanceId::<Impl, IMPL_OFFSET>,
            DismissReminderByInstanceId: DismissReminderByInstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessoryManager3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IAccessoryNotificationTriggerDetailsImpl: Sized {
    fn TimeCreated(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessoryNotificationType(&mut self) -> ::windows::core::Result<AccessoryNotificationType>;
    fn StartedProcessing(&mut self) -> ::windows::core::Result<bool>;
    fn SetStartedProcessing(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAccessoryNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryNotificationTriggerDetails";
}
#[cfg(feature = "Foundation")]
impl IAccessoryNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessoryNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessoryNotificationTriggerDetails, BASE_OFFSET>(),
            TimeCreated: TimeCreated::<Impl, IMPL_OFFSET>,
            AppDisplayName: AppDisplayName::<Impl, IMPL_OFFSET>,
            AppId: AppId::<Impl, IMPL_OFFSET>,
            AccessoryNotificationType: AccessoryNotificationType::<Impl, IMPL_OFFSET>,
            StartedProcessing: StartedProcessing::<Impl, IMPL_OFFSET>,
            SetStartedProcessing: SetStartedProcessing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessoryNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAlarmNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AlarmId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ReminderState(&mut self) -> ::windows::core::Result<ReminderState>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAlarmNotificationTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAlarmNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlarmNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlarmNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAlarmNotificationTriggerDetails, BASE_OFFSET>(),
            AlarmId: AlarmId::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            ReminderState: ReminderState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlarmNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAlarmNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAlarmNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAlarmNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IAlarmNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlarmNotificationTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlarmNotificationTriggerDetails2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAlarmNotificationTriggerDetails2, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlarmNotificationTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppNotificationInfoImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAppNotificationInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppNotificationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppNotificationInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppNotificationInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppNotificationInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppNotificationInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBinaryIdImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u8>;
    fn Length(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IBinaryId";
}
#[cfg(feature = "implement_exclusive")]
impl IBinaryIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBinaryIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBinaryIdVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBinaryId, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBinaryId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICalendarChangedNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn EventType(&mut self) -> ::windows::core::Result<CalendarChangedEvent>;
    fn ItemId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ICalendarChangedNotificationTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICalendarChangedNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarChangedNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarChangedNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICalendarChangedNotificationTriggerDetails, BASE_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            ItemId: ItemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICalendarChangedNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICortanaTileNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn TileId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Content(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LargeContent1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LargeContent2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmphasizedText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NonWrappedSmallContent4(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Source(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ICortanaTileNotificationTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICortanaTileNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICortanaTileNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICortanaTileNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICortanaTileNotificationTriggerDetails, BASE_OFFSET>(),
            TileId: TileId::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            LargeContent1: LargeContent1::<Impl, IMPL_OFFSET>,
            LargeContent2: LargeContent2::<Impl, IMPL_OFFSET>,
            EmphasizedText: EmphasizedText::<Impl, IMPL_OFFSET>,
            NonWrappedSmallContent1: NonWrappedSmallContent1::<Impl, IMPL_OFFSET>,
            NonWrappedSmallContent2: NonWrappedSmallContent2::<Impl, IMPL_OFFSET>,
            NonWrappedSmallContent3: NonWrappedSmallContent3::<Impl, IMPL_OFFSET>,
            NonWrappedSmallContent4: NonWrappedSmallContent4::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICortanaTileNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailAccountInfoImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsNotificationEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailAccountInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailAccountInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAccountInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailAccountInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEmailAccountInfo, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsNotificationEnabled: IsNotificationEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAccountInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailFolderInfoImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsNotificationEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailFolderInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailFolderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailFolderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailFolderInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEmailFolderInfo, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsNotificationEnabled: IsNotificationEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailFolderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AccountName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SenderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SenderAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessage(&mut self) -> ::windows::core::Result<super::super::super::ApplicationModel::Email::EmailMessage>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailNotificationTriggerDetails";
}
#[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEmailNotificationTriggerDetails, BASE_OFFSET>(),
            AccountName: AccountName::<Impl, IMPL_OFFSET>,
            ParentFolderName: ParentFolderName::<Impl, IMPL_OFFSET>,
            SenderName: SenderName::<Impl, IMPL_OFFSET>,
            SenderAddress: SenderAddress::<Impl, IMPL_OFFSET>,
            EmailMessage: EmailMessage::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailNotificationTriggerDetails2Impl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn MessageEntryId(&mut self) -> ::windows::core::Result<BinaryId>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailNotificationTriggerDetails2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailNotificationTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailNotificationTriggerDetails2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEmailNotificationTriggerDetails2, BASE_OFFSET>(),
            MessageEntryId: MessageEntryId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailNotificationTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEmailReadNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn AccountName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MessageEntryId(&mut self) -> ::windows::core::Result<BinaryId>;
    fn IsRead(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IEmailReadNotificationTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEmailReadNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailReadNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailReadNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEmailReadNotificationTriggerDetails, BASE_OFFSET>(),
            AccountName: AccountName::<Impl, IMPL_OFFSET>,
            ParentFolderName: ParentFolderName::<Impl, IMPL_OFFSET>,
            MessageEntryId: MessageEntryId::<Impl, IMPL_OFFSET>,
            IsRead: IsRead::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailReadNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaControlsTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn PlaybackStatus(&mut self) -> ::windows::core::Result<PlaybackStatus>;
    fn MediaMetadata(&mut self) -> ::windows::core::Result<MediaMetadata>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IMediaControlsTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaControlsTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaControlsTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaControlsTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaControlsTriggerDetails, BASE_OFFSET>(),
            PlaybackStatus: PlaybackStatus::<Impl, IMPL_OFFSET>,
            MediaMetadata: MediaMetadata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaControlsTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaMetadataImpl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Artist(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Album(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Track(&mut self) -> ::windows::core::Result<u32>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IMediaMetadata";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMetadataVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaMetadata, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            Subtitle: Subtitle::<Impl, IMPL_OFFSET>,
            Artist: Artist::<Impl, IMPL_OFFSET>,
            Album: Album::<Impl, IMPL_OFFSET>,
            Track: Track::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhoneCallDetailsImpl: Sized {
    fn PhoneLine(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CallId(&mut self) -> ::windows::core::Result<u32>;
    fn CallTransport(&mut self) -> ::windows::core::Result<PhoneCallTransport>;
    fn CallMediaType(&mut self) -> ::windows::core::Result<PhoneMediaType>;
    fn CallDirection(&mut self) -> ::windows::core::Result<PhoneCallDirection>;
    fn State(&mut self) -> ::windows::core::Result<PhoneCallState>;
    fn ConferenceCallId(&mut self) -> ::windows::core::Result<u32>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn EndTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn PhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresetTextResponses(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneCallDetails";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhoneCallDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallDetails, BASE_OFFSET>(),
            PhoneLine: PhoneLine::<Impl, IMPL_OFFSET>,
            CallId: CallId::<Impl, IMPL_OFFSET>,
            CallTransport: CallTransport::<Impl, IMPL_OFFSET>,
            CallMediaType: CallMediaType::<Impl, IMPL_OFFSET>,
            CallDirection: CallDirection::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            ConferenceCallId: ConferenceCallId::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            EndTime: EndTime::<Impl, IMPL_OFFSET>,
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
            ContactName: ContactName::<Impl, IMPL_OFFSET>,
            PresetTextResponses: PresetTextResponses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineDetailsImpl: Sized {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LineNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultOutgoingLine(&mut self) -> ::windows::core::Result<bool>;
    fn VoicemailCount(&mut self) -> ::windows::core::Result<u32>;
    fn RegistrationState(&mut self) -> ::windows::core::Result<PhoneLineRegistrationState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneLineDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineDetails, BASE_OFFSET>(),
            LineId: LineId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            LineNumber: LineNumber::<Impl, IMPL_OFFSET>,
            DefaultOutgoingLine: DefaultOutgoingLine::<Impl, IMPL_OFFSET>,
            VoicemailCount: VoicemailCount::<Impl, IMPL_OFFSET>,
            RegistrationState: RegistrationState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineDetails2Impl: Sized {
    fn MissedCallCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneLineDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineDetails2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineDetails2, BASE_OFFSET>(),
            MissedCallCount: MissedCallCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn PhoneNotificationType(&mut self) -> ::windows::core::Result<PhoneNotificationType>;
    fn CallDetails(&mut self) -> ::windows::core::Result<PhoneCallDetails>;
    fn PhoneLineChangedId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IPhoneNotificationTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneNotificationTriggerDetails, BASE_OFFSET>(),
            PhoneNotificationType: PhoneNotificationType::<Impl, IMPL_OFFSET>,
            CallDetails: CallDetails::<Impl, IMPL_OFFSET>,
            PhoneLineChangedId: PhoneLineChangedId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IReminderNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn ReminderId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Details(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Appointment(&mut self) -> ::windows::core::Result<super::super::super::ApplicationModel::Appointments::Appointment>;
    fn ReminderState(&mut self) -> ::windows::core::Result<ReminderState>;
}
#[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IReminderNotificationTriggerDetails";
}
#[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "implement_exclusive"))]
impl IReminderNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReminderNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReminderNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReminderNotificationTriggerDetails, BASE_OFFSET>(),
            ReminderId: ReminderId::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Details: Details::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            Appointment: Appointment::<Impl, IMPL_OFFSET>,
            ReminderState: ReminderState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReminderNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReminderNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReminderNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IReminderNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IReminderNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReminderNotificationTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReminderNotificationTriggerDetails2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReminderNotificationTriggerDetails2, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReminderNotificationTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeedDialEntryImpl: Sized {
    fn PhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumberType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContactName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ISpeedDialEntry";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeedDialEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeedDialEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeedDialEntryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeedDialEntry, BASE_OFFSET>(),
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
            NumberType: NumberType::<Impl, IMPL_OFFSET>,
            ContactName: ContactName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeedDialEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextResponseImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Content(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ITextResponse";
}
#[cfg(feature = "implement_exclusive")]
impl ITextResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextResponseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextResponseVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextResponse, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IToastNotificationTriggerDetailsImpl: Sized + IAccessoryNotificationTriggerDetailsImpl {
    fn Text1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text4(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SuppressPopup(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IToastNotificationTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IToastNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationTriggerDetails, BASE_OFFSET>(),
            Text1: Text1::<Impl, IMPL_OFFSET>,
            Text2: Text2::<Impl, IMPL_OFFSET>,
            Text3: Text3::<Impl, IMPL_OFFSET>,
            Text4: Text4::<Impl, IMPL_OFFSET>,
            SuppressPopup: SuppressPopup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationTriggerDetails2Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IToastNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationTriggerDetails2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationTriggerDetails2, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVolumeInfoImpl: Sized {
    fn SystemVolume(&mut self) -> ::windows::core::Result<u32>;
    fn CallVolume(&mut self) -> ::windows::core::Result<u32>;
    fn MediaVolume(&mut self) -> ::windows::core::Result<u32>;
    fn IsMuted(&mut self) -> ::windows::core::Result<bool>;
    fn IsVibrateEnabled(&mut self) -> ::windows::core::Result<VibrateState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IVolumeInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IVolumeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVolumeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVolumeInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVolumeInfo, BASE_OFFSET>(),
            SystemVolume: SystemVolume::<Impl, IMPL_OFFSET>,
            CallVolume: CallVolume::<Impl, IMPL_OFFSET>,
            MediaVolume: MediaVolume::<Impl, IMPL_OFFSET>,
            IsMuted: IsMuted::<Impl, IMPL_OFFSET>,
            IsVibrateEnabled: IsVibrateEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVolumeInfo as ::windows::core::Interface>::IID
    }
}
