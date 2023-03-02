#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Networking_WinSock\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait INetworkTransportSettings_Impl: Sized {
    fn ApplySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()>;
    fn QuerySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::RuntimeName for INetworkTransportSettings {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl INetworkTransportSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkTransportSettings_Impl, const OFFSET: isize>() -> INetworkTransportSettings_Vtbl {
        unsafe extern "system" fn ApplySetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkTransportSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplySetting(::core::mem::transmute_copy(&settingid), ::core::mem::transmute_copy(&lengthin), ::core::mem::transmute_copy(&valuein), ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)).into()
        }
        unsafe extern "system" fn QuerySetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkTransportSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QuerySetting(::core::mem::transmute_copy(&settingid), ::core::mem::transmute_copy(&lengthin), ::core::mem::transmute_copy(&valuein), ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplySetting: ApplySetting::<Identity, Impl, OFFSET>,
            QuerySetting: QuerySetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkTransportSettings as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait INotificationTransportSync_Impl: Sized {
    fn CompleteDelivery(&self) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INotificationTransportSync {}
impl INotificationTransportSync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INotificationTransportSync_Impl, const OFFSET: isize>() -> INotificationTransportSync_Vtbl {
        unsafe extern "system" fn CompleteDelivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INotificationTransportSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteDelivery().into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INotificationTransportSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompleteDelivery: CompleteDelivery::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationTransportSync as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCBuddy_Impl: Sized + IRTCPresenceContact_Impl {
    fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCBuddy {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCBuddy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy_Impl, const OFFSET: isize>() -> IRTCBuddy_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Notes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnotes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRTCPresenceContact_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Notes: Notes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddy as ::windows::core::ComInterface>::IID || iid == &<IRTCPresenceContact as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCBuddy2_Impl: Sized + IRTCBuddy_Impl {
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile2>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn EnumerateGroups(&self) -> ::windows::core::Result<IRTCEnumGroups>;
    fn Groups(&self) -> ::windows::core::Result<IRTCCollection>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<::windows::core::BSTR>;
    fn EnumeratePresenceDevices(&self) -> ::windows::core::Result<IRTCEnumPresenceDevices>;
    fn PresenceDevices(&self) -> ::windows::core::Result<IRTCCollection>;
    fn SubscriptionType(&self) -> ::windows::core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IRTCBuddy2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCBuddy2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>() -> IRTCBuddy2_Vtbl {
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn EnumerateGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Groups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumeratePresenceDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PresenceDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevicescollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriptionType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubscriptionType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensubscriptiontype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRTCBuddy_Vtbl::new::<Identity, Impl, OFFSET>(),
            Profile: Profile::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            EnumerateGroups: EnumerateGroups::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, Impl, OFFSET>,
            EnumeratePresenceDevices: EnumeratePresenceDevices::<Identity, Impl, OFFSET>,
            PresenceDevices: PresenceDevices::<Identity, Impl, OFFSET>,
            SubscriptionType: SubscriptionType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddy2 as ::windows::core::ComInterface>::IID || iid == &<IRTCPresenceContact as ::windows::core::ComInterface>::IID || iid == &<IRTCBuddy as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCBuddyEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyEvent_Impl, const OFFSET: isize>() -> IRTCBuddyEvent_Vtbl {
        unsafe extern "system" fn Buddy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Buddy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Buddy: Buddy::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyEvent2_Impl: Sized + IRTCBuddyEvent_Impl {
    fn EventType(&self) -> ::windows::core::Result<RTC_BUDDY_EVENT_TYPE>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCBuddyEvent2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: isize>() -> IRTCBuddyEvent2_Vtbl {
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRTCBuddyEvent_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventType: EventType::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyEvent2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IRTCBuddyEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyGroup_Impl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, bstrgroupname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AddBuddy(&self, pbuddy: ::core::option::Option<&IRTCBuddy>) -> ::windows::core::Result<()>;
    fn RemoveBuddy(&self, pbuddy: ::core::option::Option<&IRTCBuddy>) -> ::windows::core::Result<()>;
    fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies>;
    fn Buddies(&self) -> ::windows::core::Result<IRTCCollection>;
    fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetData(&self, bstrdata: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile2>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCBuddyGroup {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>() -> IRTCBuddyGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrgroupname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrgroupname)).into()
        }
        unsafe extern "system" fn AddBuddy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddBuddy(::windows::core::from_raw_borrowed(&pbuddy)).into()
        }
        unsafe extern "system" fn RemoveBuddy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveBuddy(::windows::core::from_raw_borrowed(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateBuddies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Buddies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            AddBuddy: AddBuddy::<Identity, Impl, OFFSET>,
            RemoveBuddy: RemoveBuddy::<Identity, Impl, OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Identity, Impl, OFFSET>,
            Buddies: Buddies::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyGroup as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCBuddyGroupEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&self) -> ::windows::core::Result<RTC_GROUP_EVENT_TYPE>;
    fn Group(&self) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy2>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCBuddyGroupEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCBuddyGroupEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>() -> IRTCBuddyGroupEvent_Vtbl {
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Group() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Buddy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventType: EventType::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            Buddy: Buddy::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCBuddyGroupEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClient_Impl: Sized {
    fn Initialize(&self) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
    fn PrepareForShutdown(&self) -> ::windows::core::Result<()>;
    fn SetEventFilter(&self, lfilter: i32) -> ::windows::core::Result<()>;
    fn EventFilter(&self) -> ::windows::core::Result<i32>;
    fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn PreferredMediaTypes(&self) -> ::windows::core::Result<i32>;
    fn MediaCapabilities(&self) -> ::windows::core::Result<i32>;
    fn CreateSession(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &::windows::core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCSession>;
    fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()>;
    fn ListenForIncomingSessions(&self) -> ::windows::core::Result<RTC_LISTEN_MODE>;
    fn get_NetworkAddresses(&self, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<super::Com::VARIANT>;
    fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()>;
    fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32>;
    fn put_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow>;
    fn put_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<::windows::core::BSTR>;
    fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()>;
    fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32>;
    fn SetPreferredAEC(&self, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn PreferredAEC(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPreferredVideoDevice(&self, bstrdevicename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PreferredVideoDevice(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ActiveMedia(&self) -> ::windows::core::Result<i32>;
    fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows::core::Result<()>;
    fn MaxBitrate(&self) -> ::windows::core::Result<i32>;
    fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows::core::Result<()>;
    fn TemporalSpatialTradeOff(&self) -> ::windows::core::Result<i32>;
    fn NetworkQuality(&self) -> ::windows::core::Result<i32>;
    fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<()>;
    fn StopT120Applets(&self) -> ::windows::core::Result<()>;
    fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LocalUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLocalUserURI(&self, bstruseruri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn LocalUserName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLocalUserName(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PlayRing(&self, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows::core::Result<()>;
    fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows::core::Result<()>;
    fn IsTuned(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCClient {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>() -> IRTCClient_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize().into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        unsafe extern "system" fn PrepareForShutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareForShutdown().into()
        }
        unsafe extern "system" fn SetEventFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventFilter(::core::mem::transmute_copy(&lfilter)).into()
        }
        unsafe extern "system" fn EventFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventFilter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfilter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreferredMediaTypes(::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&fpersistent)).into()
        }
        unsafe extern "system" fn PreferredMediaTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreferredMediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSession(::core::mem::transmute_copy(&entype), ::core::mem::transmute(&bstrlocalphoneuri), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListenForIncomingSessions(::core::mem::transmute_copy(&enlisten)).into()
        }
        unsafe extern "system" fn ListenForIncomingSessions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListenForIncomingSessions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penlisten, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NetworkAddresses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL, pvaddresses: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_NetworkAddresses(::core::mem::transmute_copy(&ftcp), ::core::mem::transmute_copy(&fexternal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaddresses, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Volume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_Volume(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn get_Volume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Volume(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_AudioMuted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_AudioMuted(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&fmuted)).into()
        }
        unsafe extern "system" fn get_AudioMuted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_AudioMuted(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmuted, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_IVideoWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_IVideoWindow(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppivideowindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_PreferredAudioDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_PreferredAudioDevice(::core::mem::transmute_copy(&endevice), ::core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn get_PreferredAudioDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PreferredAudioDevice(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_PreferredVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_PreferredVolume(::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn get_PreferredVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PreferredVolume(::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAEC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreferredAEC(::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn PreferredAEC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreferredAEC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreferredVideoDevice(::core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn PreferredVideoDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreferredVideoDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActiveMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBitrate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxBitrate(::core::mem::transmute_copy(&lmaxbitrate)).into()
        }
        unsafe extern "system" fn MaxBitrate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxbitrate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTemporalSpatialTradeOff(::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TemporalSpatialTradeOff() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkQuality() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plnetworkquality, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartT120Applet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartT120Applet(::core::mem::transmute_copy(&enapplet)).into()
        }
        unsafe extern "system" fn StopT120Applets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopT120Applets().into()
        }
        unsafe extern "system" fn get_IsT120AppletRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_IsT120AppletRunning(::core::mem::transmute_copy(&enapplet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrunning, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalUserURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalUserURI(::core::mem::transmute(&bstruseruri)).into()
        }
        unsafe extern "system" fn LocalUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LocalUserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalUserName(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn PlayRing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PlayRing(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bplay)).into()
        }
        unsafe extern "system" fn SendDTMF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendDTMF(::core::mem::transmute_copy(&endtmf)).into()
        }
        unsafe extern "system" fn InvokeTuningWizard<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeTuningWizard(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn IsTuned<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftuned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTuned() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftuned, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            PrepareForShutdown: PrepareForShutdown::<Identity, Impl, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, Impl, OFFSET>,
            EventFilter: EventFilter::<Identity, Impl, OFFSET>,
            SetPreferredMediaTypes: SetPreferredMediaTypes::<Identity, Impl, OFFSET>,
            PreferredMediaTypes: PreferredMediaTypes::<Identity, Impl, OFFSET>,
            MediaCapabilities: MediaCapabilities::<Identity, Impl, OFFSET>,
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            SetListenForIncomingSessions: SetListenForIncomingSessions::<Identity, Impl, OFFSET>,
            ListenForIncomingSessions: ListenForIncomingSessions::<Identity, Impl, OFFSET>,
            get_NetworkAddresses: get_NetworkAddresses::<Identity, Impl, OFFSET>,
            put_Volume: put_Volume::<Identity, Impl, OFFSET>,
            get_Volume: get_Volume::<Identity, Impl, OFFSET>,
            put_AudioMuted: put_AudioMuted::<Identity, Impl, OFFSET>,
            get_AudioMuted: get_AudioMuted::<Identity, Impl, OFFSET>,
            get_IVideoWindow: get_IVideoWindow::<Identity, Impl, OFFSET>,
            put_PreferredAudioDevice: put_PreferredAudioDevice::<Identity, Impl, OFFSET>,
            get_PreferredAudioDevice: get_PreferredAudioDevice::<Identity, Impl, OFFSET>,
            put_PreferredVolume: put_PreferredVolume::<Identity, Impl, OFFSET>,
            get_PreferredVolume: get_PreferredVolume::<Identity, Impl, OFFSET>,
            SetPreferredAEC: SetPreferredAEC::<Identity, Impl, OFFSET>,
            PreferredAEC: PreferredAEC::<Identity, Impl, OFFSET>,
            SetPreferredVideoDevice: SetPreferredVideoDevice::<Identity, Impl, OFFSET>,
            PreferredVideoDevice: PreferredVideoDevice::<Identity, Impl, OFFSET>,
            ActiveMedia: ActiveMedia::<Identity, Impl, OFFSET>,
            SetMaxBitrate: SetMaxBitrate::<Identity, Impl, OFFSET>,
            MaxBitrate: MaxBitrate::<Identity, Impl, OFFSET>,
            SetTemporalSpatialTradeOff: SetTemporalSpatialTradeOff::<Identity, Impl, OFFSET>,
            TemporalSpatialTradeOff: TemporalSpatialTradeOff::<Identity, Impl, OFFSET>,
            NetworkQuality: NetworkQuality::<Identity, Impl, OFFSET>,
            StartT120Applet: StartT120Applet::<Identity, Impl, OFFSET>,
            StopT120Applets: StopT120Applets::<Identity, Impl, OFFSET>,
            get_IsT120AppletRunning: get_IsT120AppletRunning::<Identity, Impl, OFFSET>,
            LocalUserURI: LocalUserURI::<Identity, Impl, OFFSET>,
            SetLocalUserURI: SetLocalUserURI::<Identity, Impl, OFFSET>,
            LocalUserName: LocalUserName::<Identity, Impl, OFFSET>,
            SetLocalUserName: SetLocalUserName::<Identity, Impl, OFFSET>,
            PlayRing: PlayRing::<Identity, Impl, OFFSET>,
            SendDTMF: SendDTMF::<Identity, Impl, OFFSET>,
            InvokeTuningWizard: InvokeTuningWizard::<Identity, Impl, OFFSET>,
            IsTuned: IsTuned::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClient as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClient2_Impl: Sized + IRTCClient_Impl {
    fn put_AnswerMode(&self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::Result<()>;
    fn get_AnswerMode(&self, entype: RTC_SESSION_TYPE) -> ::windows::core::Result<RTC_ANSWER_MODE>;
    fn InvokeTuningWizardEx(&self, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<i32>;
    fn SetClientName(&self, bstrclientname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetClientCurVer(&self, bstrclientcurver: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn InitializeEx(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn CreateSessionWithDescription(&self, bstrcontenttype: &::windows::core::BSTR, bstrsessiondescription: &::windows::core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCSession2>;
    fn SetSessionDescriptionManager(&self, psessiondescriptionmanager: ::core::option::Option<&IRTCSessionDescriptionManager>) -> ::windows::core::Result<()>;
    fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()>;
    fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn put_AllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::Result<()>;
    fn get_AllowedPorts(&self, ltransport: i32) -> ::windows::core::Result<RTC_LISTEN_MODE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCClient2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClient2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>() -> IRTCClient2_Vtbl {
        unsafe extern "system" fn put_AnswerMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_AnswerMode(::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn get_AnswerMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_AnswerMode(::core::mem::transmute_copy(&entype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvokeTuningWizardEx(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&fallowaudio), ::core::mem::transmute_copy(&fallowvideo)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientName(::core::mem::transmute(&bstrclientname)).into()
        }
        unsafe extern "system" fn SetClientCurVer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclientcurver: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCurVer(::core::mem::transmute(&bstrclientcurver)).into()
        }
        unsafe extern "system" fn InitializeEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeEx(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateSessionWithDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSessionWithDescription(::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSessionDescriptionManager(::windows::core::from_raw_borrowed(&psessiondescriptionmanager)).into()
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_AllowedPorts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_AllowedPorts(::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&enlistenmode)).into()
        }
        unsafe extern "system" fn get_AllowedPorts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_AllowedPorts(::core::mem::transmute_copy(&ltransport)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penlistenmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRTCClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            put_AnswerMode: put_AnswerMode::<Identity, Impl, OFFSET>,
            get_AnswerMode: get_AnswerMode::<Identity, Impl, OFFSET>,
            InvokeTuningWizardEx: InvokeTuningWizardEx::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            SetClientCurVer: SetClientCurVer::<Identity, Impl, OFFSET>,
            InitializeEx: InitializeEx::<Identity, Impl, OFFSET>,
            CreateSessionWithDescription: CreateSessionWithDescription::<Identity, Impl, OFFSET>,
            SetSessionDescriptionManager: SetSessionDescriptionManager::<Identity, Impl, OFFSET>,
            put_PreferredSecurityLevel: put_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            get_PreferredSecurityLevel: get_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            put_AllowedPorts: put_AllowedPorts::<Identity, Impl, OFFSET>,
            get_AllowedPorts: get_AllowedPorts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClient2 as ::windows::core::ComInterface>::IID || iid == &<IRTCClient as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClientEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&self) -> ::windows::core::Result<RTC_CLIENT_EVENT_TYPE>;
    fn Client(&self) -> ::windows::core::Result<IRTCClient>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCClientEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientEvent_Impl, const OFFSET: isize>() -> IRTCClientEvent_Vtbl {
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Client<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Client() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclient, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventType: EventType::<Identity, Impl, OFFSET>,
            Client: Client::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCClientPortManagement_Impl: Sized {
    fn StartListenAddressAndPort(&self, bstrinternallocaladdress: &::windows::core::BSTR, linternallocalport: i32) -> ::windows::core::Result<()>;
    fn StopListenAddressAndPort(&self, bstrinternallocaladdress: &::windows::core::BSTR, linternallocalport: i32) -> ::windows::core::Result<()>;
    fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRTCClientPortManagement {}
impl IRTCClientPortManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: isize>() -> IRTCClientPortManagement_Vtbl {
        unsafe extern "system" fn StartListenAddressAndPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartListenAddressAndPort(::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn StopListenAddressAndPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopListenAddressAndPort(::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn GetPortRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPortRange(::core::mem::transmute_copy(&enporttype), ::core::mem::transmute_copy(&plminvalue), ::core::mem::transmute_copy(&plmaxvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListenAddressAndPort: StartListenAddressAndPort::<Identity, Impl, OFFSET>,
            StopListenAddressAndPort: StopListenAddressAndPort::<Identity, Impl, OFFSET>,
            GetPortRange: GetPortRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientPortManagement as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClientPresence_Impl: Sized {
    fn EnablePresence(&self, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Export(&self, varstorage: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Import(&self, varstorage: &super::Com::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies>;
    fn Buddies(&self) -> ::windows::core::Result<IRTCCollection>;
    fn get_Buddy(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<IRTCBuddy>;
    fn AddBuddy(&self, bstrpresentityuri: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddy>;
    fn RemoveBuddy(&self, pbuddy: ::core::option::Option<&IRTCBuddy>) -> ::windows::core::Result<()>;
    fn EnumerateWatchers(&self) -> ::windows::core::Result<IRTCEnumWatchers>;
    fn Watchers(&self) -> ::windows::core::Result<IRTCCollection>;
    fn get_Watcher(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<IRTCWatcher>;
    fn AddWatcher(&self, bstrpresentityuri: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<IRTCWatcher>;
    fn RemoveWatcher(&self, pwatcher: ::core::option::Option<&IRTCWatcher>) -> ::windows::core::Result<()>;
    fn SetLocalPresenceInfo(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OfferWatcherMode(&self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE>;
    fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()>;
    fn PrivacyMode(&self) -> ::windows::core::Result<RTC_PRIVACY_MODE>;
    fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCClientPresence {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>() -> IRTCClientPresence_Vtbl {
        unsafe extern "system" fn EnablePresence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnablePresence(::core::mem::transmute_copy(&fusestorage), ::core::mem::transmute(&varstorage)).into()
        }
        unsafe extern "system" fn Export<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Export(::core::mem::transmute(&varstorage)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstorage: super::Com::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Import(::core::mem::transmute(&varstorage), ::core::mem::transmute_copy(&freplaceall)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateBuddies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Buddies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Buddy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Buddy(::core::mem::transmute(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBuddy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddBuddy(::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&fpersistent), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBuddy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveBuddy(::windows::core::from_raw_borrowed(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateWatchers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateWatchers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watchers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Watchers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Watcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Watcher(::core::mem::transmute(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddWatcher(::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&fblocked), ::core::mem::transmute_copy(&fpersistent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwatcher: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveWatcher(::windows::core::from_raw_borrowed(&pwatcher)).into()
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalPresenceInfo(::core::mem::transmute_copy(&enstatus), ::core::mem::transmute(&bstrnotes)).into()
        }
        unsafe extern "system" fn OfferWatcherMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OfferWatcherMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfferWatcherMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOfferWatcherMode(::core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn PrivacyMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivacyMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivacyMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivacyMode(::core::mem::transmute_copy(&enmode)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnablePresence: EnablePresence::<Identity, Impl, OFFSET>,
            Export: Export::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Identity, Impl, OFFSET>,
            Buddies: Buddies::<Identity, Impl, OFFSET>,
            get_Buddy: get_Buddy::<Identity, Impl, OFFSET>,
            AddBuddy: AddBuddy::<Identity, Impl, OFFSET>,
            RemoveBuddy: RemoveBuddy::<Identity, Impl, OFFSET>,
            EnumerateWatchers: EnumerateWatchers::<Identity, Impl, OFFSET>,
            Watchers: Watchers::<Identity, Impl, OFFSET>,
            get_Watcher: get_Watcher::<Identity, Impl, OFFSET>,
            AddWatcher: AddWatcher::<Identity, Impl, OFFSET>,
            RemoveWatcher: RemoveWatcher::<Identity, Impl, OFFSET>,
            SetLocalPresenceInfo: SetLocalPresenceInfo::<Identity, Impl, OFFSET>,
            OfferWatcherMode: OfferWatcherMode::<Identity, Impl, OFFSET>,
            SetOfferWatcherMode: SetOfferWatcherMode::<Identity, Impl, OFFSET>,
            PrivacyMode: PrivacyMode::<Identity, Impl, OFFSET>,
            SetPrivacyMode: SetPrivacyMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientPresence as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCClientPresence2_Impl: Sized + IRTCClientPresence_Impl {
    fn EnablePresenceEx(&self, pprofile: ::core::option::Option<&IRTCProfile>, varstorage: &super::Com::VARIANT, lflags: i32) -> ::windows::core::Result<()>;
    fn DisablePresence(&self) -> ::windows::core::Result<()>;
    fn AddGroup(&self, bstrgroupname: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn RemoveGroup(&self, pgroup: ::core::option::Option<&IRTCBuddyGroup>) -> ::windows::core::Result<()>;
    fn EnumerateGroups(&self) -> ::windows::core::Result<IRTCEnumGroups>;
    fn Groups(&self) -> ::windows::core::Result<IRTCCollection>;
    fn get_Group(&self, bstrgroupname: &::windows::core::BSTR) -> ::windows::core::Result<IRTCBuddyGroup>;
    fn AddWatcherEx(&self, bstrpresentityuri: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCWatcher2>;
    fn get_WatcherEx(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<IRTCWatcher2>;
    fn put_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPresenceData(&self, bstrnamespace: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetPresenceData(&self, pbstrnamespace: *mut ::windows::core::BSTR, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AddBuddyEx(&self, bstrpresentityuri: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrdata: &::windows::core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows::core::Result<IRTCBuddy2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCClientPresence2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCClientPresence2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>() -> IRTCClientPresence2_Vtbl {
        unsafe extern "system" fn EnablePresenceEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, varstorage: super::Com::VARIANT, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnablePresenceEx(::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute(&varstorage), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DisablePresence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisablePresence().into()
        }
        unsafe extern "system" fn AddGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&bstrdata), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveGroup(::windows::core::from_raw_borrowed(&pgroup)).into()
        }
        unsafe extern "system" fn EnumerateGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Groups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Group<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Group(::core::mem::transmute(&bstrgroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcherEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddWatcherEx(::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&enstate), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute_copy(&enscope), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_WatcherEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_WatcherEx(::core::mem::transmute_copy(&enmode), ::core::mem::transmute(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_PresenceProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_PresenceProperty(::core::mem::transmute_copy(&enproperty), ::core::mem::transmute(&bstrproperty)).into()
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamespace: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPresenceData(::core::mem::transmute(&bstrnamespace), ::core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn GetPresenceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into()
        }
        unsafe extern "system" fn AddBuddyEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddBuddyEx(::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute_copy(&ensubscriptiontype), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRTCClientPresence_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnablePresenceEx: EnablePresenceEx::<Identity, Impl, OFFSET>,
            DisablePresence: DisablePresence::<Identity, Impl, OFFSET>,
            AddGroup: AddGroup::<Identity, Impl, OFFSET>,
            RemoveGroup: RemoveGroup::<Identity, Impl, OFFSET>,
            EnumerateGroups: EnumerateGroups::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            get_Group: get_Group::<Identity, Impl, OFFSET>,
            AddWatcherEx: AddWatcherEx::<Identity, Impl, OFFSET>,
            get_WatcherEx: get_WatcherEx::<Identity, Impl, OFFSET>,
            put_PresenceProperty: put_PresenceProperty::<Identity, Impl, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, Impl, OFFSET>,
            SetPresenceData: SetPresenceData::<Identity, Impl, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, Impl, OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Identity, Impl, OFFSET>,
            AddBuddyEx: AddBuddyEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientPresence2 as ::windows::core::ComInterface>::IID || iid == &<IRTCClientPresence as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientProvisioning_Impl: Sized {
    fn CreateProfile(&self, bstrprofilexml: &::windows::core::BSTR) -> ::windows::core::Result<IRTCProfile>;
    fn EnableProfile(&self, pprofile: ::core::option::Option<&IRTCProfile>, lregisterflags: i32) -> ::windows::core::Result<()>;
    fn DisableProfile(&self, pprofile: ::core::option::Option<&IRTCProfile>) -> ::windows::core::Result<()>;
    fn EnumerateProfiles(&self) -> ::windows::core::Result<IRTCEnumProfiles>;
    fn Profiles(&self) -> ::windows::core::Result<IRTCCollection>;
    fn GetProfile(&self, bstruseraccount: &::windows::core::BSTR, bstruserpassword: &::windows::core::BSTR, bstruseruri: &::windows::core::BSTR, bstrserver: &::windows::core::BSTR, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn SessionCapabilities(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCClientProvisioning {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientProvisioning_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>() -> IRTCClientProvisioning_Vtbl {
        unsafe extern "system" fn CreateProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofilexml: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateProfile(::core::mem::transmute(&bstrprofilexml)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableProfile(::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lregisterflags)).into()
        }
        unsafe extern "system" fn DisableProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableProfile(::windows::core::from_raw_borrowed(&pprofile)).into()
        }
        unsafe extern "system" fn EnumerateProfiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseraccount: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstruserpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstruseruri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrserver: ::std::mem::MaybeUninit<::windows::core::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProfile(::core::mem::transmute(&bstruseraccount), ::core::mem::transmute(&bstruserpassword), ::core::mem::transmute(&bstruseruri), ::core::mem::transmute(&bstrserver), ::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsupportedsessions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateProfile: CreateProfile::<Identity, Impl, OFFSET>,
            EnableProfile: EnableProfile::<Identity, Impl, OFFSET>,
            DisableProfile: DisableProfile::<Identity, Impl, OFFSET>,
            EnumerateProfiles: EnumerateProfiles::<Identity, Impl, OFFSET>,
            Profiles: Profiles::<Identity, Impl, OFFSET>,
            GetProfile: GetProfile::<Identity, Impl, OFFSET>,
            SessionCapabilities: SessionCapabilities::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientProvisioning as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientProvisioning2_Impl: Sized + IRTCClientProvisioning_Impl {
    fn EnableProfileEx(&self, pprofile: ::core::option::Option<&IRTCProfile>, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCClientProvisioning2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientProvisioning2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning2_Impl, const OFFSET: isize>() -> IRTCClientProvisioning2_Vtbl {
        unsafe extern "system" fn EnableProfileEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCClientProvisioning2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableProfileEx(::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lregisterflags), ::core::mem::transmute_copy(&lroamingflags)).into()
        }
        Self { base__: IRTCClientProvisioning_Vtbl::new::<Identity, Impl, OFFSET>(), EnableProfileEx: EnableProfileEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCClientProvisioning2 as ::windows::core::ComInterface>::IID || iid == &<IRTCClientProvisioning as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: isize>() -> IRTCCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCCollection as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCDispatchEventNotification_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCDispatchEventNotification {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCDispatchEventNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCDispatchEventNotification_Impl, const OFFSET: isize>() -> IRTCDispatchEventNotification_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCDispatchEventNotification as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCEnumBuddies_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddy>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IRTCEnumBuddies>;
}
impl ::windows::core::RuntimeName for IRTCEnumBuddies {}
impl IRTCEnumBuddies_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>() -> IRTCEnumBuddies_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumBuddies as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCEnumGroups_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddyGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IRTCEnumGroups>;
}
impl ::windows::core::RuntimeName for IRTCEnumGroups {}
impl IRTCEnumGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: isize>() -> IRTCEnumGroups_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumGroups as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCEnumParticipants_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCParticipant>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IRTCEnumParticipants>;
}
impl ::windows::core::RuntimeName for IRTCEnumParticipants {}
impl IRTCEnumParticipants_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>() -> IRTCEnumParticipants_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumParticipants as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCEnumPresenceDevices_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCPresenceDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IRTCEnumPresenceDevices>;
}
impl ::windows::core::RuntimeName for IRTCEnumPresenceDevices {}
impl IRTCEnumPresenceDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>() -> IRTCEnumPresenceDevices_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumPresenceDevices as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCEnumProfiles_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCProfile>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IRTCEnumProfiles>;
}
impl ::windows::core::RuntimeName for IRTCEnumProfiles {}
impl IRTCEnumProfiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>() -> IRTCEnumProfiles_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumProfiles as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCEnumUserSearchResults_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCUserSearchResult>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IRTCEnumUserSearchResults>;
}
impl ::windows::core::RuntimeName for IRTCEnumUserSearchResults {}
impl IRTCEnumUserSearchResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>() -> IRTCEnumUserSearchResults_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumUserSearchResults as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCEnumWatchers_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCWatcher>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IRTCEnumWatchers>;
}
impl ::windows::core::RuntimeName for IRTCEnumWatchers {}
impl IRTCEnumWatchers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>() -> IRTCEnumWatchers_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEnumWatchers as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCEventNotification_Impl: Sized {
    fn Event(&self, rtcevent: RTC_EVENT, pevent: ::core::option::Option<&super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRTCEventNotification {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCEventNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEventNotification_Impl, const OFFSET: isize>() -> IRTCEventNotification_Vtbl {
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCEventNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Event(::core::mem::transmute_copy(&rtcevent), ::windows::core::from_raw_borrowed(&pevent)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Event: Event::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCEventNotification as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCInfoEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession2>;
    fn Participant(&self) -> ::windows::core::Result<IRTCParticipant>;
    fn Info(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn InfoHeader(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCInfoEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCInfoEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: isize>() -> IRTCInfoEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Participant() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfo: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Info() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InfoHeader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinfoheader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Participant: Participant::<Identity, Impl, OFFSET>,
            Info: Info::<Identity, Impl, OFFSET>,
            InfoHeader: InfoHeader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCInfoEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCIntensityEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Level(&self) -> ::windows::core::Result<i32>;
    fn Min(&self) -> ::windows::core::Result<i32>;
    fn Max(&self) -> ::windows::core::Result<i32>;
    fn Direction(&self) -> ::windows::core::Result<RTC_AUDIO_DEVICE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCIntensityEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCIntensityEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>() -> IRTCIntensityEvent_Vtbl {
        unsafe extern "system" fn Level<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Level() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Min() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Max() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Direction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pendirection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Level: Level::<Identity, Impl, OFFSET>,
            Min: Min::<Identity, Impl, OFFSET>,
            Max: Max::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCIntensityEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCMediaEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn MediaType(&self) -> ::windows::core::Result<i32>;
    fn EventType(&self) -> ::windows::core::Result<RTC_MEDIA_EVENT_TYPE>;
    fn EventReason(&self) -> ::windows::core::Result<RTC_MEDIA_EVENT_REASON>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCMediaEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: isize>() -> IRTCMediaEvent_Vtbl {
        unsafe extern "system" fn MediaType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmediatype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventReason<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventReason() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventreason, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            EventType: EventType::<Identity, Impl, OFFSET>,
            EventReason: EventReason::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCMediaEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCMediaRequestEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession2>;
    fn ProposedMedia(&self) -> ::windows::core::Result<i32>;
    fn CurrentMedia(&self) -> ::windows::core::Result<i32>;
    fn Accept(&self, lmediatypes: i32) -> ::windows::core::Result<()>;
    fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn Reject(&self) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<RTC_REINVITE_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCMediaRequestEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMediaRequestEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>() -> IRTCMediaRequestEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProposedMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProposedMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Accept(::core::mem::transmute_copy(&lmediatypes)).into()
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_RemotePreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reject().into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            ProposedMedia: ProposedMedia::<Identity, Impl, OFFSET>,
            CurrentMedia: CurrentMedia::<Identity, Impl, OFFSET>,
            Accept: Accept::<Identity, Impl, OFFSET>,
            get_RemotePreferredSecurityLevel: get_RemotePreferredSecurityLevel::<Identity, Impl, OFFSET>,
            Reject: Reject::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCMediaRequestEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCMessagingEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession>;
    fn Participant(&self) -> ::windows::core::Result<IRTCParticipant>;
    fn EventType(&self) -> ::windows::core::Result<RTC_MESSAGING_EVENT_TYPE>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MessageHeader(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UserStatus(&self) -> ::windows::core::Result<RTC_MESSAGING_USER_STATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCMessagingEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCMessagingEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>() -> IRTCMessagingEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Participant() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Message() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageHeader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmessageheader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penuserstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Participant: Participant::<Identity, Impl, OFFSET>,
            EventType: EventType::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
            MessageHeader: MessageHeader::<Identity, Impl, OFFSET>,
            UserStatus: UserStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCMessagingEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCParticipant_Impl: Sized {
    fn UserURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Removable(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn State(&self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE>;
    fn Session(&self) -> ::windows::core::Result<IRTCSession>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCParticipant {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCParticipant_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: isize>() -> IRTCParticipant_Vtbl {
        unsafe extern "system" fn UserURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Removable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfremovable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Removable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfremovable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UserURI: UserURI::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Removable: Removable::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Session: Session::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCParticipant as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCParticipantStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Participant(&self) -> ::windows::core::Result<IRTCParticipant>;
    fn State(&self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCParticipantStateChangeEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCParticipantStateChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>() -> IRTCParticipantStateChangeEvent_Vtbl {
        unsafe extern "system" fn Participant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Participant() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Participant: Participant::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCParticipantStateChangeEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCPortManager_Impl: Sized {
    fn GetMapping(&self, bstrremoteaddress: &::windows::core::BSTR, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::windows::core::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::windows::core::BSTR, plexternallocalport: *mut i32) -> ::windows::core::Result<()>;
    fn UpdateRemoteAddress(&self, bstrremoteaddress: &::windows::core::BSTR, bstrinternallocaladdress: &::windows::core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &::windows::core::BSTR, lexternallocalport: i32) -> ::windows::core::Result<()>;
    fn ReleaseMapping(&self, bstrinternallocaladdress: &::windows::core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &::windows::core::BSTR, lexternallocaladdress: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRTCPortManager {}
impl IRTCPortManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: isize>() -> IRTCPortManager_Vtbl {
        unsafe extern "system" fn GetMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, plexternallocalport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMapping(::core::mem::transmute(&bstrremoteaddress), ::core::mem::transmute_copy(&enporttype), ::core::mem::transmute_copy(&pbstrinternallocaladdress), ::core::mem::transmute_copy(&plinternallocalport), ::core::mem::transmute_copy(&pbstrexternallocaladdress), ::core::mem::transmute_copy(&plexternallocalport)).into()
        }
        unsafe extern "system" fn UpdateRemoteAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, lexternallocalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateRemoteAddress(::core::mem::transmute(&bstrremoteaddress), ::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport), ::core::mem::transmute(&bstrexternallocaladdress), ::core::mem::transmute_copy(&lexternallocalport)).into()
        }
        unsafe extern "system" fn ReleaseMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::std::mem::MaybeUninit<::windows::core::BSTR>, lexternallocaladdress: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMapping(::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport), ::core::mem::transmute(&bstrexternallocaladdress), ::core::mem::transmute_copy(&lexternallocaladdress)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMapping: GetMapping::<Identity, Impl, OFFSET>,
            UpdateRemoteAddress: UpdateRemoteAddress::<Identity, Impl, OFFSET>,
            ReleaseMapping: ReleaseMapping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPortManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCPresenceContact_Impl: Sized {
    fn PresentityURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPresentityURI(&self, bstrpresentityuri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetData(&self, bstrdata: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Persistent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPersistent(&self, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCPresenceContact {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCPresenceContact_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>() -> IRTCPresenceContact_Vtbl {
        unsafe extern "system" fn PresentityURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PresentityURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpresentityuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentityURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPresentityURI(::core::mem::transmute(&bstrpresentityuri)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Persistent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpersistent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Persistent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfpersistent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPersistent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPersistent(::core::mem::transmute_copy(&fpersistent)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PresentityURI: PresentityURI::<Identity, Impl, OFFSET>,
            SetPresentityURI: SetPresentityURI::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Persistent: Persistent::<Identity, Impl, OFFSET>,
            SetPersistent: SetPersistent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceContact as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCPresenceDataEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetPresenceData(&self, pbstrnamespace: *mut ::windows::core::BSTR, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCPresenceDataEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceDataEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: isize>() -> IRTCPresenceDataEvent_Vtbl {
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceDataEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCPresenceDevice_Impl: Sized {
    fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetPresenceData(&self, pbstrnamespace: *mut ::windows::core::BSTR, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRTCPresenceDevice {}
impl IRTCPresenceDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>() -> IRTCPresenceDevice_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Notes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnotes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PresenceProperty(::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresenceData(::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Notes: Notes::<Identity, Impl, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, Impl, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCPresencePropertyEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn PresenceProperty(&self) -> ::windows::core::Result<RTC_PRESENCE_PROPERTY>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCPresencePropertyEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresencePropertyEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>() -> IRTCPresencePropertyEvent_Vtbl {
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PresenceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penpresprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            PresenceProperty: PresenceProperty::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresencePropertyEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCPresenceStatusEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCPresenceStatusEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCPresenceStatusEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>() -> IRTCPresenceStatusEvent_Vtbl {
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocalPresenceInfo(::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCPresenceStatusEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCProfile_Impl: Sized {
    fn Key(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn XML(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ProviderData(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ClientName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ClientBanner(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ClientMinVer(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ClientCurVer(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ClientUpdateURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ClientData(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UserURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCredentials(&self, bstruseruri: &::windows::core::BSTR, bstruseraccount: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SessionCapabilities(&self) -> ::windows::core::Result<i32>;
    fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCProfile {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>() -> IRTCProfile_Vtbl {
        unsafe extern "system" fn Key<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrkey: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Key() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrkey, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.XML() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxml, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ProviderURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ProviderURI(::core::mem::transmute_copy(&enuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProviderData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientBanner<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbanner: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientBanner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfbanner, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientMinVer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrminver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientMinVer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrminver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCurVer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcurver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientCurVer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcurver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientUpdateURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientUpdateURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrupdateuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseraccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruseruri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstruseraccount: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentials(::core::mem::transmute(&bstruseruri), ::core::mem::transmute(&bstruseraccount), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsupportedsessions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Key: Key::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            XML: XML::<Identity, Impl, OFFSET>,
            ProviderName: ProviderName::<Identity, Impl, OFFSET>,
            get_ProviderURI: get_ProviderURI::<Identity, Impl, OFFSET>,
            ProviderData: ProviderData::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            ClientBanner: ClientBanner::<Identity, Impl, OFFSET>,
            ClientMinVer: ClientMinVer::<Identity, Impl, OFFSET>,
            ClientCurVer: ClientCurVer::<Identity, Impl, OFFSET>,
            ClientUpdateURI: ClientUpdateURI::<Identity, Impl, OFFSET>,
            ClientData: ClientData::<Identity, Impl, OFFSET>,
            UserURI: UserURI::<Identity, Impl, OFFSET>,
            UserName: UserName::<Identity, Impl, OFFSET>,
            UserAccount: UserAccount::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            SessionCapabilities: SessionCapabilities::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCProfile2_Impl: Sized + IRTCProfile_Impl {
    fn Realm(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetRealm(&self, bstrrealm: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AllowedAuth(&self) -> ::windows::core::Result<i32>;
    fn SetAllowedAuth(&self, lallowedauth: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCProfile2 {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCProfile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: isize>() -> IRTCProfile2_Vtbl {
        unsafe extern "system" fn Realm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrealm: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Realm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrealm, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRealm(::core::mem::transmute(&bstrrealm)).into()
        }
        unsafe extern "system" fn AllowedAuth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowedAuth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plallowedauth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedAuth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowedAuth(::core::mem::transmute_copy(&lallowedauth)).into()
        }
        Self {
            base__: IRTCProfile_Vtbl::new::<Identity, Impl, OFFSET>(),
            Realm: Realm::<Identity, Impl, OFFSET>,
            SetRealm: SetRealm::<Identity, Impl, OFFSET>,
            AllowedAuth: AllowedAuth::<Identity, Impl, OFFSET>,
            SetAllowedAuth: SetAllowedAuth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfile2 as ::windows::core::ComInterface>::IID || iid == &<IRTCProfile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCProfileEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile>;
    fn Cookie(&self) -> ::windows::core::Result<isize>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCProfileEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: isize>() -> IRTCProfileEvent_Vtbl {
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Profile: Profile::<Identity, Impl, OFFSET>,
            Cookie: Cookie::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfileEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCProfileEvent2_Impl: Sized + IRTCProfileEvent_Impl {
    fn EventType(&self) -> ::windows::core::Result<RTC_PROFILE_EVENT_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCProfileEvent2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCProfileEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfileEvent2_Impl, const OFFSET: isize>() -> IRTCProfileEvent2_Vtbl {
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCProfileEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IRTCProfileEvent_Vtbl::new::<Identity, Impl, OFFSET>(), EventType: EventType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCProfileEvent2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IRTCProfileEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCReInviteEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession2>;
    fn Accept(&self, bstrcontenttype: &::windows::core::BSTR, bstrsessiondescription: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Reject(&self) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<RTC_REINVITE_STATE>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::windows::core::BSTR, pbstrsessiondescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCReInviteEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCReInviteEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>() -> IRTCReInviteEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Accept(::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn Reject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reject().into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Accept: Accept::<Identity, Impl, OFFSET>,
            Reject: Reject::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCReInviteEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCRegistrationStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile>;
    fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCRegistrationStateChangeEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRegistrationStateChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>() -> IRTCRegistrationStateChangeEvent_Vtbl {
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Profile: Profile::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCRegistrationStateChangeEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCRoamingEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&self) -> ::windows::core::Result<RTC_ROAMING_EVENT_TYPE>;
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile2>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCRoamingEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCRoamingEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>() -> IRTCRoamingEvent_Vtbl {
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventType: EventType::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCRoamingEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCSession_Impl: Sized {
    fn Client(&self) -> ::windows::core::Result<IRTCClient>;
    fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE>;
    fn Type(&self) -> ::windows::core::Result<RTC_SESSION_TYPE>;
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile>;
    fn Participants(&self) -> ::windows::core::Result<IRTCCollection>;
    fn Answer(&self) -> ::windows::core::Result<()>;
    fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()>;
    fn Redirect(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &::windows::core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows::core::Result<()>;
    fn AddParticipant(&self, bstraddress: &::windows::core::BSTR, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IRTCParticipant>;
    fn RemoveParticipant(&self, pparticipant: ::core::option::Option<&IRTCParticipant>) -> ::windows::core::Result<()>;
    fn EnumerateParticipants(&self) -> ::windows::core::Result<IRTCEnumParticipants>;
    fn CanAddParticipants(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RedirectedUserURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn RedirectedUserName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn NextRedirectedUser(&self) -> ::windows::core::Result<()>;
    fn SendMessage(&self, bstrmessageheader: &::windows::core::BSTR, bstrmessage: &::windows::core::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
    fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()>;
    fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()>;
    fn put_EncryptionKey(&self, lmediatype: i32, encryptionkey: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IRTCSession {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>() -> IRTCSession_Vtbl {
        unsafe extern "system" fn Client<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Client() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclient, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pentype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Participants() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Answer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Answer().into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate(::core::mem::transmute_copy(&enreason)).into()
        }
        unsafe extern "system" fn Redirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Redirect(::core::mem::transmute_copy(&entype), ::core::mem::transmute(&bstrlocalphoneuri), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddParticipant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddParticipant(::core::mem::transmute(&bstraddress), ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveParticipant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparticipant: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveParticipant(::windows::core::from_raw_borrowed(&pparticipant)).into()
        }
        unsafe extern "system" fn EnumerateParticipants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateParticipants() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanAddParticipants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcanadd: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanAddParticipants() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanadd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RedirectedUserURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RedirectedUserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRedirectedUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NextRedirectedUser().into()
        }
        unsafe extern "system" fn SendMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrmessage: ::std::mem::MaybeUninit<::windows::core::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMessage(::core::mem::transmute(&bstrmessageheader), ::core::mem::transmute(&bstrmessage), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SendMessageStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMessageStatus(::core::mem::transmute_copy(&enuserstatus), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn AddStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn put_EncryptionKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_EncryptionKey(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute(&encryptionkey)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Client: Client::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            Participants: Participants::<Identity, Impl, OFFSET>,
            Answer: Answer::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            Redirect: Redirect::<Identity, Impl, OFFSET>,
            AddParticipant: AddParticipant::<Identity, Impl, OFFSET>,
            RemoveParticipant: RemoveParticipant::<Identity, Impl, OFFSET>,
            EnumerateParticipants: EnumerateParticipants::<Identity, Impl, OFFSET>,
            CanAddParticipants: CanAddParticipants::<Identity, Impl, OFFSET>,
            RedirectedUserURI: RedirectedUserURI::<Identity, Impl, OFFSET>,
            RedirectedUserName: RedirectedUserName::<Identity, Impl, OFFSET>,
            NextRedirectedUser: NextRedirectedUser::<Identity, Impl, OFFSET>,
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
            SendMessageStatus: SendMessageStatus::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            put_EncryptionKey: put_EncryptionKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSession as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCSession2_Impl: Sized + IRTCSession_Impl {
    fn SendInfo(&self, bstrinfoheader: &::windows::core::BSTR, bstrinfo: &::windows::core::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
    fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()>;
    fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerWithSessionDescription(&self, bstrcontenttype: &::windows::core::BSTR, bstrsessiondescription: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ReInviteWithSessionDescription(&self, bstrcontenttype: &::windows::core::BSTR, bstrsessiondescription: &::windows::core::BSTR, lcookie: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IRTCSession2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IRTCSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: isize>() -> IRTCSession2_Vtbl {
        unsafe extern "system" fn SendInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfoheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrinfo: ::std::mem::MaybeUninit<::windows::core::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendInfo(::core::mem::transmute(&bstrinfoheader), ::core::mem::transmute(&bstrinfo), ::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSecurityEnabled(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsecurityenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnswerWithSessionDescription(::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReInviteWithSessionDescription(::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription), ::core::mem::transmute_copy(&lcookie)).into()
        }
        Self {
            base__: IRTCSession_Vtbl::new::<Identity, Impl, OFFSET>(),
            SendInfo: SendInfo::<Identity, Impl, OFFSET>,
            put_PreferredSecurityLevel: put_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            get_PreferredSecurityLevel: get_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            AnswerWithSessionDescription: AnswerWithSessionDescription::<Identity, Impl, OFFSET>,
            ReInviteWithSessionDescription: ReInviteWithSessionDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSession2 as ::windows::core::ComInterface>::IID || iid == &<IRTCSession as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCSessionCallControl_Impl: Sized {
    fn Hold(&self, lcookie: isize) -> ::windows::core::Result<()>;
    fn UnHold(&self, lcookie: isize) -> ::windows::core::Result<()>;
    fn Forward(&self, bstrforwardtouri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Refer(&self, bstrrefertouri: &::windows::core::BSTR, bstrrefercookie: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetReferredByURI(&self, bstrreferredbyuri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ReferredByURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetReferCookie(&self, bstrrefercookie: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ReferCookie(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn IsReferred(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCSessionCallControl {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionCallControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>() -> IRTCSessionCallControl_Vtbl {
        unsafe extern "system" fn Hold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Hold(::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn UnHold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnHold(::core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn Forward<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Forward(::core::mem::transmute(&bstrforwardtouri)).into()
        }
        unsafe extern "system" fn Refer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefertouri: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrrefercookie: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refer(::core::mem::transmute(&bstrrefertouri), ::core::mem::transmute(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn SetReferredByURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReferredByURI(::core::mem::transmute(&bstrreferredbyuri)).into()
        }
        unsafe extern "system" fn ReferredByURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReferredByURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreferredbyuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrefercookie: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReferCookie(::core::mem::transmute(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn ReferCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReferCookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrefercookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReferred<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreferred: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsReferred() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisreferred, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Hold: Hold::<Identity, Impl, OFFSET>,
            UnHold: UnHold::<Identity, Impl, OFFSET>,
            Forward: Forward::<Identity, Impl, OFFSET>,
            Refer: Refer::<Identity, Impl, OFFSET>,
            SetReferredByURI: SetReferredByURI::<Identity, Impl, OFFSET>,
            ReferredByURI: ReferredByURI::<Identity, Impl, OFFSET>,
            SetReferCookie: SetReferCookie::<Identity, Impl, OFFSET>,
            ReferCookie: ReferCookie::<Identity, Impl, OFFSET>,
            IsReferred: IsReferred::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionCallControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCSessionDescriptionManager_Impl: Sized {
    fn EvaluateSessionDescription(&self, bstrcontenttype: &::windows::core::BSTR, bstrsessiondescription: &::windows::core::BSTR, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCSessionDescriptionManager {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCSessionDescriptionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionDescriptionManager_Impl, const OFFSET: isize>() -> IRTCSessionDescriptionManager_Vtbl {
        unsafe extern "system" fn EvaluateSessionDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionDescriptionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EvaluateSessionDescription(::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription), ::core::mem::transmute_copy(&pfapplicationsession)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EvaluateSessionDescription: EvaluateSessionDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionDescriptionManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionOperationCompleteEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession>;
    fn Cookie(&self) -> ::windows::core::Result<isize>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCSessionOperationCompleteEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>() -> IRTCSessionOperationCompleteEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Cookie: Cookie::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionOperationCompleteEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionOperationCompleteEvent2_Impl: Sized + IRTCSessionOperationCompleteEvent_Impl {
    fn Participant(&self) -> ::windows::core::Result<IRTCParticipant>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::windows::core::BSTR, pbstrsessiondescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCSessionOperationCompleteEvent2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionOperationCompleteEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>() -> IRTCSessionOperationCompleteEvent2_Vtbl {
        unsafe extern "system" fn Participant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Participant() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
        }
        Self {
            base__: IRTCSessionOperationCompleteEvent_Vtbl::new::<Identity, Impl, OFFSET>(),
            Participant: Participant::<Identity, Impl, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionOperationCompleteEvent2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IRTCSessionOperationCompleteEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCSessionPortManagement_Impl: Sized {
    fn SetPortManager(&self, pportmanager: ::core::option::Option<&IRTCPortManager>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRTCSessionPortManagement {}
impl IRTCSessionPortManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionPortManagement_Impl, const OFFSET: isize>() -> IRTCSessionPortManagement_Vtbl {
        unsafe extern "system" fn SetPortManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionPortManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPortManager(::windows::core::from_raw_borrowed(&pportmanager)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetPortManager: SetPortManager::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionPortManagement as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionReferStatusEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession2>;
    fn ReferStatus(&self) -> ::windows::core::Result<RTC_SESSION_REFER_STATUS>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCSessionReferStatusEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferStatusEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>() -> IRTCSessionReferStatusEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReferStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penreferstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            ReferStatus: ReferStatus::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionReferStatusEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionReferredEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession2>;
    fn ReferredByURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ReferToURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ReferCookie(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Accept(&self) -> ::windows::core::Result<()>;
    fn Reject(&self) -> ::windows::core::Result<()>;
    fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCSessionReferredEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionReferredEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>() -> IRTCSessionReferredEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferredByURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReferredByURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreferredbyuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferToURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReferToURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreferouri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReferCookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrefercookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Accept().into()
        }
        unsafe extern "system" fn Reject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reject().into()
        }
        unsafe extern "system" fn SetReferredSessionState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReferredSessionState(::core::mem::transmute_copy(&enstate)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            ReferredByURI: ReferredByURI::<Identity, Impl, OFFSET>,
            ReferToURI: ReferToURI::<Identity, Impl, OFFSET>,
            ReferCookie: ReferCookie::<Identity, Impl, OFFSET>,
            Accept: Accept::<Identity, Impl, OFFSET>,
            Reject: Reject::<Identity, Impl, OFFSET>,
            SetReferredSessionState: SetReferredSessionState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionReferredEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IRTCSession>;
    fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn StatusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCSessionStateChangeEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>() -> IRTCSessionStateChangeEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionStateChangeEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCSessionStateChangeEvent2_Impl: Sized + IRTCSessionStateChangeEvent_Impl {
    fn MediaTypes(&self) -> ::windows::core::Result<i32>;
    fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL>;
    fn IsForked(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut ::windows::core::BSTR, pbstrsessiondescription: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCSessionStateChangeEvent2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCSessionStateChangeEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>() -> IRTCSessionStateChangeEvent2_Vtbl {
        unsafe extern "system" fn MediaTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmediatypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_RemotePreferredSecurityLevel(::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsForked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisforked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsForked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisforked, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRemoteSessionDescription(::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into()
        }
        Self {
            base__: IRTCSessionStateChangeEvent_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
            get_RemotePreferredSecurityLevel: get_RemotePreferredSecurityLevel::<Identity, Impl, OFFSET>,
            IsForked: IsForked::<Identity, Impl, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCSessionStateChangeEvent2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IRTCSessionStateChangeEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCUserSearch_Impl: Sized {
    fn CreateQuery(&self) -> ::windows::core::Result<IRTCUserSearchQuery>;
    fn ExecuteSearch(&self, pquery: ::core::option::Option<&IRTCUserSearchQuery>, pprofile: ::core::option::Option<&IRTCProfile>, lcookie: isize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRTCUserSearch {}
impl IRTCUserSearch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearch_Impl, const OFFSET: isize>() -> IRTCUserSearch_Vtbl {
        unsafe extern "system" fn CreateQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteSearch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecuteSearch(::windows::core::from_raw_borrowed(&pquery), ::windows::core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lcookie)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCUserSearchQuery_Impl: Sized {
    fn put_SearchTerm(&self, bstrname: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn get_SearchTerm(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SearchTerms(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn put_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::Result<()>;
    fn get_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> ::windows::core::Result<i32>;
    fn SetSearchDomain(&self, bstrdomain: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SearchDomain(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
impl ::windows::core::RuntimeName for IRTCUserSearchQuery {}
impl IRTCUserSearchQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>() -> IRTCUserSearchQuery_Vtbl {
        unsafe extern "system" fn put_SearchTerm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_SearchTerm(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn get_SearchTerm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_SearchTerm(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchTerms<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnames: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchTerms() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnames, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_SearchPreference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_SearchPreference(::core::mem::transmute_copy(&enpreference), ::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn get_SearchPreference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_SearchPreference(::core::mem::transmute_copy(&enpreference)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSearchDomain(::core::mem::transmute(&bstrdomain)).into()
        }
        unsafe extern "system" fn SearchDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_SearchTerm: put_SearchTerm::<Identity, Impl, OFFSET>,
            get_SearchTerm: get_SearchTerm::<Identity, Impl, OFFSET>,
            SearchTerms: SearchTerms::<Identity, Impl, OFFSET>,
            put_SearchPreference: put_SearchPreference::<Identity, Impl, OFFSET>,
            get_SearchPreference: get_SearchPreference::<Identity, Impl, OFFSET>,
            SetSearchDomain: SetSearchDomain::<Identity, Impl, OFFSET>,
            SearchDomain: SearchDomain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearchQuery as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"implement\"`*"]
pub trait IRTCUserSearchResult_Impl: Sized {
    fn get_Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> ::windows::core::Result<::windows::core::BSTR>;
}
impl ::windows::core::RuntimeName for IRTCUserSearchResult {}
impl IRTCUserSearchResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResult_Impl, const OFFSET: isize>() -> IRTCUserSearchResult_Vtbl {
        unsafe extern "system" fn get_Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Value(::core::mem::transmute_copy(&encolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_Value: get_Value::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearchResult as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCUserSearchResultsEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EnumerateResults(&self) -> ::windows::core::Result<IRTCEnumUserSearchResults>;
    fn Results(&self) -> ::windows::core::Result<IRTCCollection>;
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile2>;
    fn Query(&self) -> ::windows::core::Result<IRTCUserSearchQuery>;
    fn Cookie(&self) -> ::windows::core::Result<isize>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
    fn MoreAvailable(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCUserSearchResultsEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCUserSearchResultsEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>() -> IRTCUserSearchResultsEvent_Vtbl {
        unsafe extern "system" fn EnumerateResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateResults() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Results() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Query() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoreAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmoreavailable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumerateResults: EnumerateResults::<Identity, Impl, OFFSET>,
            Results: Results::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            Cookie: Cookie::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            MoreAvailable: MoreAvailable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCUserSearchResultsEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCWatcher_Impl: Sized + IRTCPresenceContact_Impl {
    fn State(&self) -> ::windows::core::Result<RTC_WATCHER_STATE>;
    fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCWatcher {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcher_Impl, const OFFSET: isize>() -> IRTCWatcher_Vtbl {
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&enstate)).into()
        }
        Self {
            base__: IRTCPresenceContact_Vtbl::new::<Identity, Impl, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcher as ::windows::core::ComInterface>::IID || iid == &<IRTCPresenceContact as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCWatcher2_Impl: Sized + IRTCWatcher_Impl {
    fn Profile(&self) -> ::windows::core::Result<IRTCProfile2>;
    fn Scope(&self) -> ::windows::core::Result<RTC_ACE_SCOPE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRTCWatcher2 {}
#[cfg(feature = "Win32_Foundation")]
impl IRTCWatcher2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcher2_Impl, const OFFSET: isize>() -> IRTCWatcher2_Vtbl {
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Scope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penscope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IRTCWatcher_Vtbl::new::<Identity, Impl, OFFSET>(), Profile: Profile::<Identity, Impl, OFFSET>, Scope: Scope::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcher2 as ::windows::core::ComInterface>::IID || iid == &<IRTCPresenceContact as ::windows::core::ComInterface>::IID || iid == &<IRTCWatcher as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCWatcherEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Watcher(&self) -> ::windows::core::Result<IRTCWatcher>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCWatcherEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcherEvent_Impl, const OFFSET: isize>() -> IRTCWatcherEvent_Vtbl {
        unsafe extern "system" fn Watcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcherEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Watcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Watcher: Watcher::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcherEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRTCWatcherEvent2_Impl: Sized + IRTCWatcherEvent_Impl {
    fn EventType(&self) -> ::windows::core::Result<RTC_WATCHER_EVENT_TYPE>;
    fn StatusCode(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRTCWatcherEvent2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRTCWatcherEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcherEvent2_Impl, const OFFSET: isize>() -> IRTCWatcherEvent2_Vtbl {
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcherEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRTCWatcherEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRTCWatcherEvent_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventType: EventType::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRTCWatcherEvent2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IRTCWatcherEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RealTimeCommunications\"`, `\"Win32_Networking_WinSock\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait ITransportSettingsInternal_Impl: Sized {
    fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()>;
    fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::RuntimeName for ITransportSettingsInternal {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ITransportSettingsInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransportSettingsInternal_Impl, const OFFSET: isize>() -> ITransportSettingsInternal_Vtbl {
        unsafe extern "system" fn ApplySetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransportSettingsInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplySetting(::core::mem::transmute_copy(&setting)).into()
        }
        unsafe extern "system" fn QuerySetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransportSettingsInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QuerySetting(::core::mem::transmute_copy(&setting)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplySetting: ApplySetting::<Identity, Impl, OFFSET>,
            QuerySetting: QuerySetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransportSettingsInternal as ::windows::core::ComInterface>::IID
    }
}
