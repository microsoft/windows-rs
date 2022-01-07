pub trait INetworkTransportSettingsImpl: Sized {
    fn ApplySetting();
    fn QuerySetting();
}
impl ::windows::core::RuntimeName for INetworkTransportSettings {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.INetworkTransportSettings";
}
impl INetworkTransportSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkTransportSettingsImpl, const OFFSET: isize>() -> INetworkTransportSettingsVtbl {
        unsafe extern "system" fn ApplySetting<Impl: INetworkTransportSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplySetting(&*(&settingid as *const <super::super::Networking::WinSock::TRANSPORT_SETTING_ID as ::windows::core::Abi>::Abi as *const <super::super::Networking::WinSock::TRANSPORT_SETTING_ID as ::windows::core::DefaultType>::DefaultType), lengthin, valuein, ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySetting<Impl: INetworkTransportSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySetting(&*(&settingid as *const <super::super::Networking::WinSock::TRANSPORT_SETTING_ID as ::windows::core::Abi>::Abi as *const <super::super::Networking::WinSock::TRANSPORT_SETTING_ID as ::windows::core::DefaultType>::DefaultType), lengthin, valuein, ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkTransportSettings>, ::windows::core::GetTrustLevel, ApplySetting::<Impl, OFFSET>, QuerySetting::<Impl, OFFSET>)
    }
}
pub trait INotificationTransportSyncImpl: Sized {
    fn CompleteDelivery();
    fn Flush();
}
impl ::windows::core::RuntimeName for INotificationTransportSync {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.INotificationTransportSync";
}
impl INotificationTransportSyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationTransportSyncImpl, const OFFSET: isize>() -> INotificationTransportSyncVtbl {
        unsafe extern "system" fn CompleteDelivery<Impl: INotificationTransportSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteDelivery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: INotificationTransportSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INotificationTransportSync>, ::windows::core::GetTrustLevel, CompleteDelivery::<Impl, OFFSET>, Flush::<Impl, OFFSET>)
    }
}
pub trait IRTCBuddyImpl: Sized + IRTCPresenceContactImpl {
    fn Status();
    fn Notes();
}
impl ::windows::core::RuntimeName for IRTCBuddy {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCBuddy";
}
impl IRTCBuddyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyImpl, const OFFSET: isize>() -> IRTCBuddyVtbl {
        unsafe extern "system" fn Status<Impl: IRTCBuddyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&penstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Impl: IRTCBuddyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notes(::core::mem::transmute_copy(&pbstrnotes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCBuddy>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Notes::<Impl, OFFSET>)
    }
}
pub trait IRTCBuddy2Impl: Sized + IRTCBuddyImpl + IRTCPresenceContactImpl {
    fn Profile();
    fn Refresh();
    fn EnumerateGroups();
    fn Groups();
    fn PresenceProperty();
    fn EnumeratePresenceDevices();
    fn PresenceDevices();
    fn SubscriptionType();
}
impl ::windows::core::RuntimeName for IRTCBuddy2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCBuddy2";
}
impl IRTCBuddy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddy2Impl, const OFFSET: isize>() -> IRTCBuddy2Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateGroups(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(enproperty, ::core::mem::transmute_copy(&pbstrproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePresenceDevices(::core::mem::transmute_copy(&ppenumdevices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceDevices<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceDevices(::core::mem::transmute_copy(&ppdevicescollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriptionType<Impl: IRTCBuddy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriptionType(::core::mem::transmute_copy(&pensubscriptiontype)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCBuddy2>,
            ::windows::core::GetTrustLevel,
            Profile::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            EnumerateGroups::<Impl, OFFSET>,
            Groups::<Impl, OFFSET>,
            PresenceProperty::<Impl, OFFSET>,
            EnumeratePresenceDevices::<Impl, OFFSET>,
            PresenceDevices::<Impl, OFFSET>,
            SubscriptionType::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyEventImpl: Sized + IDispatchImpl {
    fn Buddy();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCBuddyEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCBuddyEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEventImpl, const OFFSET: isize>() -> IRTCBuddyEventVtbl {
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy(::core::mem::transmute_copy(&ppbuddy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCBuddyEvent>, ::windows::core::GetTrustLevel, Buddy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyEvent2Impl: Sized + IRTCBuddyEventImpl + IDispatchImpl {
    fn EventType();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCBuddyEvent2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCBuddyEvent2";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>() -> IRTCBuddyEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCBuddyEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCBuddyEvent2>, ::windows::core::GetTrustLevel, EventType::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IRTCBuddyGroup {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCBuddyGroup";
}
impl IRTCBuddyGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroupImpl, const OFFSET: isize>() -> IRTCBuddyGroupVtbl {
        unsafe extern "system" fn Name<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrgroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBuddy(&*(&pbuddy as *const <IRTCBuddy as ::windows::core::Abi>::Abi as *const <IRTCBuddy as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveBuddy(&*(&pbuddy as *const <IRTCBuddy as ::windows::core::Abi>::Abi as *const <IRTCBuddy as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateBuddies(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddies(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data(::core::mem::transmute_copy(&pbstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCBuddyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCBuddyGroup>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            AddBuddy::<Impl, OFFSET>,
            RemoveBuddy::<Impl, OFFSET>,
            EnumerateBuddies::<Impl, OFFSET>,
            Buddies::<Impl, OFFSET>,
            Data::<Impl, OFFSET>,
            SetData::<Impl, OFFSET>,
            Profile::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyGroupEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Group();
    fn Buddy();
    fn StatusCode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCBuddyGroupEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCBuddyGroupEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyGroupEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>() -> IRTCBuddyGroupEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group(::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy(::core::mem::transmute_copy(&ppbuddy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCBuddyGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCBuddyGroupEvent>, ::windows::core::GetTrustLevel, EventType::<Impl, OFFSET>, Group::<Impl, OFFSET>, Buddy::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IRTCClient {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClient";
}
impl IRTCClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientImpl, const OFFSET: isize>() -> IRTCClientVtbl {
        unsafe extern "system" fn Initialize<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareForShutdown<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareForShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventFilter(lfilter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventFilter<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&plfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredMediaTypes(lmediatypes, fpersistent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredMediaTypes<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredMediaTypes(::core::mem::transmute_copy(&plmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCapabilities<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCapabilities(::core::mem::transmute_copy(&plmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession(entype, &*(&bstrlocalphoneuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType), lflags, ::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetListenForIncomingSessions(enlisten) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListenForIncomingSessions<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenForIncomingSessions(::core::mem::transmute_copy(&penlisten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAddresses<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftcp: i16, fexternal: i16, pvaddresses: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAddresses(ftcp, fexternal, ::core::mem::transmute_copy(&pvaddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolume(endevice, lvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume(endevice, ::core::mem::transmute_copy(&plvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioMuted<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAudioMuted(endevice, fmuted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioMuted<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioMuted(endevice, ::core::mem::transmute_copy(&pfmuted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IVideoWindow<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IVideoWindow(endevice, ::core::mem::transmute_copy(&ppivideowindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAudioDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredAudioDevice(endevice, &*(&bstrdevicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredAudioDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAudioDevice(endevice, ::core::mem::transmute_copy(&pbstrdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredVolume(endevice, lvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredVolume<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredVolume(endevice, ::core::mem::transmute_copy(&plvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAEC<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredAEC(benable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredAEC<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAEC(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredVideoDevice(&*(&bstrdevicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredVideoDevice<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredVideoDevice(::core::mem::transmute_copy(&pbstrdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveMedia<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveMedia(::core::mem::transmute_copy(&plmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBitrate<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxBitrate(lmaxbitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBitrate<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBitrate(::core::mem::transmute_copy(&plmaxbitrate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTemporalSpatialTradeOff(lvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemporalSpatialTradeOff(::core::mem::transmute_copy(&plvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkQuality<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkQuality(::core::mem::transmute_copy(&plnetworkquality)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartT120Applet<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartT120Applet(enapplet) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopT120Applets<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopT120Applets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsT120AppletRunning<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsT120AppletRunning(enapplet, ::core::mem::transmute_copy(&pfrunning)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserURI<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserURI(::core::mem::transmute_copy(&pbstruseruri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserURI<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalUserURI(&*(&bstruseruri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserName<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserName(::core::mem::transmute_copy(&pbstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserName<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalUserName(&*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayRing<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayRing(entype, bplay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDTMF<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDTMF(endtmf) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTuningWizard<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeTuningWizard(hwndparent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTuned<Impl: IRTCClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftuned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTuned(::core::mem::transmute_copy(&pftuned)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCClient>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            Shutdown::<Impl, OFFSET>,
            PrepareForShutdown::<Impl, OFFSET>,
            SetEventFilter::<Impl, OFFSET>,
            EventFilter::<Impl, OFFSET>,
            SetPreferredMediaTypes::<Impl, OFFSET>,
            PreferredMediaTypes::<Impl, OFFSET>,
            MediaCapabilities::<Impl, OFFSET>,
            CreateSession::<Impl, OFFSET>,
            SetListenForIncomingSessions::<Impl, OFFSET>,
            ListenForIncomingSessions::<Impl, OFFSET>,
            NetworkAddresses::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            Volume::<Impl, OFFSET>,
            SetAudioMuted::<Impl, OFFSET>,
            AudioMuted::<Impl, OFFSET>,
            IVideoWindow::<Impl, OFFSET>,
            SetPreferredAudioDevice::<Impl, OFFSET>,
            PreferredAudioDevice::<Impl, OFFSET>,
            SetPreferredVolume::<Impl, OFFSET>,
            PreferredVolume::<Impl, OFFSET>,
            SetPreferredAEC::<Impl, OFFSET>,
            PreferredAEC::<Impl, OFFSET>,
            SetPreferredVideoDevice::<Impl, OFFSET>,
            PreferredVideoDevice::<Impl, OFFSET>,
            ActiveMedia::<Impl, OFFSET>,
            SetMaxBitrate::<Impl, OFFSET>,
            MaxBitrate::<Impl, OFFSET>,
            SetTemporalSpatialTradeOff::<Impl, OFFSET>,
            TemporalSpatialTradeOff::<Impl, OFFSET>,
            NetworkQuality::<Impl, OFFSET>,
            StartT120Applet::<Impl, OFFSET>,
            StopT120Applets::<Impl, OFFSET>,
            IsT120AppletRunning::<Impl, OFFSET>,
            LocalUserURI::<Impl, OFFSET>,
            SetLocalUserURI::<Impl, OFFSET>,
            LocalUserName::<Impl, OFFSET>,
            SetLocalUserName::<Impl, OFFSET>,
            PlayRing::<Impl, OFFSET>,
            SendDTMF::<Impl, OFFSET>,
            InvokeTuningWizard::<Impl, OFFSET>,
            IsTuned::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IRTCClient2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClient2";
}
impl IRTCClient2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClient2Impl, const OFFSET: isize>() -> IRTCClient2Vtbl {
        unsafe extern "system" fn SetAnswerMode<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAnswerMode(entype, enmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerMode<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnswerMode(entype, ::core::mem::transmute_copy(&penmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeTuningWizardEx(hwndparent, fallowaudio, fallowvideo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&plversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientName(&*(&bstrclientname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientCurVer<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientcurver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientCurVer(&*(&bstrclientcurver as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeEx<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeEx(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionWithDescription<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionWithDescription(
                &*(&bstrcontenttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrsessiondescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType),
                lflags,
                ::core::mem::transmute_copy(&ppsession2),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSessionDescriptionManager(&*(&psessiondescriptionmanager as *const <IRTCSessionDescriptionManager as ::windows::core::Abi>::Abi as *const <IRTCSessionDescriptionManager as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredSecurityLevel(ensecuritytype, ensecuritylevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSecurityLevel(ensecuritytype, ::core::mem::transmute_copy(&pensecuritylevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedPorts<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowedPorts(ltransport, enlistenmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedPorts<Impl: IRTCClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedPorts(ltransport, ::core::mem::transmute_copy(&penlistenmode)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCClient2>,
            ::windows::core::GetTrustLevel,
            SetAnswerMode::<Impl, OFFSET>,
            AnswerMode::<Impl, OFFSET>,
            InvokeTuningWizardEx::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            SetClientName::<Impl, OFFSET>,
            SetClientCurVer::<Impl, OFFSET>,
            InitializeEx::<Impl, OFFSET>,
            CreateSessionWithDescription::<Impl, OFFSET>,
            SetSessionDescriptionManager::<Impl, OFFSET>,
            SetPreferredSecurityLevel::<Impl, OFFSET>,
            PreferredSecurityLevel::<Impl, OFFSET>,
            SetAllowedPorts::<Impl, OFFSET>,
            AllowedPorts::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Client();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCClientEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClientEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientEventImpl, const OFFSET: isize>() -> IRTCClientEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCClientEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peneventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Client<Impl: IRTCClientEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Client(::core::mem::transmute_copy(&ppclient)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCClientEvent>, ::windows::core::GetTrustLevel, EventType::<Impl, OFFSET>, Client::<Impl, OFFSET>)
    }
}
pub trait IRTCClientPortManagementImpl: Sized {
    fn StartListenAddressAndPort();
    fn StopListenAddressAndPort();
    fn GetPortRange();
}
impl ::windows::core::RuntimeName for IRTCClientPortManagement {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClientPortManagement";
}
impl IRTCClientPortManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPortManagementImpl, const OFFSET: isize>() -> IRTCClientPortManagementVtbl {
        unsafe extern "system" fn StartListenAddressAndPort<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartListenAddressAndPort(&*(&bstrinternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), linternallocalport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopListenAddressAndPort<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopListenAddressAndPort(&*(&bstrinternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), linternallocalport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortRange<Impl: IRTCClientPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPortRange(enporttype, ::core::mem::transmute_copy(&plminvalue), ::core::mem::transmute_copy(&plmaxvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCClientPortManagement>, ::windows::core::GetTrustLevel, StartListenAddressAndPort::<Impl, OFFSET>, StopListenAddressAndPort::<Impl, OFFSET>, GetPortRange::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IRTCClientPresence {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClientPresence";
}
impl IRTCClientPresenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresenceImpl, const OFFSET: isize>() -> IRTCClientPresenceVtbl {
        unsafe extern "system" fn EnablePresence<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnablePresence(fusestorage, &*(&varstorage as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Export<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Export(&*(&varstorage as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Import<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Import(&*(&varstorage as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), freplaceall) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateBuddies<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateBuddies(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddies(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buddy(&*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbuddy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBuddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBuddy(
                &*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                fpersistent,
                &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType),
                lflags,
                ::core::mem::transmute_copy(&ppbuddy),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBuddy<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveBuddy(&*(&pbuddy as *const <IRTCBuddy as ::windows::core::Abi>::Abi as *const <IRTCBuddy as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateWatchers<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateWatchers(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watchers<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watchers(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watcher(&*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwatcher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWatcher(
                &*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                fblocked,
                fpersistent,
                ::core::mem::transmute_copy(&ppwatcher),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWatcher<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwatcher: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveWatcher(&*(&pwatcher as *const <IRTCWatcher as ::windows::core::Abi>::Abi as *const <IRTCWatcher as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalPresenceInfo(enstatus, &*(&bstrnotes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferWatcherMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferWatcherMode(::core::mem::transmute_copy(&penmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfferWatcherMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOfferWatcherMode(enmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivacyMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivacyMode(::core::mem::transmute_copy(&penmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivacyMode<Impl: IRTCClientPresenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivacyMode(enmode) {
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
            ::windows::core::GetRuntimeClassName::<IRTCClientPresence>,
            ::windows::core::GetTrustLevel,
            EnablePresence::<Impl, OFFSET>,
            Export::<Impl, OFFSET>,
            Import::<Impl, OFFSET>,
            EnumerateBuddies::<Impl, OFFSET>,
            Buddies::<Impl, OFFSET>,
            Buddy::<Impl, OFFSET>,
            AddBuddy::<Impl, OFFSET>,
            RemoveBuddy::<Impl, OFFSET>,
            EnumerateWatchers::<Impl, OFFSET>,
            Watchers::<Impl, OFFSET>,
            Watcher::<Impl, OFFSET>,
            AddWatcher::<Impl, OFFSET>,
            RemoveWatcher::<Impl, OFFSET>,
            SetLocalPresenceInfo::<Impl, OFFSET>,
            OfferWatcherMode::<Impl, OFFSET>,
            SetOfferWatcherMode::<Impl, OFFSET>,
            PrivacyMode::<Impl, OFFSET>,
            SetPrivacyMode::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IRTCClientPresence2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClientPresence2";
}
impl IRTCClientPresence2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientPresence2Impl, const OFFSET: isize>() -> IRTCClientPresence2Vtbl {
        unsafe extern "system" fn EnablePresenceEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnablePresenceEx(&*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType), &*(&varstorage as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisablePresence<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisablePresence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddGroup<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddGroup(
                &*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType),
                lflags,
                ::core::mem::transmute_copy(&ppgroup),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGroup<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveGroup(&*(&pgroup as *const <IRTCBuddyGroup as ::windows::core::Abi>::Abi as *const <IRTCBuddyGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateGroups<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateGroups(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcherEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddWatcherEx(
                &*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                enstate,
                fpersistent,
                enscope,
                &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType),
                lflags,
                ::core::mem::transmute_copy(&ppwatcher),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WatcherEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WatcherEx(enmode, &*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwatcher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceProperty<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresenceProperty(enproperty, &*(&bstrproperty as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(enproperty, ::core::mem::transmute_copy(&pbstrproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceData<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresenceData(&*(&bstrnamespace as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBuddyEx<Impl: IRTCClientPresence2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBuddyEx(
                &*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                fpersistent,
                ensubscriptiontype,
                &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType),
                lflags,
                ::core::mem::transmute_copy(&ppbuddy),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IRTCClientPresence2>,
            ::windows::core::GetTrustLevel,
            EnablePresenceEx::<Impl, OFFSET>,
            DisablePresence::<Impl, OFFSET>,
            AddGroup::<Impl, OFFSET>,
            RemoveGroup::<Impl, OFFSET>,
            EnumerateGroups::<Impl, OFFSET>,
            Groups::<Impl, OFFSET>,
            Group::<Impl, OFFSET>,
            AddWatcherEx::<Impl, OFFSET>,
            WatcherEx::<Impl, OFFSET>,
            SetPresenceProperty::<Impl, OFFSET>,
            PresenceProperty::<Impl, OFFSET>,
            SetPresenceData::<Impl, OFFSET>,
            GetPresenceData::<Impl, OFFSET>,
            GetLocalPresenceInfo::<Impl, OFFSET>,
            AddBuddyEx::<Impl, OFFSET>,
        )
    }
}
pub trait IRTCClientProvisioningImpl: Sized {
    fn CreateProfile();
    fn EnableProfile();
    fn DisableProfile();
    fn EnumerateProfiles();
    fn Profiles();
    fn GetProfile();
    fn SessionCapabilities();
}
impl ::windows::core::RuntimeName for IRTCClientProvisioning {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClientProvisioning";
}
impl IRTCClientProvisioningVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioningImpl, const OFFSET: isize>() -> IRTCClientProvisioningVtbl {
        unsafe extern "system" fn CreateProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofilexml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProfile(&*(&bstrprofilexml as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableProfile(&*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType), lregisterflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableProfile(&*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateProfiles<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateProfiles(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profiles<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profiles(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfile<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfile(
                &*(&bstruseraccount as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstruserpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstruseruri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrserver as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ltransport,
                lcookie,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCClientProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionCapabilities(::core::mem::transmute_copy(&plsupportedsessions)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCClientProvisioning>,
            ::windows::core::GetTrustLevel,
            CreateProfile::<Impl, OFFSET>,
            EnableProfile::<Impl, OFFSET>,
            DisableProfile::<Impl, OFFSET>,
            EnumerateProfiles::<Impl, OFFSET>,
            Profiles::<Impl, OFFSET>,
            GetProfile::<Impl, OFFSET>,
            SessionCapabilities::<Impl, OFFSET>,
        )
    }
}
pub trait IRTCClientProvisioning2Impl: Sized + IRTCClientProvisioningImpl {
    fn EnableProfileEx();
}
impl ::windows::core::RuntimeName for IRTCClientProvisioning2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCClientProvisioning2";
}
impl IRTCClientProvisioning2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCClientProvisioning2Impl, const OFFSET: isize>() -> IRTCClientProvisioning2Vtbl {
        unsafe extern "system" fn EnableProfileEx<Impl: IRTCClientProvisioning2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableProfileEx(&*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType), lregisterflags, lroamingflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCClientProvisioning2>, ::windows::core::GetTrustLevel, EnableProfileEx::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCCollection {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCCollectionImpl, const OFFSET: isize>() -> IRTCCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&lcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IRTCCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppnewenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCCollection>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCDispatchEventNotificationImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCDispatchEventNotification {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCDispatchEventNotification";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCDispatchEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCDispatchEventNotificationImpl, const OFFSET: isize>() -> IRTCDispatchEventNotificationVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCDispatchEventNotification>, ::windows::core::GetTrustLevel)
    }
}
pub trait IRTCEnumBuddiesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IRTCEnumBuddies {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEnumBuddies";
}
impl IRTCEnumBuddiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>() -> IRTCEnumBuddiesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumBuddiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEnumBuddies>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IRTCEnumGroupsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IRTCEnumGroups {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEnumGroups";
}
impl IRTCEnumGroupsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumGroupsImpl, const OFFSET: isize>() -> IRTCEnumGroupsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEnumGroups>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IRTCEnumParticipantsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IRTCEnumParticipants {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEnumParticipants";
}
impl IRTCEnumParticipantsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>() -> IRTCEnumParticipantsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumParticipantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEnumParticipants>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IRTCEnumPresenceDevicesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IRTCEnumPresenceDevices {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEnumPresenceDevices";
}
impl IRTCEnumPresenceDevicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>() -> IRTCEnumPresenceDevicesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumPresenceDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEnumPresenceDevices>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IRTCEnumProfilesImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IRTCEnumProfiles {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEnumProfiles";
}
impl IRTCEnumProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumProfilesImpl, const OFFSET: isize>() -> IRTCEnumProfilesVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEnumProfiles>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IRTCEnumUserSearchResultsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IRTCEnumUserSearchResults {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEnumUserSearchResults";
}
impl IRTCEnumUserSearchResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>() -> IRTCEnumUserSearchResultsVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumUserSearchResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEnumUserSearchResults>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IRTCEnumWatchersImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IRTCEnumWatchers {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEnumWatchers";
}
impl IRTCEnumWatchersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEnumWatchersImpl, const OFFSET: isize>() -> IRTCEnumWatchersVtbl {
        unsafe extern "system" fn Next<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IRTCEnumWatchersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEnumWatchers>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IRTCEventNotificationImpl: Sized {
    fn Event();
}
impl ::windows::core::RuntimeName for IRTCEventNotification {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCEventNotification";
}
impl IRTCEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCEventNotificationImpl, const OFFSET: isize>() -> IRTCEventNotificationVtbl {
        unsafe extern "system" fn Event<Impl: IRTCEventNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event(rtcevent, &*(&pevent as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCEventNotification>, ::windows::core::GetTrustLevel, Event::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCInfoEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Participant();
    fn Info();
    fn InfoHeader();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCInfoEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCInfoEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCInfoEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCInfoEventImpl, const OFFSET: isize>() -> IRTCInfoEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant(::core::mem::transmute_copy(&ppparticipant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info(::core::mem::transmute_copy(&pbstrinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoHeader<Impl: IRTCInfoEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfoHeader(::core::mem::transmute_copy(&pbstrinfoheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCInfoEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, Participant::<Impl, OFFSET>, Info::<Impl, OFFSET>, InfoHeader::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCIntensityEventImpl: Sized + IDispatchImpl {
    fn Level();
    fn Min();
    fn Max();
    fn Direction();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCIntensityEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCIntensityEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCIntensityEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCIntensityEventImpl, const OFFSET: isize>() -> IRTCIntensityEventVtbl {
        unsafe extern "system" fn Level<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Level(::core::mem::transmute_copy(&pllevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Min(::core::mem::transmute_copy(&plmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Max(::core::mem::transmute_copy(&plmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: IRTCIntensityEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction(::core::mem::transmute_copy(&pendirection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCIntensityEvent>, ::windows::core::GetTrustLevel, Level::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Direction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMediaEventImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn EventType();
    fn EventReason();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCMediaEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCMediaEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaEventImpl, const OFFSET: isize>() -> IRTCMediaEventVtbl {
        unsafe extern "system" fn MediaType<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType(::core::mem::transmute_copy(&pmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peneventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventReason<Impl: IRTCMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventReason(::core::mem::transmute_copy(&peneventreason)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCMediaEvent>, ::windows::core::GetTrustLevel, MediaType::<Impl, OFFSET>, EventType::<Impl, OFFSET>, EventReason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMediaRequestEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ProposedMedia();
    fn CurrentMedia();
    fn Accept();
    fn RemotePreferredSecurityLevel();
    fn Reject();
    fn State();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCMediaRequestEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCMediaRequestEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaRequestEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>() -> IRTCMediaRequestEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProposedMedia<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProposedMedia(::core::mem::transmute_copy(&plmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMedia<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMedia(::core::mem::transmute_copy(&plmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accept(lmediatypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePreferredSecurityLevel(ensecuritytype, ::core::mem::transmute_copy(&pensecuritylevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCMediaRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCMediaRequestEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, ProposedMedia::<Impl, OFFSET>, CurrentMedia::<Impl, OFFSET>, Accept::<Impl, OFFSET>, RemotePreferredSecurityLevel::<Impl, OFFSET>, Reject::<Impl, OFFSET>, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMessagingEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Participant();
    fn EventType();
    fn Message();
    fn MessageHeader();
    fn UserStatus();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCMessagingEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCMessagingEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCMessagingEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCMessagingEventImpl, const OFFSET: isize>() -> IRTCMessagingEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant(::core::mem::transmute_copy(&ppparticipant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peneventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message(::core::mem::transmute_copy(&pbstrmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageHeader<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageHeader(::core::mem::transmute_copy(&pbstrmessageheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserStatus<Impl: IRTCMessagingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserStatus(::core::mem::transmute_copy(&penuserstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCMessagingEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, Participant::<Impl, OFFSET>, EventType::<Impl, OFFSET>, Message::<Impl, OFFSET>, MessageHeader::<Impl, OFFSET>, UserStatus::<Impl, OFFSET>)
    }
}
pub trait IRTCParticipantImpl: Sized {
    fn UserURI();
    fn Name();
    fn Removable();
    fn State();
    fn Session();
}
impl ::windows::core::RuntimeName for IRTCParticipant {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCParticipant";
}
impl IRTCParticipantVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipantImpl, const OFFSET: isize>() -> IRTCParticipantVtbl {
        unsafe extern "system" fn UserURI<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserURI(::core::mem::transmute_copy(&pbstruseruri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Removable<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfremovable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removable(::core::mem::transmute_copy(&pfremovable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&penstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IRTCParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCParticipant>, ::windows::core::GetTrustLevel, UserURI::<Impl, OFFSET>, Name::<Impl, OFFSET>, Removable::<Impl, OFFSET>, State::<Impl, OFFSET>, Session::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCParticipantStateChangeEventImpl: Sized + IDispatchImpl {
    fn Participant();
    fn State();
    fn StatusCode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCParticipantStateChangeEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCParticipantStateChangeEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCParticipantStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>() -> IRTCParticipantStateChangeEventVtbl {
        unsafe extern "system" fn Participant<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant(::core::mem::transmute_copy(&ppparticipant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&penstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCParticipantStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCParticipantStateChangeEvent>, ::windows::core::GetTrustLevel, Participant::<Impl, OFFSET>, State::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>)
    }
}
pub trait IRTCPortManagerImpl: Sized {
    fn GetMapping();
    fn UpdateRemoteAddress();
    fn ReleaseMapping();
}
impl ::windows::core::RuntimeName for IRTCPortManager {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCPortManager";
}
impl IRTCPortManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPortManagerImpl, const OFFSET: isize>() -> IRTCPortManagerVtbl {
        unsafe extern "system" fn GetMapping<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMapping(
                &*(&bstrremoteaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                enporttype,
                &*(&pbstrinternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                plinternallocalport,
                &*(&pbstrexternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                plexternallocalport,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateRemoteAddress<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateRemoteAddress(
                &*(&bstrremoteaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrinternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                linternallocalport,
                &*(&bstrexternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lexternallocalport,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMapping<Impl: IRTCPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocaladdress: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseMapping(&*(&bstrinternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), linternallocalport, &*(&bstrexternallocaladdress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lexternallocaladdress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCPortManager>, ::windows::core::GetTrustLevel, GetMapping::<Impl, OFFSET>, UpdateRemoteAddress::<Impl, OFFSET>, ReleaseMapping::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IRTCPresenceContact {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCPresenceContact";
}
impl IRTCPresenceContactVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceContactImpl, const OFFSET: isize>() -> IRTCPresenceContactVtbl {
        unsafe extern "system" fn PresentityURI<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentityURI(::core::mem::transmute_copy(&pbstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentityURI<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresentityURI(&*(&bstrpresentityuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data(::core::mem::transmute_copy(&pbstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Persistent<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpersistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Persistent(::core::mem::transmute_copy(&pfpersistent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPersistent<Impl: IRTCPresenceContactImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersistent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPersistent(fpersistent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCPresenceContact>, ::windows::core::GetTrustLevel, PresentityURI::<Impl, OFFSET>, SetPresentityURI::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>, Persistent::<Impl, OFFSET>, SetPersistent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresenceDataEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn GetPresenceData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCPresenceDataEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCPresenceDataEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceDataEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>() -> IRTCPresenceDataEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDataEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCPresenceDataEvent>, ::windows::core::GetTrustLevel, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>, GetPresenceData::<Impl, OFFSET>)
    }
}
pub trait IRTCPresenceDeviceImpl: Sized {
    fn Status();
    fn Notes();
    fn PresenceProperty();
    fn GetPresenceData();
}
impl ::windows::core::RuntimeName for IRTCPresenceDevice {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCPresenceDevice";
}
impl IRTCPresenceDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>() -> IRTCPresenceDeviceVtbl {
        unsafe extern "system" fn Status<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&penstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notes(::core::mem::transmute_copy(&pbstrnotes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(enproperty, ::core::mem::transmute_copy(&pbstrproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Impl: IRTCPresenceDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCPresenceDevice>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Notes::<Impl, OFFSET>, PresenceProperty::<Impl, OFFSET>, GetPresenceData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresencePropertyEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn PresenceProperty();
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCPresencePropertyEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCPresencePropertyEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresencePropertyEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>() -> IRTCPresencePropertyEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceProperty(::core::mem::transmute_copy(&penpresprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IRTCPresencePropertyEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCPresencePropertyEvent>, ::windows::core::GetTrustLevel, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>, PresenceProperty::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresenceStatusEventImpl: Sized + IDispatchImpl {
    fn StatusCode();
    fn StatusText();
    fn GetLocalPresenceInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCPresenceStatusEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCPresenceStatusEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>() -> IRTCPresenceStatusEventVtbl {
        unsafe extern "system" fn StatusCode<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Impl: IRTCPresenceStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCPresenceStatusEvent>, ::windows::core::GetTrustLevel, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>, GetLocalPresenceInfo::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IRTCProfile {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCProfile";
}
impl IRTCProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileImpl, const OFFSET: isize>() -> IRTCProfileVtbl {
        unsafe extern "system" fn Key<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrkey: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key(::core::mem::transmute_copy(&pbstrkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XML<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XML(::core::mem::transmute_copy(&pbstrxml)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderURI(enuri, ::core::mem::transmute_copy(&pbstruri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderData<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderData(::core::mem::transmute_copy(&pbstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientBanner<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbanner: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientBanner(::core::mem::transmute_copy(&pfbanner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientMinVer<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrminver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientMinVer(::core::mem::transmute_copy(&pbstrminver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCurVer<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCurVer(::core::mem::transmute_copy(&pbstrcurver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientUpdateURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientUpdateURI(::core::mem::transmute_copy(&pbstrupdateuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientData<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientData(::core::mem::transmute_copy(&pbstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserURI<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserURI(::core::mem::transmute_copy(&pbstruseruri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName(::core::mem::transmute_copy(&pbstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAccount(::core::mem::transmute_copy(&pbstruseraccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCredentials(
                &*(&bstruseruri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstruseraccount as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionCapabilities<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionCapabilities(::core::mem::transmute_copy(&plsupportedsessions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&penstate)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCProfile>,
            ::windows::core::GetTrustLevel,
            Key::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            XML::<Impl, OFFSET>,
            ProviderName::<Impl, OFFSET>,
            ProviderURI::<Impl, OFFSET>,
            ProviderData::<Impl, OFFSET>,
            ClientName::<Impl, OFFSET>,
            ClientBanner::<Impl, OFFSET>,
            ClientMinVer::<Impl, OFFSET>,
            ClientCurVer::<Impl, OFFSET>,
            ClientUpdateURI::<Impl, OFFSET>,
            ClientData::<Impl, OFFSET>,
            UserURI::<Impl, OFFSET>,
            UserName::<Impl, OFFSET>,
            UserAccount::<Impl, OFFSET>,
            SetCredentials::<Impl, OFFSET>,
            SessionCapabilities::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
        )
    }
}
pub trait IRTCProfile2Impl: Sized + IRTCProfileImpl {
    fn Realm();
    fn SetRealm();
    fn AllowedAuth();
    fn SetAllowedAuth();
}
impl ::windows::core::RuntimeName for IRTCProfile2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCProfile2";
}
impl IRTCProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfile2Impl, const OFFSET: isize>() -> IRTCProfile2Vtbl {
        unsafe extern "system" fn Realm<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrealm: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Realm(::core::mem::transmute_copy(&pbstrrealm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealm<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRealm(&*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedAuth<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedAuth(::core::mem::transmute_copy(&plallowedauth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedAuth<Impl: IRTCProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowedAuth(lallowedauth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCProfile2>, ::windows::core::GetTrustLevel, Realm::<Impl, OFFSET>, SetRealm::<Impl, OFFSET>, AllowedAuth::<Impl, OFFSET>, SetAllowedAuth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCProfileEventImpl: Sized + IDispatchImpl {
    fn Profile();
    fn Cookie();
    fn StatusCode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCProfileEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCProfileEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEventImpl, const OFFSET: isize>() -> IRTCProfileEventVtbl {
        unsafe extern "system" fn Profile<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie(::core::mem::transmute_copy(&plcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCProfileEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCProfileEvent>, ::windows::core::GetTrustLevel, Profile::<Impl, OFFSET>, Cookie::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCProfileEvent2Impl: Sized + IRTCProfileEventImpl + IDispatchImpl {
    fn EventType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCProfileEvent2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCProfileEvent2";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCProfileEvent2Impl, const OFFSET: isize>() -> IRTCProfileEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCProfileEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCProfileEvent2>, ::windows::core::GetTrustLevel, EventType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCReInviteEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Accept();
    fn Reject();
    fn State();
    fn GetRemoteSessionDescription();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCReInviteEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCReInviteEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCReInviteEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCReInviteEventImpl, const OFFSET: isize>() -> IRTCReInviteEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accept(&*(&bstrcontenttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrsessiondescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCReInviteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCReInviteEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, Accept::<Impl, OFFSET>, Reject::<Impl, OFFSET>, State::<Impl, OFFSET>, GetRemoteSessionDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCRegistrationStateChangeEventImpl: Sized + IDispatchImpl {
    fn Profile();
    fn State();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCRegistrationStateChangeEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCRegistrationStateChangeEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCRegistrationStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>() -> IRTCRegistrationStateChangeEventVtbl {
        unsafe extern "system" fn Profile<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&penstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRegistrationStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCRegistrationStateChangeEvent>, ::windows::core::GetTrustLevel, Profile::<Impl, OFFSET>, State::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCRoamingEventImpl: Sized + IDispatchImpl {
    fn EventType();
    fn Profile();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCRoamingEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCRoamingEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCRoamingEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCRoamingEventImpl, const OFFSET: isize>() -> IRTCRoamingEventVtbl {
        unsafe extern "system" fn EventType<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCRoamingEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCRoamingEvent>, ::windows::core::GetTrustLevel, EventType::<Impl, OFFSET>, Profile::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IRTCSession {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSession";
}
impl IRTCSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionImpl, const OFFSET: isize>() -> IRTCSessionVtbl {
        unsafe extern "system" fn Client<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Client(::core::mem::transmute_copy(&ppclient)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&penstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&pentype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participants(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Answer<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Answer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate(enreason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Redirect<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Redirect(entype, &*(&bstrlocalphoneuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddParticipant<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddParticipant(&*(&bstraddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppparticipant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveParticipant<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparticipant: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveParticipant(&*(&pparticipant as *const <IRTCParticipant as ::windows::core::Abi>::Abi as *const <IRTCParticipant as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateParticipants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateParticipants(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanAddParticipants<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcanadd: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAddParticipants(::core::mem::transmute_copy(&pfcanadd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserURI<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectedUserURI(::core::mem::transmute_copy(&pbstruseruri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserName<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectedUserName(::core::mem::transmute_copy(&pbstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRedirectedUser<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextRedirectedUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessage<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessage(&*(&bstrmessageheader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessageStatus<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessageStatus(enuserstatus, lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStream(lmediatype, lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStream(lmediatype, lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionKey<Impl: IRTCSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEncryptionKey(lmediatype, &*(&encryptionkey as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCSession>,
            ::windows::core::GetTrustLevel,
            Client::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            Profile::<Impl, OFFSET>,
            Participants::<Impl, OFFSET>,
            Answer::<Impl, OFFSET>,
            Terminate::<Impl, OFFSET>,
            Redirect::<Impl, OFFSET>,
            AddParticipant::<Impl, OFFSET>,
            RemoveParticipant::<Impl, OFFSET>,
            EnumerateParticipants::<Impl, OFFSET>,
            CanAddParticipants::<Impl, OFFSET>,
            RedirectedUserURI::<Impl, OFFSET>,
            RedirectedUserName::<Impl, OFFSET>,
            NextRedirectedUser::<Impl, OFFSET>,
            SendMessage::<Impl, OFFSET>,
            SendMessageStatus::<Impl, OFFSET>,
            AddStream::<Impl, OFFSET>,
            RemoveStream::<Impl, OFFSET>,
            SetEncryptionKey::<Impl, OFFSET>,
        )
    }
}
pub trait IRTCSession2Impl: Sized + IRTCSessionImpl {
    fn SendInfo();
    fn SetPreferredSecurityLevel();
    fn PreferredSecurityLevel();
    fn IsSecurityEnabled();
    fn AnswerWithSessionDescription();
    fn ReInviteWithSessionDescription();
}
impl ::windows::core::RuntimeName for IRTCSession2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSession2";
}
impl IRTCSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSession2Impl, const OFFSET: isize>() -> IRTCSession2Vtbl {
        unsafe extern "system" fn SendInfo<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinfo: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendInfo(&*(&bstrinfoheader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrinfo as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredSecurityLevel<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredSecurityLevel(ensecuritytype, ensecuritylevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredSecurityLevel<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSecurityLevel(ensecuritytype, ::core::mem::transmute_copy(&pensecuritylevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled(ensecuritytype, ::core::mem::transmute_copy(&pfsecurityenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnswerWithSessionDescription(&*(&bstrcontenttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrsessiondescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Impl: IRTCSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReInviteWithSessionDescription(&*(&bstrcontenttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrsessiondescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lcookie) {
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
            ::windows::core::GetRuntimeClassName::<IRTCSession2>,
            ::windows::core::GetTrustLevel,
            SendInfo::<Impl, OFFSET>,
            SetPreferredSecurityLevel::<Impl, OFFSET>,
            PreferredSecurityLevel::<Impl, OFFSET>,
            IsSecurityEnabled::<Impl, OFFSET>,
            AnswerWithSessionDescription::<Impl, OFFSET>,
            ReInviteWithSessionDescription::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IRTCSessionCallControl {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionCallControl";
}
impl IRTCSessionCallControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionCallControlImpl, const OFFSET: isize>() -> IRTCSessionCallControlVtbl {
        unsafe extern "system" fn Hold<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hold(lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnHold<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnHold(lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Forward(&*(&bstrforwardtouri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refer<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefertouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refer(&*(&bstrrefertouri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrrefercookie as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferredByURI<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReferredByURI(&*(&bstrreferredbyuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferredByURI(::core::mem::transmute_copy(&pbstrreferredbyuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferCookie<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReferCookie(&*(&bstrrefercookie as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferCookie(::core::mem::transmute_copy(&pbstrrefercookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReferred<Impl: IRTCSessionCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreferred: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReferred(::core::mem::transmute_copy(&pfisreferred)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCSessionCallControl>,
            ::windows::core::GetTrustLevel,
            Hold::<Impl, OFFSET>,
            UnHold::<Impl, OFFSET>,
            Forward::<Impl, OFFSET>,
            Refer::<Impl, OFFSET>,
            SetReferredByURI::<Impl, OFFSET>,
            ReferredByURI::<Impl, OFFSET>,
            SetReferCookie::<Impl, OFFSET>,
            ReferCookie::<Impl, OFFSET>,
            IsReferred::<Impl, OFFSET>,
        )
    }
}
pub trait IRTCSessionDescriptionManagerImpl: Sized {
    fn EvaluateSessionDescription();
}
impl ::windows::core::RuntimeName for IRTCSessionDescriptionManager {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionDescriptionManager";
}
impl IRTCSessionDescriptionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionDescriptionManagerImpl, const OFFSET: isize>() -> IRTCSessionDescriptionManagerVtbl {
        unsafe extern "system" fn EvaluateSessionDescription<Impl: IRTCSessionDescriptionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfapplicationsession: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateSessionDescription(&*(&bstrcontenttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrsessiondescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pfapplicationsession) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionDescriptionManager>, ::windows::core::GetTrustLevel, EvaluateSessionDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionOperationCompleteEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Cookie();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCSessionOperationCompleteEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionOperationCompleteEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>() -> IRTCSessionOperationCompleteEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie(::core::mem::transmute_copy(&plcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionOperationCompleteEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionOperationCompleteEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, Cookie::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionOperationCompleteEvent2Impl: Sized + IRTCSessionOperationCompleteEventImpl + IDispatchImpl {
    fn Participant();
    fn GetRemoteSessionDescription();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCSessionOperationCompleteEvent2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionOperationCompleteEvent2";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionOperationCompleteEvent2Impl, const OFFSET: isize>() -> IRTCSessionOperationCompleteEvent2Vtbl {
        unsafe extern "system" fn Participant<Impl: IRTCSessionOperationCompleteEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant(::core::mem::transmute_copy(&ppparticipant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionOperationCompleteEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionOperationCompleteEvent2>, ::windows::core::GetTrustLevel, Participant::<Impl, OFFSET>, GetRemoteSessionDescription::<Impl, OFFSET>)
    }
}
pub trait IRTCSessionPortManagementImpl: Sized {
    fn SetPortManager();
}
impl ::windows::core::RuntimeName for IRTCSessionPortManagement {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionPortManagement";
}
impl IRTCSessionPortManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionPortManagementImpl, const OFFSET: isize>() -> IRTCSessionPortManagementVtbl {
        unsafe extern "system" fn SetPortManager<Impl: IRTCSessionPortManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPortManager(&*(&pportmanager as *const <IRTCPortManager as ::windows::core::Abi>::Abi as *const <IRTCPortManager as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionPortManagement>, ::windows::core::GetTrustLevel, SetPortManager::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionReferStatusEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ReferStatus();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCSessionReferStatusEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionReferStatusEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferStatusEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>() -> IRTCSessionReferStatusEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferStatus<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferStatus(::core::mem::transmute_copy(&penreferstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionReferStatusEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionReferStatusEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, ReferStatus::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionReferredEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn ReferredByURI();
    fn ReferToURI();
    fn ReferCookie();
    fn Accept();
    fn Reject();
    fn SetReferredSessionState();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCSessionReferredEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionReferredEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferredEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>() -> IRTCSessionReferredEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferredByURI<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferredByURI(::core::mem::transmute_copy(&pbstrreferredbyuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferToURI<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferToURI(::core::mem::transmute_copy(&pbstrreferouri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferCookie<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferCookie(::core::mem::transmute_copy(&pbstrrefercookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accept() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferredSessionState<Impl: IRTCSessionReferredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReferredSessionState(enstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionReferredEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, ReferredByURI::<Impl, OFFSET>, ReferToURI::<Impl, OFFSET>, ReferCookie::<Impl, OFFSET>, Accept::<Impl, OFFSET>, Reject::<Impl, OFFSET>, SetReferredSessionState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionStateChangeEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn State();
    fn StatusCode();
    fn StatusText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCSessionStateChangeEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionStateChangeEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>() -> IRTCSessionStateChangeEventVtbl {
        unsafe extern "system" fn Session<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&penstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Impl: IRTCSessionStateChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusText(::core::mem::transmute_copy(&pbstrstatustext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionStateChangeEvent>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, State::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>, StatusText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionStateChangeEvent2Impl: Sized + IRTCSessionStateChangeEventImpl + IDispatchImpl {
    fn MediaTypes();
    fn RemotePreferredSecurityLevel();
    fn IsForked();
    fn GetRemoteSessionDescription();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCSessionStateChangeEvent2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCSessionStateChangeEvent2";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>() -> IRTCSessionStateChangeEvent2Vtbl {
        unsafe extern "system" fn MediaTypes<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypes(::core::mem::transmute_copy(&pmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePreferredSecurityLevel<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePreferredSecurityLevel(ensecuritytype, ::core::mem::transmute_copy(&pensecuritylevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsForked<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisforked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsForked(::core::mem::transmute_copy(&pfisforked)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Impl: IRTCSessionStateChangeEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCSessionStateChangeEvent2>, ::windows::core::GetTrustLevel, MediaTypes::<Impl, OFFSET>, RemotePreferredSecurityLevel::<Impl, OFFSET>, IsForked::<Impl, OFFSET>, GetRemoteSessionDescription::<Impl, OFFSET>)
    }
}
pub trait IRTCUserSearchImpl: Sized {
    fn CreateQuery();
    fn ExecuteSearch();
}
impl ::windows::core::RuntimeName for IRTCUserSearch {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCUserSearch";
}
impl IRTCUserSearchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchImpl, const OFFSET: isize>() -> IRTCUserSearchVtbl {
        unsafe extern "system" fn CreateQuery<Impl: IRTCUserSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuery(::core::mem::transmute_copy(&ppquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteSearch<Impl: IRTCUserSearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteSearch(&*(&pquery as *const <IRTCUserSearchQuery as ::windows::core::Abi>::Abi as *const <IRTCUserSearchQuery as ::windows::core::DefaultType>::DefaultType), &*(&pprofile as *const <IRTCProfile as ::windows::core::Abi>::Abi as *const <IRTCProfile as ::windows::core::DefaultType>::DefaultType), lcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCUserSearch>, ::windows::core::GetTrustLevel, CreateQuery::<Impl, OFFSET>, ExecuteSearch::<Impl, OFFSET>)
    }
}
pub trait IRTCUserSearchQueryImpl: Sized {
    fn SetSearchTerm();
    fn SearchTerm();
    fn SearchTerms();
    fn SetSearchPreference();
    fn SearchPreference();
    fn SetSearchDomain();
    fn SearchDomain();
}
impl ::windows::core::RuntimeName for IRTCUserSearchQuery {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCUserSearchQuery";
}
impl IRTCUserSearchQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>() -> IRTCUserSearchQueryVtbl {
        unsafe extern "system" fn SetSearchTerm<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSearchTerm(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchTerm<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchTerm(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchTerms<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnames: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchTerms(::core::mem::transmute_copy(&pbstrnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchPreference<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSearchPreference(enpreference, lvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPreference<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchPreference(enpreference, ::core::mem::transmute_copy(&plvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchDomain<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSearchDomain(&*(&bstrdomain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchDomain<Impl: IRTCUserSearchQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchDomain(::core::mem::transmute_copy(&pbstrdomain)) {
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
            ::windows::core::GetRuntimeClassName::<IRTCUserSearchQuery>,
            ::windows::core::GetTrustLevel,
            SetSearchTerm::<Impl, OFFSET>,
            SearchTerm::<Impl, OFFSET>,
            SearchTerms::<Impl, OFFSET>,
            SetSearchPreference::<Impl, OFFSET>,
            SearchPreference::<Impl, OFFSET>,
            SetSearchDomain::<Impl, OFFSET>,
            SearchDomain::<Impl, OFFSET>,
        )
    }
}
pub trait IRTCUserSearchResultImpl: Sized {
    fn Value();
}
impl ::windows::core::RuntimeName for IRTCUserSearchResult {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCUserSearchResult";
}
impl IRTCUserSearchResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResultImpl, const OFFSET: isize>() -> IRTCUserSearchResultVtbl {
        unsafe extern "system" fn Value<Impl: IRTCUserSearchResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(encolumn, ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCUserSearchResult>, ::windows::core::GetTrustLevel, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCUserSearchResultsEventImpl: Sized + IDispatchImpl {
    fn EnumerateResults();
    fn Results();
    fn Profile();
    fn Query();
    fn Cookie();
    fn StatusCode();
    fn MoreAvailable();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCUserSearchResultsEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCUserSearchResultsEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCUserSearchResultsEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>() -> IRTCUserSearchResultsEventVtbl {
        unsafe extern "system" fn EnumerateResults<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateResults(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(::core::mem::transmute_copy(&ppquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookie(::core::mem::transmute_copy(&plcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreAvailable<Impl: IRTCUserSearchResultsEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoreAvailable(::core::mem::transmute_copy(&pfmoreavailable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCUserSearchResultsEvent>, ::windows::core::GetTrustLevel, EnumerateResults::<Impl, OFFSET>, Results::<Impl, OFFSET>, Profile::<Impl, OFFSET>, Query::<Impl, OFFSET>, Cookie::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>, MoreAvailable::<Impl, OFFSET>)
    }
}
pub trait IRTCWatcherImpl: Sized + IRTCPresenceContactImpl {
    fn State();
    fn SetState();
}
impl ::windows::core::RuntimeName for IRTCWatcher {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCWatcher";
}
impl IRTCWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherImpl, const OFFSET: isize>() -> IRTCWatcherVtbl {
        unsafe extern "system" fn State<Impl: IRTCWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&penstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IRTCWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(enstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCWatcher>, ::windows::core::GetTrustLevel, State::<Impl, OFFSET>, SetState::<Impl, OFFSET>)
    }
}
pub trait IRTCWatcher2Impl: Sized + IRTCWatcherImpl + IRTCPresenceContactImpl {
    fn Profile();
    fn Scope();
}
impl ::windows::core::RuntimeName for IRTCWatcher2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCWatcher2";
}
impl IRTCWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcher2Impl, const OFFSET: isize>() -> IRTCWatcher2Vtbl {
        unsafe extern "system" fn Profile<Impl: IRTCWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile(::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: IRTCWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope(::core::mem::transmute_copy(&penscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCWatcher2>, ::windows::core::GetTrustLevel, Profile::<Impl, OFFSET>, Scope::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCWatcherEventImpl: Sized + IDispatchImpl {
    fn Watcher();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCWatcherEvent {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCWatcherEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEventImpl, const OFFSET: isize>() -> IRTCWatcherEventVtbl {
        unsafe extern "system" fn Watcher<Impl: IRTCWatcherEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watcher(::core::mem::transmute_copy(&ppwatcher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCWatcherEvent>, ::windows::core::GetTrustLevel, Watcher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCWatcherEvent2Impl: Sized + IRTCWatcherEventImpl + IDispatchImpl {
    fn EventType();
    fn StatusCode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCWatcherEvent2 {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.IRTCWatcherEvent2";
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRTCWatcherEvent2Impl, const OFFSET: isize>() -> IRTCWatcherEvent2Vtbl {
        unsafe extern "system" fn EventType<Impl: IRTCWatcherEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType(::core::mem::transmute_copy(&peventtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Impl: IRTCWatcherEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode(::core::mem::transmute_copy(&plstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRTCWatcherEvent2>, ::windows::core::GetTrustLevel, EventType::<Impl, OFFSET>, StatusCode::<Impl, OFFSET>)
    }
}
pub trait ITransportSettingsInternalImpl: Sized {
    fn ApplySetting();
    fn QuerySetting();
}
impl ::windows::core::RuntimeName for ITransportSettingsInternal {
    const NAME: &'static str = "Windows.Win32.System.RealTimeCommunications.ITransportSettingsInternal";
}
impl ITransportSettingsInternalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransportSettingsInternalImpl, const OFFSET: isize>() -> ITransportSettingsInternalVtbl {
        unsafe extern "system" fn ApplySetting<Impl: ITransportSettingsInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplySetting(&*(&setting as *const <TRANSPORT_SETTING as ::windows::core::Abi>::Abi as *const <TRANSPORT_SETTING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySetting<Impl: ITransportSettingsInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySetting(&*(&setting as *const <TRANSPORT_SETTING as ::windows::core::Abi>::Abi as *const <TRANSPORT_SETTING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransportSettingsInternal>, ::windows::core::GetTrustLevel, ApplySetting::<Impl, OFFSET>, QuerySetting::<Impl, OFFSET>)
    }
}
