#[cfg(feature = "Win32_Networking_WinSock")]
pub trait INetworkTransportSettings_Impl: Sized {
    fn ApplySetting(&mut self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()>;
    fn QuerySetting(&mut self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl INetworkTransportSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkTransportSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkTransportSettings_Vtbl {
        unsafe extern "system" fn ApplySetting<Impl: INetworkTransportSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplySetting(::core::mem::transmute_copy(&settingid), ::core::mem::transmute_copy(&lengthin), ::core::mem::transmute_copy(&valuein), ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)).into()
        }
        unsafe extern "system" fn QuerySetting<Impl: INetworkTransportSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
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
pub trait INotificationTransportSync_Impl: Sized {
    fn CompleteDelivery(&mut self) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
impl INotificationTransportSync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationTransportSync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationTransportSync_Vtbl {
        unsafe extern "system" fn CompleteDelivery<Impl: INotificationTransportSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteDelivery().into()
        }
        unsafe extern "system" fn Flush<Impl: INotificationTransportSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IRTCBuddy_Impl: Sized + IRTCPresenceContact_Impl {
    fn Status(&mut self) -> ::windows::core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCBuddy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddy_Vtbl {
        unsafe extern "system" fn Status<Impl: IRTCBuddy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *penstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Impl: IRTCBuddy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: IRTCPresenceContact_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Notes: Notes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCBuddy2_Impl: Sized + IRTCPresenceContact_Impl + IRTCBuddy_Impl {
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
impl IRTCBuddy2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddy2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddy2_Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePresenceDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceDevices<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevicescollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriptionType<Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows::core::HRESULT {
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
            base: IRTCBuddy_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCBuddyEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Buddy(&mut self) -> ::windows::core::Result<IRTCBuddy>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyEvent_Vtbl {
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Buddy: Buddy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyEvent2_Impl: Sized + super::Com::IDispatch_Impl + IRTCBuddyEvent_Impl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_BUDDY_EVENT_TYPE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEvent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyEvent2_Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: IRTCBuddyEvent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCBuddyGroup_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrgroupname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddBuddy(&mut self, pbuddy: &::core::option::Option<IRTCBuddy>) -> ::windows::core::Result<()>;
    fn RemoveBuddy(&mut self, pbuddy: &::core::option::Option<IRTCBuddy>) -> ::windows::core::Result<()>;
    fn EnumerateBuddies(&mut self) -> ::windows::core::Result<IRTCEnumBuddies>;
    fn Buddies(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetData(&mut self, bstrdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCBuddyGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyGroup_Vtbl {
        unsafe extern "system" fn Name<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrgroupname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrgroupname)).into()
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBuddy(::core::mem::transmute(&pbuddy)).into()
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBuddy(::core::mem::transmute(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateBuddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn Profile<Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCBuddyGroupEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_GROUP_EVENT_TYPE>;
    fn Group(&mut self) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn Buddy(&mut self) -> ::windows::core::Result<IRTCBuddy2>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyGroupEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroupEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCBuddyGroupEvent_Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCClient_Impl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn PrepareForShutdown(&mut self) -> ::windows::core::Result<()>;
    fn SetEventFilter(&mut self, lfilter: i32) -> ::windows::core::Result<()>;
    fn EventFilter(&mut self) -> ::windows::core::Result<i32>;
    fn SetPreferredMediaTypes(&mut self, lmediatypes: i32, fpersistent: i16) -> ::windows::core::Result<()>;
    fn PreferredMediaTypes(&mut self) -> ::windows::core::Result<i32>;
    fn MediaCapabilities(&mut self) -> ::windows::core::Result<i32>;
    fn CreateSession(&mut self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &super::super::Foundation::BSTR, pprofile: &::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCSession>;
    fn SetListenForIncomingSessions(&mut self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()>;
    fn ListenForIncomingSessions(&mut self) -> ::windows::core::Result<RTC_LISTEN_MODE>;
    fn NetworkAddresses(&mut self, ftcp: i16, fexternal: i16) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetVolume(&mut self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()>;
    fn Volume(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32>;
    fn SetAudioMuted(&mut self, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::Result<()>;
    fn AudioMuted(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i16>;
    fn IVideoWindow(&mut self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow>;
    fn SetPreferredAudioDevice(&mut self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PreferredAudioDevice(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPreferredVolume(&mut self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()>;
    fn PreferredVolume(&mut self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32>;
    fn SetPreferredAEC(&mut self, benable: i16) -> ::windows::core::Result<()>;
    fn PreferredAEC(&mut self) -> ::windows::core::Result<i16>;
    fn SetPreferredVideoDevice(&mut self, bstrdevicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetLocalUserURI(&mut self, bstruseruri: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalUserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalUserName(&mut self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlayRing(&mut self, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::Result<()>;
    fn SendDTMF(&mut self, endtmf: RTC_DTMF) -> ::windows::core::Result<()>;
    fn InvokeTuningWizard(&mut self, hwndparent: isize) -> ::windows::core::Result<()>;
    fn IsTuned(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClient_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn Shutdown<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn PrepareForShutdown<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareForShutdown().into()
        }
        unsafe extern "system" fn SetEventFilter<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&lfilter)).into()
        }
        unsafe extern "system" fn EventFilter<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *plfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredMediaTypes(::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&fpersistent)).into()
        }
        unsafe extern "system" fn PreferredMediaTypes<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredMediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCapabilities<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bstrlocalphoneuri), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListenForIncomingSessions(::core::mem::transmute_copy(&enlisten)).into()
        }
        unsafe extern "system" fn ListenForIncomingSessions<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenForIncomingSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *penlisten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAddresses<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftcp: i16, fexternal: i16, pvaddresses: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAddresses(::core::mem::transmute_copy(&ftcp), ::core::mem::transmute_copy(&fexternal)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvaddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn Volume<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioMuted<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioMuted(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&fmuted)).into()
        }
        unsafe extern "system" fn AudioMuted<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioMuted(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmuted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IVideoWindow<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IVideoWindow(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppivideowindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAudioDevice<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredAudioDevice(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn PreferredAudioDevice<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAudioDevice(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVolume<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredVolume(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn PreferredVolume<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredVolume(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAEC<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredAEC(::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn PreferredAEC<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAEC() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredVideoDevice(::core::mem::transmute_copy(&bstrdevicename)).into()
        }
        unsafe extern "system" fn PreferredVideoDevice<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredVideoDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveMedia<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBitrate<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBitrate(::core::mem::transmute_copy(&lmaxbitrate)).into()
        }
        unsafe extern "system" fn MaxBitrate<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxbitrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTemporalSpatialTradeOff(::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemporalSpatialTradeOff() {
                ::core::result::Result::Ok(ok__) => {
                    *plvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkQuality<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *plnetworkquality = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartT120Applet<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartT120Applet(::core::mem::transmute_copy(&enapplet)).into()
        }
        unsafe extern "system" fn StopT120Applets<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopT120Applets().into()
        }
        unsafe extern "system" fn IsT120AppletRunning<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsT120AppletRunning(::core::mem::transmute_copy(&enapplet)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfrunning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserURI<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserURI<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalUserURI(::core::mem::transmute_copy(&bstruseruri)).into()
        }
        unsafe extern "system" fn LocalUserName<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserName<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalUserName(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn PlayRing<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayRing(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bplay)).into()
        }
        unsafe extern "system" fn SendDTMF<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDTMF(::core::mem::transmute_copy(&endtmf)).into()
        }
        unsafe extern "system" fn InvokeTuningWizard<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeTuningWizard(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn IsTuned<Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftuned: *mut i16) -> ::windows::core::HRESULT {
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
pub trait IRTCClient2_Impl: Sized + IRTCClient_Impl {
    fn SetAnswerMode(&mut self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::Result<()>;
    fn AnswerMode(&mut self, entype: RTC_SESSION_TYPE) -> ::windows::core::Result<RTC_ANSWER_MODE>;
    fn InvokeTuningWizardEx(&mut self, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<i32>;
    fn SetClientName(&mut self, bstrclientname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCurVer(&mut self, bstrclientcurver: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitializeEx(&mut self, lflags: i32) -> ::windows::core::Result<()>;
    fn CreateSessionWithDescription(&mut self, bstrcontenttype: &super::super::Foundation::BSTR, bstrsessiondescription: &super::super::Foundation::BSTR, pprofile: &::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCSession2>;
    fn SetSessionDescriptionManager(&mut self, psessiondescriptionmanager: &::core::option::Option<IRTCSessionDescriptionManager>) -> ::windows::core::Result<()>;
    fn SetPreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()>;
    fn PreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn SetAllowedPorts(&mut self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::Result<()>;
    fn AllowedPorts(&mut self, ltransport: i32) -> ::windows::core::Result<RTC_LISTEN_MODE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClient2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClient2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClient2_Vtbl {
        unsafe extern "system" fn SetAnswerMode<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnswerMode(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn AnswerMode<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnswerMode(::core::mem::transmute_copy(&entype)) {
                ::core::result::Result::Ok(ok__) => {
                    *penmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeTuningWizardEx(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&fallowaudio), ::core::mem::transmute_copy(&fallowvideo)).into()
        }
        unsafe extern "system" fn Version<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *plversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&bstrclientname)).into()
        }
        unsafe extern "system" fn SetClientCurVer<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientcurver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCurVer(::core::mem::transmute_copy(&bstrclientcurver)).into()
        }
        unsafe extern "system" fn InitializeEx<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEx(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateSessionWithDescription<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionWithDescription(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionDescriptionManager(::core::mem::transmute(&psessiondescriptionmanager)).into()
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedPorts<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedPorts(::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&enlistenmode)).into()
        }
        unsafe extern "system" fn AllowedPorts<Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
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
            base: IRTCClient_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCClientEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_CLIENT_EVENT_TYPE>;
    fn Client(&mut self) -> ::windows::core::Result<IRTCClient>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientEvent_Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCClientEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peneventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Client<Impl: IRTCClientEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            Client: Client::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCClientPortManagement_Impl: Sized {
    fn StartListenAddressAndPort(&mut self, bstrinternallocaladdress: &super::super::Foundation::BSTR, linternallocalport: i32) -> ::windows::core::Result<()>;
    fn StopListenAddressAndPort(&mut self, bstrinternallocaladdress: &super::super::Foundation::BSTR, linternallocalport: i32) -> ::windows::core::Result<()>;
    fn GetPortRange(&mut self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCClientPortManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPortManagement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPortManagement_Vtbl {
        unsafe extern "system" fn StartListenAddressAndPort<Impl: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListenAddressAndPort(::core::mem::transmute_copy(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn StopListenAddressAndPort<Impl: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopListenAddressAndPort(::core::mem::transmute_copy(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn GetPortRange<Impl: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IRTCClientPresence_Impl: Sized {
    fn EnablePresence(&mut self, fusestorage: i16, varstorage: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Export(&mut self, varstorage: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Import(&mut self, varstorage: &super::Com::VARIANT, freplaceall: i16) -> ::windows::core::Result<()>;
    fn EnumerateBuddies(&mut self) -> ::windows::core::Result<IRTCEnumBuddies>;
    fn Buddies(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Buddy(&mut self, bstrpresentityuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCBuddy>;
    fn AddBuddy(&mut self, bstrpresentityuri: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrdata: &super::super::Foundation::BSTR, fpersistent: i16, pprofile: &::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddy>;
    fn RemoveBuddy(&mut self, pbuddy: &::core::option::Option<IRTCBuddy>) -> ::windows::core::Result<()>;
    fn EnumerateWatchers(&mut self) -> ::windows::core::Result<IRTCEnumWatchers>;
    fn Watchers(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Watcher(&mut self, bstrpresentityuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCWatcher>;
    fn AddWatcher(&mut self, bstrpresentityuri: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrdata: &super::super::Foundation::BSTR, fblocked: i16, fpersistent: i16) -> ::windows::core::Result<IRTCWatcher>;
    fn RemoveWatcher(&mut self, pwatcher: &::core::option::Option<IRTCWatcher>) -> ::windows::core::Result<()>;
    fn SetLocalPresenceInfo(&mut self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OfferWatcherMode(&mut self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE>;
    fn SetOfferWatcherMode(&mut self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()>;
    fn PrivacyMode(&mut self) -> ::windows::core::Result<RTC_PRIVACY_MODE>;
    fn SetPrivacyMode(&mut self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresence_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPresence_Vtbl {
        unsafe extern "system" fn EnablePresence<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePresence(::core::mem::transmute_copy(&fusestorage), ::core::mem::transmute_copy(&varstorage)).into()
        }
        unsafe extern "system" fn Export<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Export(::core::mem::transmute_copy(&varstorage)).into()
        }
        unsafe extern "system" fn Import<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&varstorage), ::core::mem::transmute_copy(&freplaceall)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateBuddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddies() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy(::core::mem::transmute_copy(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBuddy(::core::mem::transmute_copy(&bstrpresentityuri), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuddy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBuddy(::core::mem::transmute(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateWatchers<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateWatchers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watchers<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watchers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watcher<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watcher(::core::mem::transmute_copy(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcher<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWatcher(::core::mem::transmute_copy(&bstrpresentityuri), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&fblocked), ::core::mem::transmute_copy(&fpersistent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWatcher<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwatcher: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWatcher(::core::mem::transmute(&pwatcher)).into()
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalPresenceInfo(::core::mem::transmute_copy(&enstatus), ::core::mem::transmute_copy(&bstrnotes)).into()
        }
        unsafe extern "system" fn OfferWatcherMode<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferWatcherMode() {
                ::core::result::Result::Ok(ok__) => {
                    *penmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfferWatcherMode<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOfferWatcherMode(::core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn PrivacyMode<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivacyMode() {
                ::core::result::Result::Ok(ok__) => {
                    *penmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivacyMode<Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
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
pub trait IRTCClientPresence2_Impl: Sized + IRTCClientPresence_Impl {
    fn EnablePresenceEx(&mut self, pprofile: &::core::option::Option<IRTCProfile>, varstorage: &super::Com::VARIANT, lflags: i32) -> ::windows::core::Result<()>;
    fn DisablePresence(&mut self) -> ::windows::core::Result<()>;
    fn AddGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, bstrdata: &super::super::Foundation::BSTR, pprofile: &::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn RemoveGroup(&mut self, pgroup: &::core::option::Option<IRTCBuddyGroup>) -> ::windows::core::Result<()>;
    fn EnumerateGroups(&mut self) -> ::windows::core::Result<IRTCEnumGroups>;
    fn Groups(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Group(&mut self, bstrgroupname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn AddWatcherEx(&mut self, bstrpresentityuri: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrdata: &super::super::Foundation::BSTR, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: &::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCWatcher2>;
    fn WatcherEx(&mut self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCWatcher2>;
    fn SetPresenceProperty(&mut self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PresenceProperty(&mut self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPresenceData(&mut self, bstrnamespace: &super::super::Foundation::BSTR, bstrdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPresenceData(&mut self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetLocalPresenceInfo(&mut self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddBuddyEx(&mut self, bstrpresentityuri: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrdata: &super::super::Foundation::BSTR, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: &::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddy2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresence2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresence2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientPresence2_Vtbl {
        unsafe extern "system" fn EnablePresenceEx<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePresenceEx(::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&varstorage), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DisablePresence<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisablePresence().into()
        }
        unsafe extern "system" fn AddGroup<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGroup<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGroup(::core::mem::transmute(&pgroup)).into()
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group(::core::mem::transmute_copy(&bstrgroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcherEx<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWatcherEx(::core::mem::transmute_copy(&bstrpresentityuri), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&enstate), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute_copy(&enscope), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WatcherEx<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WatcherEx(::core::mem::transmute_copy(&enmode), ::core::mem::transmute_copy(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceProperty<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresenceProperty(::core::mem::transmute_copy(&enproperty), ::core::mem::transmute_copy(&bstrproperty)).into()
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceData<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresenceData(::core::mem::transmute_copy(&bstrnamespace), ::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into()
        }
        unsafe extern "system" fn AddBuddyEx<Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IRTCClientPresence_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCClientProvisioning_Impl: Sized {
    fn CreateProfile(&mut self, bstrprofilexml: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCProfile>;
    fn EnableProfile(&mut self, pprofile: &::core::option::Option<IRTCProfile>, lregisterflags: i32) -> ::windows::core::Result<()>;
    fn DisableProfile(&mut self, pprofile: &::core::option::Option<IRTCProfile>) -> ::windows::core::Result<()>;
    fn EnumerateProfiles(&mut self) -> ::windows::core::Result<IRTCEnumProfiles>;
    fn Profiles(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn GetProfile(&mut self, bstruseraccount: &super::super::Foundation::BSTR, bstruserpassword: &super::super::Foundation::BSTR, bstruseruri: &super::super::Foundation::BSTR, bstrserver: &super::super::Foundation::BSTR, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn SessionCapabilities(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCClientProvisioning_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioning_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientProvisioning_Vtbl {
        unsafe extern "system" fn CreateProfile<Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofilexml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProfile(::core::mem::transmute_copy(&bstrprofilexml)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableProfile<Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableProfile(::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lregisterflags)).into()
        }
        unsafe extern "system" fn DisableProfile<Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableProfile(::core::mem::transmute(&pprofile)).into()
        }
        unsafe extern "system" fn EnumerateProfiles<Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profiles<Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfile<Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProfile(::core::mem::transmute_copy(&bstruseraccount), ::core::mem::transmute_copy(&bstruserpassword), ::core::mem::transmute_copy(&bstruseruri), ::core::mem::transmute_copy(&bstrserver), ::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IRTCClientProvisioning2_Impl: Sized + IRTCClientProvisioning_Impl {
    fn EnableProfileEx(&mut self, pprofile: &::core::option::Option<IRTCProfile>, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCClientProvisioning2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioning2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCClientProvisioning2_Vtbl {
        unsafe extern "system" fn EnableProfileEx<Impl: IRTCClientProvisioning2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableProfileEx(::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lregisterflags), ::core::mem::transmute_copy(&lroamingflags)).into()
        }
        Self { base: IRTCClientProvisioning_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EnableProfileEx: EnableProfileEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientProvisioning2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IRTCCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *lcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRTCCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IRTCCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCDispatchEventNotification_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCDispatchEventNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCDispatchEventNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCDispatchEventNotification_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCDispatchEventNotification as ::windows::core::Interface>::IID
    }
}
pub trait IRTCEnumBuddies_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddy>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumBuddies>;
}
impl IRTCEnumBuddies_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumBuddies_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumBuddies_Vtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCEnumGroups_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddyGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumGroups>;
}
impl IRTCEnumGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumGroups_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumGroups_Vtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCEnumParticipants_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCParticipant>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumParticipants>;
}
impl IRTCEnumParticipants_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumParticipants_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumParticipants_Vtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCEnumPresenceDevices_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCPresenceDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumPresenceDevices>;
}
impl IRTCEnumPresenceDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumPresenceDevices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumPresenceDevices_Vtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCEnumProfiles_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCProfile>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumProfiles>;
}
impl IRTCEnumProfiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumProfiles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumProfiles_Vtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCEnumUserSearchResults_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCUserSearchResult>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumUserSearchResults>;
}
impl IRTCEnumUserSearchResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumUserSearchResults_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumUserSearchResults_Vtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCEnumWatchers_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<IRTCWatcher>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IRTCEnumWatchers>;
}
impl IRTCEnumWatchers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumWatchers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEnumWatchers_Vtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCEventNotification_Impl: Sized {
    fn Event(&mut self, rtcevent: RTC_EVENT, pevent: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCEventNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEventNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCEventNotification_Vtbl {
        unsafe extern "system" fn Event<Impl: IRTCEventNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCInfoEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn Info(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InfoHeader(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCInfoEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCInfoEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCInfoEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoHeader<Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCIntensityEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Level(&mut self) -> ::windows::core::Result<i32>;
    fn Min(&mut self) -> ::windows::core::Result<i32>;
    fn Max(&mut self) -> ::windows::core::Result<i32>;
    fn Direction(&mut self) -> ::windows::core::Result<RTC_AUDIO_DEVICE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCIntensityEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCIntensityEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCIntensityEvent_Vtbl {
        unsafe extern "system" fn Level<Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *pllevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *plmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *plmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCMediaEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
    fn EventType(&mut self) -> ::windows::core::Result<RTC_MEDIA_EVENT_TYPE>;
    fn EventReason(&mut self) -> ::windows::core::Result<RTC_MEDIA_EVENT_REASON>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMediaEvent_Vtbl {
        unsafe extern "system" fn MediaType<Impl: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Impl: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peneventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventReason<Impl: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCMediaRequestEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn ProposedMedia(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentMedia(&mut self) -> ::windows::core::Result<i32>;
    fn Accept(&mut self, lmediatypes: i32) -> ::windows::core::Result<()>;
    fn RemotePreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REINVITE_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaRequestEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaRequestEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMediaRequestEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProposedMedia<Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProposedMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMedia<Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept(::core::mem::transmute_copy(&lmediatypes)).into()
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn State<Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCMessagingEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn EventType(&mut self) -> ::windows::core::Result<RTC_MESSAGING_EVENT_TYPE>;
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MessageHeader(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserStatus(&mut self) -> ::windows::core::Result<RTC_MESSAGING_USER_STATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMessagingEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMessagingEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCMessagingEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peneventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageHeader<Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmessageheader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserStatus<Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCParticipant_Impl: Sized {
    fn UserURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Removable(&mut self) -> ::windows::core::Result<i16>;
    fn State(&mut self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE>;
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCParticipant_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipant_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCParticipant_Vtbl {
        unsafe extern "system" fn UserURI<Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Removable<Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfremovable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfremovable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCParticipantStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn State(&mut self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCParticipantStateChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipantStateChangeEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCParticipantStateChangeEvent_Vtbl {
        unsafe extern "system" fn Participant<Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCPortManager_Impl: Sized {
    fn GetMapping(&mut self, bstrremoteaddress: &super::super::Foundation::BSTR, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows::core::Result<()>;
    fn UpdateRemoteAddress(&mut self, bstrremoteaddress: &super::super::Foundation::BSTR, bstrinternallocaladdress: &super::super::Foundation::BSTR, linternallocalport: i32, bstrexternallocaladdress: &super::super::Foundation::BSTR, lexternallocalport: i32) -> ::windows::core::Result<()>;
    fn ReleaseMapping(&mut self, bstrinternallocaladdress: &super::super::Foundation::BSTR, linternallocalport: i32, bstrexternallocaladdress: &super::super::Foundation::BSTR, lexternallocaladdress: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPortManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPortManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPortManager_Vtbl {
        unsafe extern "system" fn GetMapping<Impl: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMapping(::core::mem::transmute_copy(&bstrremoteaddress), ::core::mem::transmute_copy(&enporttype), ::core::mem::transmute_copy(&pbstrinternallocaladdress), ::core::mem::transmute_copy(&plinternallocalport), ::core::mem::transmute_copy(&pbstrexternallocaladdress), ::core::mem::transmute_copy(&plexternallocalport)).into()
        }
        unsafe extern "system" fn UpdateRemoteAddress<Impl: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateRemoteAddress(::core::mem::transmute_copy(&bstrremoteaddress), ::core::mem::transmute_copy(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport), ::core::mem::transmute_copy(&bstrexternallocaladdress), ::core::mem::transmute_copy(&lexternallocalport)).into()
        }
        unsafe extern "system" fn ReleaseMapping<Impl: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocaladdress: i32) -> ::windows::core::HRESULT {
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
pub trait IRTCPresenceContact_Impl: Sized {
    fn PresentityURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPresentityURI(&mut self, bstrpresentityuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetData(&mut self, bstrdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Persistent(&mut self) -> ::windows::core::Result<i16>;
    fn SetPersistent(&mut self, fpersistent: i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPresenceContact_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceContact_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceContact_Vtbl {
        unsafe extern "system" fn PresentityURI<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentityURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpresentityuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentityURI<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentityURI(::core::mem::transmute_copy(&bstrpresentityuri)).into()
        }
        unsafe extern "system" fn Name<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Data<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn Persistent<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpersistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Persistent() {
                ::core::result::Result::Ok(ok__) => {
                    *pfpersistent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPersistent<Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersistent: i16) -> ::windows::core::HRESULT {
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
pub trait IRTCPresenceDataEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPresenceData(&mut self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceDataEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDataEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceDataEvent_Vtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCPresenceDevice_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PresenceProperty(&mut self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPresenceData(&mut self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPresenceDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceDevice_Vtbl {
        unsafe extern "system" fn Status<Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *penstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notes() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnotes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait IRTCPresencePropertyEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PresenceProperty(&mut self) -> ::windows::core::Result<RTC_PRESENCE_PROPERTY>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresencePropertyEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresencePropertyEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresencePropertyEvent_Vtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *penpresprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCPresenceStatusEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetLocalPresenceInfo(&mut self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceStatusEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceStatusEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCPresenceStatusEvent_Vtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatustext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCProfile_Impl: Sized {
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
    fn SetCredentials(&mut self, bstruseruri: &super::super::Foundation::BSTR, bstruseraccount: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SessionCapabilities(&mut self) -> ::windows::core::Result<i32>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REGISTRATION_STATE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfile_Vtbl {
        unsafe extern "system" fn Key<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XML<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XML() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderURI<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderURI(::core::mem::transmute_copy(&enuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderData<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientBanner<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbanner: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientBanner() {
                ::core::result::Result::Ok(ok__) => {
                    *pfbanner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientMinVer<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrminver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientMinVer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrminver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCurVer<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCurVer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcurver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientUpdateURI<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientUpdateURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrupdateuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientData<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserURI<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseraccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&bstruseruri), ::core::mem::transmute_copy(&bstruseraccount), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *plsupportedsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
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
pub trait IRTCProfile2_Impl: Sized + IRTCProfile_Impl {
    fn Realm(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRealm(&mut self, bstrrealm: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllowedAuth(&mut self) -> ::windows::core::Result<i32>;
    fn SetAllowedAuth(&mut self, lallowedauth: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfile2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfile2_Vtbl {
        unsafe extern "system" fn Realm<Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrealm: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Realm() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrealm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealm<Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRealm(::core::mem::transmute_copy(&bstrrealm)).into()
        }
        unsafe extern "system" fn AllowedAuth<Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedAuth() {
                ::core::result::Result::Ok(ok__) => {
                    *plallowedauth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedAuth<Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedAuth(::core::mem::transmute_copy(&lallowedauth)).into()
        }
        Self {
            base: IRTCProfile_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCProfileEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile>;
    fn Cookie(&mut self) -> ::windows::core::Result<isize>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileEvent_Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    *plcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCProfileEvent2_Impl: Sized + super::Com::IDispatch_Impl + IRTCProfileEvent_Impl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_PROFILE_EVENT_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEvent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCProfileEvent2_Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCProfileEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IRTCProfileEvent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EventType: EventType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfileEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCReInviteEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn Accept(&mut self, bstrcontenttype: &super::super::Foundation::BSTR, bstrsessiondescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REINVITE_STATE>;
    fn GetRemoteSessionDescription(&mut self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCReInviteEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCReInviteEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCReInviteEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn Reject<Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn State<Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCRegistrationStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile>;
    fn State(&mut self) -> ::windows::core::Result<RTC_REGISTRATION_STATE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRegistrationStateChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRegistrationStateChangeEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCRegistrationStateChangeEvent_Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCRoamingEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_ROAMING_EVENT_TYPE>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRoamingEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRoamingEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCRoamingEvent_Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCSession_Impl: Sized {
    fn Client(&mut self) -> ::windows::core::Result<IRTCClient>;
    fn State(&mut self) -> ::windows::core::Result<RTC_SESSION_STATE>;
    fn Type(&mut self) -> ::windows::core::Result<RTC_SESSION_TYPE>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile>;
    fn Participants(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Answer(&mut self) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()>;
    fn Redirect(&mut self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &super::super::Foundation::BSTR, pprofile: &::core::option::Option<IRTCProfile>, lflags: i32) -> ::windows::core::Result<()>;
    fn AddParticipant(&mut self, bstraddress: &super::super::Foundation::BSTR, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRTCParticipant>;
    fn RemoveParticipant(&mut self, pparticipant: &::core::option::Option<IRTCParticipant>) -> ::windows::core::Result<()>;
    fn EnumerateParticipants(&mut self) -> ::windows::core::Result<IRTCEnumParticipants>;
    fn CanAddParticipants(&mut self) -> ::windows::core::Result<i16>;
    fn RedirectedUserURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RedirectedUserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn NextRedirectedUser(&mut self) -> ::windows::core::Result<()>;
    fn SendMessage(&mut self, bstrmessageheader: &super::super::Foundation::BSTR, bstrmessage: &super::super::Foundation::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
    fn SendMessageStatus(&mut self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()>;
    fn AddStream(&mut self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn RemoveStream(&mut self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn SetEncryptionKey(&mut self, lmediatype: i32, encryptionkey: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSession_Vtbl {
        unsafe extern "system" fn Client<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Client() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclient = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pentype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participants<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participants() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Answer<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Answer().into()
        }
        unsafe extern "system" fn Terminate<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&enreason)).into()
        }
        unsafe extern "system" fn Redirect<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Redirect(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bstrlocalphoneuri), ::core::mem::transmute(&pprofile), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddParticipant<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddParticipant(::core::mem::transmute_copy(&bstraddress), ::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveParticipant<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparticipant: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveParticipant(::core::mem::transmute(&pparticipant)).into()
        }
        unsafe extern "system" fn EnumerateParticipants<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateParticipants() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanAddParticipants<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcanadd: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAddParticipants() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcanadd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserURI<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectedUserURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruseruri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserName<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectedUserName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRedirectedUser<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextRedirectedUser().into()
        }
        unsafe extern "system" fn SendMessage<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessage(::core::mem::transmute_copy(&bstrmessageheader), ::core::mem::transmute_copy(&bstrmessage), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SendMessageStatus<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessageStatus(::core::mem::transmute_copy(&enuserstatus), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn AddStream<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn RemoveStream<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SetEncryptionKey<Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
pub trait IRTCSession2_Impl: Sized + IRTCSession_Impl {
    fn SendInfo(&mut self, bstrinfoheader: &super::super::Foundation::BSTR, bstrinfo: &super::super::Foundation::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
    fn SetPreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()>;
    fn PreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn IsSecurityEnabled(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<i16>;
    fn AnswerWithSessionDescription(&mut self, bstrcontenttype: &super::super::Foundation::BSTR, bstrsessiondescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReInviteWithSessionDescription(&mut self, bstrcontenttype: &super::super::Foundation::BSTR, bstrsessiondescription: &super::super::Foundation::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSession2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSession2_Vtbl {
        unsafe extern "system" fn SendInfo<Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinfo: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendInfo(::core::mem::transmute_copy(&bstrinfoheader), ::core::mem::transmute_copy(&bstrinfo), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfsecurityenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AnswerWithSessionDescription(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReInviteWithSessionDescription(::core::mem::transmute_copy(&bstrcontenttype), ::core::mem::transmute_copy(&bstrsessiondescription), ::core::mem::transmute_copy(&lcookie)).into()
        }
        Self {
            base: IRTCSession_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCSessionCallControl_Impl: Sized {
    fn Hold(&mut self, lcookie: isize) -> ::windows::core::Result<()>;
    fn UnHold(&mut self, lcookie: isize) -> ::windows::core::Result<()>;
    fn Forward(&mut self, bstrforwardtouri: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refer(&mut self, bstrrefertouri: &super::super::Foundation::BSTR, bstrrefercookie: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetReferredByURI(&mut self, bstrreferredbyuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReferredByURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetReferCookie(&mut self, bstrrefercookie: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReferCookie(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsReferred(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionCallControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionCallControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionCallControl_Vtbl {
        unsafe extern "system" fn Hold<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hold(::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn UnHold<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnHold(::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn Forward<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forward(::core::mem::transmute_copy(&bstrforwardtouri)).into()
        }
        unsafe extern "system" fn Refer<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefertouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refer(::core::mem::transmute_copy(&bstrrefertouri), ::core::mem::transmute_copy(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn SetReferredByURI<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferredByURI(::core::mem::transmute_copy(&bstrreferredbyuri)).into()
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferredByURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreferredbyuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferCookie<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferCookie(::core::mem::transmute_copy(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrefercookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReferred<Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreferred: *mut i16) -> ::windows::core::HRESULT {
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
pub trait IRTCSessionDescriptionManager_Impl: Sized {
    fn EvaluateSessionDescription(&mut self, bstrcontenttype: &super::super::Foundation::BSTR, bstrsessiondescription: &super::super::Foundation::BSTR, pfapplicationsession: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionDescriptionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionDescriptionManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionDescriptionManager_Vtbl {
        unsafe extern "system" fn EvaluateSessionDescription<Impl: IRTCSessionDescriptionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfapplicationsession: *mut i16) -> ::windows::core::HRESULT {
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
pub trait IRTCSessionOperationCompleteEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
    fn Cookie(&mut self) -> ::windows::core::Result<isize>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionOperationCompleteEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    *plcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCSessionOperationCompleteEvent2_Impl: Sized + super::Com::IDispatch_Impl + IRTCSessionOperationCompleteEvent_Impl {
    fn Participant(&mut self) -> ::windows::core::Result<IRTCParticipant>;
    fn GetRemoteSessionDescription(&mut self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEvent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionOperationCompleteEvent2_Vtbl {
        unsafe extern "system" fn Participant<Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparticipant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
        }
        Self {
            base: IRTCSessionOperationCompleteEvent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Participant: Participant::<Impl, IMPL_OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionOperationCompleteEvent2 as ::windows::core::Interface>::IID
    }
}
pub trait IRTCSessionPortManagement_Impl: Sized {
    fn SetPortManager(&mut self, pportmanager: &::core::option::Option<IRTCPortManager>) -> ::windows::core::Result<()>;
}
impl IRTCSessionPortManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionPortManagement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionPortManagement_Vtbl {
        unsafe extern "system" fn SetPortManager<Impl: IRTCSessionPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRTCSessionReferStatusEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn ReferStatus(&mut self) -> ::windows::core::Result<RTC_SESSION_REFER_STATUS>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferStatusEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferStatusEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionReferStatusEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferStatus<Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *penreferstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCSessionReferredEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession2>;
    fn ReferredByURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ReferToURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ReferCookie(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Accept(&mut self) -> ::windows::core::Result<()>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn SetReferredSessionState(&mut self, enstate: RTC_SESSION_STATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferredEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferredEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionReferredEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferredByURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreferredbyuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferToURI<Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferToURI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreferouri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrefercookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        unsafe extern "system" fn Reject<Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn SetReferredSessionState<Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferredSessionState(::core::mem::transmute_copy(&enstate)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCSessionStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IRTCSession>;
    fn State(&mut self) -> ::windows::core::Result<RTC_SESSION_STATE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn StatusText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionStateChangeEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCSessionStateChangeEvent2_Impl: Sized + super::Com::IDispatch_Impl + IRTCSessionStateChangeEvent_Impl {
    fn MediaTypes(&mut self) -> ::windows::core::Result<i32>;
    fn RemotePreferredSecurityLevel(&mut self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn IsForked(&mut self) -> ::windows::core::Result<i16>;
    fn GetRemoteSessionDescription(&mut self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEvent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCSessionStateChangeEvent2_Vtbl {
        unsafe extern "system" fn MediaTypes<Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pensecuritylevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsForked<Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisforked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsForked() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisforked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
        }
        Self {
            base: IRTCSessionStateChangeEvent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCUserSearch_Impl: Sized {
    fn CreateQuery(&mut self) -> ::windows::core::Result<IRTCUserSearchQuery>;
    fn ExecuteSearch(&mut self, pquery: &::core::option::Option<IRTCUserSearchQuery>, pprofile: &::core::option::Option<IRTCProfile>, lcookie: isize) -> ::windows::core::Result<()>;
}
impl IRTCUserSearch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearch_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearch_Vtbl {
        unsafe extern "system" fn CreateQuery<Impl: IRTCUserSearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteSearch<Impl: IRTCUserSearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lcookie: isize) -> ::windows::core::HRESULT {
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
pub trait IRTCUserSearchQuery_Impl: Sized {
    fn SetSearchTerm(&mut self, bstrname: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SearchTerm(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SearchTerms(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSearchPreference(&mut self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::Result<()>;
    fn SearchPreference(&mut self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> ::windows::core::Result<i32>;
    fn SetSearchDomain(&mut self, bstrdomain: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SearchDomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCUserSearchQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchQuery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchQuery_Vtbl {
        unsafe extern "system" fn SetSearchTerm<Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchTerm(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn SearchTerm<Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchTerm(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchTerms<Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchTerms() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchPreference<Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchPreference(::core::mem::transmute_copy(&enpreference), ::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn SearchPreference<Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPreference(::core::mem::transmute_copy(&enpreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *plvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchDomain<Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchDomain(::core::mem::transmute_copy(&bstrdomain)).into()
        }
        unsafe extern "system" fn SearchDomain<Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait IRTCUserSearchResult_Impl: Sized {
    fn Value(&mut self, encolumn: RTC_USER_SEARCH_COLUMN) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCUserSearchResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchResult_Vtbl {
        unsafe extern "system" fn Value<Impl: IRTCUserSearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait IRTCUserSearchResultsEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EnumerateResults(&mut self) -> ::windows::core::Result<IRTCEnumUserSearchResults>;
    fn Results(&mut self) -> ::windows::core::Result<IRTCCollection>;
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
    fn Query(&mut self) -> ::windows::core::Result<IRTCUserSearchQuery>;
    fn Cookie(&mut self) -> ::windows::core::Result<isize>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
    fn MoreAvailable(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCUserSearchResultsEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResultsEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCUserSearchResultsEvent_Vtbl {
        unsafe extern "system" fn EnumerateResults<Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateResults() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query() {
                ::core::result::Result::Ok(ok__) => {
                    *ppquery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    *plcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreAvailable<Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut i16) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRTCWatcher_Impl: Sized + IRTCPresenceContact_Impl {
    fn State(&mut self) -> ::windows::core::Result<RTC_WATCHER_STATE>;
    fn SetState(&mut self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcher_Vtbl {
        unsafe extern "system" fn State<Impl: IRTCWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *penstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IRTCWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&enstate)).into()
        }
        Self {
            base: IRTCPresenceContact_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCWatcher2_Impl: Sized + IRTCPresenceContact_Impl + IRTCWatcher_Impl {
    fn Profile(&mut self) -> ::windows::core::Result<IRTCProfile2>;
    fn Scope(&mut self) -> ::windows::core::Result<RTC_ACE_SCOPE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcher2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcher2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcher2_Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCWatcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: IRTCWatcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows::core::HRESULT {
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
            base: IRTCWatcher_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Profile: Profile::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCWatcherEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Watcher(&mut self) -> ::windows::core::Result<IRTCWatcher>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherEvent_Vtbl {
        unsafe extern "system" fn Watcher<Impl: IRTCWatcherEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watcher() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Watcher: Watcher::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcherEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCWatcherEvent2_Impl: Sized + super::Com::IDispatch_Impl + IRTCWatcherEvent_Impl {
    fn EventType(&mut self) -> ::windows::core::Result<RTC_WATCHER_EVENT_TYPE>;
    fn StatusCode(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEvent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRTCWatcherEvent2_Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCWatcherEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCWatcherEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
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
            base: IRTCWatcherEvent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcherEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait ITransportSettingsInternal_Impl: Sized {
    fn ApplySetting(&mut self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()>;
    fn QuerySetting(&mut self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ITransportSettingsInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransportSettingsInternal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransportSettingsInternal_Vtbl {
        unsafe extern "system" fn ApplySetting<Impl: ITransportSettingsInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplySetting(::core::mem::transmute_copy(&setting)).into()
        }
        unsafe extern "system" fn QuerySetting<Impl: ITransportSettingsInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
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
