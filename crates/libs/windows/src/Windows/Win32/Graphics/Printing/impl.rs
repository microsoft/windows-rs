#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSendNotificationCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData(&mut self, param0: ::core::option::Option<IPrintAsyncNotifyDataObject>, param1: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAsyncGetSendNotificationCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSendNotificationCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncGetSendNotificationCookieVtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IAsyncGetSendNotificationCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishAsyncCallWithData(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: IPrintAsyncCookieVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncGetSendNotificationCookie as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSrvReferralCookieImpl: Sized {
    fn FinishAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn CancelAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn FinishAsyncCallWithData(&mut self, param0: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAsyncGetSrvReferralCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSrvReferralCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncGetSrvReferralCookieVtbl {
        unsafe extern "system" fn FinishAsyncCall<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishAsyncCallWithData(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Impl, IMPL_OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Impl, IMPL_OFFSET>,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncGetSrvReferralCookie as ::windows::core::Interface>::IID
    }
}
pub trait IBidiAsyncNotifyChannelImpl: Sized + IPrintAsyncNotifyChannelImpl {
    fn CreateNotificationChannel(&mut self) -> ::windows::core::Result<()>;
    fn GetPrintName(&mut self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn GetChannelNotificationType(&mut self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn AsyncGetNotificationSendResponse(&mut self, param0: ::core::option::Option<IPrintAsyncNotifyDataObject>, param1: ::core::option::Option<IAsyncGetSendNotificationCookie>) -> ::windows::core::Result<()>;
    fn AsyncCloseChannel(&mut self, param0: ::core::option::Option<IPrintAsyncNotifyDataObject>, param1: ::core::option::Option<IPrintAsyncCookie>) -> ::windows::core::Result<()>;
}
impl IBidiAsyncNotifyChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBidiAsyncNotifyChannelVtbl {
        unsafe extern "system" fn CreateNotificationChannel<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateNotificationChannel().into()
        }
        unsafe extern "system" fn GetPrintName<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrintName(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetChannelNotificationType<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelNotificationType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncGetNotificationSendResponse(::core::mem::transmute(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn AsyncCloseChannel<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncCloseChannel(::core::mem::transmute(&param0), ::core::mem::transmute(&param1)).into()
        }
        Self {
            base: IPrintAsyncNotifyChannelVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateNotificationChannel: CreateNotificationChannel::<Impl, IMPL_OFFSET>,
            GetPrintName: GetPrintName::<Impl, IMPL_OFFSET>,
            GetChannelNotificationType: GetChannelNotificationType::<Impl, IMPL_OFFSET>,
            AsyncGetNotificationSendResponse: AsyncGetNotificationSendResponse::<Impl, IMPL_OFFSET>,
            AsyncCloseChannel: AsyncCloseChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBidiAsyncNotifyChannel as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncCookieImpl: Sized {
    fn FinishAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn CancelAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IPrintAsyncCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncCookieVtbl {
        unsafe extern "system" fn FinishAsyncCall<Impl: IPrintAsyncCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IPrintAsyncCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Impl, IMPL_OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncCookie as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNewChannelCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData(&mut self, param0: *const ::core::option::Option<IPrintAsyncNotifyChannel>, param1: u32) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNewChannelCookieVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNewChannelCookieImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNewChannelCookieVtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IPrintAsyncNewChannelCookieImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishAsyncCallWithData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: IPrintAsyncCookieVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNewChannelCookie as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyImpl: Sized {
    fn CreatePrintAsyncNotifyChannel(&mut self, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: ::core::option::Option<IPrintAsyncNotifyCallback>) -> ::windows::core::Result<IPrintAsyncNotifyChannel>;
    fn CreatePrintAsyncNotifyRegistration(&mut self, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: ::core::option::Option<IPrintAsyncNotifyCallback>) -> ::windows::core::Result<IPrintAsyncNotifyRegistration>;
}
impl IPrintAsyncNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyVtbl {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Impl: IPrintAsyncNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: ::windows::core::RawPtr, param5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePrintAsyncNotifyChannel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute(&param4)) {
                ::core::result::Result::Ok(ok__) => {
                    *param5 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Impl: IPrintAsyncNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: ::windows::core::RawPtr, param4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePrintAsyncNotifyRegistration(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)) {
                ::core::result::Result::Ok(ok__) => {
                    *param4 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePrintAsyncNotifyChannel: CreatePrintAsyncNotifyChannel::<Impl, IMPL_OFFSET>,
            CreatePrintAsyncNotifyRegistration: CreatePrintAsyncNotifyRegistration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotify as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyCallbackImpl: Sized {
    fn OnEventNotify(&mut self, pchannel: ::core::option::Option<IPrintAsyncNotifyChannel>, pdata: ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn ChannelClosed(&mut self, pchannel: ::core::option::Option<IPrintAsyncNotifyChannel>, pdata: ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyCallbackVtbl {
        unsafe extern "system" fn OnEventNotify<Impl: IPrintAsyncNotifyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEventNotify(::core::mem::transmute(&pchannel), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn ChannelClosed<Impl: IPrintAsyncNotifyCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChannelClosed(::core::mem::transmute(&pchannel), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnEventNotify: OnEventNotify::<Impl, IMPL_OFFSET>,
            ChannelClosed: ChannelClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyChannelImpl: Sized {
    fn SendNotification(&mut self, pdata: ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn CloseChannel(&mut self, pdata: ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyChannelVtbl {
        unsafe extern "system" fn SendNotification<Impl: IPrintAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendNotification(::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn CloseChannel<Impl: IPrintAsyncNotifyChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseChannel(::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SendNotification: SendNotification::<Impl, IMPL_OFFSET>,
            CloseChannel: CloseChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyChannel as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyDataObjectImpl: Sized {
    fn AcquireData(&mut self, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ReleaseData(&mut self) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyDataObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyDataObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyDataObjectVtbl {
        unsafe extern "system" fn AcquireData<Impl: IPrintAsyncNotifyDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireData(::core::mem::transmute_copy(&ppnotificationdata), ::core::mem::transmute_copy(&psize), ::core::mem::transmute_copy(&ppschema)).into()
        }
        unsafe extern "system" fn ReleaseData<Impl: IPrintAsyncNotifyDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseData().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AcquireData: AcquireData::<Impl, IMPL_OFFSET>,
            ReleaseData: ReleaseData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyDataObject as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyRegistrationImpl: Sized {
    fn RegisterForNotifications(&mut self) -> ::windows::core::Result<()>;
    fn UnregisterForNotifications(&mut self) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn RegisterForNotifications<Impl: IPrintAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterForNotifications().into()
        }
        unsafe extern "system" fn UnregisterForNotifications<Impl: IPrintAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterForNotifications().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterForNotifications: RegisterForNotifications::<Impl, IMPL_OFFSET>,
            UnregisterForNotifications: UnregisterForNotifications::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintAsyncNotifyServerReferralImpl: Sized {
    fn GetServerReferral(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn AsyncGetServerReferral(&mut self, param0: ::core::option::Option<IAsyncGetSrvReferralCookie>) -> ::windows::core::Result<()>;
    fn SetServerReferral(&mut self, prmtserverreferral: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintAsyncNotifyServerReferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyServerReferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintAsyncNotifyServerReferralVtbl {
        unsafe extern "system" fn GetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerReferral() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncGetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncGetServerReferral(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prmtserverreferral: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerReferral(::core::mem::transmute_copy(&prmtserverreferral)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetServerReferral: GetServerReferral::<Impl, IMPL_OFFSET>,
            AsyncGetServerReferral: AsyncGetServerReferral::<Impl, IMPL_OFFSET>,
            SetServerReferral: SetServerReferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyServerReferral as ::windows::core::Interface>::IID
    }
}
pub trait IPrintBidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNewChannel(&mut self, param0: ::core::option::Option<IPrintAsyncNewChannelCookie>) -> ::windows::core::Result<()>;
}
impl IPrintBidiAsyncNotifyRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintBidiAsyncNotifyRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintBidiAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn AsyncGetNewChannel<Impl: IPrintBidiAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncGetNewChannel(::core::mem::transmute(&param0)).into()
        }
        Self {
            base: IPrintAsyncNotifyRegistrationVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AsyncGetNewChannel: AsyncGetNewChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintBidiAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelperImpl: Sized {
    fn GetOption(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: super::super::Foundation::PSTR) -> ::windows::core::Result<super::super::Foundation::PSTR>;
    fn SetOptions(&mut self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn EnumConstrainedOptions(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pconstrainedoptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>;
    fn WhyConstrained(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>;
    fn EnumFeatures(&mut self, pfeaturelist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::Result<()>;
    fn EnumOptions(&mut self, pszfeaturekeyword: super::super::Foundation::PSTR, poptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>;
    fn GetFontSubstitution(&mut self, psztruetypefontname: super::super::Foundation::PWSTR, ppszdevfontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetFontSubstitution(&mut self, psztruetypefontname: super::super::Foundation::PWSTR, pszdevfontname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateInstanceOfMSXMLObject(&mut self, rclsid: *const ::windows::core::GUID, punkouter: ::core::option::Option<::windows::core::IUnknown>, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperVtbl {
        unsafe extern "system" fn GetOption<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: super::super::Foundation::PSTR, ppszoption: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOption(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszfeaturerequested)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptions(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&bresolveconflicts), ::core::mem::transmute_copy(&pfopairs), ::core::mem::transmute_copy(&cpairs), ::core::mem::transmute_copy(&pcpairswritten), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn EnumConstrainedOptions<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pconstrainedoptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumConstrainedOptions(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pconstrainedoptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn WhyConstrained<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WhyConstrained(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&ppfoconstraints), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn EnumFeatures<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeaturelist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumFeatures(::core::mem::transmute_copy(&pfeaturelist), ::core::mem::transmute_copy(&pdwnumfeatures)).into()
        }
        unsafe extern "system" fn EnumOptions<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, poptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumOptions(::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&poptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn GetFontSubstitution<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, ppszdevfontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFontSubstitution(::core::mem::transmute_copy(&psztruetypefontname), ::core::mem::transmute_copy(&ppszdevfontname)).into()
        }
        unsafe extern "system" fn SetFontSubstitution<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, pszdevfontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontSubstitution(::core::mem::transmute_copy(&psztruetypefontname), ::core::mem::transmute_copy(&pszdevfontname)).into()
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Impl: IPrintCoreHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstanceOfMSXMLObject(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOption: GetOption::<Impl, IMPL_OFFSET>,
            SetOptions: SetOptions::<Impl, IMPL_OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Impl, IMPL_OFFSET>,
            WhyConstrained: WhyConstrained::<Impl, IMPL_OFFSET>,
            EnumFeatures: EnumFeatures::<Impl, IMPL_OFFSET>,
            EnumOptions: EnumOptions::<Impl, IMPL_OFFSET>,
            GetFontSubstitution: GetFontSubstitution::<Impl, IMPL_OFFSET>,
            SetFontSubstitution: SetFontSubstitution::<Impl, IMPL_OFFSET>,
            CreateInstanceOfMSXMLObject: CreateInstanceOfMSXMLObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelperPSImpl: Sized + IPrintCoreHelperImpl {
    fn GetGlobalAttribute(&mut self, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetFeatureAttribute(&mut self, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetOptionAttribute(&mut self, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelperPSVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperPSImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperPSVtbl {
        unsafe extern "system" fn GetGlobalAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlobalAttribute(::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureAttribute(::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOptionAttribute(::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        Self {
            base: IPrintCoreHelperVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetGlobalAttribute: GetGlobalAttribute::<Impl, IMPL_OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Impl, IMPL_OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperPS as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUniImpl: Sized + IPrintCoreHelperImpl {
    fn CreateGDLSnapshot(&mut self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CreateDefaultGDLSnapshot(&mut self, dwflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUniVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUniImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperUniVtbl {
        unsafe extern "system" fn CreateGDLSnapshot<Impl: IPrintCoreHelperUniImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateGDLSnapshot(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppsnapshotstream)).into()
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Impl: IPrintCoreHelperUniImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDefaultGDLSnapshot(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsnapshotstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintCoreHelperVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateGDLSnapshot: CreateGDLSnapshot::<Impl, IMPL_OFFSET>,
            CreateDefaultGDLSnapshot: CreateDefaultGDLSnapshot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni2Impl: Sized + IPrintCoreHelperImpl + IPrintCoreHelperUniImpl {
    fn GetNamedCommand(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: super::super::Foundation::PWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUni2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreHelperUni2Vtbl {
        unsafe extern "system" fn GetNamedCommand<Impl: IPrintCoreHelperUni2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: super::super::Foundation::PWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNamedCommand(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszcommandname), ::core::mem::transmute_copy(&ppcommandbytes), ::core::mem::transmute_copy(&pcbcommandsize)).into()
        }
        Self { base: IPrintCoreHelperUniVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetNamedCommand: GetNamedCommand::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintCoreUI2Impl: Sized + IPrintOemDriverUIImpl {
    fn GetOptions(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn SetOptions(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32) -> ::windows::core::Result<u32>;
    fn EnumConstrainedOptions(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszconstrainedoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn WhyConstrained(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pmszreasonlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn GetGlobalAttribute(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn GetFeatureAttribute(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn GetOptionAttribute(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn EnumFeatures(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn EnumOptions(&mut self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn QuerySimulationSupport(&mut self, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintCoreUI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintCoreUI2Vtbl {
        unsafe extern "system" fn GetOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturesrequested), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn SetOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbin)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszconstrainedoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumConstrainedOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszconstrainedoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn WhyConstrained<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pmszreasonlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WhyConstrained(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&pmszreasonlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetGlobalAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlobalAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOptionAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumFeatures<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumFeatures(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturelist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumOptions<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn QuerySimulationSupport<Impl: IPrintCoreUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QuerySimulationSupport(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pcaps), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        Self {
            base: IPrintOemDriverUIVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOptions: GetOptions::<Impl, IMPL_OFFSET>,
            SetOptions: SetOptions::<Impl, IMPL_OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Impl, IMPL_OFFSET>,
            WhyConstrained: WhyConstrained::<Impl, IMPL_OFFSET>,
            GetGlobalAttribute: GetGlobalAttribute::<Impl, IMPL_OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Impl, IMPL_OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Impl, IMPL_OFFSET>,
            EnumFeatures: EnumFeatures::<Impl, IMPL_OFFSET>,
            EnumOptions: EnumOptions::<Impl, IMPL_OFFSET>,
            QuerySimulationSupport: QuerySimulationSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreUI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintJobImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn PrintedPages(&mut self) -> ::windows::core::Result<u32>;
    fn TotalPages(&mut self) -> ::windows::core::Result<u32>;
    fn Status(&mut self) -> ::windows::core::Result<PrintJobStatus>;
    fn SubmissionTime(&mut self) -> ::windows::core::Result<f64>;
    fn RequestCancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintJobVtbl {
        unsafe extern "system" fn Name<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintedPages<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintedPages() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalPages() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut PrintJobStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *psubmissiontime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCancel<Impl: IPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestCancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            PrintedPages: PrintedPages::<Impl, IMPL_OFFSET>,
            TotalPages: TotalPages::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            SubmissionTime: SubmissionTime::<Impl, IMPL_OFFSET>,
            RequestCancel: RequestCancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintJobCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, ulindex: u32) -> ::windows::core::Result<IPrintJob>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintJobCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintJobCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrintJobCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPrintJobCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrintJobCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintJobCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemCommonImpl: Sized {
    fn GetInfo(&mut self, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn DevMode(&mut self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemCommonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemCommonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemCommonVtbl {
        unsafe extern "system" fn GetInfo<Impl: IPrintOemCommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInfo(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn DevMode<Impl: IPrintOemCommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DevMode(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemdmparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
            DevMode: DevMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemCommon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintOemDriverUIImpl: Sized {
    fn DrvGetDriverSetting(&mut self, pci: *mut ::core::ffi::c_void, feature: super::super::Foundation::PSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::Result<()>;
    fn DrvUpgradeRegistrySetting(&mut self, hprinter: super::super::Foundation::HANDLE, pfeature: super::super::Foundation::PSTR, poption: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn DrvUpdateUISetting(&mut self, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintOemDriverUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemDriverUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemDriverUIVtbl {
        unsafe extern "system" fn DrvGetDriverSetting<Impl: IPrintOemDriverUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, feature: super::super::Foundation::PSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrvGetDriverSetting(::core::mem::transmute_copy(&pci), ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded), ::core::mem::transmute_copy(&pdwoptionsreturned)).into()
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Impl: IPrintOemDriverUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: super::super::Foundation::PSTR, poption: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrvUpgradeRegistrySetting(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pfeature), ::core::mem::transmute_copy(&poption)).into()
        }
        unsafe extern "system" fn DrvUpdateUISetting<Impl: IPrintOemDriverUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrvUpdateUISetting(::core::mem::transmute_copy(&pci), ::core::mem::transmute_copy(&poptitem), ::core::mem::transmute_copy(&dwpreviousselection), ::core::mem::transmute_copy(&dwmode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DrvGetDriverSetting: DrvGetDriverSetting::<Impl, IMPL_OFFSET>,
            DrvUpgradeRegistrySetting: DrvUpgradeRegistrySetting::<Impl, IMPL_OFFSET>,
            DrvUpdateUISetting: DrvUpdateUISetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemDriverUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUIImpl: Sized + IPrintOemCommonImpl {
    fn PublishDriverInterface(&mut self, piunknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CommonUIProp(&mut self, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::Result<()>;
    fn DocumentPropertySheets(&mut self, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn DevicePropertySheets(&mut self, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn DevQueryPrintEx(&mut self, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DeviceCapabilitiesA(&mut self, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::Result<()>;
    fn UpgradePrinter(&mut self, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::Result<()>;
    fn PrinterEvent(&mut self, pprintername: super::super::Foundation::PWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn DriverEvent(&mut self, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn QueryColorProfile(&mut self, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::Result<()>;
    fn FontInstallerDlgProc(&mut self, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn UpdateExternalFonts(&mut self, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemUIVtbl {
        unsafe extern "system" fn PublishDriverInterface<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PublishDriverInterface(::core::mem::transmute(&piunknown)).into()
        }
        unsafe extern "system" fn CommonUIProp<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommonUIProp(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemcuipparam)).into()
        }
        unsafe extern "system" fn DocumentPropertySheets<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DocumentPropertySheets(::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevicePropertySheets<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DevicePropertySheets(::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevQueryPrintEx<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DevQueryPrintEx(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&pdqpinfo), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm)).into()
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceCapabilitiesA(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pdevicename), ::core::mem::transmute_copy(&wcapability), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&dwold), ::core::mem::transmute_copy(&dwresult)).into()
        }
        unsafe extern "system" fn UpgradePrinter<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpgradePrinter(::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverupgradeinfo)).into()
        }
        unsafe extern "system" fn PrinterEvent<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintername: super::super::Foundation::PWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrinterEvent(::core::mem::transmute_copy(&pprintername), ::core::mem::transmute_copy(&idriverevent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DriverEvent<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DriverEvent(::core::mem::transmute_copy(&dwdriverevent), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn QueryColorProfile<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryColorProfile(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&ulquerymode), ::core::mem::transmute_copy(&pvprofiledata), ::core::mem::transmute_copy(&pcbprofiledata), ::core::mem::transmute_copy(&pflprofiledata)).into()
        }
        unsafe extern "system" fn FontInstallerDlgProc<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FontInstallerDlgProc(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&usmsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn UpdateExternalFonts<Impl: IPrintOemUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateExternalFonts(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hheap), ::core::mem::transmute_copy(&pwstrcartridges)).into()
        }
        Self {
            base: IPrintOemCommonVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PublishDriverInterface: PublishDriverInterface::<Impl, IMPL_OFFSET>,
            CommonUIProp: CommonUIProp::<Impl, IMPL_OFFSET>,
            DocumentPropertySheets: DocumentPropertySheets::<Impl, IMPL_OFFSET>,
            DevicePropertySheets: DevicePropertySheets::<Impl, IMPL_OFFSET>,
            DevQueryPrintEx: DevQueryPrintEx::<Impl, IMPL_OFFSET>,
            DeviceCapabilitiesA: DeviceCapabilitiesA::<Impl, IMPL_OFFSET>,
            UpgradePrinter: UpgradePrinter::<Impl, IMPL_OFFSET>,
            PrinterEvent: PrinterEvent::<Impl, IMPL_OFFSET>,
            DriverEvent: DriverEvent::<Impl, IMPL_OFFSET>,
            QueryColorProfile: QueryColorProfile::<Impl, IMPL_OFFSET>,
            FontInstallerDlgProc: FontInstallerDlgProc::<Impl, IMPL_OFFSET>,
            UpdateExternalFonts: UpdateExternalFonts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI2Impl: Sized + IPrintOemCommonImpl + IPrintOemUIImpl {
    fn QueryJobAttributes(&mut self, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::Result<()>;
    fn HideStandardUI(&mut self, dwmode: u32) -> ::windows::core::Result<()>;
    fn DocumentEvent(&mut self, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemUI2Vtbl {
        unsafe extern "system" fn QueryJobAttributes<Impl: IPrintOemUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryJobAttributes(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&lpattributeinfo)).into()
        }
        unsafe extern "system" fn HideStandardUI<Impl: IPrintOemUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HideStandardUI(::core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn DocumentEvent<Impl: IPrintOemUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DocumentEvent(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&iesc), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pvin), ::core::mem::transmute_copy(&cbout), ::core::mem::transmute_copy(&pvout), ::core::mem::transmute_copy(&piresult)).into()
        }
        Self {
            base: IPrintOemUIVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryJobAttributes: QueryJobAttributes::<Impl, IMPL_OFFSET>,
            HideStandardUI: HideStandardUI::<Impl, IMPL_OFFSET>,
            DocumentEvent: DocumentEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemUIMXDCImpl: Sized {
    fn AdjustImageableArea(&mut self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::Result<()>;
    fn AdjustImageCompression(&mut self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::Result<()>;
    fn AdjustDPI(&mut self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemUIMXDCVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIMXDCImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintOemUIMXDCVtbl {
        unsafe extern "system" fn AdjustImageableArea<Impl: IPrintOemUIMXDCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustImageableArea(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&prclimageablearea)).into()
        }
        unsafe extern "system" fn AdjustImageCompression<Impl: IPrintOemUIMXDCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustImageCompression(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pcompressionmode)).into()
        }
        unsafe extern "system" fn AdjustDPI<Impl: IPrintOemUIMXDCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdjustDPI(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pdpi)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdjustImageableArea: AdjustImageableArea::<Impl, IMPL_OFFSET>,
            AdjustImageCompression: AdjustImageCompression::<Impl, IMPL_OFFSET>,
            AdjustDPI: AdjustDPI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUIMXDC as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait IPrintPreviewDxgiPackageTargetImpl: Sized {
    fn SetJobPageCount(&mut self, counttype: PageCountType, count: u32) -> ::windows::core::Result<()>;
    fn DrawPage(&mut self, jobpagenumber: u32, pageimage: ::core::option::Option<super::Dxgi::IDXGISurface>, dpix: f32, dpiy: f32) -> ::windows::core::Result<()>;
    fn InvalidatePreview(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl IPrintPreviewDxgiPackageTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPreviewDxgiPackageTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPreviewDxgiPackageTargetVtbl {
        unsafe extern "system" fn SetJobPageCount<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counttype: PageCountType, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJobPageCount(::core::mem::transmute_copy(&counttype), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DrawPage<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, pageimage: ::windows::core::RawPtr, dpix: f32, dpiy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawPage(::core::mem::transmute_copy(&jobpagenumber), ::core::mem::transmute(&pageimage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn InvalidatePreview<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidatePreview().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetJobPageCount: SetJobPageCount::<Impl, IMPL_OFFSET>,
            DrawPage: DrawPage::<Impl, IMPL_OFFSET>,
            InvalidatePreview: InvalidatePreview::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintPreviewDxgiPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaAsyncOperationImpl: Sized + IDispatchImpl {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaAsyncOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaAsyncOperationVtbl {
        unsafe extern "system" fn Start<Impl: IPrintSchemaAsyncOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Cancel<Impl: IPrintSchemaAsyncOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Start: Start::<Impl, IMPL_OFFSET>, Cancel: Cancel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaAsyncOperationEventImpl: Sized + IDispatchImpl {
    fn Completed(&mut self, pticket: ::core::option::Option<IPrintSchemaTicket>, hroperation: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaAsyncOperationEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperationEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaAsyncOperationEventVtbl {
        unsafe extern "system" fn Completed<Impl: IPrintSchemaAsyncOperationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pticket: ::windows::core::RawPtr, hroperation: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Completed(::core::mem::transmute(&pticket), ::core::mem::transmute_copy(&hroperation)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Completed: Completed::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperationEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaCapabilitiesImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl {
    fn GetFeatureByKeyName(&mut self, bstrkeyname: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn GetFeature(&mut self, bstrname: super::super::Foundation::BSTR, bstrnamespaceuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn PageImageableSize(&mut self) -> ::windows::core::Result<IPrintSchemaPageImageableSize>;
    fn JobCopiesAllDocumentsMinValue(&mut self) -> ::windows::core::Result<u32>;
    fn JobCopiesAllDocumentsMaxValue(&mut self) -> ::windows::core::Result<u32>;
    fn GetSelectedOptionInPrintTicket(&mut self, pfeature: ::core::option::Option<IPrintSchemaFeature>) -> ::windows::core::Result<IPrintSchemaOption>;
    fn GetOptions(&mut self, pfeature: ::core::option::Option<IPrintSchemaFeature>) -> ::windows::core::Result<IPrintSchemaOptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaCapabilitiesVtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureByKeyName(::core::mem::transmute_copy(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeature(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageImageableSize<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppageimageablesize: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageImageableSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pppageimageablesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobCopiesAllDocumentsMinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *puljobcopiesalldocumentsminvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobCopiesAllDocumentsMaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    *puljobcopiesalldocumentsmaxvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectedOptionInPrintTicket(::core::mem::transmute(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptions<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoptioncollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptions(::core::mem::transmute(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptioncollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Impl, IMPL_OFFSET>,
            GetFeature: GetFeature::<Impl, IMPL_OFFSET>,
            PageImageableSize: PageImageableSize::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocumentsMinValue: JobCopiesAllDocumentsMinValue::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocumentsMaxValue: JobCopiesAllDocumentsMaxValue::<Impl, IMPL_OFFSET>,
            GetSelectedOptionInPrintTicket: GetSelectedOptionInPrintTicket::<Impl, IMPL_OFFSET>,
            GetOptions: GetOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaCapabilities2Impl: Sized + IDispatchImpl + IPrintSchemaElementImpl + IPrintSchemaCapabilitiesImpl {
    fn GetParameterDefinition(&mut self, bstrname: super::super::Foundation::BSTR, bstrnamespaceuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaParameterDefinition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaCapabilities2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaCapabilities2Vtbl {
        unsafe extern "system" fn GetParameterDefinition<Impl: IPrintSchemaCapabilities2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameterDefinition(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparameterdefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaCapabilitiesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetParameterDefinition: GetParameterDefinition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaDisplayableElementImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl {
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaDisplayableElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaDisplayableElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaDisplayableElementVtbl {
        unsafe extern "system" fn DisplayName<Impl: IPrintSchemaDisplayableElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPrintSchemaElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DisplayName: DisplayName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaElementImpl: Sized + IDispatchImpl {
    fn XmlNode(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn NamespaceUri(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaElementVtbl {
        unsafe extern "system" fn XmlNode<Impl: IPrintSchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppxmlnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IPrintSchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Impl: IPrintSchemaElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespaceuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamespaceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnamespaceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            XmlNode: XmlNode::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            NamespaceUri: NamespaceUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaFeatureImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl + IPrintSchemaDisplayableElementImpl {
    fn SelectedOption(&mut self) -> ::windows::core::Result<IPrintSchemaOption>;
    fn SetSelectedOption(&mut self, poption: ::core::option::Option<IPrintSchemaOption>) -> ::windows::core::Result<()>;
    fn SelectionType(&mut self) -> ::windows::core::Result<PrintSchemaSelectionType>;
    fn GetOption(&mut self, bstrname: super::super::Foundation::BSTR, bstrnamespaceuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaOption>;
    fn DisplayUI(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaFeatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaFeatureVtbl {
        unsafe extern "system" fn SelectedOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedOption() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poption: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedOption(::core::mem::transmute(&poption)).into()
        }
        unsafe extern "system" fn SelectionType<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionType() {
                ::core::result::Result::Ok(ok__) => {
                    *pselectiontype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOption(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: IPrintSchemaFeatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbshow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaDisplayableElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SelectedOption: SelectedOption::<Impl, IMPL_OFFSET>,
            SetSelectedOption: SetSelectedOption::<Impl, IMPL_OFFSET>,
            SelectionType: SelectionType::<Impl, IMPL_OFFSET>,
            GetOption: GetOption::<Impl, IMPL_OFFSET>,
            DisplayUI: DisplayUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaFeature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaNUpOptionImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaOptionImpl {
    fn PagesPerSheet(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaNUpOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaNUpOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaNUpOptionVtbl {
        unsafe extern "system" fn PagesPerSheet<Impl: IPrintSchemaNUpOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpagespersheet: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PagesPerSheet() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpagespersheet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPrintSchemaOptionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), PagesPerSheet: PagesPerSheet::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaNUpOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaOptionImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl + IPrintSchemaDisplayableElementImpl {
    fn Selected(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Constrained(&mut self) -> ::windows::core::Result<PrintSchemaConstrainedSetting>;
    fn GetPropertyValue(&mut self, bstrname: super::super::Foundation::BSTR, bstrnamespaceuri: super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaOptionVtbl {
        unsafe extern "system" fn Selected<Impl: IPrintSchemaOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisselected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Constrained<Impl: IPrintSchemaOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Constrained() {
                ::core::result::Result::Ok(ok__) => {
                    *psetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IPrintSchemaOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlvaluenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValue(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppxmlvaluenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaDisplayableElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Selected: Selected::<Impl, IMPL_OFFSET>,
            Constrained: Constrained::<Impl, IMPL_OFFSET>,
            GetPropertyValue: GetPropertyValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaOptionCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, ulindex: u32) -> ::windows::core::Result<IPrintSchemaOption>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaOptionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaOptionCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaOptionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaPageImageableSizeImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl {
    fn ImageableSizeWidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn ImageableSizeHeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn OriginWidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn OriginHeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn ExtentWidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn ExtentHeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaPageImageableSizeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSizeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaPageImageableSizeVtbl {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizewidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageableSizeWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulimageablesizewidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizeheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageableSizeHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulimageablesizeheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *puloriginwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *puloriginheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtentWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulextentwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtentHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulextentheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ImageableSizeWidthInMicrons: ImageableSizeWidthInMicrons::<Impl, IMPL_OFFSET>,
            ImageableSizeHeightInMicrons: ImageableSizeHeightInMicrons::<Impl, IMPL_OFFSET>,
            OriginWidthInMicrons: OriginWidthInMicrons::<Impl, IMPL_OFFSET>,
            OriginHeightInMicrons: OriginHeightInMicrons::<Impl, IMPL_OFFSET>,
            ExtentWidthInMicrons: ExtentWidthInMicrons::<Impl, IMPL_OFFSET>,
            ExtentHeightInMicrons: ExtentHeightInMicrons::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaPageImageableSize as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaPageMediaSizeOptionImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaOptionImpl {
    fn WidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn HeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaPageMediaSizeOptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageMediaSizeOptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaPageMediaSizeOptionVtbl {
        unsafe extern "system" fn WidthInMicrons<Impl: IPrintSchemaPageMediaSizeOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeightInMicrons<Impl: IPrintSchemaPageMediaSizeOptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaOptionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WidthInMicrons: WidthInMicrons::<Impl, IMPL_OFFSET>,
            HeightInMicrons: HeightInMicrons::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaPageMediaSizeOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaParameterDefinitionImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl + IPrintSchemaDisplayableElementImpl {
    fn UserInputRequired(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn UnitType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DataType(&mut self) -> ::windows::core::Result<PrintSchemaParameterDataType>;
    fn RangeMin(&mut self) -> ::windows::core::Result<i32>;
    fn RangeMax(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaParameterDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaParameterDefinitionVtbl {
        unsafe extern "system" fn UserInputRequired<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserInputRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisrequired = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnitType<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrunittype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnitType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrunittype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataType<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMin<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeMin() {
                ::core::result::Result::Ok(ok__) => {
                    *prangemin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMax<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeMax() {
                ::core::result::Result::Ok(ok__) => {
                    *prangemax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaDisplayableElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UserInputRequired: UserInputRequired::<Impl, IMPL_OFFSET>,
            UnitType: UnitType::<Impl, IMPL_OFFSET>,
            DataType: DataType::<Impl, IMPL_OFFSET>,
            RangeMin: RangeMin::<Impl, IMPL_OFFSET>,
            RangeMax: RangeMax::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaParameterDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaParameterInitializerImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl {
    fn Value(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&mut self, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaParameterInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterInitializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaParameterInitializerVtbl {
        unsafe extern "system" fn Value<Impl: IPrintSchemaParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPrintSchemaParameterInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&pvar)).into()
        }
        Self {
            base: IPrintSchemaElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaParameterInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaTicketImpl: Sized + IDispatchImpl + IPrintSchemaElementImpl {
    fn GetFeatureByKeyName(&mut self, bstrkeyname: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn GetFeature(&mut self, bstrname: super::super::Foundation::BSTR, bstrnamespaceuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn ValidateAsync(&mut self) -> ::windows::core::Result<IPrintSchemaAsyncOperation>;
    fn CommitAsync(&mut self, pprintticketcommit: ::core::option::Option<IPrintSchemaTicket>) -> ::windows::core::Result<IPrintSchemaAsyncOperation>;
    fn NotifyXmlChanged(&mut self) -> ::windows::core::Result<()>;
    fn GetCapabilities(&mut self) -> ::windows::core::Result<IPrintSchemaCapabilities>;
    fn JobCopiesAllDocuments(&mut self) -> ::windows::core::Result<u32>;
    fn SetJobCopiesAllDocuments(&mut self, uljobcopiesalldocuments: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaTicketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaTicketVtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeatureByKeyName(::core::mem::transmute_copy(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeature(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateAsync<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticketcommit: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAsync(::core::mem::transmute(&pprintticketcommit)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyXmlChanged<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyXmlChanged().into()
        }
        unsafe extern "system" fn GetCapabilities<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobCopiesAllDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *puljobcopiesalldocuments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Impl: IPrintSchemaTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uljobcopiesalldocuments: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJobCopiesAllDocuments(::core::mem::transmute_copy(&uljobcopiesalldocuments)).into()
        }
        Self {
            base: IPrintSchemaElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Impl, IMPL_OFFSET>,
            GetFeature: GetFeature::<Impl, IMPL_OFFSET>,
            ValidateAsync: ValidateAsync::<Impl, IMPL_OFFSET>,
            CommitAsync: CommitAsync::<Impl, IMPL_OFFSET>,
            NotifyXmlChanged: NotifyXmlChanged::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            JobCopiesAllDocuments: JobCopiesAllDocuments::<Impl, IMPL_OFFSET>,
            SetJobCopiesAllDocuments: SetJobCopiesAllDocuments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaTicket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaTicket2Impl: Sized + IDispatchImpl + IPrintSchemaElementImpl + IPrintSchemaTicketImpl {
    fn GetParameterInitializer(&mut self, bstrname: super::super::Foundation::BSTR, bstrnamespaceuri: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaParameterInitializer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaTicket2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSchemaTicket2Vtbl {
        unsafe extern "system" fn GetParameterInitializer<Impl: IPrintSchemaTicket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterinitializer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameterInitializer(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparameterinitializer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaTicketVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetParameterInitializer: GetParameterInitializer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaTicket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProviderImpl: Sized {
    fn GetSupportedVersions(&mut self, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::Result<()>;
    fn BindPrinter(&mut self, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryDeviceNamespace(&mut self, pdefaultnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ConvertPrintTicketToDevMode(&mut self, pprintticket: ::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::Result<()>;
    fn ConvertDevModeToPrintTicket(&mut self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: ::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<()>;
    fn GetPrintCapabilities(&mut self, pprintticket: ::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn ValidatePrintTicket(&mut self, pbaseticket: ::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketProviderVtbl {
        unsafe extern "system" fn GetSupportedVersions<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSupportedVersions(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&ppversions), ::core::mem::transmute_copy(&cversions)).into()
        }
        unsafe extern "system" fn BindPrinter<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindPrinter(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&version), ::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&pdevmodeflags), ::core::mem::transmute_copy(&cnamespaces), ::core::mem::transmute_copy(&ppnamespaces)).into()
        }
        unsafe extern "system" fn QueryDeviceNamespace<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryDeviceNamespace(::core::mem::transmute_copy(&pdefaultnamespace)).into()
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertPrintTicketToDevMode(::core::mem::transmute(&pprintticket), ::core::mem::transmute_copy(&cbdevmodein), ::core::mem::transmute_copy(&pdevmodein), ::core::mem::transmute_copy(&pcbdevmodeout), ::core::mem::transmute_copy(&ppdevmodeout)).into()
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertDevModeToPrintTicket(::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute(&pprintticket)).into()
        }
        unsafe extern "system" fn GetPrintCapabilities<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintCapabilities(::core::mem::transmute(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidatePrintTicket<Impl: IPrintTicketProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbaseticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidatePrintTicket(::core::mem::transmute(&pbaseticket)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSupportedVersions: GetSupportedVersions::<Impl, IMPL_OFFSET>,
            BindPrinter: BindPrinter::<Impl, IMPL_OFFSET>,
            QueryDeviceNamespace: QueryDeviceNamespace::<Impl, IMPL_OFFSET>,
            ConvertPrintTicketToDevMode: ConvertPrintTicketToDevMode::<Impl, IMPL_OFFSET>,
            ConvertDevModeToPrintTicket: ConvertDevModeToPrintTicket::<Impl, IMPL_OFFSET>,
            GetPrintCapabilities: GetPrintCapabilities::<Impl, IMPL_OFFSET>,
            ValidatePrintTicket: ValidatePrintTicket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider2Impl: Sized + IPrintTicketProviderImpl {
    fn GetPrintDeviceCapabilities(&mut self, pprintticket: ::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn GetPrintDeviceResources(&mut self, pszlocalename: super::super::Foundation::PWSTR, pprintticket: ::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTicketProvider2Vtbl {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Impl: IPrintTicketProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppdevicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintDeviceCapabilities(::core::mem::transmute(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevicecapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintDeviceResources<Impl: IPrintTicketProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalename: super::super::Foundation::PWSTR, pprintticket: ::windows::core::RawPtr, ppdeviceresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintDeviceResources(::core::mem::transmute_copy(&pszlocalename), ::core::mem::transmute(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeviceresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintTicketProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPrintDeviceCapabilities: GetPrintDeviceCapabilities::<Impl, IMPL_OFFSET>,
            GetPrintDeviceResources: GetPrintDeviceResources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketProvider2 as ::windows::core::Interface>::IID
    }
}
pub trait IPrintUnidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNotification(&mut self, param0: ::core::option::Option<IAsyncGetSendNotificationCookie>) -> ::windows::core::Result<()>;
}
impl IPrintUnidiAsyncNotifyRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintUnidiAsyncNotifyRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintUnidiAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn AsyncGetNotification<Impl: IPrintUnidiAsyncNotifyRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncGetNotification(::core::mem::transmute(&param0)).into()
        }
        Self {
            base: IPrintAsyncNotifyRegistrationVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AsyncGetNotification: AsyncGetNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintUnidiAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrinterBidiSetRequestCallbackImpl: Sized {
    fn Completed(&mut self, bstrresponse: super::super::Foundation::BSTR, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrinterBidiSetRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterBidiSetRequestCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterBidiSetRequestCallbackVtbl {
        unsafe extern "system" fn Completed<Impl: IPrinterBidiSetRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Completed(::core::mem::transmute_copy(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Completed: Completed::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterBidiSetRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPrinterExtensionAsyncOperationImpl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
impl IPrinterExtensionAsyncOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionAsyncOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionAsyncOperationVtbl {
        unsafe extern "system" fn Cancel<Impl: IPrinterExtensionAsyncOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Cancel: Cancel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionAsyncOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionContextImpl: Sized + IDispatchImpl {
    fn PrinterQueue(&mut self) -> ::windows::core::Result<IPrinterQueue>;
    fn PrintSchemaTicket(&mut self) -> ::windows::core::Result<IPrintSchemaTicket>;
    fn DriverProperties(&mut self) -> ::windows::core::Result<IPrinterPropertyBag>;
    fn UserProperties(&mut self) -> ::windows::core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionContextVtbl {
        unsafe extern "system" fn PrinterQueue<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintSchemaTicket<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppticket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintSchemaTicket() {
                ::core::result::Result::Ok(ok__) => {
                    *ppticket = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Impl: IPrinterExtensionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PrinterQueue: PrinterQueue::<Impl, IMPL_OFFSET>,
            PrintSchemaTicket: PrintSchemaTicket::<Impl, IMPL_OFFSET>,
            DriverProperties: DriverProperties::<Impl, IMPL_OFFSET>,
            UserProperties: UserProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionContextCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, ulindex: u32) -> ::windows::core::Result<IPrinterExtensionContext>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionContextCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionContextCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionContextCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionEventImpl: Sized + IDispatchImpl {
    fn OnDriverEvent(&mut self, peventargs: ::core::option::Option<IPrinterExtensionEventArgs>) -> ::windows::core::Result<()>;
    fn OnPrinterQueuesEnumerated(&mut self, pcontextcollection: ::core::option::Option<IPrinterExtensionContextCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionEventVtbl {
        unsafe extern "system" fn OnDriverEvent<Impl: IPrinterExtensionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDriverEvent(::core::mem::transmute(&peventargs)).into()
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Impl: IPrinterExtensionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPrinterQueuesEnumerated(::core::mem::transmute(&pcontextcollection)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnDriverEvent: OnDriverEvent::<Impl, IMPL_OFFSET>,
            OnPrinterQueuesEnumerated: OnPrinterQueuesEnumerated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionEventArgsImpl: Sized + IDispatchImpl + IPrinterExtensionContextImpl {
    fn BidiNotification(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ReasonId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Request(&mut self) -> ::windows::core::Result<IPrinterExtensionRequest>;
    fn SourceApplication(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DetailedReasonId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn WindowModal(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn WindowParent(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionEventArgsVtbl {
        unsafe extern "system" fn BidiNotification<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbidinotification: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BidiNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbidinotification = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReasonId<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReasonId() {
                ::core::result::Result::Ok(ok__) => {
                    *preasonid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *pprequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceApplication<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplication: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailedReasonId<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetailedreasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailedReasonId() {
                ::core::result::Result::Ok(ok__) => {
                    *pdetailedreasonid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowModal<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowModal() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmodal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowParent<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowParent() {
                ::core::result::Result::Ok(ok__) => {
                    *phwndparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrinterExtensionContextVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BidiNotification: BidiNotification::<Impl, IMPL_OFFSET>,
            ReasonId: ReasonId::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
            SourceApplication: SourceApplication::<Impl, IMPL_OFFSET>,
            DetailedReasonId: DetailedReasonId::<Impl, IMPL_OFFSET>,
            WindowModal: WindowModal::<Impl, IMPL_OFFSET>,
            WindowParent: WindowParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IPrinterExtensionManagerImpl: Sized {
    fn EnableEvents(&mut self, printerdriverid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DisableEvents(&mut self) -> ::windows::core::Result<()>;
}
impl IPrinterExtensionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionManagerVtbl {
        unsafe extern "system" fn EnableEvents<Impl: IPrinterExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printerdriverid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableEvents(::core::mem::transmute_copy(&printerdriverid)).into()
        }
        unsafe extern "system" fn DisableEvents<Impl: IPrinterExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableEvents().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnableEvents: EnableEvents::<Impl, IMPL_OFFSET>,
            DisableEvents: DisableEvents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionRequestImpl: Sized + IDispatchImpl {
    fn Cancel(&mut self, hrstatus: ::windows::core::HRESULT, bstrlogmessage: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterExtensionRequestVtbl {
        unsafe extern "system" fn Cancel<Impl: IPrinterExtensionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, bstrlogmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel(::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&bstrlogmessage)).into()
        }
        unsafe extern "system" fn Complete<Impl: IPrinterExtensionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterPropertyBagImpl: Sized + IDispatchImpl {
    fn GetBool(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&mut self, bstrname: super::super::Foundation::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInt32(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn SetInt32(&mut self, bstrname: super::super::Foundation::BSTR, nvalue: i32) -> ::windows::core::Result<()>;
    fn GetString(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetString(&mut self, bstrname: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBytes(&mut self, bstrname: super::super::Foundation::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetBytes(&mut self, bstrname: super::super::Foundation::BSTR, cbvalue: u32, pvalue: *const u8) -> ::windows::core::Result<()>;
    fn GetReadStream(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn GetWriteStream(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterPropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBagImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterPropertyBagVtbl {
        unsafe extern "system" fn GetBool<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBool(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBool(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt32(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInt32(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetString(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBytes(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&pcbvalue), ::core::mem::transmute_copy(&ppvalue)).into()
        }
        unsafe extern "system" fn SetBytes<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cbvalue: u32, pvalue: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBytes(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&cbvalue), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetReadStream<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Impl: IPrinterPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWriteStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetBool: GetBool::<Impl, IMPL_OFFSET>,
            SetBool: SetBool::<Impl, IMPL_OFFSET>,
            GetInt32: GetInt32::<Impl, IMPL_OFFSET>,
            SetInt32: SetInt32::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            SetString: SetString::<Impl, IMPL_OFFSET>,
            GetBytes: GetBytes::<Impl, IMPL_OFFSET>,
            SetBytes: SetBytes::<Impl, IMPL_OFFSET>,
            GetReadStream: GetReadStream::<Impl, IMPL_OFFSET>,
            GetWriteStream: GetWriteStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueImpl: Sized + IDispatchImpl {
    fn Handle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SendBidiQuery(&mut self, bstrbidiquery: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetProperties(&mut self) -> ::windows::core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueVtbl {
        unsafe extern "system" fn Handle<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phprinter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendBidiQuery<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidiquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendBidiQuery(::core::mem::transmute_copy(&bstrbidiquery)).into()
        }
        unsafe extern "system" fn GetProperties<Impl: IPrinterQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Handle: Handle::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SendBidiQuery: SendBidiQuery::<Impl, IMPL_OFFSET>,
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueue2Impl: Sized + IDispatchImpl + IPrinterQueueImpl {
    fn SendBidiSetRequestAsync(&mut self, bstrbidirequest: super::super::Foundation::BSTR, pcallback: ::core::option::Option<IPrinterBidiSetRequestCallback>) -> ::windows::core::Result<IPrinterExtensionAsyncOperation>;
    fn GetPrinterQueueView(&mut self, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::Result<IPrinterQueueView>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueue2Vtbl {
        unsafe extern "system" fn SendBidiSetRequestAsync<Impl: IPrinterQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidirequest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcallback: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendBidiSetRequestAsync(::core::mem::transmute_copy(&bstrbidirequest), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrinterQueueView<Impl: IPrinterQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrinterQueueView(::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjobview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrinterQueueVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SendBidiSetRequestAsync: SendBidiSetRequestAsync::<Impl, IMPL_OFFSET>,
            GetPrinterQueueView: GetPrinterQueueView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueEventImpl: Sized + IDispatchImpl {
    fn OnBidiResponseReceived(&mut self, bstrresponse: super::super::Foundation::BSTR, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueEventVtbl {
        unsafe extern "system" fn OnBidiResponseReceived<Impl: IPrinterQueueEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBidiResponseReceived(::core::mem::transmute_copy(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnBidiResponseReceived: OnBidiResponseReceived::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueViewImpl: Sized + IDispatchImpl {
    fn SetViewRange(&mut self, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueViewVtbl {
        unsafe extern "system" fn SetViewRange<Impl: IPrinterQueueViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewRange(::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetViewRange: SetViewRange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueViewEventImpl: Sized + IDispatchImpl {
    fn OnChanged(&mut self, pcollection: ::core::option::Option<IPrintJobCollection>, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueViewEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueViewEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterQueueViewEventVtbl {
        unsafe extern "system" fn OnChanged<Impl: IPrinterQueueViewEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcollection: ::windows::core::RawPtr, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChanged(::core::mem::transmute(&pcollection), ::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize), ::core::mem::transmute_copy(&ulcountjobsinprintqueue)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnChanged: OnChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueViewEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptContextImpl: Sized + IDispatchImpl {
    fn DriverProperties(&mut self) -> ::windows::core::Result<IPrinterScriptablePropertyBag>;
    fn QueueProperties(&mut self) -> ::windows::core::Result<IPrinterScriptablePropertyBag>;
    fn UserProperties(&mut self) -> ::windows::core::Result<IPrinterScriptablePropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptContextVtbl {
        unsafe extern "system" fn DriverProperties<Impl: IPrinterScriptContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueProperties<Impl: IPrinterScriptContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Impl: IPrinterScriptContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DriverProperties: DriverProperties::<Impl, IMPL_OFFSET>,
            QueueProperties: QueueProperties::<Impl, IMPL_OFFSET>,
            UserProperties: UserProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptablePropertyBagImpl: Sized + IDispatchImpl {
    fn GetBool(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&mut self, bstrname: super::super::Foundation::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInt32(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn SetInt32(&mut self, bstrname: super::super::Foundation::BSTR, nvalue: i32) -> ::windows::core::Result<()>;
    fn GetString(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetString(&mut self, bstrname: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBytes(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetBytes(&mut self, bstrname: super::super::Foundation::BSTR, parray: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn GetReadStream(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrinterScriptableStream>;
    fn GetWriteStream(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<IPrinterScriptableStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptablePropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBagImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptablePropertyBagVtbl {
        unsafe extern "system" fn GetBool<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBool(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBool(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt32(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInt32(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetString(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBytes(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pparray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytes<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parray: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBytes(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute(&parray)).into()
        }
        unsafe extern "system" fn GetReadStream<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWriteStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetBool: GetBool::<Impl, IMPL_OFFSET>,
            SetBool: SetBool::<Impl, IMPL_OFFSET>,
            GetInt32: GetInt32::<Impl, IMPL_OFFSET>,
            SetInt32: SetInt32::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            SetString: SetString::<Impl, IMPL_OFFSET>,
            GetBytes: GetBytes::<Impl, IMPL_OFFSET>,
            SetBytes: SetBytes::<Impl, IMPL_OFFSET>,
            GetReadStream: GetReadStream::<Impl, IMPL_OFFSET>,
            GetWriteStream: GetWriteStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptablePropertyBag2Impl: Sized + IDispatchImpl + IPrinterScriptablePropertyBagImpl {
    fn GetReadStreamAsXML(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptablePropertyBag2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptablePropertyBag2Vtbl {
        unsafe extern "system" fn GetReadStreamAsXML<Impl: IPrinterScriptablePropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadStreamAsXML(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppxmlnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrinterScriptablePropertyBagVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetReadStreamAsXML: GetReadStreamAsXML::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptableSequentialStreamImpl: Sized + IDispatchImpl {
    fn Read(&mut self, cbread: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Write(&mut self, parray: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptableSequentialStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableSequentialStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptableSequentialStreamVtbl {
        unsafe extern "system" fn Read<Impl: IPrinterScriptableSequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbread: i32, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&cbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *pparray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IPrinterScriptableSequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: ::windows::core::RawPtr, pcbwritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(::core::mem::transmute(&parray)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Read: Read::<Impl, IMPL_OFFSET>, Write: Write::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptableSequentialStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptableStreamImpl: Sized + IDispatchImpl + IPrinterScriptableSequentialStreamImpl {
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn Seek(&mut self, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<i32>;
    fn SetSize(&mut self, lsize: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptableStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinterScriptableStreamVtbl {
        unsafe extern "system" fn Commit<Impl: IPrinterScriptableStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn Seek<Impl: IPrinterScriptableStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&streamseek)) {
                ::core::result::Result::Ok(ok__) => {
                    *plposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IPrinterScriptableStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&lsize)).into()
        }
        Self {
            base: IPrinterScriptableSequentialStreamVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptableStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactoryImpl: Sized {
    fn CreateRasterizer(&mut self, xpspage: ::core::option::Option<super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> ::windows::core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizationFactoryVtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizer(::core::mem::transmute(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppixpsrasterizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateRasterizer: CreateRasterizer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory1Impl: Sized {
    fn CreateRasterizer(&mut self, xpspage: ::core::option::Option<super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> ::windows::core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizationFactory1Vtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizer(::core::mem::transmute(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppixpsrasterizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateRasterizer: CreateRasterizer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory2Impl: Sized {
    fn CreateRasterizer(&mut self, xpspage: ::core::option::Option<super::super::Storage::Xps::IXpsOMPage>, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> ::windows::core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizationFactory2Vtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRasterizer(::core::mem::transmute(&xpspage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&backgroundcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppixpsrasterizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateRasterizer: CreateRasterizer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IXpsRasterizerImpl: Sized {
    fn RasterizeRect(&mut self, x: i32, y: i32, width: i32, height: i32, notificationcallback: ::core::option::Option<IXpsRasterizerNotificationCallback>) -> ::windows::core::Result<super::Imaging::IWICBitmap>;
    fn SetMinimalLineWidth(&mut self, width: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IXpsRasterizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizerVtbl {
        unsafe extern "system" fn RasterizeRect<Impl: IXpsRasterizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: ::windows::core::RawPtr, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RasterizeRect(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute(&notificationcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *bitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimalLineWidth<Impl: IXpsRasterizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimalLineWidth(::core::mem::transmute_copy(&width)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RasterizeRect: RasterizeRect::<Impl, IMPL_OFFSET>,
            SetMinimalLineWidth: SetMinimalLineWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizer as ::windows::core::Interface>::IID
    }
}
pub trait IXpsRasterizerNotificationCallbackImpl: Sized {
    fn Continue(&mut self) -> ::windows::core::Result<()>;
}
impl IXpsRasterizerNotificationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizerNotificationCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXpsRasterizerNotificationCallbackVtbl {
        unsafe extern "system" fn Continue<Impl: IXpsRasterizerNotificationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Continue().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Continue: Continue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizerNotificationCallback as ::windows::core::Interface>::IID
    }
}
