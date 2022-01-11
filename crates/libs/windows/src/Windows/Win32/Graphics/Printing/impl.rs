#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSendNotificationCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData();
}
#[cfg(feature = "Win32_Foundation")]
impl IAsyncGetSendNotificationCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSendNotificationCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncGetSendNotificationCookieVtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IAsyncGetSendNotificationCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FinishAsyncCall::<Impl, IMPL_OFFSET>, CancelAsyncCall::<Impl, IMPL_OFFSET>, FinishAsyncCallWithData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncGetSendNotificationCookie as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSrvReferralCookieImpl: Sized {
    fn FinishAsyncCall();
    fn CancelAsyncCall();
    fn FinishAsyncCallWithData();
}
#[cfg(feature = "Win32_Foundation")]
impl IAsyncGetSrvReferralCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSrvReferralCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncGetSrvReferralCookieVtbl {
        unsafe extern "system" fn FinishAsyncCall<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FinishAsyncCall::<Impl, IMPL_OFFSET>, CancelAsyncCall::<Impl, IMPL_OFFSET>, FinishAsyncCallWithData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncGetSrvReferralCookie as ::windows::core::Interface>::IID
    }
}
pub trait IBidiAsyncNotifyChannelImpl: Sized + IPrintAsyncNotifyChannelImpl {
    fn CreateNotificationChannel();
    fn GetPrintName();
    fn GetChannelNotificationType();
    fn AsyncGetNotificationSendResponse();
    fn AsyncCloseChannel();
}
impl IBidiAsyncNotifyChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBidiAsyncNotifyChannelVtbl {
        unsafe extern "system" fn CreateNotificationChannel<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrintName<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelNotificationType<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncCloseChannel<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SendNotification::<Impl, IMPL_OFFSET>, CloseChannel::<Impl, IMPL_OFFSET>, CreateNotificationChannel::<Impl, IMPL_OFFSET>, GetPrintName::<Impl, IMPL_OFFSET>, GetChannelNotificationType::<Impl, IMPL_OFFSET>, AsyncGetNotificationSendResponse::<Impl, IMPL_OFFSET>, AsyncCloseChannel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBidiAsyncNotifyChannel as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncCookieImpl: Sized {
    fn FinishAsyncCall();
    fn CancelAsyncCall();
}
impl IPrintAsyncCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncCookieVtbl {
        unsafe extern "system" fn FinishAsyncCall<Impl: IPrintAsyncCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IPrintAsyncCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FinishAsyncCall::<Impl, IMPL_OFFSET>, CancelAsyncCall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncCookie as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNewChannelCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData();
}
impl IPrintAsyncNewChannelCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNewChannelCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNewChannelCookieVtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IPrintAsyncNewChannelCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FinishAsyncCall::<Impl, IMPL_OFFSET>, CancelAsyncCall::<Impl, IMPL_OFFSET>, FinishAsyncCallWithData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNewChannelCookie as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyImpl: Sized {
    fn CreatePrintAsyncNotifyChannel();
    fn CreatePrintAsyncNotifyRegistration();
}
impl IPrintAsyncNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyVtbl {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Impl: IPrintAsyncNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: ::windows::core::RawPtr, param5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Impl: IPrintAsyncNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: ::windows::core::RawPtr, param4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreatePrintAsyncNotifyChannel::<Impl, IMPL_OFFSET>, CreatePrintAsyncNotifyRegistration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotify as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyCallbackImpl: Sized {
    fn OnEventNotify();
    fn ChannelClosed();
}
impl IPrintAsyncNotifyCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyCallbackVtbl {
        unsafe extern "system" fn OnEventNotify<Impl: IPrintAsyncNotifyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChannelClosed<Impl: IPrintAsyncNotifyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnEventNotify::<Impl, IMPL_OFFSET>, ChannelClosed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyChannelImpl: Sized {
    fn SendNotification();
    fn CloseChannel();
}
impl IPrintAsyncNotifyChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyChannelVtbl {
        unsafe extern "system" fn SendNotification<Impl: IPrintAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseChannel<Impl: IPrintAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SendNotification::<Impl, IMPL_OFFSET>, CloseChannel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyChannel as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyDataObjectImpl: Sized {
    fn AcquireData();
    fn ReleaseData();
}
impl IPrintAsyncNotifyDataObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyDataObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyDataObjectVtbl {
        unsafe extern "system" fn AcquireData<Impl: IPrintAsyncNotifyDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseData<Impl: IPrintAsyncNotifyDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AcquireData::<Impl, IMPL_OFFSET>, ReleaseData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyDataObject as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyRegistrationImpl: Sized {
    fn RegisterForNotifications();
    fn UnregisterForNotifications();
}
impl IPrintAsyncNotifyRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn RegisterForNotifications<Impl: IPrintAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterForNotifications<Impl: IPrintAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterForNotifications::<Impl, IMPL_OFFSET>, UnregisterForNotifications::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintAsyncNotifyServerReferralImpl: Sized {
    fn GetServerReferral();
    fn AsyncGetServerReferral();
    fn SetServerReferral();
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintAsyncNotifyServerReferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyServerReferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyServerReferralVtbl {
        unsafe extern "system" fn GetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncGetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prmtserverreferral: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetServerReferral::<Impl, IMPL_OFFSET>, AsyncGetServerReferral::<Impl, IMPL_OFFSET>, SetServerReferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyServerReferral as ::windows::core::Interface>::IID
    }
}
pub trait IPrintBidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNewChannel();
}
impl IPrintBidiAsyncNotifyRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintBidiAsyncNotifyRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintBidiAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn AsyncGetNewChannel<Impl: IPrintBidiAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterForNotifications::<Impl, IMPL_OFFSET>, UnregisterForNotifications::<Impl, IMPL_OFFSET>, AsyncGetNewChannel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintBidiAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelperImpl: Sized {
    fn GetOption();
    fn SetOptions();
    fn EnumConstrainedOptions();
    fn WhyConstrained();
    fn EnumFeatures();
    fn EnumOptions();
    fn GetFontSubstitution();
    fn SetFontSubstitution();
    fn CreateInstanceOfMSXMLObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperVtbl {
        unsafe extern "system" fn GetOption<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: super::super::Foundation::PSTR, ppszoption: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOptions<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumConstrainedOptions<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pconstrainedoptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WhyConstrained<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFeatures<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeaturelist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumOptions<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, poptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontSubstitution<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, ppszdevfontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFontSubstitution<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, pszdevfontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOption::<Impl, IMPL_OFFSET>,
            SetOptions::<Impl, IMPL_OFFSET>,
            EnumConstrainedOptions::<Impl, IMPL_OFFSET>,
            WhyConstrained::<Impl, IMPL_OFFSET>,
            EnumFeatures::<Impl, IMPL_OFFSET>,
            EnumOptions::<Impl, IMPL_OFFSET>,
            GetFontSubstitution::<Impl, IMPL_OFFSET>,
            SetFontSubstitution::<Impl, IMPL_OFFSET>,
            CreateInstanceOfMSXMLObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelperPSImpl: Sized + IPrintCoreHelperImpl {
    fn GetGlobalAttribute();
    fn GetFeatureAttribute();
    fn GetOptionAttribute();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelperPSVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperPSImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperPSVtbl {
        unsafe extern "system" fn GetGlobalAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeatureAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptionAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOption::<Impl, IMPL_OFFSET>,
            SetOptions::<Impl, IMPL_OFFSET>,
            EnumConstrainedOptions::<Impl, IMPL_OFFSET>,
            WhyConstrained::<Impl, IMPL_OFFSET>,
            EnumFeatures::<Impl, IMPL_OFFSET>,
            EnumOptions::<Impl, IMPL_OFFSET>,
            GetFontSubstitution::<Impl, IMPL_OFFSET>,
            SetFontSubstitution::<Impl, IMPL_OFFSET>,
            CreateInstanceOfMSXMLObject::<Impl, IMPL_OFFSET>,
            GetGlobalAttribute::<Impl, IMPL_OFFSET>,
            GetFeatureAttribute::<Impl, IMPL_OFFSET>,
            GetOptionAttribute::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperPS as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUniImpl: Sized + IPrintCoreHelperImpl {
    fn CreateGDLSnapshot();
    fn CreateDefaultGDLSnapshot();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUniVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUniImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperUniVtbl {
        unsafe extern "system" fn CreateGDLSnapshot<Impl: IPrintCoreHelperUniImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Impl: IPrintCoreHelperUniImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOption::<Impl, IMPL_OFFSET>,
            SetOptions::<Impl, IMPL_OFFSET>,
            EnumConstrainedOptions::<Impl, IMPL_OFFSET>,
            WhyConstrained::<Impl, IMPL_OFFSET>,
            EnumFeatures::<Impl, IMPL_OFFSET>,
            EnumOptions::<Impl, IMPL_OFFSET>,
            GetFontSubstitution::<Impl, IMPL_OFFSET>,
            SetFontSubstitution::<Impl, IMPL_OFFSET>,
            CreateInstanceOfMSXMLObject::<Impl, IMPL_OFFSET>,
            CreateGDLSnapshot::<Impl, IMPL_OFFSET>,
            CreateDefaultGDLSnapshot::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni2Impl: Sized + IPrintCoreHelperUniImpl + IPrintCoreHelperImpl {
    fn GetNamedCommand();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUni2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperUni2Vtbl {
        unsafe extern "system" fn GetNamedCommand<Impl: IPrintCoreHelperUni2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: super::super::Foundation::PWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOption::<Impl, IMPL_OFFSET>,
            SetOptions::<Impl, IMPL_OFFSET>,
            EnumConstrainedOptions::<Impl, IMPL_OFFSET>,
            WhyConstrained::<Impl, IMPL_OFFSET>,
            EnumFeatures::<Impl, IMPL_OFFSET>,
            EnumOptions::<Impl, IMPL_OFFSET>,
            GetFontSubstitution::<Impl, IMPL_OFFSET>,
            SetFontSubstitution::<Impl, IMPL_OFFSET>,
            CreateInstanceOfMSXMLObject::<Impl, IMPL_OFFSET>,
            CreateGDLSnapshot::<Impl, IMPL_OFFSET>,
            CreateDefaultGDLSnapshot::<Impl, IMPL_OFFSET>,
            GetNamedCommand::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintCoreUI2Impl: Sized + IPrintOemDriverUIImpl {
    fn GetOptions();
    fn SetOptions();
    fn EnumConstrainedOptions();
    fn WhyConstrained();
    fn GetGlobalAttribute();
    fn GetFeatureAttribute();
    fn GetOptionAttribute();
    fn EnumFeatures();
    fn EnumOptions();
    fn QuerySimulationSupport();
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintCoreUI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreUI2Vtbl {
        unsafe extern "system" fn GetOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumConstrainedOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszconstrainedoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WhyConstrained<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pmszreasonlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGlobalAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeatureAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptionAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFeatures<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuerySimulationSupport<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            DrvGetDriverSetting::<Impl, IMPL_OFFSET>,
            DrvUpgradeRegistrySetting::<Impl, IMPL_OFFSET>,
            DrvUpdateUISetting::<Impl, IMPL_OFFSET>,
            GetOptions::<Impl, IMPL_OFFSET>,
            SetOptions::<Impl, IMPL_OFFSET>,
            EnumConstrainedOptions::<Impl, IMPL_OFFSET>,
            WhyConstrained::<Impl, IMPL_OFFSET>,
            GetGlobalAttribute::<Impl, IMPL_OFFSET>,
            GetFeatureAttribute::<Impl, IMPL_OFFSET>,
            GetOptionAttribute::<Impl, IMPL_OFFSET>,
            EnumFeatures::<Impl, IMPL_OFFSET>,
            EnumOptions::<Impl, IMPL_OFFSET>,
            QuerySimulationSupport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreUI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintJobImpl: Sized {
    fn Name();
    fn Id();
    fn PrintedPages();
    fn TotalPages();
    fn Status();
    fn SubmissionTime();
    fn RequestCancel();
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintJobVtbl {
        unsafe extern "system" fn Name<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrintedPages<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalPages<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut PrintJobStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubmissionTime<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestCancel<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Name::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, PrintedPages::<Impl, IMPL_OFFSET>, TotalPages::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>, SubmissionTime::<Impl, IMPL_OFFSET>, RequestCancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintJobCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintJobCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintJobCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrintJobCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPrintJobCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrintJobCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintJobCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemCommonImpl: Sized {
    fn GetInfo();
    fn DevMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemCommonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemCommonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemCommonVtbl {
        unsafe extern "system" fn GetInfo<Impl: IPrintOemCommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DevMode<Impl: IPrintOemCommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInfo::<Impl, IMPL_OFFSET>, DevMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemCommon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintOemDriverUIImpl: Sized {
    fn DrvGetDriverSetting();
    fn DrvUpgradeRegistrySetting();
    fn DrvUpdateUISetting();
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintOemDriverUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemDriverUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemDriverUIVtbl {
        unsafe extern "system" fn DrvGetDriverSetting<Impl: IPrintOemDriverUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, feature: super::super::Foundation::PSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Impl: IPrintOemDriverUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: super::super::Foundation::PSTR, poption: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrvUpdateUISetting<Impl: IPrintOemDriverUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DrvGetDriverSetting::<Impl, IMPL_OFFSET>, DrvUpgradeRegistrySetting::<Impl, IMPL_OFFSET>, DrvUpdateUISetting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemDriverUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUIImpl: Sized + IPrintOemCommonImpl {
    fn PublishDriverInterface();
    fn CommonUIProp();
    fn DocumentPropertySheets();
    fn DevicePropertySheets();
    fn DevQueryPrintEx();
    fn DeviceCapabilitiesA();
    fn UpgradePrinter();
    fn PrinterEvent();
    fn DriverEvent();
    fn QueryColorProfile();
    fn FontInstallerDlgProc();
    fn UpdateExternalFonts();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemUIVtbl {
        unsafe extern "system" fn PublishDriverInterface<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommonUIProp<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DocumentPropertySheets<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DevicePropertySheets<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DevQueryPrintEx<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpgradePrinter<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrinterEvent<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintername: super::super::Foundation::PWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DriverEvent<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryColorProfile<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FontInstallerDlgProc<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateExternalFonts<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            DevMode::<Impl, IMPL_OFFSET>,
            PublishDriverInterface::<Impl, IMPL_OFFSET>,
            CommonUIProp::<Impl, IMPL_OFFSET>,
            DocumentPropertySheets::<Impl, IMPL_OFFSET>,
            DevicePropertySheets::<Impl, IMPL_OFFSET>,
            DevQueryPrintEx::<Impl, IMPL_OFFSET>,
            DeviceCapabilitiesA::<Impl, IMPL_OFFSET>,
            UpgradePrinter::<Impl, IMPL_OFFSET>,
            PrinterEvent::<Impl, IMPL_OFFSET>,
            DriverEvent::<Impl, IMPL_OFFSET>,
            QueryColorProfile::<Impl, IMPL_OFFSET>,
            FontInstallerDlgProc::<Impl, IMPL_OFFSET>,
            UpdateExternalFonts::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI2Impl: Sized + IPrintOemUIImpl + IPrintOemCommonImpl {
    fn QueryJobAttributes();
    fn HideStandardUI();
    fn DocumentEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemUI2Vtbl {
        unsafe extern "system" fn QueryJobAttributes<Impl: IPrintOemUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HideStandardUI<Impl: IPrintOemUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DocumentEvent<Impl: IPrintOemUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            DevMode::<Impl, IMPL_OFFSET>,
            PublishDriverInterface::<Impl, IMPL_OFFSET>,
            CommonUIProp::<Impl, IMPL_OFFSET>,
            DocumentPropertySheets::<Impl, IMPL_OFFSET>,
            DevicePropertySheets::<Impl, IMPL_OFFSET>,
            DevQueryPrintEx::<Impl, IMPL_OFFSET>,
            DeviceCapabilitiesA::<Impl, IMPL_OFFSET>,
            UpgradePrinter::<Impl, IMPL_OFFSET>,
            PrinterEvent::<Impl, IMPL_OFFSET>,
            DriverEvent::<Impl, IMPL_OFFSET>,
            QueryColorProfile::<Impl, IMPL_OFFSET>,
            FontInstallerDlgProc::<Impl, IMPL_OFFSET>,
            UpdateExternalFonts::<Impl, IMPL_OFFSET>,
            QueryJobAttributes::<Impl, IMPL_OFFSET>,
            HideStandardUI::<Impl, IMPL_OFFSET>,
            DocumentEvent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemUIMXDCImpl: Sized {
    fn AdjustImageableArea();
    fn AdjustImageCompression();
    fn AdjustDPI();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemUIMXDCVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIMXDCImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemUIMXDCVtbl {
        unsafe extern "system" fn AdjustImageableArea<Impl: IPrintOemUIMXDCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdjustImageCompression<Impl: IPrintOemUIMXDCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdjustDPI<Impl: IPrintOemUIMXDCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AdjustImageableArea::<Impl, IMPL_OFFSET>, AdjustImageCompression::<Impl, IMPL_OFFSET>, AdjustDPI::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUIMXDC as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait IPrintPreviewDxgiPackageTargetImpl: Sized {
    fn SetJobPageCount();
    fn DrawPage();
    fn InvalidatePreview();
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl IPrintPreviewDxgiPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPreviewDxgiPackageTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPreviewDxgiPackageTargetVtbl {
        unsafe extern "system" fn SetJobPageCount<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counttype: PageCountType, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DrawPage<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, pageimage: ::windows::core::RawPtr, dpix: f32, dpiy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvalidatePreview<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetJobPageCount::<Impl, IMPL_OFFSET>, DrawPage::<Impl, IMPL_OFFSET>, InvalidatePreview::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintPreviewDxgiPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaAsyncOperationImpl: Sized + IDispatchImpl {
    fn Start();
    fn Cancel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaAsyncOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaAsyncOperationVtbl {
        unsafe extern "system" fn Start<Impl: IPrintSchemaAsyncOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPrintSchemaAsyncOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Start::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaAsyncOperationEventImpl: Sized + IDispatchImpl {
    fn Completed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaAsyncOperationEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperationEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaAsyncOperationEventVtbl {
        unsafe extern "system" fn Completed<Impl: IPrintSchemaAsyncOperationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pticket: ::windows::core::RawPtr, hroperation: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Completed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperationEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaCapabilitiesImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetFeatureByKeyName();
    fn GetFeature();
    fn PageImageableSize();
    fn JobCopiesAllDocumentsMinValue();
    fn JobCopiesAllDocumentsMaxValue();
    fn GetSelectedOptionInPrintTicket();
    fn GetOptions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaCapabilitiesVtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeature<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PageImageableSize<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppageimageablesize: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptions<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoptioncollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            GetFeatureByKeyName::<Impl, IMPL_OFFSET>,
            GetFeature::<Impl, IMPL_OFFSET>,
            PageImageableSize::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocumentsMinValue::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocumentsMaxValue::<Impl, IMPL_OFFSET>,
            GetSelectedOptionInPrintTicket::<Impl, IMPL_OFFSET>,
            GetOptions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaCapabilities2Impl: Sized + IPrintSchemaCapabilitiesImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetParameterDefinition();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaCapabilities2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaCapabilities2Vtbl {
        unsafe extern "system" fn GetParameterDefinition<Impl: IPrintSchemaCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            GetFeatureByKeyName::<Impl, IMPL_OFFSET>,
            GetFeature::<Impl, IMPL_OFFSET>,
            PageImageableSize::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocumentsMinValue::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocumentsMaxValue::<Impl, IMPL_OFFSET>,
            GetSelectedOptionInPrintTicket::<Impl, IMPL_OFFSET>,
            GetOptions::<Impl, IMPL_OFFSET>,
            GetParameterDefinition::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaDisplayableElementImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn DisplayName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaDisplayableElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaDisplayableElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaDisplayableElementVtbl {
        unsafe extern "system" fn DisplayName<Impl: IPrintSchemaDisplayableElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, XmlNode::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, NamespaceUri::<Impl, IMPL_OFFSET>, DisplayName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaElementImpl: Sized + IDispatchImpl {
    fn XmlNode();
    fn Name();
    fn NamespaceUri();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaElementVtbl {
        unsafe extern "system" fn XmlNode<Impl: IPrintSchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IPrintSchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NamespaceUri<Impl: IPrintSchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespaceuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, XmlNode::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, NamespaceUri::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaFeatureImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn SelectedOption();
    fn SetSelectedOption();
    fn SelectionType();
    fn GetOption();
    fn DisplayUI();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaFeatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaFeatureVtbl {
        unsafe extern "system" fn SelectedOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSelectedOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poption: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectionType<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayUI<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SelectedOption::<Impl, IMPL_OFFSET>,
            SetSelectedOption::<Impl, IMPL_OFFSET>,
            SelectionType::<Impl, IMPL_OFFSET>,
            GetOption::<Impl, IMPL_OFFSET>,
            DisplayUI::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaFeature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaNUpOptionImpl: Sized + IPrintSchemaOptionImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn PagesPerSheet();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaNUpOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaNUpOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaNUpOptionVtbl {
        unsafe extern "system" fn PagesPerSheet<Impl: IPrintSchemaNUpOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpagespersheet: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            Selected::<Impl, IMPL_OFFSET>,
            Constrained::<Impl, IMPL_OFFSET>,
            GetPropertyValue::<Impl, IMPL_OFFSET>,
            PagesPerSheet::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaNUpOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaOptionImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn Selected();
    fn Constrained();
    fn GetPropertyValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaOptionVtbl {
        unsafe extern "system" fn Selected<Impl: IPrintSchemaOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Constrained<Impl: IPrintSchemaOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IPrintSchemaOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlvaluenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            Selected::<Impl, IMPL_OFFSET>,
            Constrained::<Impl, IMPL_OFFSET>,
            GetPropertyValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaOptionCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaOptionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaOptionCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaOptionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaPageImageableSizeImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn ImageableSizeWidthInMicrons();
    fn ImageableSizeHeightInMicrons();
    fn OriginWidthInMicrons();
    fn OriginHeightInMicrons();
    fn ExtentWidthInMicrons();
    fn ExtentHeightInMicrons();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaPageImageableSizeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSizeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaPageImageableSizeVtbl {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizewidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizeheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OriginWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OriginHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            ImageableSizeWidthInMicrons::<Impl, IMPL_OFFSET>,
            ImageableSizeHeightInMicrons::<Impl, IMPL_OFFSET>,
            OriginWidthInMicrons::<Impl, IMPL_OFFSET>,
            OriginHeightInMicrons::<Impl, IMPL_OFFSET>,
            ExtentWidthInMicrons::<Impl, IMPL_OFFSET>,
            ExtentHeightInMicrons::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaPageImageableSize as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaPageMediaSizeOptionImpl: Sized + IPrintSchemaOptionImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn WidthInMicrons();
    fn HeightInMicrons();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaPageMediaSizeOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageMediaSizeOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaPageMediaSizeOptionVtbl {
        unsafe extern "system" fn WidthInMicrons<Impl: IPrintSchemaPageMediaSizeOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HeightInMicrons<Impl: IPrintSchemaPageMediaSizeOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            Selected::<Impl, IMPL_OFFSET>,
            Constrained::<Impl, IMPL_OFFSET>,
            GetPropertyValue::<Impl, IMPL_OFFSET>,
            WidthInMicrons::<Impl, IMPL_OFFSET>,
            HeightInMicrons::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaPageMediaSizeOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaParameterDefinitionImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn UserInputRequired();
    fn UnitType();
    fn DataType();
    fn RangeMin();
    fn RangeMax();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaParameterDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaParameterDefinitionVtbl {
        unsafe extern "system" fn UserInputRequired<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnitType<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrunittype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataType<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RangeMin<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RangeMax<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            UserInputRequired::<Impl, IMPL_OFFSET>,
            UnitType::<Impl, IMPL_OFFSET>,
            DataType::<Impl, IMPL_OFFSET>,
            RangeMin::<Impl, IMPL_OFFSET>,
            RangeMax::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaParameterDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaParameterInitializerImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn Value();
    fn SetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaParameterInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterInitializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaParameterInitializerVtbl {
        unsafe extern "system" fn Value<Impl: IPrintSchemaParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IPrintSchemaParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, XmlNode::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, NamespaceUri::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaParameterInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaTicketImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetFeatureByKeyName();
    fn GetFeature();
    fn ValidateAsync();
    fn CommitAsync();
    fn NotifyXmlChanged();
    fn GetCapabilities();
    fn JobCopiesAllDocuments();
    fn SetJobCopiesAllDocuments();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaTicketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaTicketVtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeature<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateAsync<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitAsync<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticketcommit: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyXmlChanged<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCapabilities<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uljobcopiesalldocuments: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            GetFeatureByKeyName::<Impl, IMPL_OFFSET>,
            GetFeature::<Impl, IMPL_OFFSET>,
            ValidateAsync::<Impl, IMPL_OFFSET>,
            CommitAsync::<Impl, IMPL_OFFSET>,
            NotifyXmlChanged::<Impl, IMPL_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocuments::<Impl, IMPL_OFFSET>,
            SetJobCopiesAllDocuments::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaTicket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaTicket2Impl: Sized + IPrintSchemaTicketImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetParameterInitializer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaTicket2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaTicket2Vtbl {
        unsafe extern "system" fn GetParameterInitializer<Impl: IPrintSchemaTicket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterinitializer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            XmlNode::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            NamespaceUri::<Impl, IMPL_OFFSET>,
            GetFeatureByKeyName::<Impl, IMPL_OFFSET>,
            GetFeature::<Impl, IMPL_OFFSET>,
            ValidateAsync::<Impl, IMPL_OFFSET>,
            CommitAsync::<Impl, IMPL_OFFSET>,
            NotifyXmlChanged::<Impl, IMPL_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocuments::<Impl, IMPL_OFFSET>,
            SetJobCopiesAllDocuments::<Impl, IMPL_OFFSET>,
            GetParameterInitializer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaTicket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintTicketProviderImpl: Sized {
    fn GetSupportedVersions();
    fn BindPrinter();
    fn QueryDeviceNamespace();
    fn ConvertPrintTicketToDevMode();
    fn ConvertDevModeToPrintTicket();
    fn GetPrintCapabilities();
    fn ValidatePrintTicket();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintTicketProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketProviderVtbl {
        unsafe extern "system" fn GetSupportedVersions<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindPrinter<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDeviceNamespace<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrintCapabilities<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidatePrintTicket<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbaseticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSupportedVersions::<Impl, IMPL_OFFSET>, BindPrinter::<Impl, IMPL_OFFSET>, QueryDeviceNamespace::<Impl, IMPL_OFFSET>, ConvertPrintTicketToDevMode::<Impl, IMPL_OFFSET>, ConvertDevModeToPrintTicket::<Impl, IMPL_OFFSET>, GetPrintCapabilities::<Impl, IMPL_OFFSET>, ValidatePrintTicket::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintTicketProvider2Impl: Sized + IPrintTicketProviderImpl {
    fn GetPrintDeviceCapabilities();
    fn GetPrintDeviceResources();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintTicketProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketProvider2Vtbl {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Impl: IPrintTicketProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppdevicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrintDeviceResources<Impl: IPrintTicketProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalename: super::super::Foundation::PWSTR, pprintticket: ::windows::core::RawPtr, ppdeviceresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSupportedVersions::<Impl, IMPL_OFFSET>,
            BindPrinter::<Impl, IMPL_OFFSET>,
            QueryDeviceNamespace::<Impl, IMPL_OFFSET>,
            ConvertPrintTicketToDevMode::<Impl, IMPL_OFFSET>,
            ConvertDevModeToPrintTicket::<Impl, IMPL_OFFSET>,
            GetPrintCapabilities::<Impl, IMPL_OFFSET>,
            ValidatePrintTicket::<Impl, IMPL_OFFSET>,
            GetPrintDeviceCapabilities::<Impl, IMPL_OFFSET>,
            GetPrintDeviceResources::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketProvider2 as ::windows::core::Interface>::IID
    }
}
pub trait IPrintUnidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNotification();
}
impl IPrintUnidiAsyncNotifyRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintUnidiAsyncNotifyRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintUnidiAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn AsyncGetNotification<Impl: IPrintUnidiAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterForNotifications::<Impl, IMPL_OFFSET>, UnregisterForNotifications::<Impl, IMPL_OFFSET>, AsyncGetNotification::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintUnidiAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrinterBidiSetRequestCallbackImpl: Sized {
    fn Completed();
}
#[cfg(feature = "Win32_Foundation")]
impl IPrinterBidiSetRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterBidiSetRequestCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterBidiSetRequestCallbackVtbl {
        unsafe extern "system" fn Completed<Impl: IPrinterBidiSetRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Completed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterBidiSetRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPrinterExtensionAsyncOperationImpl: Sized {
    fn Cancel();
}
impl IPrinterExtensionAsyncOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionAsyncOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionAsyncOperationVtbl {
        unsafe extern "system" fn Cancel<Impl: IPrinterExtensionAsyncOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionAsyncOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionContextImpl: Sized + IDispatchImpl {
    fn PrinterQueue();
    fn PrintSchemaTicket();
    fn DriverProperties();
    fn UserProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionContextVtbl {
        unsafe extern "system" fn PrinterQueue<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrintSchemaTicket<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppticket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DriverProperties<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserProperties<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, PrinterQueue::<Impl, IMPL_OFFSET>, PrintSchemaTicket::<Impl, IMPL_OFFSET>, DriverProperties::<Impl, IMPL_OFFSET>, UserProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionContextCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionContextCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionContextCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionContextCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionEventImpl: Sized + IDispatchImpl {
    fn OnDriverEvent();
    fn OnPrinterQueuesEnumerated();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionEventVtbl {
        unsafe extern "system" fn OnDriverEvent<Impl: IPrinterExtensionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Impl: IPrinterExtensionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OnDriverEvent::<Impl, IMPL_OFFSET>, OnPrinterQueuesEnumerated::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionEventArgsImpl: Sized + IPrinterExtensionContextImpl + IDispatchImpl {
    fn BidiNotification();
    fn ReasonId();
    fn Request();
    fn SourceApplication();
    fn DetailedReasonId();
    fn WindowModal();
    fn WindowParent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionEventArgsVtbl {
        unsafe extern "system" fn BidiNotification<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbidinotification: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReasonId<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Request<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceApplication<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplication: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetailedReasonId<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetailedreasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WindowModal<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WindowParent<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            PrinterQueue::<Impl, IMPL_OFFSET>,
            PrintSchemaTicket::<Impl, IMPL_OFFSET>,
            DriverProperties::<Impl, IMPL_OFFSET>,
            UserProperties::<Impl, IMPL_OFFSET>,
            BidiNotification::<Impl, IMPL_OFFSET>,
            ReasonId::<Impl, IMPL_OFFSET>,
            Request::<Impl, IMPL_OFFSET>,
            SourceApplication::<Impl, IMPL_OFFSET>,
            DetailedReasonId::<Impl, IMPL_OFFSET>,
            WindowModal::<Impl, IMPL_OFFSET>,
            WindowParent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IPrinterExtensionManagerImpl: Sized {
    fn EnableEvents();
    fn DisableEvents();
}
impl IPrinterExtensionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionManagerVtbl {
        unsafe extern "system" fn EnableEvents<Impl: IPrinterExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printerdriverid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableEvents<Impl: IPrinterExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnableEvents::<Impl, IMPL_OFFSET>, DisableEvents::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionRequestImpl: Sized + IDispatchImpl {
    fn Cancel();
    fn Complete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionRequestVtbl {
        unsafe extern "system" fn Cancel<Impl: IPrinterExtensionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, bstrlogmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Complete<Impl: IPrinterExtensionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, Complete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterPropertyBagImpl: Sized + IDispatchImpl {
    fn GetBool();
    fn SetBool();
    fn GetInt32();
    fn SetInt32();
    fn GetString();
    fn SetString();
    fn GetBytes();
    fn SetBytes();
    fn GetReadStream();
    fn GetWriteStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterPropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBagImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterPropertyBagVtbl {
        unsafe extern "system" fn GetBool<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBool<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInt32<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInt32<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetString<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetString<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBytes<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBytes<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cbvalue: u32, pvalue: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadStream<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWriteStream<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetBool::<Impl, IMPL_OFFSET>,
            SetBool::<Impl, IMPL_OFFSET>,
            GetInt32::<Impl, IMPL_OFFSET>,
            SetInt32::<Impl, IMPL_OFFSET>,
            GetString::<Impl, IMPL_OFFSET>,
            SetString::<Impl, IMPL_OFFSET>,
            GetBytes::<Impl, IMPL_OFFSET>,
            SetBytes::<Impl, IMPL_OFFSET>,
            GetReadStream::<Impl, IMPL_OFFSET>,
            GetWriteStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueImpl: Sized + IDispatchImpl {
    fn Handle();
    fn Name();
    fn SendBidiQuery();
    fn GetProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueVtbl {
        unsafe extern "system" fn Handle<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendBidiQuery<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidiquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Handle::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, SendBidiQuery::<Impl, IMPL_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueue2Impl: Sized + IPrinterQueueImpl + IDispatchImpl {
    fn SendBidiSetRequestAsync();
    fn GetPrinterQueueView();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueue2Vtbl {
        unsafe extern "system" fn SendBidiSetRequestAsync<Impl: IPrinterQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidirequest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcallback: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrinterQueueView<Impl: IPrinterQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Handle::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SendBidiQuery::<Impl, IMPL_OFFSET>,
            GetProperties::<Impl, IMPL_OFFSET>,
            SendBidiSetRequestAsync::<Impl, IMPL_OFFSET>,
            GetPrinterQueueView::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueEventImpl: Sized + IDispatchImpl {
    fn OnBidiResponseReceived();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueEventVtbl {
        unsafe extern "system" fn OnBidiResponseReceived<Impl: IPrinterQueueEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OnBidiResponseReceived::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueViewImpl: Sized + IDispatchImpl {
    fn SetViewRange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueViewVtbl {
        unsafe extern "system" fn SetViewRange<Impl: IPrinterQueueViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, SetViewRange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueViewEventImpl: Sized + IDispatchImpl {
    fn OnChanged();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueViewEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueViewEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueViewEventVtbl {
        unsafe extern "system" fn OnChanged<Impl: IPrinterQueueViewEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcollection: ::windows::core::RawPtr, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OnChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueViewEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptContextImpl: Sized + IDispatchImpl {
    fn DriverProperties();
    fn QueueProperties();
    fn UserProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptContextVtbl {
        unsafe extern "system" fn DriverProperties<Impl: IPrinterScriptContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueProperties<Impl: IPrinterScriptContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserProperties<Impl: IPrinterScriptContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, DriverProperties::<Impl, IMPL_OFFSET>, QueueProperties::<Impl, IMPL_OFFSET>, UserProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptablePropertyBagImpl: Sized + IDispatchImpl {
    fn GetBool();
    fn SetBool();
    fn GetInt32();
    fn SetInt32();
    fn GetString();
    fn SetString();
    fn GetBytes();
    fn SetBytes();
    fn GetReadStream();
    fn GetWriteStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptablePropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBagImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptablePropertyBagVtbl {
        unsafe extern "system" fn GetBool<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBool<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInt32<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInt32<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetString<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetString<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBytes<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBytes<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parray: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadStream<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWriteStream<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetBool::<Impl, IMPL_OFFSET>,
            SetBool::<Impl, IMPL_OFFSET>,
            GetInt32::<Impl, IMPL_OFFSET>,
            SetInt32::<Impl, IMPL_OFFSET>,
            GetString::<Impl, IMPL_OFFSET>,
            SetString::<Impl, IMPL_OFFSET>,
            GetBytes::<Impl, IMPL_OFFSET>,
            SetBytes::<Impl, IMPL_OFFSET>,
            GetReadStream::<Impl, IMPL_OFFSET>,
            GetWriteStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptablePropertyBag2Impl: Sized + IPrinterScriptablePropertyBagImpl + IDispatchImpl {
    fn GetReadStreamAsXML();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptablePropertyBag2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptablePropertyBag2Vtbl {
        unsafe extern "system" fn GetReadStreamAsXML<Impl: IPrinterScriptablePropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetBool::<Impl, IMPL_OFFSET>,
            SetBool::<Impl, IMPL_OFFSET>,
            GetInt32::<Impl, IMPL_OFFSET>,
            SetInt32::<Impl, IMPL_OFFSET>,
            GetString::<Impl, IMPL_OFFSET>,
            SetString::<Impl, IMPL_OFFSET>,
            GetBytes::<Impl, IMPL_OFFSET>,
            SetBytes::<Impl, IMPL_OFFSET>,
            GetReadStream::<Impl, IMPL_OFFSET>,
            GetWriteStream::<Impl, IMPL_OFFSET>,
            GetReadStreamAsXML::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptableSequentialStreamImpl: Sized + IDispatchImpl {
    fn Read();
    fn Write();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptableSequentialStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableSequentialStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptableSequentialStreamVtbl {
        unsafe extern "system" fn Read<Impl: IPrinterScriptableSequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbread: i32, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IPrinterScriptableSequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: ::windows::core::RawPtr, pcbwritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptableSequentialStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptableStreamImpl: Sized + IPrinterScriptableSequentialStreamImpl + IDispatchImpl {
    fn Commit();
    fn Seek();
    fn SetSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptableStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptableStreamVtbl {
        unsafe extern "system" fn Commit<Impl: IPrinterScriptableStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Seek<Impl: IPrinterScriptableStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: IPrinterScriptableStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Seek::<Impl, IMPL_OFFSET>, SetSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptableStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactoryImpl: Sized {
    fn CreateRasterizer();
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizationFactoryVtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateRasterizer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory1Impl: Sized {
    fn CreateRasterizer();
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizationFactory1Vtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateRasterizer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory2Impl: Sized {
    fn CreateRasterizer();
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizationFactory2Vtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateRasterizer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IXpsRasterizerImpl: Sized {
    fn RasterizeRect();
    fn SetMinimalLineWidth();
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IXpsRasterizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizerVtbl {
        unsafe extern "system" fn RasterizeRect<Impl: IXpsRasterizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: ::windows::core::RawPtr, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinimalLineWidth<Impl: IXpsRasterizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RasterizeRect::<Impl, IMPL_OFFSET>, SetMinimalLineWidth::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizer as ::windows::core::Interface>::IID
    }
}
pub trait IXpsRasterizerNotificationCallbackImpl: Sized {
    fn Continue();
}
impl IXpsRasterizerNotificationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizerNotificationCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizerNotificationCallbackVtbl {
        unsafe extern "system" fn Continue<Impl: IXpsRasterizerNotificationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Continue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizerNotificationCallback as ::windows::core::Interface>::IID
    }
}
