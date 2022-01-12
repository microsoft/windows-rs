#[cfg(feature = "Win32_Networking_WinSock")]
pub trait INetworkTransportSettingsImpl: Sized {
    fn ApplySetting();
    fn QuerySetting();
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl INetworkTransportSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkTransportSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkTransportSettingsVtbl {
        unsafe extern "system" fn ApplySetting<Impl: INetworkTransportSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuerySetting<Impl: INetworkTransportSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ApplySetting: ApplySetting::<Impl, IMPL_OFFSET>,
            QuerySetting: QuerySetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkTransportSettings as ::windows::core::Interface>::IID
    }
}
pub trait INotificationTransportSyncImpl: Sized {
    fn CompleteDelivery();
    fn Flush();
}
impl INotificationTransportSyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationTransportSyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationTransportSyncVtbl {
        unsafe extern "system" fn CompleteDelivery<Impl: INotificationTransportSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: INotificationTransportSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CompleteDelivery: CompleteDelivery::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationTransportSync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCBuddyImpl: Sized + IRTCPresenceContactImpl {
    fn Status();
    fn Notes();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCBuddyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyVtbl {
        unsafe extern "system" fn Status<Impl: IRTCBuddyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notes<Impl: IRTCBuddyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCPresenceContactVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Notes: Notes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCBuddy2Impl: Sized + IRTCPresenceContactImpl + IRTCBuddyImpl {
    fn Profile();
    fn Refresh();
    fn EnumerateGroups();
    fn Groups();
    fn PresenceProperty();
    fn EnumeratePresenceDevices();
    fn PresenceDevices();
    fn SubscriptionType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCBuddy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddy2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddy2Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Groups<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PresenceDevices<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubscriptionType<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCBuddyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Profile: Profile::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            EnumerateGroups: EnumerateGroups::<Impl, IMPL_OFFSET>,
            Groups: Groups::<Impl, IMPL_OFFSET>,
            PresenceProperty: PresenceProperty::<Impl, IMPL_OFFSET>,
            EnumeratePresenceDevices: EnumeratePresenceDevices::<Impl, IMPL_OFFSET>,
            PresenceDevices: PresenceDevices::<Impl, IMPL_OFFSET>,
            SubscriptionType: SubscriptionType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddy2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyEventImpl: Sized + IDispatchImpl {
    fn Buddy();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyEventVtbl {
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Buddy: Buddy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyEvent2Impl: Sized + IDispatchImpl + IRTCBuddyEventImpl {
    fn EventType();
    fn StatusCode();
    fn StatusText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCBuddyEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCBuddyGroupImpl: Sized {
    fn Name();
    fn SetName();
    fn AddBuddy();
    fn RemoveBuddy();
    fn EnumerateBuddies();
    fn Buddies();
    fn Data();
    fn SetData();
    fn Profile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCBuddyGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyGroupVtbl {
        unsafe extern "system" fn Name<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Buddies<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Profile<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            AddBuddy: AddBuddy::<Impl, IMPL_OFFSET>,
            RemoveBuddy: RemoveBuddy::<Impl, IMPL_OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Impl, IMPL_OFFSET>,
            Buddies: Buddies::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Profile: Profile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyGroupEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Group();
    fn Buddy();
    fn StatusCode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyGroupEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroupEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyGroupEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Group<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            Buddy: Buddy::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyGroupEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClientImpl: Sized {
    fn Initialize();
    fn Shutdown();
    fn PrepareForShutdown();
    fn SetEventFilter();
    fn EventFilter();
    fn SetPreferredMediaTypes();
    fn PreferredMediaTypes();
    fn MediaCapabilities();
    fn CreateSession();
    fn SetListenForIncomingSessions();
    fn ListenForIncomingSessions();
    fn NetworkAddresses();
    fn SetVolume();
    fn Volume();
    fn SetAudioMuted();
    fn AudioMuted();
    fn IVideoWindow();
    fn SetPreferredAudioDevice();
    fn PreferredAudioDevice();
    fn SetPreferredVolume();
    fn PreferredVolume();
    fn SetPreferredAEC();
    fn PreferredAEC();
    fn SetPreferredVideoDevice();
    fn PreferredVideoDevice();
    fn ActiveMedia();
    fn SetMaxBitrate();
    fn MaxBitrate();
    fn SetTemporalSpatialTradeOff();
    fn TemporalSpatialTradeOff();
    fn NetworkQuality();
    fn StartT120Applet();
    fn StopT120Applets();
    fn IsT120AppletRunning();
    fn LocalUserURI();
    fn SetLocalUserURI();
    fn LocalUserName();
    fn SetLocalUserName();
    fn PlayRing();
    fn SendDTMF();
    fn InvokeTuningWizard();
    fn IsTuned();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientVtbl {
        unsafe extern "system" fn Initialize<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrepareForShutdown<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventFilter<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventFilter<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredMediaTypes<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCapabilities<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSession<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ListenForIncomingSessions<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkAddresses<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftcp: i16, fexternal: i16, pvaddresses: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Volume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAudioMuted<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AudioMuted<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IVideoWindow<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredAudioDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredAudioDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredAEC<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredAEC<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredVideoDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActiveMedia<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxBitrate<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxBitrate<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkQuality<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartT120Applet<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopT120Applets<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsT120AppletRunning<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalUserURI<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalUserURI<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalUserName<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalUserName<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlayRing<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendDTMF<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeTuningWizard<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTuned<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftuned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            PrepareForShutdown: PrepareForShutdown::<Impl, IMPL_OFFSET>,
            SetEventFilter: SetEventFilter::<Impl, IMPL_OFFSET>,
            EventFilter: EventFilter::<Impl, IMPL_OFFSET>,
            SetPreferredMediaTypes: SetPreferredMediaTypes::<Impl, IMPL_OFFSET>,
            PreferredMediaTypes: PreferredMediaTypes::<Impl, IMPL_OFFSET>,
            MediaCapabilities: MediaCapabilities::<Impl, IMPL_OFFSET>,
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
            SetListenForIncomingSessions: SetListenForIncomingSessions::<Impl, IMPL_OFFSET>,
            ListenForIncomingSessions: ListenForIncomingSessions::<Impl, IMPL_OFFSET>,
            NetworkAddresses: NetworkAddresses::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetAudioMuted: SetAudioMuted::<Impl, IMPL_OFFSET>,
            AudioMuted: AudioMuted::<Impl, IMPL_OFFSET>,
            IVideoWindow: IVideoWindow::<Impl, IMPL_OFFSET>,
            SetPreferredAudioDevice: SetPreferredAudioDevice::<Impl, IMPL_OFFSET>,
            PreferredAudioDevice: PreferredAudioDevice::<Impl, IMPL_OFFSET>,
            SetPreferredVolume: SetPreferredVolume::<Impl, IMPL_OFFSET>,
            PreferredVolume: PreferredVolume::<Impl, IMPL_OFFSET>,
            SetPreferredAEC: SetPreferredAEC::<Impl, IMPL_OFFSET>,
            PreferredAEC: PreferredAEC::<Impl, IMPL_OFFSET>,
            SetPreferredVideoDevice: SetPreferredVideoDevice::<Impl, IMPL_OFFSET>,
            PreferredVideoDevice: PreferredVideoDevice::<Impl, IMPL_OFFSET>,
            ActiveMedia: ActiveMedia::<Impl, IMPL_OFFSET>,
            SetMaxBitrate: SetMaxBitrate::<Impl, IMPL_OFFSET>,
            MaxBitrate: MaxBitrate::<Impl, IMPL_OFFSET>,
            SetTemporalSpatialTradeOff: SetTemporalSpatialTradeOff::<Impl, IMPL_OFFSET>,
            TemporalSpatialTradeOff: TemporalSpatialTradeOff::<Impl, IMPL_OFFSET>,
            NetworkQuality: NetworkQuality::<Impl, IMPL_OFFSET>,
            StartT120Applet: StartT120Applet::<Impl, IMPL_OFFSET>,
            StopT120Applets: StopT120Applets::<Impl, IMPL_OFFSET>,
            IsT120AppletRunning: IsT120AppletRunning::<Impl, IMPL_OFFSET>,
            LocalUserURI: LocalUserURI::<Impl, IMPL_OFFSET>,
            SetLocalUserURI: SetLocalUserURI::<Impl, IMPL_OFFSET>,
            LocalUserName: LocalUserName::<Impl, IMPL_OFFSET>,
            SetLocalUserName: SetLocalUserName::<Impl, IMPL_OFFSET>,
            PlayRing: PlayRing::<Impl, IMPL_OFFSET>,
            SendDTMF: SendDTMF::<Impl, IMPL_OFFSET>,
            InvokeTuningWizard: InvokeTuningWizard::<Impl, IMPL_OFFSET>,
            IsTuned: IsTuned::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClient2Impl: Sized + IRTCClientImpl {
    fn SetAnswerMode();
    fn AnswerMode();
    fn InvokeTuningWizardEx();
    fn Version();
    fn SetClientName();
    fn SetClientCurVer();
    fn InitializeEx();
    fn CreateSessionWithDescription();
    fn SetSessionDescriptionManager();
    fn SetPreferredSecurityLevel();
    fn PreferredSecurityLevel();
    fn SetAllowedPorts();
    fn AllowedPorts();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClient2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClient2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClient2Vtbl {
        unsafe extern "system" fn SetAnswerMode<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnswerMode<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientName<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientCurVer<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientcurver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeEx<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSessionWithDescription<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowedPorts<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowedPorts<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCClientVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAnswerMode: SetAnswerMode::<Impl, IMPL_OFFSET>,
            AnswerMode: AnswerMode::<Impl, IMPL_OFFSET>,
            InvokeTuningWizardEx: InvokeTuningWizardEx::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetClientName: SetClientName::<Impl, IMPL_OFFSET>,
            SetClientCurVer: SetClientCurVer::<Impl, IMPL_OFFSET>,
            InitializeEx: InitializeEx::<Impl, IMPL_OFFSET>,
            CreateSessionWithDescription: CreateSessionWithDescription::<Impl, IMPL_OFFSET>,
            SetSessionDescriptionManager: SetSessionDescriptionManager::<Impl, IMPL_OFFSET>,
            SetPreferredSecurityLevel: SetPreferredSecurityLevel::<Impl, IMPL_OFFSET>,
            PreferredSecurityLevel: PreferredSecurityLevel::<Impl, IMPL_OFFSET>,
            SetAllowedPorts: SetAllowedPorts::<Impl, IMPL_OFFSET>,
            AllowedPorts: AllowedPorts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClient2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClientEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Client();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCClientEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Client<Impl: IRTCClientEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            Client: Client::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCClientPortManagementImpl: Sized {
    fn StartListenAddressAndPort();
    fn StopListenAddressAndPort();
    fn GetPortRange();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCClientPortManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPortManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPortManagementVtbl {
        unsafe extern "system" fn StartListenAddressAndPort<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopListenAddressAndPort<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPortRange<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartListenAddressAndPort: StartListenAddressAndPort::<Impl, IMPL_OFFSET>,
            StopListenAddressAndPort: StopListenAddressAndPort::<Impl, IMPL_OFFSET>,
            GetPortRange: GetPortRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientPortManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClientPresenceImpl: Sized {
    fn EnablePresence();
    fn Export();
    fn Import();
    fn EnumerateBuddies();
    fn Buddies();
    fn Buddy();
    fn AddBuddy();
    fn RemoveBuddy();
    fn EnumerateWatchers();
    fn Watchers();
    fn Watcher();
    fn AddWatcher();
    fn RemoveWatcher();
    fn SetLocalPresenceInfo();
    fn OfferWatcherMode();
    fn SetOfferWatcherMode();
    fn PrivacyMode();
    fn SetPrivacyMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPresenceVtbl {
        unsafe extern "system" fn EnablePresence<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Export<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Import<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Buddies<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Buddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateWatchers<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Watchers<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Watcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddWatcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveWatcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwatcher: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OfferWatcherMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOfferWatcherMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivacyMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivacyMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnablePresence: EnablePresence::<Impl, IMPL_OFFSET>,
            Export: Export::<Impl, IMPL_OFFSET>,
            Import: Import::<Impl, IMPL_OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Impl, IMPL_OFFSET>,
            Buddies: Buddies::<Impl, IMPL_OFFSET>,
            Buddy: Buddy::<Impl, IMPL_OFFSET>,
            AddBuddy: AddBuddy::<Impl, IMPL_OFFSET>,
            RemoveBuddy: RemoveBuddy::<Impl, IMPL_OFFSET>,
            EnumerateWatchers: EnumerateWatchers::<Impl, IMPL_OFFSET>,
            Watchers: Watchers::<Impl, IMPL_OFFSET>,
            Watcher: Watcher::<Impl, IMPL_OFFSET>,
            AddWatcher: AddWatcher::<Impl, IMPL_OFFSET>,
            RemoveWatcher: RemoveWatcher::<Impl, IMPL_OFFSET>,
            SetLocalPresenceInfo: SetLocalPresenceInfo::<Impl, IMPL_OFFSET>,
            OfferWatcherMode: OfferWatcherMode::<Impl, IMPL_OFFSET>,
            SetOfferWatcherMode: SetOfferWatcherMode::<Impl, IMPL_OFFSET>,
            PrivacyMode: PrivacyMode::<Impl, IMPL_OFFSET>,
            SetPrivacyMode: SetPrivacyMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientPresence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClientPresence2Impl: Sized + IRTCClientPresenceImpl {
    fn EnablePresenceEx();
    fn DisablePresence();
    fn AddGroup();
    fn RemoveGroup();
    fn EnumerateGroups();
    fn Groups();
    fn Group();
    fn AddWatcherEx();
    fn WatcherEx();
    fn SetPresenceProperty();
    fn PresenceProperty();
    fn SetPresenceData();
    fn GetPresenceData();
    fn GetLocalPresenceInfo();
    fn AddBuddyEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresence2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresence2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPresence2Vtbl {
        unsafe extern "system" fn EnablePresenceEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisablePresence<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddGroup<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveGroup<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Groups<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Group<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddWatcherEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WatcherEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPresenceProperty<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPresenceData<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBuddyEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCClientPresenceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnablePresenceEx: EnablePresenceEx::<Impl, IMPL_OFFSET>,
            DisablePresence: DisablePresence::<Impl, IMPL_OFFSET>,
            AddGroup: AddGroup::<Impl, IMPL_OFFSET>,
            RemoveGroup: RemoveGroup::<Impl, IMPL_OFFSET>,
            EnumerateGroups: EnumerateGroups::<Impl, IMPL_OFFSET>,
            Groups: Groups::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            AddWatcherEx: AddWatcherEx::<Impl, IMPL_OFFSET>,
            WatcherEx: WatcherEx::<Impl, IMPL_OFFSET>,
            SetPresenceProperty: SetPresenceProperty::<Impl, IMPL_OFFSET>,
            PresenceProperty: PresenceProperty::<Impl, IMPL_OFFSET>,
            SetPresenceData: SetPresenceData::<Impl, IMPL_OFFSET>,
            GetPresenceData: GetPresenceData::<Impl, IMPL_OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Impl, IMPL_OFFSET>,
            AddBuddyEx: AddBuddyEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientPresence2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCClientProvisioningImpl: Sized {
    fn CreateProfile();
    fn EnableProfile();
    fn DisableProfile();
    fn EnumerateProfiles();
    fn Profiles();
    fn GetProfile();
    fn SessionCapabilities();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCClientProvisioningVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioningImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientProvisioningVtbl {
        unsafe extern "system" fn CreateProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofilexml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateProfiles<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Profiles<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateProfile: CreateProfile::<Impl, IMPL_OFFSET>,
            EnableProfile: EnableProfile::<Impl, IMPL_OFFSET>,
            DisableProfile: DisableProfile::<Impl, IMPL_OFFSET>,
            EnumerateProfiles: EnumerateProfiles::<Impl, IMPL_OFFSET>,
            Profiles: Profiles::<Impl, IMPL_OFFSET>,
            GetProfile: GetProfile::<Impl, IMPL_OFFSET>,
            SessionCapabilities: SessionCapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientProvisioning as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCClientProvisioning2Impl: Sized + IRTCClientProvisioningImpl {
    fn EnableProfileEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCClientProvisioning2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioning2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientProvisioning2Vtbl {
        unsafe extern "system" fn EnableProfileEx<Impl: IRTCClientProvisioning2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IRTCClientProvisioningVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EnableProfileEx: EnableProfileEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientProvisioning2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCDispatchEventNotificationImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCDispatchEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCDispatchEventNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCDispatchEventNotificationVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCDispatchEventNotification as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumBuddiesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IRTCEnumBuddiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumBuddiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumBuddiesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumBuddies as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumGroupsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IRTCEnumGroupsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumGroupsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumGroupsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumGroups as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumParticipantsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IRTCEnumParticipantsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumParticipantsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumParticipantsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumParticipants as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumPresenceDevicesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IRTCEnumPresenceDevicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumPresenceDevicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumPresenceDevicesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumPresenceDevices as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumProfilesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IRTCEnumProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumProfilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumProfilesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumProfiles as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumUserSearchResultsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IRTCEnumUserSearchResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumUserSearchResultsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumUserSearchResultsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumUserSearchResults as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumWatchersImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IRTCEnumWatchersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumWatchersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumWatchersVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumWatchers as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCEventNotificationImpl: Sized {
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEventNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEventNotificationVtbl {
        unsafe extern "system" fn Event<Impl: IRTCEventNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Event: Event::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEventNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCInfoEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Participant();
    fn Info();
    fn InfoHeader();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCInfoEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCInfoEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCInfoEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Participant<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Info<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InfoHeader<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            Participant: Participant::<Impl, IMPL_OFFSET>,
            Info: Info::<Impl, IMPL_OFFSET>,
            InfoHeader: InfoHeader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCInfoEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCIntensityEventImpl: Sized + IDispatchImpl {
    fn Level();
    fn Min();
    fn Max();
    fn Direction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCIntensityEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCIntensityEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCIntensityEventVtbl {
        unsafe extern "system" fn Level<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Min<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Max<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Direction<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Level: Level::<Impl, IMPL_OFFSET>,
            Min: Min::<Impl, IMPL_OFFSET>,
            Max: Max::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCIntensityEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCMediaEventImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn EventType();
    fn EventReason();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMediaEventVtbl {
        unsafe extern "system" fn MediaType<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventType<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventReason<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            EventType: EventType::<Impl, IMPL_OFFSET>,
            EventReason: EventReason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCMediaEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCMediaRequestEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ProposedMedia();
    fn CurrentMedia();
    fn Accept();
    fn RemotePreferredSecurityLevel();
    fn Reject();
    fn State();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaRequestEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaRequestEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMediaRequestEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProposedMedia<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentMedia<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Accept<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reject<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            ProposedMedia: ProposedMedia::<Impl, IMPL_OFFSET>,
            CurrentMedia: CurrentMedia::<Impl, IMPL_OFFSET>,
            Accept: Accept::<Impl, IMPL_OFFSET>,
            RemotePreferredSecurityLevel: RemotePreferredSecurityLevel::<Impl, IMPL_OFFSET>,
            Reject: Reject::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCMediaRequestEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCMessagingEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Participant();
    fn EventType();
    fn Message();
    fn MessageHeader();
    fn UserStatus();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMessagingEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMessagingEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMessagingEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Participant<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventType<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Message<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MessageHeader<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserStatus<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            Participant: Participant::<Impl, IMPL_OFFSET>,
            EventType: EventType::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            MessageHeader: MessageHeader::<Impl, IMPL_OFFSET>,
            UserStatus: UserStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCMessagingEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCParticipantImpl: Sized {
    fn UserURI();
    fn Name();
    fn Removable();
    fn State();
    fn Session();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCParticipantVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipantImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCParticipantVtbl {
        unsafe extern "system" fn UserURI<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Removable<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfremovable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Session<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            UserURI: UserURI::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Removable: Removable::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Session: Session::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCParticipant as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCParticipantStateChangeEventImpl: Sized + IDispatchImpl {
    fn Participant();
    fn State();
    fn StatusCode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCParticipantStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipantStateChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCParticipantStateChangeEventVtbl {
        unsafe extern "system" fn Participant<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Participant: Participant::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCParticipantStateChangeEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCPortManagerImpl: Sized {
    fn GetMapping();
    fn UpdateRemoteAddress();
    fn ReleaseMapping();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPortManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPortManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPortManagerVtbl {
        unsafe extern "system" fn GetMapping<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateRemoteAddress<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseMapping<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocaladdress: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMapping: GetMapping::<Impl, IMPL_OFFSET>,
            UpdateRemoteAddress: UpdateRemoteAddress::<Impl, IMPL_OFFSET>,
            ReleaseMapping: ReleaseMapping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPortManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCPresenceContactImpl: Sized {
    fn PresentityURI();
    fn SetPresentityURI();
    fn Name();
    fn SetName();
    fn Data();
    fn SetData();
    fn Persistent();
    fn SetPersistent();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPresenceContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceContactImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceContactVtbl {
        unsafe extern "system" fn PresentityURI<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPresentityURI<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Persistent<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpersistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPersistent<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersistent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PresentityURI: PresentityURI::<Impl, IMPL_OFFSET>,
            SetPresentityURI: SetPresentityURI::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Persistent: Persistent::<Impl, IMPL_OFFSET>,
            SetPersistent: SetPersistent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceContact as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCPresenceDataEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn GetPresenceData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceDataEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDataEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceDataEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
            GetPresenceData: GetPresenceData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceDataEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCPresenceDeviceImpl: Sized {
    fn Status();
    fn Notes();
    fn PresenceProperty();
    fn GetPresenceData();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPresenceDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceDeviceVtbl {
        unsafe extern "system" fn Status<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notes<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Notes: Notes::<Impl, IMPL_OFFSET>,
            PresenceProperty: PresenceProperty::<Impl, IMPL_OFFSET>,
            GetPresenceData: GetPresenceData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCPresencePropertyEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn PresenceProperty();
    fn Value();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresencePropertyEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresencePropertyEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresencePropertyEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
            PresenceProperty: PresenceProperty::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresencePropertyEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCPresenceStatusEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn GetLocalPresenceInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceStatusEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceStatusEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceStatusEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCProfileImpl: Sized {
    fn Key();
    fn Name();
    fn XML();
    fn ProviderName();
    fn ProviderURI();
    fn ProviderData();
    fn ClientName();
    fn ClientBanner();
    fn ClientMinVer();
    fn ClientCurVer();
    fn ClientUpdateURI();
    fn ClientData();
    fn UserURI();
    fn UserName();
    fn UserAccount();
    fn SetCredentials();
    fn SessionCapabilities();
    fn State();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileVtbl {
        unsafe extern "system" fn Key<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn XML<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProviderData<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientBanner<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbanner: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientMinVer<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrminver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientCurVer<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientUpdateURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientData<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserAccount<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredentials<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Key: Key::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            XML: XML::<Impl, IMPL_OFFSET>,
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            ProviderURI: ProviderURI::<Impl, IMPL_OFFSET>,
            ProviderData: ProviderData::<Impl, IMPL_OFFSET>,
            ClientName: ClientName::<Impl, IMPL_OFFSET>,
            ClientBanner: ClientBanner::<Impl, IMPL_OFFSET>,
            ClientMinVer: ClientMinVer::<Impl, IMPL_OFFSET>,
            ClientCurVer: ClientCurVer::<Impl, IMPL_OFFSET>,
            ClientUpdateURI: ClientUpdateURI::<Impl, IMPL_OFFSET>,
            ClientData: ClientData::<Impl, IMPL_OFFSET>,
            UserURI: UserURI::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            UserAccount: UserAccount::<Impl, IMPL_OFFSET>,
            SetCredentials: SetCredentials::<Impl, IMPL_OFFSET>,
            SessionCapabilities: SessionCapabilities::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCProfile2Impl: Sized + IRTCProfileImpl {
    fn Realm();
    fn SetRealm();
    fn AllowedAuth();
    fn SetAllowedAuth();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfile2Vtbl {
        unsafe extern "system" fn Realm<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrealm: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRealm<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowedAuth<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowedAuth<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCProfileVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Realm: Realm::<Impl, IMPL_OFFSET>,
            SetRealm: SetRealm::<Impl, IMPL_OFFSET>,
            AllowedAuth: AllowedAuth::<Impl, IMPL_OFFSET>,
            SetAllowedAuth: SetAllowedAuth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCProfileEventImpl: Sized + IDispatchImpl {
    fn Profile();
    fn Cookie();
    fn StatusCode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileEventVtbl {
        unsafe extern "system" fn Profile<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cookie<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Profile: Profile::<Impl, IMPL_OFFSET>,
            Cookie: Cookie::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfileEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCProfileEvent2Impl: Sized + IDispatchImpl + IRTCProfileEventImpl {
    fn EventType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCProfileEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IRTCProfileEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EventType: EventType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfileEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCReInviteEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Accept();
    fn Reject();
    fn State();
    fn GetRemoteSessionDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCReInviteEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCReInviteEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCReInviteEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Accept<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reject<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            Accept: Accept::<Impl, IMPL_OFFSET>,
            Reject: Reject::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCReInviteEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCRegistrationStateChangeEventImpl: Sized + IDispatchImpl {
    fn Profile();
    fn State();
    fn StatusCode();
    fn StatusText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRegistrationStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRegistrationStateChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCRegistrationStateChangeEventVtbl {
        unsafe extern "system" fn Profile<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Profile: Profile::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCRegistrationStateChangeEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCRoamingEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Profile();
    fn StatusCode();
    fn StatusText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRoamingEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRoamingEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCRoamingEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Profile<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            Profile: Profile::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCRoamingEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCSessionImpl: Sized {
    fn Client();
    fn State();
    fn Type();
    fn Profile();
    fn Participants();
    fn Answer();
    fn Terminate();
    fn Redirect();
    fn AddParticipant();
    fn RemoveParticipant();
    fn EnumerateParticipants();
    fn CanAddParticipants();
    fn RedirectedUserURI();
    fn RedirectedUserName();
    fn NextRedirectedUser();
    fn SendMessage();
    fn SendMessageStatus();
    fn AddStream();
    fn RemoveStream();
    fn SetEncryptionKey();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionVtbl {
        unsafe extern "system" fn Client<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Profile<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Participants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Answer<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminate<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Redirect<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddParticipant<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveParticipant<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparticipant: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateParticipants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanAddParticipants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcanadd: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectedUserURI<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectedUserName<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextRedirectedUser<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendMessage<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendMessageStatus<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStream<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStream<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptionKey<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Client: Client::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Profile: Profile::<Impl, IMPL_OFFSET>,
            Participants: Participants::<Impl, IMPL_OFFSET>,
            Answer: Answer::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
            Redirect: Redirect::<Impl, IMPL_OFFSET>,
            AddParticipant: AddParticipant::<Impl, IMPL_OFFSET>,
            RemoveParticipant: RemoveParticipant::<Impl, IMPL_OFFSET>,
            EnumerateParticipants: EnumerateParticipants::<Impl, IMPL_OFFSET>,
            CanAddParticipants: CanAddParticipants::<Impl, IMPL_OFFSET>,
            RedirectedUserURI: RedirectedUserURI::<Impl, IMPL_OFFSET>,
            RedirectedUserName: RedirectedUserName::<Impl, IMPL_OFFSET>,
            NextRedirectedUser: NextRedirectedUser::<Impl, IMPL_OFFSET>,
            SendMessage: SendMessage::<Impl, IMPL_OFFSET>,
            SendMessageStatus: SendMessageStatus::<Impl, IMPL_OFFSET>,
            AddStream: AddStream::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
            SetEncryptionKey: SetEncryptionKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCSession2Impl: Sized + IRTCSessionImpl {
    fn SendInfo();
    fn SetPreferredSecurityLevel();
    fn PreferredSecurityLevel();
    fn IsSecurityEnabled();
    fn AnswerWithSessionDescription();
    fn ReInviteWithSessionDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSession2Vtbl {
        unsafe extern "system" fn SendInfo<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinfo: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCSessionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SendInfo: SendInfo::<Impl, IMPL_OFFSET>,
            SetPreferredSecurityLevel: SetPreferredSecurityLevel::<Impl, IMPL_OFFSET>,
            PreferredSecurityLevel: PreferredSecurityLevel::<Impl, IMPL_OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Impl, IMPL_OFFSET>,
            AnswerWithSessionDescription: AnswerWithSessionDescription::<Impl, IMPL_OFFSET>,
            ReInviteWithSessionDescription: ReInviteWithSessionDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCSessionCallControlImpl: Sized {
    fn Hold();
    fn UnHold();
    fn Forward();
    fn Refer();
    fn SetReferredByURI();
    fn ReferredByURI();
    fn SetReferCookie();
    fn ReferCookie();
    fn IsReferred();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionCallControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionCallControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionCallControlVtbl {
        unsafe extern "system" fn Hold<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnHold<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forward<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refer<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefertouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReferredByURI<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReferCookie<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsReferred<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreferred: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Hold: Hold::<Impl, IMPL_OFFSET>,
            UnHold: UnHold::<Impl, IMPL_OFFSET>,
            Forward: Forward::<Impl, IMPL_OFFSET>,
            Refer: Refer::<Impl, IMPL_OFFSET>,
            SetReferredByURI: SetReferredByURI::<Impl, IMPL_OFFSET>,
            ReferredByURI: ReferredByURI::<Impl, IMPL_OFFSET>,
            SetReferCookie: SetReferCookie::<Impl, IMPL_OFFSET>,
            ReferCookie: ReferCookie::<Impl, IMPL_OFFSET>,
            IsReferred: IsReferred::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionCallControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCSessionDescriptionManagerImpl: Sized {
    fn EvaluateSessionDescription();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionDescriptionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionDescriptionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionDescriptionManagerVtbl {
        unsafe extern "system" fn EvaluateSessionDescription<Impl: IRTCSessionDescriptionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfapplicationsession: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EvaluateSessionDescription: EvaluateSessionDescription::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionDescriptionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionOperationCompleteEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Cookie();
    fn StatusCode();
    fn StatusText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionOperationCompleteEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cookie<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            Cookie: Cookie::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionOperationCompleteEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionOperationCompleteEvent2Impl: Sized + IDispatchImpl + IRTCSessionOperationCompleteEventImpl {
    fn Participant();
    fn GetRemoteSessionDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionOperationCompleteEvent2Vtbl {
        unsafe extern "system" fn Participant<Impl: IRTCSessionOperationCompleteEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionOperationCompleteEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCSessionOperationCompleteEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Participant: Participant::<Impl, IMPL_OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionOperationCompleteEvent2 as ::windows::core::Interface>::IID
    }
}
pub trait IRTCSessionPortManagementImpl: Sized {
    fn SetPortManager();
}
impl IRTCSessionPortManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionPortManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionPortManagementVtbl {
        unsafe extern "system" fn SetPortManager<Impl: IRTCSessionPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetPortManager: SetPortManager::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionPortManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionReferStatusEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ReferStatus();
    fn StatusCode();
    fn StatusText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferStatusEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionReferStatusEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferStatus<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            ReferStatus: ReferStatus::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionReferStatusEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionReferredEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ReferredByURI();
    fn ReferToURI();
    fn ReferCookie();
    fn Accept();
    fn Reject();
    fn SetReferredSessionState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferredEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferredEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionReferredEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferToURI<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Accept<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reject<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReferredSessionState<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            ReferredByURI: ReferredByURI::<Impl, IMPL_OFFSET>,
            ReferToURI: ReferToURI::<Impl, IMPL_OFFSET>,
            ReferCookie: ReferCookie::<Impl, IMPL_OFFSET>,
            Accept: Accept::<Impl, IMPL_OFFSET>,
            Reject: Reject::<Impl, IMPL_OFFSET>,
            SetReferredSessionState: SetReferredSessionState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionReferredEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionStateChangeEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn State();
    fn StatusCode();
    fn StatusText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionStateChangeEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            StatusText: StatusText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionStateChangeEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionStateChangeEvent2Impl: Sized + IDispatchImpl + IRTCSessionStateChangeEventImpl {
    fn MediaTypes();
    fn RemotePreferredSecurityLevel();
    fn IsForked();
    fn GetRemoteSessionDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionStateChangeEvent2Vtbl {
        unsafe extern "system" fn MediaTypes<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsForked<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisforked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCSessionStateChangeEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MediaTypes: MediaTypes::<Impl, IMPL_OFFSET>,
            RemotePreferredSecurityLevel: RemotePreferredSecurityLevel::<Impl, IMPL_OFFSET>,
            IsForked: IsForked::<Impl, IMPL_OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionStateChangeEvent2 as ::windows::core::Interface>::IID
    }
}
pub trait IRTCUserSearchImpl: Sized {
    fn CreateQuery();
    fn ExecuteSearch();
}
impl IRTCUserSearchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchVtbl {
        unsafe extern "system" fn CreateQuery<Impl: IRTCUserSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteSearch<Impl: IRTCUserSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateQuery: CreateQuery::<Impl, IMPL_OFFSET>,
            ExecuteSearch: ExecuteSearch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCUserSearchQueryImpl: Sized {
    fn SetSearchTerm();
    fn SearchTerm();
    fn SearchTerms();
    fn SetSearchPreference();
    fn SearchPreference();
    fn SetSearchDomain();
    fn SearchDomain();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCUserSearchQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchQueryVtbl {
        unsafe extern "system" fn SetSearchTerm<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchTerm<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchTerms<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSearchPreference<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchPreference<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSearchDomain<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SearchDomain<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetSearchTerm: SetSearchTerm::<Impl, IMPL_OFFSET>,
            SearchTerm: SearchTerm::<Impl, IMPL_OFFSET>,
            SearchTerms: SearchTerms::<Impl, IMPL_OFFSET>,
            SetSearchPreference: SetSearchPreference::<Impl, IMPL_OFFSET>,
            SearchPreference: SearchPreference::<Impl, IMPL_OFFSET>,
            SetSearchDomain: SetSearchDomain::<Impl, IMPL_OFFSET>,
            SearchDomain: SearchDomain::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearchQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCUserSearchResultImpl: Sized {
    fn Value();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCUserSearchResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchResultVtbl {
        unsafe extern "system" fn Value<Impl: IRTCUserSearchResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Value: Value::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearchResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCUserSearchResultsEventImpl: Sized + IDispatchImpl {
    fn EnumerateResults();
    fn Results();
    fn Profile();
    fn Query();
    fn Cookie();
    fn StatusCode();
    fn MoreAvailable();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCUserSearchResultsEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResultsEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchResultsEventVtbl {
        unsafe extern "system" fn EnumerateResults<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Results<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Profile<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cookie<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoreAvailable<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumerateResults: EnumerateResults::<Impl, IMPL_OFFSET>,
            Results: Results::<Impl, IMPL_OFFSET>,
            Profile: Profile::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            Cookie: Cookie::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            MoreAvailable: MoreAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearchResultsEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCWatcherImpl: Sized + IRTCPresenceContactImpl {
    fn State();
    fn SetState();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherVtbl {
        unsafe extern "system" fn State<Impl: IRTCWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetState<Impl: IRTCWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCPresenceContactVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCWatcher2Impl: Sized + IRTCPresenceContactImpl + IRTCWatcherImpl {
    fn Profile();
    fn Scope();
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcher2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcher2Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scope<Impl: IRTCWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCWatcherVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Profile: Profile::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCWatcherEventImpl: Sized + IDispatchImpl {
    fn Watcher();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherEventVtbl {
        unsafe extern "system" fn Watcher<Impl: IRTCWatcherEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Watcher: Watcher::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcherEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCWatcherEvent2Impl: Sized + IDispatchImpl + IRTCWatcherEventImpl {
    fn EventType();
    fn StatusCode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCWatcherEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCWatcherEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IRTCWatcherEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcherEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait ITransportSettingsInternalImpl: Sized {
    fn ApplySetting();
    fn QuerySetting();
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ITransportSettingsInternalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransportSettingsInternalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransportSettingsInternalVtbl {
        unsafe extern "system" fn ApplySetting<Impl: ITransportSettingsInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuerySetting<Impl: ITransportSettingsInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ApplySetting: ApplySetting::<Impl, IMPL_OFFSET>,
            QuerySetting: QuerySetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransportSettingsInternal as ::windows::core::Interface>::IID
    }
}
