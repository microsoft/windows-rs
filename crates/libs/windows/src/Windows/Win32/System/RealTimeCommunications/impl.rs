#[cfg(feature = "Win32_Networking_WinSock")]
pub trait INetworkTransportSettingsImpl: Sized {
    fn ApplySetting(&mut self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()>;
    fn QuerySetting(&mut self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl INetworkTransportSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkTransportSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkTransportSettingsVtbl {
        unsafe extern "system" fn ApplySetting<Impl: INetworkTransportSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplySetting(::core::mem::transmute_copy(&settingid), ::core::mem::transmute_copy(&lengthin), ::core::mem::transmute_copy(&valuein), ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)).into()
        }
        unsafe extern "system" fn QuerySetting<Impl: INetworkTransportSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QuerySetting(::core::mem::transmute_copy(&settingid), ::core::mem::transmute_copy(&lengthin), ::core::mem::transmute_copy(&valuein), ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)).into()
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
    fn CompleteDelivery(&mut self) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
impl INotificationTransportSyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationTransportSyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationTransportSyncVtbl {
        unsafe extern "system" fn CompleteDelivery<Impl: INotificationTransportSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteDelivery().into()
        }
        unsafe extern "system" fn Flush<Impl: INotificationTransportSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
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
    fn Status(&mut self) -> ::windows::core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCBuddyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyVtbl {
        unsafe extern "system" fn Status<Impl: IRTCBuddyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *penstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Impl: IRTCBuddyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notes() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnotes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn EnumerateGroups(&mut self) -> ::windows::core::Result<IRTCEnumGroups>;
    fn Groups(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn PresenceProperty(&mut self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumeratePresenceDevices(&mut self) -> ::windows::core::Result<IRTCEnumPresenceDevices>;
    fn PresenceDevices(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn SubscriptionType(&mut self) -> ::windows::core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCBuddy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddy2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddy2Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePresenceDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceDevices<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevicescollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriptionType<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriptionType() {
                ::core::result::Result::Ok(ok__) => {
                    *pensubscriptiontype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Buddy(&mut self) -> ::windows::core::Result<IRTCBuddy>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyEventVtbl {
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Buddy: Buddy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyEvent2Impl: Sized + IDispatchImpl + IRTCBuddyEventImpl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_BUDDY_EVENT_TYPE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrgroupname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddBuddy(&mut self, pbuddy: ::core::option::Option<IRTCBuddy>) -> ::windows::core::Result<()>;
    fn RemoveBuddy(&mut self, pbuddy: ::core::option::Option<IRTCBuddy>) -> ::windows::core::Result<()>;
    fn EnumerateBuddies(&mut self) -> ::windows::core::Result<IRTCEnumBuddies>;
    fn Buddies(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetData(&mut self, bstrdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCBuddyGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyGroupVtbl {
        unsafe extern "system" fn Name<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrgroupname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrgroupname)).into()
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBuddy(::core::mem::transmute(&pbuddy)).into()
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBuddy(::core::mem::transmute(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateBuddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn Profile<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn EventType(&mut self) -> ::windows::core::Result<RTC_GROUP_EVENT_TYPE>;
    fn Group(&mut self) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn Buddy(&mut self) -> ::windows::core::Result<IRTCBuddy2>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyGroupEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroupEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyGroupEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn PrepareForShutdown(&mut self) -> ::windows::core::Result<()>;
    fn SetEventFilter(&mut self, lfilter: i32) -> ::windows::core::Result<()>;
    fn EventFilter(&mut self) -> ::windows::core::Result<i32>;
    fn SetPreferredMediaTypes(&mut self, lmediatypes: i32, fpersistent: i16) -> ::windows::core::Result<()>;
    fn PreferredMediaTypes(&mut self) -> ::windows::core::Result<i32>;
    fn MediaCapabilities(&mut self) -> ::windows::core::Result<i32>;
    fn CreateSession(&mut self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: super::super::Foundation::BSTR, pprofile: ::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCSession>;
    fn SetListenForIncomingSessions(&mut self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()>;
    fn ListenForIncomingSessions(&mut self) -> ::windows::core::Result<RTC_LISTEN_MODE>;
    fn NetworkAddresses(&mut self, ftcp: i16, fexternal: i16) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetVolume(&mut self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()>;
    fn Volume(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32>;
    fn SetAudioMuted(&mut self, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::Result<()>;
    fn AudioMuted(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i16>;
    fn IVideoWindow(&mut self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow>;
    fn SetPreferredAudioDevice(&mut self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PreferredAudioDevice(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPreferredVolume(&mut self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()>;
    fn PreferredVolume(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32>;
    fn SetPreferredAEC(&mut self, benable: i16) -> ::windows::core::Result<()>;
    fn PreferredAEC(&mut self) -> ::windows::core::Result<i16>;
    fn SetPreferredVideoDevice(&mut self, bstrdevicename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PreferredVideoDevice(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ActiveMedia(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxBitrate(&mut self, lmaxbitrate: i32) -> ::windows::core::Result<()>;
    fn MaxBitrate(&mut self) -> ::windows::core::Result<i32>;
    fn SetTemporalSpatialTradeOff(&mut self, lvalue: i32) -> ::windows::core::Result<()>;
    fn TemporalSpatialTradeOff(&mut self) -> ::windows::core::Result<i32>;
    fn NetworkQuality(&mut self) -> ::windows::core::Result<i32>;
    fn StartT120Applet(&mut self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<()>;
    fn StopT120Applets(&mut self) -> ::windows::core::Result<()>;
    fn IsT120AppletRunning(&mut self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<i16>;
    fn LocalUserURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalUserURI(&mut self, bstruseruri: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalUserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalUserName(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlayRing(&mut self, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::Result<()>;
    fn SendDTMF(&mut self, endtmf: RTC_DTMF) -> ::windows::core::Result<()>;
    fn InvokeTuningWizard(&mut self, hwndparent: isize) -> ::windows::core::Result<()>;
    fn IsTuned(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientVtbl {
        unsafe extern "system" fn Initialize<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn Shutdown<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn PrepareForShutdown<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareForShutdown().into()
        }
        unsafe extern "system" fn SetEventFilter<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&lfilter)).into()
        }
        unsafe extern "system" fn EventFilter<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *plfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredMediaTypes(::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&fpersistent)).into()
        }
        unsafe extern "system" fn PreferredMediaTypes<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredMediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCapabilities<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bstrlocalphoneuri), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListenForIncomingSessions(::core::mem::transmute_copy(&enlisten)).into()
        }
        unsafe extern "system" fn ListenForIncomingSessions<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenForIncomingSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *penlisten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAddresses<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftcp: i16, fexternal: i16, pvaddresses: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAddresses(::core::mem::transmute_copy(&ftcp), ::core::mem::transmute_copy(&fexternal)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvaddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn Volume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioMuted<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioMuted(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&fmuted)).into()
        }
        unsafe extern "system" fn AudioMuted<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioMuted(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmuted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IVideoWindow<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IVideoWindow(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppivideowindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAudioDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredAudioDevice(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn PreferredAudioDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAudioDevice(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredVolume(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn PreferredVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredVolume(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAEC<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredAEC(::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn PreferredAEC<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAEC() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredVideoDevice(::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn PreferredVideoDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredVideoDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveMedia<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBitrate<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBitrate(::core::mem::transmute_copy(&lmaxbitrate)).into()
        }
        unsafe extern "system" fn MaxBitrate<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxbitrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTemporalSpatialTradeOff(::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemporalSpatialTradeOff() {
                ::core::result::Result::Ok(ok__) => {
                    *plvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkQuality<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *plnetworkquality = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartT120Applet<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartT120Applet(::core::mem::transmute_copy(&enapplet)).into()
        }
        unsafe extern "system" fn StopT120Applets<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopT120Applets().into()
        }
        unsafe extern "system" fn IsT120AppletRunning<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsT120AppletRunning(::core::mem::transmute_copy(&enapplet)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfrunning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserURI<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserURI<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalUserURI(::core::mem::transmute_copy(&bstruseruri)).into()
        }
        unsafe extern "system" fn LocalUserName<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserName<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalUserName(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn PlayRing<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayRing(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bplay)).into()
        }
        unsafe extern "system" fn SendDTMF<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDTMF(::core::mem::transmute_copy(&endtmf)).into()
        }
        unsafe extern "system" fn InvokeTuningWizard<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeTuningWizard(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn IsTuned<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftuned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTuned() {
                ::core::result::Result::Ok(ok__) => {
                    *pftuned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn SetAnswerMode(&mut self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::Result<()>;
    fn AnswerMode(&mut self, entype: RTC_SESSION_TYPE) -> ::windows::core::Result<RTC_ANSWER_MODE>;
    fn InvokeTuningWizardEx(&mut self, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<i32>;
    fn SetClientName(&mut self, bstrclientname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCurVer(&mut self, bstrclientcurver: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeEx(&mut self, lflags: i32) -> ::windows::core::Result<()>;
    fn CreateSessionWithDescription(&mut self, bstrcontenttype: super::super::Foundation::BSTR, bstrsessiondescription: super::super::Foundation::BSTR, pprofile: ::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCSession2>;
    fn SetSessionDescriptionManager(&mut self, psessiondescriptionmanager: ::core::option::Option<IRTCSessionDescriptionManager>) -> ::windows::core::Result<()>;
    fn SetPreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()>;
    fn PreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn SetAllowedPorts(&mut self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::Result<()>;
    fn AllowedPorts(&mut self, ltransport: i32) -> ::windows::core::Result<RTC_LISTEN_MODE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClient2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClient2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClient2Vtbl {
        unsafe extern "system" fn SetAnswerMode<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnswerMode(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn AnswerMode<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnswerMode(::core::mem::transmute_copy(&entype)) {
                ::core::result::Result::Ok(ok__) => {
                    *penmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeTuningWizardEx(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&fallowaudio), ::core::mem::transmute_copy(&fallowvideo)).into()
        }
        unsafe extern "system" fn Version<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *plversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&bstrclientname)).into()
        }
        unsafe extern "system" fn SetClientCurVer<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientcurver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCurVer(::core::mem::transmute_copy(&bstrclientcurver)).into()
        }
        unsafe extern "system" fn InitializeEx<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEx(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateSessionWithDescription<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionWithDescription(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionDescriptionManager(::core::mem::transmute(&psessiondescriptionmanager)).into()
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedPorts<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedPorts(::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&enlistenmode)).into()
        }
        unsafe extern "system" fn AllowedPorts<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedPorts(::core::mem::transmute_copy(&ltransport)) {
                ::core::result::Result::Ok(ok__) => {
                    *penlistenmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn EventType(&mut self) -> ::windows::core::Result<RTC_CLIENT_EVENT_TYPE>;
    fn Client(&mut self) -> ::windows::core::Result<IRTCClient>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCClientEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peneventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Client<Impl: IRTCClientEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Client() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclient = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn StartListenAddressAndPort(&mut self, bstrinternallocaladdress: super::super::Foundation::BSTR, linternallocalport: i32) -> ::windows::core::Result<()>;
    fn StopListenAddressAndPort(&mut self, bstrinternallocaladdress: super::super::Foundation::BSTR, linternallocalport: i32) -> ::windows::core::Result<()>;
    fn GetPortRange(&mut self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCClientPortManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPortManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPortManagementVtbl {
        unsafe extern "system" fn StartListenAddressAndPort<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListenAddressAndPort(::core::mem::transmute_copy(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn StopListenAddressAndPort<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopListenAddressAndPort(::core::mem::transmute_copy(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn GetPortRange<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPortRange(::core::mem::transmute_copy(&enporttype), ::core::mem::transmute_copy(&plminvalue), ::core::mem::transmute_copy(&plmaxvalue)).into()
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
    fn EnablePresence(&mut self, fusestorage: i16, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Export(&mut self, varstorage: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Import(&mut self, varstorage: super::Com::VARIANT, freplaceall: i16) -> ::windows::core::Result<()>;
    fn EnumerateBuddies(&mut self) -> ::windows::core::Result<IRTCEnumBuddies>;
    fn Buddies(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Buddy(&mut self, bstrpresentityuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCBuddy>;
    fn AddBuddy(&mut self, bstrpresentityuri: super::super::Foundation::BSTR, bstrusername: super::super::Foundation::BSTR, bstrdata: super::super::Foundation::BSTR, fpersistent: i16, pprofile: ::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddy>;
    fn RemoveBuddy(&mut self, pbuddy: ::core::option::Option<IRTCBuddy>) -> ::windows::core::Result<()>;
    fn EnumerateWatchers(&mut self) -> ::windows::core::Result<IRTCEnumWatchers>;
    fn Watchers(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Watcher(&mut self, bstrpresentityuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCWatcher>;
    fn AddWatcher(&mut self, bstrpresentityuri: super::super::Foundation::BSTR, bstrusername: super::super::Foundation::BSTR, bstrdata: super::super::Foundation::BSTR, fblocked: i16, fpersistent: i16) -> ::windows::core::Result<IRTCWatcher>;
    fn RemoveWatcher(&mut self, pwatcher: ::core::option::Option<IRTCWatcher>) -> ::windows::core::Result<()>;
    fn SetLocalPresenceInfo(&mut self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OfferWatcherMode(&mut self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE>;
    fn SetOfferWatcherMode(&mut self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()>;
    fn PrivacyMode(&mut self) -> ::windows::core::Result<RTC_PRIVACY_MODE>;
    fn SetPrivacyMode(&mut self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPresenceVtbl {
        unsafe extern "system" fn EnablePresence<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePresence(::core::mem::transmute_copy(&fusestorage), ::core::mem::transmute_copy(&varstorage)).into()
        }
        unsafe extern "system" fn Export<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Export(::core::mem::transmute_copy(&varstorage)).into()
        }
        unsafe extern "system" fn Import<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&varstorage), ::core::mem::transmute_copy(&freplaceall)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateBuddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy(::core::mem::transmute_copy(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBuddy(::core::mem::transmute_copy(&bstrpresentityuri), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBuddy(::core::mem::transmute(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateWatchers<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateWatchers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watchers<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watchers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watcher(::core::mem::transmute_copy(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWatcher(::core::mem::transmute_copy(&bstrpresentityuri), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&fblocked), ::core::mem::transmute_copy(&fpersistent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWatcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwatcher: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWatcher(::core::mem::transmute(&pwatcher)).into()
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalPresenceInfo(::core::mem::transmute_copy(&enstatus), ::core::mem::transmute_copy(&bstrnotes)).into()
        }
        unsafe extern "system" fn OfferWatcherMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferWatcherMode() {
                ::core::result::Result::Ok(ok__) => {
                    *penmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfferWatcherMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOfferWatcherMode(::core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn PrivacyMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivacyMode() {
                ::core::result::Result::Ok(ok__) => {
                    *penmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivacyMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivacyMode(::core::mem::transmute_copy(&enmode)).into()
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
    fn EnablePresenceEx(&mut self, pprofile: ::core::option::Option<IRTCProfile>, varstorage: super::Com::VARIANT, lflags: i32) -> ::windows::core::Result<()>;
    fn DisablePresence(&mut self) -> ::windows::core::Result<()>;
    fn AddGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, bstrdata: super::super::Foundation::BSTR, pprofile: ::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn RemoveGroup(&mut self, pgroup: ::core::option::Option<IRTCBuddyGroup>) -> ::windows::core::Result<()>;
    fn EnumerateGroups(&mut self) -> ::windows::core::Result<IRTCEnumGroups>;
    fn Groups(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Group(&mut self, bstrgroupname: super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn AddWatcherEx(&mut self, bstrpresentityuri: super::super::Foundation::BSTR, bstrusername: super::super::Foundation::BSTR, bstrdata: super::super::Foundation::BSTR, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: ::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCWatcher2>;
    fn WatcherEx(&mut self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCWatcher2>;
    fn SetPresenceProperty(&mut self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PresenceProperty(&mut self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPresenceData(&mut self, bstrnamespace: super::super::Foundation::BSTR, bstrdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPresenceData(&mut self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetLocalPresenceInfo(&mut self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddBuddyEx(&mut self, bstrpresentityuri: super::super::Foundation::BSTR, bstrusername: super::super::Foundation::BSTR, bstrdata: super::super::Foundation::BSTR, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddy2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresence2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresence2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPresence2Vtbl {
        unsafe extern "system" fn EnablePresenceEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePresenceEx(::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&varstorage), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DisablePresence<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisablePresence().into()
        }
        unsafe extern "system" fn AddGroup<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGroup<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGroup(::core::mem::transmute(&pgroup)).into()
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group(::core::mem::transmute_copy(&bstrgroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcherEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWatcherEx(::core::mem::transmute_copy(&bstrpresentityuri), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&enstate), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute_copy(&enscope), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WatcherEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WatcherEx(::core::mem::transmute_copy(&enmode), ::core::mem::transmute_copy(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceProperty<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresenceProperty(::core::mem::transmute_copy(&enproperty), ::core::mem::transmute_copy(&bstrproperty)).into()
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceData<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresenceData(::core::mem::transmute_copy(&bstrnamespace), ::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into()
        }
        unsafe extern "system" fn AddBuddyEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBuddyEx(::core::mem::transmute_copy(&bstrpresentityuri), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute_copy(&ensubscriptiontype), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn CreateProfile(&mut self, bstrprofilexml: super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCProfile>;
    fn EnableProfile(&mut self, pprofile: ::core::option::Option<IRTCProfile>, lregisterflags: i32) -> ::windows::core::Result<()>;
    fn DisableProfile(&mut self, pprofile: ::core::option::Option<IRTCProfile>) -> ::windows::core::Result<()>;
    fn EnumerateProfiles(&mut self) -> ::windows::core::Result<IRTCEnumProfiles>;
    fn Profiles(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn GetProfile(&mut self, bstruseraccount: super::super::Foundation::BSTR, bstruserpassword: super::super::Foundation::BSTR, bstruseruri: super::super::Foundation::BSTR, bstrserver: super::super::Foundation::BSTR, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn SessionCapabilities(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCClientProvisioningVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioningImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientProvisioningVtbl {
        unsafe extern "system" fn CreateProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofilexml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProfile(::core::mem::transmute_copy(&bstrprofilexml)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableProfile(::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lregisterflags)).into()
        }
        unsafe extern "system" fn DisableProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableProfile(::core::mem::transmute(&pprofile)).into()
        }
        unsafe extern "system" fn EnumerateProfiles<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profiles<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProfile(::core::mem::transmute_copy(&bstruseraccount), ::core::mem::transmute_copy(&bstruserpassword), ::core::mem::transmute_copy(&bstruseruri), ::core::mem::transmute_copy(&bstrserver), ::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *plsupportedsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn EnableProfileEx(&mut self, pprofile: ::core::option::Option<IRTCProfile>, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCClientProvisioning2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioning2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientProvisioning2Vtbl {
        unsafe extern "system" fn EnableProfileEx<Impl: IRTCClientProvisioning2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableProfileEx(::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lregisterflags), ::core::mem::transmute_copy(&lroamingflags)).into()
        }
        Self { base: IRTCClientProvisioningVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EnableProfileEx: EnableProfileEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientProvisioning2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *lcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddy>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumBuddies>;
}
impl IRTCEnumBuddiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumBuddiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumBuddiesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddyGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumGroups>;
}
impl IRTCEnumGroupsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumGroupsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumGroupsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCParticipant>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumParticipants>;
}
impl IRTCEnumParticipantsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumParticipantsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumParticipantsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCPresenceDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumPresenceDevices>;
}
impl IRTCEnumPresenceDevicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumPresenceDevicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumPresenceDevicesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCProfile>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumProfiles>;
}
impl IRTCEnumProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumProfilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumProfilesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCUserSearchResult>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumUserSearchResults>;
}
impl IRTCEnumUserSearchResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumUserSearchResultsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumUserSearchResultsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCWatcher>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumWatchers>;
}
impl IRTCEnumWatchersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumWatchersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumWatchersVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Event(&mut self, rtcevent: RTC_EVENT, pevent: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEventNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEventNotificationVtbl {
        unsafe extern "system" fn Event<Impl: IRTCEventNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Event(::core::mem::transmute_copy(&rtcevent), ::core::mem::transmute(&pevent)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Event: Event::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEventNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCInfoEventImpl: Sized + IDispatchImpl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn Info(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InfoHeader(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCInfoEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCInfoEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCInfoEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoHeader<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfoHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrinfoheader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Level(&mut self) -> ::windows::core::Result<i32>;
    fn Min(&mut self) -> ::windows::core::Result<i32>;
    fn Max(&mut self) -> ::windows::core::Result<i32>;
    fn Direction(&mut self) -> ::windows::core::Result<RTC_AUDIO_DEVICE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCIntensityEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCIntensityEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCIntensityEventVtbl {
        unsafe extern "system" fn Level<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *pllevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *plmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *plmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *pendirection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
    fn EventType(&mut self) -> ::windows::core::Result<RTC_MEDIA_EVENT_TYPE>;
    fn EventReason(&mut self) -> ::windows::core::Result<RTC_MEDIA_EVENT_REASON>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMediaEventVtbl {
        unsafe extern "system" fn MediaType<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peneventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventReason<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventReason() {
                ::core::result::Result::Ok(ok__) => {
                    *peneventreason = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn ProposedMedia(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentMedia(&mut self) -> ::windows::core::Result<i32>;
    fn Accept(&mut self, lmediatypes: i32) -> ::windows::core::Result<()>;
    fn RemotePreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REINVITE_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaRequestEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaRequestEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMediaRequestEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProposedMedia<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProposedMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMedia<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept(::core::mem::transmute_copy(&lmediatypes)).into()
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn State<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn EventType(&mut self) -> ::windows::core::Result<RTC_MESSAGING_EVENT_TYPE>;
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MessageHeader(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserStatus(&mut self) -> ::windows::core::Result<RTC_MESSAGING_USER_STATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMessagingEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMessagingEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMessagingEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peneventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageHeader<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmessageheader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserStatus<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *penuserstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn UserURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Removable(&mut self) -> ::windows::core::Result<i16>;
    fn State(&mut self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE>;
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCParticipantVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipantImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCParticipantVtbl {
        unsafe extern "system" fn UserURI<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Removable<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfremovable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfremovable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn State(&mut self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCParticipantStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipantStateChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCParticipantStateChangeEventVtbl {
        unsafe extern "system" fn Participant<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn GetMapping(&mut self, bstrremoteaddress: super::super::Foundation::BSTR, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows::core::Result<()>;
    fn UpdateRemoteAddress(&mut self, bstrremoteaddress: super::super::Foundation::BSTR, bstrinternallocaladdress: super::super::Foundation::BSTR, linternallocalport: i32, bstrexternallocaladdress: super::super::Foundation::BSTR, lexternallocalport: i32) -> ::windows::core::Result<()>;
    fn ReleaseMapping(&mut self, bstrinternallocaladdress: super::super::Foundation::BSTR, linternallocalport: i32, bstrexternallocaladdress: super::super::Foundation::BSTR, lexternallocaladdress: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPortManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPortManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPortManagerVtbl {
        unsafe extern "system" fn GetMapping<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMapping(::core::mem::transmute_copy(&bstrremoteaddress), ::core::mem::transmute_copy(&enporttype), ::core::mem::transmute_copy(&pbstrinternallocaladdress), ::core::mem::transmute_copy(&plinternallocalport), ::core::mem::transmute_copy(&pbstrexternallocaladdress), ::core::mem::transmute_copy(&plexternallocalport)).into()
        }
        unsafe extern "system" fn UpdateRemoteAddress<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateRemoteAddress(::core::mem::transmute_copy(&bstrremoteaddress), ::core::mem::transmute_copy(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport), ::core::mem::transmute_copy(&bstrexternallocaladdress), ::core::mem::transmute_copy(&lexternallocalport)).into()
        }
        unsafe extern "system" fn ReleaseMapping<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocaladdress: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseMapping(::core::mem::transmute_copy(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport), ::core::mem::transmute_copy(&bstrexternallocaladdress), ::core::mem::transmute_copy(&lexternallocaladdress)).into()
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
    fn PresentityURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPresentityURI(&mut self, bstrpresentityuri: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetData(&mut self, bstrdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Persistent(&mut self) -> ::windows::core::Result<i16>;
    fn SetPersistent(&mut self, fpersistent: i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPresenceContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceContactImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceContactVtbl {
        unsafe extern "system" fn PresentityURI<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentityURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpresentityuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentityURI<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentityURI(::core::mem::transmute_copy(&bstrpresentityuri)).into()
        }
        unsafe extern "system" fn Name<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Data<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn Persistent<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpersistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Persistent() {
                ::core::result::Result::Ok(ok__) => {
                    *pfpersistent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPersistent<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersistent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPersistent(::core::mem::transmute_copy(&fpersistent)).into()
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
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPresenceData(&mut self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceDataEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDataEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceDataEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
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
    fn Status(&mut self) -> ::windows::core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PresenceProperty(&mut self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPresenceData(&mut self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPresenceDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceDeviceVtbl {
        unsafe extern "system" fn Status<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *penstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notes() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnotes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
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
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PresenceProperty(&mut self) -> ::windows::core::Result<RTC_PRESENCE_PROPERTY>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresencePropertyEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresencePropertyEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresencePropertyEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *penpresprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetLocalPresenceInfo(&mut self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceStatusEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceStatusEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into()
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
    fn Key(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn XML(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProviderURI(&mut self, enuri: RTC_PROVIDER_URI) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProviderData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClientBanner(&mut self) -> ::windows::core::Result<i16>;
    fn ClientMinVer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClientCurVer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClientUpdateURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClientData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserAccount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCredentials(&mut self, bstruseruri: super::super::Foundation::BSTR, bstruseraccount: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SessionCapabilities(&mut self) -> ::windows::core::Result<i32>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REGISTRATION_STATE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileVtbl {
        unsafe extern "system" fn Key<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XML<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XML() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderURI(::core::mem::transmute_copy(&enuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderData<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientBanner<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbanner: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientBanner() {
                ::core::result::Result::Ok(ok__) => {
                    *pfbanner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientMinVer<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrminver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientMinVer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrminver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCurVer<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCurVer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcurver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientUpdateURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientUpdateURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrupdateuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientData<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseraccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&bstruseruri), ::core::mem::transmute_copy(&bstruseraccount), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *plsupportedsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Realm(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRealm(&mut self, bstrrealm: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllowedAuth(&mut self) -> ::windows::core::Result<i32>;
    fn SetAllowedAuth(&mut self, lallowedauth: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfile2Vtbl {
        unsafe extern "system" fn Realm<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrealm: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Realm() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrealm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealm<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRealm(::core::mem::transmute_copy(&bstrrealm)).into()
        }
        unsafe extern "system" fn AllowedAuth<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedAuth() {
                ::core::result::Result::Ok(ok__) => {
                    *plallowedauth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedAuth<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedAuth(::core::mem::transmute_copy(&lallowedauth)).into()
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
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile>;
    fn Cookie(&mut self) -> ::windows::core::Result<isize>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileEventVtbl {
        unsafe extern "system" fn Profile<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    *plcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn EventType(&mut self) -> ::windows::core::Result<RTC_PROFILE_EVENT_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCProfileEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IRTCProfileEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EventType: EventType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfileEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCReInviteEventImpl: Sized + IDispatchImpl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn Accept(&mut self, bstrcontenttype: super::super::Foundation::BSTR, bstrsessiondescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REINVITE_STATE>;
    fn GetRemoteSessionDescription(&mut self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCReInviteEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCReInviteEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCReInviteEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn Reject<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn State<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
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
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REGISTRATION_STATE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRegistrationStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRegistrationStateChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCRegistrationStateChangeEventVtbl {
        unsafe extern "system" fn Profile<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn EventType(&mut self) -> ::windows::core::Result<RTC_ROAMING_EVENT_TYPE>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRoamingEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRoamingEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCRoamingEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Client(&mut self) -> ::windows::core::Result<IRTCClient>;
    fn State(&mut self) -> ::windows::core::Result<RTC_SESSION_STATE>;
    fn Type(&mut self) -> ::windows::core::Result<RTC_SESSION_TYPE>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile>;
    fn Participants(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Answer(&mut self) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()>;
    fn Redirect(&mut self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: super::super::Foundation::BSTR, pprofile: ::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<()>;
    fn AddParticipant(&mut self, bstraddress: super::super::Foundation::BSTR, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCParticipant>;
    fn RemoveParticipant(&mut self, pparticipant: ::core::option::Option<IRTCParticipant>) -> ::windows::core::Result<()>;
    fn EnumerateParticipants(&mut self) -> ::windows::core::Result<IRTCEnumParticipants>;
    fn CanAddParticipants(&mut self) -> ::windows::core::Result<i16>;
    fn RedirectedUserURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RedirectedUserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn NextRedirectedUser(&mut self) -> ::windows::core::Result<()>;
    fn SendMessage(&mut self, bstrmessageheader: super::super::Foundation::BSTR, bstrmessage: super::super::Foundation::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
    fn SendMessageStatus(&mut self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()>;
    fn AddStream(&mut self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn RemoveStream(&mut self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn SetEncryptionKey(&mut self, lmediatype: i32, encryptionkey: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionVtbl {
        unsafe extern "system" fn Client<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Client() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclient = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pentype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participants() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Answer<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Answer().into()
        }
        unsafe extern "system" fn Terminate<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&enreason)).into()
        }
        unsafe extern "system" fn Redirect<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Redirect(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bstrlocalphoneuri), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddParticipant<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddParticipant(::core::mem::transmute_copy(&bstraddress), ::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveParticipant<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparticipant: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveParticipant(::core::mem::transmute(&pparticipant)).into()
        }
        unsafe extern "system" fn EnumerateParticipants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateParticipants() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanAddParticipants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcanadd: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAddParticipants() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcanadd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserURI<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectedUserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserName<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectedUserName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRedirectedUser<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextRedirectedUser().into()
        }
        unsafe extern "system" fn SendMessage<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessage(::core::mem::transmute_copy(&bstrmessageheader), ::core::mem::transmute_copy(&bstrmessage), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SendMessageStatus<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessageStatus(::core::mem::transmute_copy(&enuserstatus), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn AddStream<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn RemoveStream<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SetEncryptionKey<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptionKey(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&encryptionkey)).into()
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
    fn SendInfo(&mut self, bstrinfoheader: super::super::Foundation::BSTR, bstrinfo: super::super::Foundation::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
    fn SetPreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()>;
    fn PreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn IsSecurityEnabled(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<i16>;
    fn AnswerWithSessionDescription(&mut self, bstrcontenttype: super::super::Foundation::BSTR, bstrsessiondescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReInviteWithSessionDescription(&mut self, bstrcontenttype: super::super::Foundation::BSTR, bstrsessiondescription: super::super::Foundation::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSession2Vtbl {
        unsafe extern "system" fn SendInfo<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinfo: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendInfo(::core::mem::transmute_copy(&bstrinfoheader), ::core::mem::transmute_copy(&bstrinfo), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfsecurityenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AnswerWithSessionDescription(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReInviteWithSessionDescription(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription), ::core::mem::transmute_copy(&lcookie)).into()
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
    fn Hold(&mut self, lcookie: isize) -> ::windows::core::Result<()>;
    fn UnHold(&mut self, lcookie: isize) -> ::windows::core::Result<()>;
    fn Forward(&mut self, bstrforwardtouri: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refer(&mut self, bstrrefertouri: super::super::Foundation::BSTR, bstrrefercookie: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetReferredByURI(&mut self, bstrreferredbyuri: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReferredByURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetReferCookie(&mut self, bstrrefercookie: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReferCookie(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsReferred(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionCallControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionCallControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionCallControlVtbl {
        unsafe extern "system" fn Hold<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hold(::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn UnHold<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnHold(::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn Forward<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forward(::core::mem::transmute_copy(&bstrforwardtouri)).into()
        }
        unsafe extern "system" fn Refer<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefertouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refer(::core::mem::transmute_copy(&bstrrefertouri), ::core::mem::transmute_copy(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn SetReferredByURI<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferredByURI(::core::mem::transmute_copy(&bstrreferredbyuri)).into()
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferredByURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreferredbyuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferCookie<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferCookie(::core::mem::transmute_copy(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrefercookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReferred<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreferred: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReferred() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisreferred = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn EvaluateSessionDescription(&mut self, bstrcontenttype: super::super::Foundation::BSTR, bstrsessiondescription: super::super::Foundation::BSTR, pfapplicationsession: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionDescriptionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionDescriptionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionDescriptionManagerVtbl {
        unsafe extern "system" fn EvaluateSessionDescription<Impl: IRTCSessionDescriptionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfapplicationsession: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EvaluateSessionDescription(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription), ::core::mem::transmute_copy(&pfapplicationsession)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EvaluateSessionDescription: EvaluateSessionDescription::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionDescriptionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionOperationCompleteEventImpl: Sized + IDispatchImpl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
    fn Cookie(&mut self) -> ::windows::core::Result<isize>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionOperationCompleteEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    *plcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn GetRemoteSessionDescription(&mut self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionOperationCompleteEvent2Vtbl {
        unsafe extern "system" fn Participant<Impl: IRTCSessionOperationCompleteEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionOperationCompleteEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
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
    fn SetPortManager(&mut self, pportmanager: ::core::option::Option<IRTCPortManager>) -> ::windows::core::Result<()>;
}
impl IRTCSessionPortManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionPortManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionPortManagementVtbl {
        unsafe extern "system" fn SetPortManager<Impl: IRTCSessionPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPortManager(::core::mem::transmute(&pportmanager)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetPortManager: SetPortManager::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionPortManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionReferStatusEventImpl: Sized + IDispatchImpl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn ReferStatus(&mut self) -> ::windows::core::Result<RTC_SESSION_REFER_STATUS>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferStatusEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionReferStatusEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferStatus<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *penreferstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn ReferredByURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ReferToURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ReferCookie(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Accept(&mut self) -> ::windows::core::Result<()>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn SetReferredSessionState(&mut self, enstate: RTC_SESSION_STATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferredEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferredEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionReferredEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferredByURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreferredbyuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferToURI<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferToURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreferouri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrefercookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        unsafe extern "system" fn Reject<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn SetReferredSessionState<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferredSessionState(::core::mem::transmute_copy(&enstate)).into()
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
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
    fn State(&mut self) -> ::windows::core::Result<RTC_SESSION_STATE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionStateChangeEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn MediaTypes(&mut self) -> ::windows::core::Result<i32>;
    fn RemotePreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn IsForked(&mut self) -> ::windows::core::Result<i16>;
    fn GetRemoteSessionDescription(&mut self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionStateChangeEvent2Vtbl {
        unsafe extern "system" fn MediaTypes<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsForked<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisforked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsForked() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisforked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
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
    fn CreateQuery(&mut self) -> ::windows::core::Result<IRTCUserSearchQuery>;
    fn ExecuteSearch(&mut self, pquery: ::core::option::Option<IRTCUserSearchQuery>, pprofile: ::core::option::Option<IRTCProfile>, lcookie: isize) -> ::windows::core::Result<()>;
}
impl IRTCUserSearchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchVtbl {
        unsafe extern "system" fn CreateQuery<Impl: IRTCUserSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteSearch<Impl: IRTCUserSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteSearch(::core::mem::transmute(&pquery), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lcookie)).into()
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
    fn SetSearchTerm(&mut self, bstrname: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SearchTerm(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SearchTerms(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSearchPreference(&mut self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::Result<()>;
    fn SearchPreference(&mut self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> ::windows::core::Result<i32>;
    fn SetSearchDomain(&mut self, bstrdomain: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SearchDomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCUserSearchQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchQueryVtbl {
        unsafe extern "system" fn SetSearchTerm<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchTerm(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn SearchTerm<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchTerm(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchTerms<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchTerms() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchPreference<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchPreference(::core::mem::transmute_copy(&enpreference), ::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn SearchPreference<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPreference(::core::mem::transmute_copy(&enpreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *plvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchDomain<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchDomain(::core::mem::transmute_copy(&bstrdomain)).into()
        }
        unsafe extern "system" fn SearchDomain<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchDomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdomain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Value(&mut self, encolumn: RTC_USER_SEARCH_COLUMN) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCUserSearchResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchResultVtbl {
        unsafe extern "system" fn Value<Impl: IRTCUserSearchResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&encolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Value: Value::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearchResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCUserSearchResultsEventImpl: Sized + IDispatchImpl {
    fn EnumerateResults(&mut self) -> ::windows::core::Result<IRTCEnumUserSearchResults>;
    fn Results(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
    fn Query(&mut self) -> ::windows::core::Result<IRTCUserSearchQuery>;
    fn Cookie(&mut self) -> ::windows::core::Result<isize>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn MoreAvailable(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCUserSearchResultsEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResultsEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchResultsEventVtbl {
        unsafe extern "system" fn EnumerateResults<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateResults() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query() {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    *plcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreAvailable<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoreAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfmoreavailable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn State(&mut self) -> ::windows::core::Result<RTC_WATCHER_STATE>;
    fn SetState(&mut self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherVtbl {
        unsafe extern "system" fn State<Impl: IRTCWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IRTCWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&enstate)).into()
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
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
    fn Scope(&mut self) -> ::windows::core::Result<RTC_ACE_SCOPE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcher2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcher2Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: IRTCWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *penscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Watcher(&mut self) -> ::windows::core::Result<IRTCWatcher>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherEventVtbl {
        unsafe extern "system" fn Watcher<Impl: IRTCWatcherEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watcher() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Watcher: Watcher::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcherEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCWatcherEvent2Impl: Sized + IDispatchImpl + IRTCWatcherEventImpl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_WATCHER_EVENT_TYPE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCWatcherEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCWatcherEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn ApplySetting(&mut self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()>;
    fn QuerySetting(&mut self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ITransportSettingsInternalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransportSettingsInternalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransportSettingsInternalVtbl {
        unsafe extern "system" fn ApplySetting<Impl: ITransportSettingsInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplySetting(::core::mem::transmute_copy(&setting)).into()
        }
        unsafe extern "system" fn QuerySetting<Impl: ITransportSettingsInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QuerySetting(::core::mem::transmute_copy(&setting)).into()
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
