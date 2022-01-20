#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSendNotificationCookie_Impl: Sized + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&mut self, param0: &::core::option::Option<IPrintAsyncNotifyDataObject>, param1: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAsyncGetSendNotificationCookie_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSendNotificationCookie_Impl, const OFFSET: isize>() -> IAsyncGetSendNotificationCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSendNotificationCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishAsyncCallWithData(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self { base: IPrintAsyncCookie_Vtbl::new::<Identity, Impl, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncGetSendNotificationCookie as ::windows::core::Interface>::IID || iid == &<IPrintAsyncCookie as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSrvReferralCookie_Impl: Sized {
    fn FinishAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn CancelAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn FinishAsyncCallWithData(&mut self, param0: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAsyncGetSrvReferralCookie_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>() -> IAsyncGetSrvReferralCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCall<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishAsyncCallWithData(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncGetSrvReferralCookie as ::windows::core::Interface>::IID
    }
}
pub trait IBidiAsyncNotifyChannel_Impl: Sized + IPrintAsyncNotifyChannel_Impl {
    fn CreateNotificationChannel(&mut self) -> ::windows::core::Result<()>;
    fn GetPrintName(&mut self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn GetChannelNotificationType(&mut self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn AsyncGetNotificationSendResponse(&mut self, param0: &::core::option::Option<IPrintAsyncNotifyDataObject>, param1: &::core::option::Option<IAsyncGetSendNotificationCookie>) -> ::windows::core::Result<()>;
    fn AsyncCloseChannel(&mut self, param0: &::core::option::Option<IPrintAsyncNotifyDataObject>, param1: &::core::option::Option<IPrintAsyncCookie>) -> ::windows::core::Result<()>;
}
impl IBidiAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>() -> IBidiAsyncNotifyChannel_Vtbl {
        unsafe extern "system" fn CreateNotificationChannel<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNotificationChannel().into()
        }
        unsafe extern "system" fn GetPrintName<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrintName(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetChannelNotificationType<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChannelNotificationType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncGetNotificationSendResponse(::core::mem::transmute(&param0), ::core::mem::transmute(&param1)).into()
        }
        unsafe extern "system" fn AsyncCloseChannel<Identity: ::windows::core::IUnknownImpl, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncCloseChannel(::core::mem::transmute(&param0), ::core::mem::transmute(&param1)).into()
        }
        Self {
            base: IPrintAsyncNotifyChannel_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateNotificationChannel: CreateNotificationChannel::<Identity, Impl, OFFSET>,
            GetPrintName: GetPrintName::<Identity, Impl, OFFSET>,
            GetChannelNotificationType: GetChannelNotificationType::<Identity, Impl, OFFSET>,
            AsyncGetNotificationSendResponse: AsyncGetNotificationSendResponse::<Identity, Impl, OFFSET>,
            AsyncCloseChannel: AsyncCloseChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBidiAsyncNotifyChannel as ::windows::core::Interface>::IID || iid == &<IPrintAsyncNotifyChannel as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncCookie_Impl: Sized {
    fn FinishAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn CancelAsyncCall(&mut self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IPrintAsyncCookie_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncCookie_Impl, const OFFSET: isize>() -> IPrintAsyncCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCall<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelAsyncCall(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncCookie as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNewChannelCookie_Impl: Sized + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&mut self, param0: *const ::core::option::Option<IPrintAsyncNotifyChannel>, param1: u32) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNewChannelCookie_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNewChannelCookie_Impl, const OFFSET: isize>() -> IPrintAsyncNewChannelCookie_Vtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNewChannelCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishAsyncCallWithData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self { base: IPrintAsyncCookie_Vtbl::new::<Identity, Impl, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNewChannelCookie as ::windows::core::Interface>::IID || iid == &<IPrintAsyncCookie as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotify_Impl: Sized {
    fn CreatePrintAsyncNotifyChannel(&mut self, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: &::core::option::Option<IPrintAsyncNotifyCallback>) -> ::windows::core::Result<IPrintAsyncNotifyChannel>;
    fn CreatePrintAsyncNotifyRegistration(&mut self, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: &::core::option::Option<IPrintAsyncNotifyCallback>) -> ::windows::core::Result<IPrintAsyncNotifyRegistration>;
}
impl IPrintAsyncNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotify_Impl, const OFFSET: isize>() -> IPrintAsyncNotify_Vtbl {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: ::windows::core::RawPtr, param5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePrintAsyncNotifyChannel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute(&param4)) {
                ::core::result::Result::Ok(ok__) => {
                    *param5 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: ::windows::core::RawPtr, param4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePrintAsyncNotifyRegistration(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)) {
                ::core::result::Result::Ok(ok__) => {
                    *param4 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreatePrintAsyncNotifyChannel: CreatePrintAsyncNotifyChannel::<Identity, Impl, OFFSET>,
            CreatePrintAsyncNotifyRegistration: CreatePrintAsyncNotifyRegistration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotify as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyCallback_Impl: Sized {
    fn OnEventNotify(&mut self, pchannel: &::core::option::Option<IPrintAsyncNotifyChannel>, pdata: &::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn ChannelClosed(&mut self, pchannel: &::core::option::Option<IPrintAsyncNotifyChannel>, pdata: &::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyCallback_Vtbl {
        unsafe extern "system" fn OnEventNotify<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEventNotify(::core::mem::transmute(&pchannel), ::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn ChannelClosed<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChannelClosed(::core::mem::transmute(&pchannel), ::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnEventNotify: OnEventNotify::<Identity, Impl, OFFSET>,
            ChannelClosed: ChannelClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyChannel_Impl: Sized {
    fn SendNotification(&mut self, pdata: &::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
    fn CloseChannel(&mut self, pdata: &::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyChannel_Vtbl {
        unsafe extern "system" fn SendNotification<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendNotification(::core::mem::transmute(&pdata)).into()
        }
        unsafe extern "system" fn CloseChannel<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseChannel(::core::mem::transmute(&pdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SendNotification: SendNotification::<Identity, Impl, OFFSET>,
            CloseChannel: CloseChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyChannel as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyDataObject_Impl: Sized {
    fn AcquireData(&mut self, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ReleaseData(&mut self) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyDataObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyDataObject_Vtbl {
        unsafe extern "system" fn AcquireData<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireData(::core::mem::transmute_copy(&ppnotificationdata), ::core::mem::transmute_copy(&psize), ::core::mem::transmute_copy(&ppschema)).into()
        }
        unsafe extern "system" fn ReleaseData<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseData().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AcquireData: AcquireData::<Identity, Impl, OFFSET>,
            ReleaseData: ReleaseData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyDataObject as ::windows::core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyRegistration_Impl: Sized {
    fn RegisterForNotifications(&mut self) -> ::windows::core::Result<()>;
    fn UnregisterForNotifications(&mut self) -> ::windows::core::Result<()>;
}
impl IPrintAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyRegistration_Vtbl {
        unsafe extern "system" fn RegisterForNotifications<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterForNotifications().into()
        }
        unsafe extern "system" fn UnregisterForNotifications<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterForNotifications().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterForNotifications: RegisterForNotifications::<Identity, Impl, OFFSET>,
            UnregisterForNotifications: UnregisterForNotifications::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintAsyncNotifyServerReferral_Impl: Sized {
    fn GetServerReferral(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn AsyncGetServerReferral(&mut self, param0: &::core::option::Option<IAsyncGetSrvReferralCookie>) -> ::windows::core::Result<()>;
    fn SetServerReferral(&mut self, prmtserverreferral: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintAsyncNotifyServerReferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>() -> IPrintAsyncNotifyServerReferral_Vtbl {
        unsafe extern "system" fn GetServerReferral<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetServerReferral() {
                ::core::result::Result::Ok(ok__) => {
                    *param0 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncGetServerReferral<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncGetServerReferral(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn SetServerReferral<Identity: ::windows::core::IUnknownImpl, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prmtserverreferral: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServerReferral(::core::mem::transmute_copy(&prmtserverreferral)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetServerReferral: GetServerReferral::<Identity, Impl, OFFSET>,
            AsyncGetServerReferral: AsyncGetServerReferral::<Identity, Impl, OFFSET>,
            SetServerReferral: SetServerReferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyServerReferral as ::windows::core::Interface>::IID
    }
}
pub trait IPrintBidiAsyncNotifyRegistration_Impl: Sized + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNewChannel(&mut self, param0: &::core::option::Option<IPrintAsyncNewChannelCookie>) -> ::windows::core::Result<()>;
}
impl IPrintBidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> IPrintBidiAsyncNotifyRegistration_Vtbl {
        unsafe extern "system" fn AsyncGetNewChannel<Identity: ::windows::core::IUnknownImpl, Impl: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncGetNewChannel(::core::mem::transmute(&param0)).into()
        }
        Self { base: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, Impl, OFFSET>(), AsyncGetNewChannel: AsyncGetNewChannel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintBidiAsyncNotifyRegistration as ::windows::core::Interface>::IID || iid == &<IPrintAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelper_Impl: Sized {
    fn GetOption(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: super::super::Foundation::PSTR) -> ::windows::core::Result<super::super::Foundation::PSTR>;
    fn SetOptions(&mut self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn EnumConstrainedOptions(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pconstrainedoptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>;
    fn WhyConstrained(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>;
    fn EnumFeatures(&mut self, pfeaturelist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::Result<()>;
    fn EnumOptions(&mut self, pszfeaturekeyword: super::super::Foundation::PSTR, poptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>;
    fn GetFontSubstitution(&mut self, psztruetypefontname: super::super::Foundation::PWSTR, ppszdevfontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetFontSubstitution(&mut self, psztruetypefontname: super::super::Foundation::PWSTR, pszdevfontname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateInstanceOfMSXMLObject(&mut self, rclsid: *const ::windows::core::GUID, punkouter: &::core::option::Option<::windows::core::IUnknown>, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>() -> IPrintCoreHelper_Vtbl {
        unsafe extern "system" fn GetOption<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: super::super::Foundation::PSTR, ppszoption: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOption(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszfeaturerequested)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOptions(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&bresolveconflicts), ::core::mem::transmute_copy(&pfopairs), ::core::mem::transmute_copy(&cpairs), ::core::mem::transmute_copy(&pcpairswritten), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pconstrainedoptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumConstrainedOptions(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pconstrainedoptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn WhyConstrained<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WhyConstrained(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&ppfoconstraints), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn EnumFeatures<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeaturelist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumFeatures(::core::mem::transmute_copy(&pfeaturelist), ::core::mem::transmute_copy(&pdwnumfeatures)).into()
        }
        unsafe extern "system" fn EnumOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, poptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumOptions(::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&poptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn GetFontSubstitution<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, ppszdevfontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFontSubstitution(::core::mem::transmute_copy(&psztruetypefontname), ::core::mem::transmute_copy(&ppszdevfontname)).into()
        }
        unsafe extern "system" fn SetFontSubstitution<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, pszdevfontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFontSubstitution(::core::mem::transmute_copy(&psztruetypefontname), ::core::mem::transmute_copy(&pszdevfontname)).into()
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateInstanceOfMSXMLObject(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, Impl, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, Impl, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, Impl, OFFSET>,
            EnumOptions: EnumOptions::<Identity, Impl, OFFSET>,
            GetFontSubstitution: GetFontSubstitution::<Identity, Impl, OFFSET>,
            SetFontSubstitution: SetFontSubstitution::<Identity, Impl, OFFSET>,
            CreateInstanceOfMSXMLObject: CreateInstanceOfMSXMLObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelperPS_Impl: Sized + IPrintCoreHelper_Impl {
    fn GetGlobalAttribute(&mut self, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetFeatureAttribute(&mut self, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetOptionAttribute(&mut self, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintCoreHelperPS_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>() -> IPrintCoreHelperPS_Vtbl {
        unsafe extern "system" fn GetGlobalAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGlobalAttribute(::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeatureAttribute(::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperPS_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOptionAttribute(::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        Self {
            base: IPrintCoreHelper_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGlobalAttribute: GetGlobalAttribute::<Identity, Impl, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, Impl, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperPS as ::windows::core::Interface>::IID || iid == &<IPrintCoreHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni_Impl: Sized + IPrintCoreHelper_Impl {
    fn CreateGDLSnapshot(&mut self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CreateDefaultGDLSnapshot(&mut self, dwflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUni_Impl, const OFFSET: isize>() -> IPrintCoreHelperUni_Vtbl {
        unsafe extern "system" fn CreateGDLSnapshot<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUni_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateGDLSnapshot(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppsnapshotstream)).into()
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUni_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDefaultGDLSnapshot(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsnapshotstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintCoreHelper_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateGDLSnapshot: CreateGDLSnapshot::<Identity, Impl, OFFSET>,
            CreateDefaultGDLSnapshot: CreateDefaultGDLSnapshot::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni as ::windows::core::Interface>::IID || iid == &<IPrintCoreHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni2_Impl: Sized + IPrintCoreHelper_Impl + IPrintCoreHelperUni_Impl {
    fn GetNamedCommand(&mut self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: super::super::Foundation::PWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUni2_Impl, const OFFSET: isize>() -> IPrintCoreHelperUni2_Vtbl {
        unsafe extern "system" fn GetNamedCommand<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreHelperUni2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: super::super::Foundation::PWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNamedCommand(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pszcommandname), ::core::mem::transmute_copy(&ppcommandbytes), ::core::mem::transmute_copy(&pcbcommandsize)).into()
        }
        Self { base: IPrintCoreHelperUni_Vtbl::new::<Identity, Impl, OFFSET>(), GetNamedCommand: GetNamedCommand::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni2 as ::windows::core::Interface>::IID || iid == &<IPrintCoreHelper as ::windows::core::Interface>::IID || iid == &<IPrintCoreHelperUni as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintCoreUI2_Impl: Sized + IPrintOemDriverUI_Impl {
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
impl IPrintCoreUI2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>() -> IPrintCoreUI2_Vtbl {
        unsafe extern "system" fn GetOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturesrequested), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbin)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszconstrainedoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumConstrainedOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszconstrainedoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn WhyConstrained<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pmszreasonlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WhyConstrained(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&pmszreasonlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetGlobalAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGlobalAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFeatureAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOptionAttribute(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pszoptionkeyword), ::core::mem::transmute_copy(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumFeatures<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumFeatures(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturelist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumOptions(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn QuerySimulationSupport<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCoreUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QuerySimulationSupport(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pcaps), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        Self {
            base: IPrintOemDriverUI_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, Impl, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, Impl, OFFSET>,
            GetGlobalAttribute: GetGlobalAttribute::<Identity, Impl, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, Impl, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, Impl, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, Impl, OFFSET>,
            EnumOptions: EnumOptions::<Identity, Impl, OFFSET>,
            QuerySimulationSupport: QuerySimulationSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintCoreUI2 as ::windows::core::Interface>::IID || iid == &<IPrintOemDriverUI as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintJob_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn PrintedPages(&mut self) -> ::windows::core::Result<u32>;
    fn TotalPages(&mut self) -> ::windows::core::Result<u32>;
    fn Status(&mut self) -> ::windows::core::Result<PrintJobStatus>;
    fn SubmissionTime(&mut self) -> ::windows::core::Result<f64>;
    fn RequestCancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>() -> IPrintJob_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintedPages<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrintedPages() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalPages() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut PrintJobStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubmissionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *psubmissiontime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCancel<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestCancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            PrintedPages: PrintedPages::<Identity, Impl, OFFSET>,
            TotalPages: TotalPages::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            RequestCancel: RequestCancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintJobCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, ulindex: u32) -> ::windows::core::Result<IPrintJob>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintJobCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobCollection_Impl, const OFFSET: isize>() -> IPrintJobCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IPrintJobCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintJobCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemCommon_Impl: Sized {
    fn GetInfo(&mut self, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()>;
    fn DevMode(&mut self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemCommon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemCommon_Impl, const OFFSET: isize>() -> IPrintOemCommon_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemCommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInfo(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn DevMode<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemCommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DevMode(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemdmparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            DevMode: DevMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemCommon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintOemDriverUI_Impl: Sized {
    fn DrvGetDriverSetting(&mut self, pci: *mut ::core::ffi::c_void, feature: super::super::Foundation::PSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::Result<()>;
    fn DrvUpgradeRegistrySetting(&mut self, hprinter: super::super::Foundation::HANDLE, pfeature: super::super::Foundation::PSTR, poption: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn DrvUpdateUISetting(&mut self, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintOemDriverUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>() -> IPrintOemDriverUI_Vtbl {
        unsafe extern "system" fn DrvGetDriverSetting<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, feature: super::super::Foundation::PSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrvGetDriverSetting(::core::mem::transmute_copy(&pci), ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded), ::core::mem::transmute_copy(&pdwoptionsreturned)).into()
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: super::super::Foundation::PSTR, poption: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrvUpgradeRegistrySetting(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pfeature), ::core::mem::transmute_copy(&poption)).into()
        }
        unsafe extern "system" fn DrvUpdateUISetting<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemDriverUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrvUpdateUISetting(::core::mem::transmute_copy(&pci), ::core::mem::transmute_copy(&poptitem), ::core::mem::transmute_copy(&dwpreviousselection), ::core::mem::transmute_copy(&dwmode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DrvGetDriverSetting: DrvGetDriverSetting::<Identity, Impl, OFFSET>,
            DrvUpgradeRegistrySetting: DrvUpgradeRegistrySetting::<Identity, Impl, OFFSET>,
            DrvUpdateUISetting: DrvUpdateUISetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemDriverUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI_Impl: Sized + IPrintOemCommon_Impl {
    fn PublishDriverInterface(&mut self, piunknown: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
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
impl IPrintOemUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>() -> IPrintOemUI_Vtbl {
        unsafe extern "system" fn PublishDriverInterface<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PublishDriverInterface(::core::mem::transmute(&piunknown)).into()
        }
        unsafe extern "system" fn CommonUIProp<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommonUIProp(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemcuipparam)).into()
        }
        unsafe extern "system" fn DocumentPropertySheets<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DocumentPropertySheets(::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevicePropertySheets<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DevicePropertySheets(::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevQueryPrintEx<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DevQueryPrintEx(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&pdqpinfo), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm)).into()
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceCapabilitiesA(::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pdevicename), ::core::mem::transmute_copy(&wcapability), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&dwold), ::core::mem::transmute_copy(&dwresult)).into()
        }
        unsafe extern "system" fn UpgradePrinter<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpgradePrinter(::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverupgradeinfo)).into()
        }
        unsafe extern "system" fn PrinterEvent<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintername: super::super::Foundation::PWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrinterEvent(::core::mem::transmute_copy(&pprintername), ::core::mem::transmute_copy(&idriverevent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DriverEvent<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DriverEvent(::core::mem::transmute_copy(&dwdriverevent), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverinfo), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn QueryColorProfile<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryColorProfile(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&ulquerymode), ::core::mem::transmute_copy(&pvprofiledata), ::core::mem::transmute_copy(&pcbprofiledata), ::core::mem::transmute_copy(&pflprofiledata)).into()
        }
        unsafe extern "system" fn FontInstallerDlgProc<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FontInstallerDlgProc(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&usmsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn UpdateExternalFonts<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateExternalFonts(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hheap), ::core::mem::transmute_copy(&pwstrcartridges)).into()
        }
        Self {
            base: IPrintOemCommon_Vtbl::new::<Identity, Impl, OFFSET>(),
            PublishDriverInterface: PublishDriverInterface::<Identity, Impl, OFFSET>,
            CommonUIProp: CommonUIProp::<Identity, Impl, OFFSET>,
            DocumentPropertySheets: DocumentPropertySheets::<Identity, Impl, OFFSET>,
            DevicePropertySheets: DevicePropertySheets::<Identity, Impl, OFFSET>,
            DevQueryPrintEx: DevQueryPrintEx::<Identity, Impl, OFFSET>,
            DeviceCapabilitiesA: DeviceCapabilitiesA::<Identity, Impl, OFFSET>,
            UpgradePrinter: UpgradePrinter::<Identity, Impl, OFFSET>,
            PrinterEvent: PrinterEvent::<Identity, Impl, OFFSET>,
            DriverEvent: DriverEvent::<Identity, Impl, OFFSET>,
            QueryColorProfile: QueryColorProfile::<Identity, Impl, OFFSET>,
            FontInstallerDlgProc: FontInstallerDlgProc::<Identity, Impl, OFFSET>,
            UpdateExternalFonts: UpdateExternalFonts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUI as ::windows::core::Interface>::IID || iid == &<IPrintOemCommon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI2_Impl: Sized + IPrintOemCommon_Impl + IPrintOemUI_Impl {
    fn QueryJobAttributes(&mut self, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::Result<()>;
    fn HideStandardUI(&mut self, dwmode: u32) -> ::windows::core::Result<()>;
    fn DocumentEvent(&mut self, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI2_Impl, const OFFSET: isize>() -> IPrintOemUI2_Vtbl {
        unsafe extern "system" fn QueryJobAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryJobAttributes(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&lpattributeinfo)).into()
        }
        unsafe extern "system" fn HideStandardUI<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HideStandardUI(::core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn DocumentEvent<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DocumentEvent(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&iesc), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pvin), ::core::mem::transmute_copy(&cbout), ::core::mem::transmute_copy(&pvout), ::core::mem::transmute_copy(&piresult)).into()
        }
        Self {
            base: IPrintOemUI_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryJobAttributes: QueryJobAttributes::<Identity, Impl, OFFSET>,
            HideStandardUI: HideStandardUI::<Identity, Impl, OFFSET>,
            DocumentEvent: DocumentEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUI2 as ::windows::core::Interface>::IID || iid == &<IPrintOemCommon as ::windows::core::Interface>::IID || iid == &<IPrintOemUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemUIMXDC_Impl: Sized {
    fn AdjustImageableArea(&mut self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::Result<()>;
    fn AdjustImageCompression(&mut self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::Result<()>;
    fn AdjustDPI(&mut self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintOemUIMXDC_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>() -> IPrintOemUIMXDC_Vtbl {
        unsafe extern "system" fn AdjustImageableArea<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdjustImageableArea(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&prclimageablearea)).into()
        }
        unsafe extern "system" fn AdjustImageCompression<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdjustImageCompression(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pcompressionmode)).into()
        }
        unsafe extern "system" fn AdjustDPI<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOemUIMXDC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdjustDPI(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pdpi)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AdjustImageableArea: AdjustImageableArea::<Identity, Impl, OFFSET>,
            AdjustImageCompression: AdjustImageCompression::<Identity, Impl, OFFSET>,
            AdjustDPI: AdjustDPI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintOemUIMXDC as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait IPrintPreviewDxgiPackageTarget_Impl: Sized {
    fn SetJobPageCount(&mut self, counttype: PageCountType, count: u32) -> ::windows::core::Result<()>;
    fn DrawPage(&mut self, jobpagenumber: u32, pageimage: &::core::option::Option<super::Dxgi::IDXGISurface>, dpix: f32, dpiy: f32) -> ::windows::core::Result<()>;
    fn InvalidatePreview(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl IPrintPreviewDxgiPackageTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>() -> IPrintPreviewDxgiPackageTarget_Vtbl {
        unsafe extern "system" fn SetJobPageCount<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counttype: PageCountType, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJobPageCount(::core::mem::transmute_copy(&counttype), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DrawPage<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, pageimage: ::windows::core::RawPtr, dpix: f32, dpiy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DrawPage(::core::mem::transmute_copy(&jobpagenumber), ::core::mem::transmute(&pageimage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn InvalidatePreview<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvalidatePreview().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetJobPageCount: SetJobPageCount::<Identity, Impl, OFFSET>,
            DrawPage: DrawPage::<Identity, Impl, OFFSET>,
            InvalidatePreview: InvalidatePreview::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintPreviewDxgiPackageTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaAsyncOperation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaAsyncOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>() -> IPrintSchemaAsyncOperation_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaAsyncOperationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Completed(&mut self, pticket: &::core::option::Option<IPrintSchemaTicket>, hroperation: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaAsyncOperationEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: isize>() -> IPrintSchemaAsyncOperationEvent_Vtbl {
        unsafe extern "system" fn Completed<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pticket: ::windows::core::RawPtr, hroperation: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Completed(::core::mem::transmute(&pticket), ::core::mem::transmute_copy(&hroperation)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Completed: Completed::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperationEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaCapabilities_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&mut self, bstrkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn GetFeature(&mut self, bstrname: &super::super::Foundation::BSTR, bstrnamespaceuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn PageImageableSize(&mut self) -> ::windows::core::Result<IPrintSchemaPageImageableSize>;
    fn JobCopiesAllDocumentsMinValue(&mut self) -> ::windows::core::Result<u32>;
    fn JobCopiesAllDocumentsMaxValue(&mut self) -> ::windows::core::Result<u32>;
    fn GetSelectedOptionInPrintTicket(&mut self, pfeature: &::core::option::Option<IPrintSchemaFeature>) -> ::windows::core::Result<IPrintSchemaOption>;
    fn GetOptions(&mut self, pfeature: &::core::option::Option<IPrintSchemaFeature>) -> ::windows::core::Result<IPrintSchemaOptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>() -> IPrintSchemaCapabilities_Vtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFeatureByKeyName(::core::mem::transmute_copy(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFeature(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageImageableSize<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppageimageablesize: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PageImageableSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pppageimageablesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JobCopiesAllDocumentsMinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *puljobcopiesalldocumentsminvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JobCopiesAllDocumentsMaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    *puljobcopiesalldocumentsmaxvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelectedOptionInPrintTicket(::core::mem::transmute(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoptioncollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOptions(::core::mem::transmute(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptioncollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, Impl, OFFSET>,
            GetFeature: GetFeature::<Identity, Impl, OFFSET>,
            PageImageableSize: PageImageableSize::<Identity, Impl, OFFSET>,
            JobCopiesAllDocumentsMinValue: JobCopiesAllDocumentsMinValue::<Identity, Impl, OFFSET>,
            JobCopiesAllDocumentsMaxValue: JobCopiesAllDocumentsMaxValue::<Identity, Impl, OFFSET>,
            GetSelectedOptionInPrintTicket: GetSelectedOptionInPrintTicket::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaCapabilities2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl + IPrintSchemaCapabilities_Impl {
    fn GetParameterDefinition(&mut self, bstrname: &super::super::Foundation::BSTR, bstrnamespaceuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaParameterDefinition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaCapabilities2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities2_Impl, const OFFSET: isize>() -> IPrintSchemaCapabilities2_Vtbl {
        unsafe extern "system" fn GetParameterDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaCapabilities2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameterDefinition(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparameterdefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPrintSchemaCapabilities_Vtbl::new::<Identity, Impl, OFFSET>(), GetParameterDefinition: GetParameterDefinition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaDisplayableElement_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl {
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaDisplayableElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaDisplayableElement_Impl, const OFFSET: isize>() -> IPrintSchemaDisplayableElement_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaDisplayableElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(), DisplayName: DisplayName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaElement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn XmlNode(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn NamespaceUri(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>() -> IPrintSchemaElement_Vtbl {
        unsafe extern "system" fn XmlNode<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).XmlNode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppxmlnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnamespaceuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NamespaceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnamespaceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            XmlNode: XmlNode::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaFeature_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl + IPrintSchemaDisplayableElement_Impl {
    fn SelectedOption(&mut self) -> ::windows::core::Result<IPrintSchemaOption>;
    fn SetSelectedOption(&mut self, poption: &::core::option::Option<IPrintSchemaOption>) -> ::windows::core::Result<()>;
    fn SelectionType(&mut self) -> ::windows::core::Result<PrintSchemaSelectionType>;
    fn GetOption(&mut self, bstrname: &super::super::Foundation::BSTR, bstrnamespaceuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaOption>;
    fn DisplayUI(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaFeature_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>() -> IPrintSchemaFeature_Vtbl {
        unsafe extern "system" fn SelectedOption<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectedOption() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedOption<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poption: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelectedOption(::core::mem::transmute(&poption)).into()
        }
        unsafe extern "system" fn SelectionType<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectionType() {
                ::core::result::Result::Ok(ok__) => {
                    *pselectiontype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOption<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOption(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaFeature_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayUI() {
                ::core::result::Result::Ok(ok__) => {
                    *pbshow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            SelectedOption: SelectedOption::<Identity, Impl, OFFSET>,
            SetSelectedOption: SetSelectedOption::<Identity, Impl, OFFSET>,
            SelectionType: SelectionType::<Identity, Impl, OFFSET>,
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaFeature as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaNUpOption_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl + IPrintSchemaDisplayableElement_Impl + IPrintSchemaOption_Impl {
    fn PagesPerSheet(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaNUpOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaNUpOption_Impl, const OFFSET: isize>() -> IPrintSchemaNUpOption_Vtbl {
        unsafe extern "system" fn PagesPerSheet<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaNUpOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpagespersheet: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PagesPerSheet() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpagespersheet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPrintSchemaOption_Vtbl::new::<Identity, Impl, OFFSET>(), PagesPerSheet: PagesPerSheet::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaNUpOption as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaOption_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl + IPrintSchemaDisplayableElement_Impl {
    fn Selected(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Constrained(&mut self) -> ::windows::core::Result<PrintSchemaConstrainedSetting>;
    fn GetPropertyValue(&mut self, bstrname: &super::super::Foundation::BSTR, bstrnamespaceuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>() -> IPrintSchemaOption_Vtbl {
        unsafe extern "system" fn Selected<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisselected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Constrained<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Constrained() {
                ::core::result::Result::Ok(ok__) => {
                    *psetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlvaluenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyValue(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppxmlvaluenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            Selected: Selected::<Identity, Impl, OFFSET>,
            Constrained: Constrained::<Identity, Impl, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaOption as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaOptionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, ulindex: u32) -> ::windows::core::Result<IPrintSchemaOption>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaOptionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>() -> IPrintSchemaOptionCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaOptionCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaPageImageableSize_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl {
    fn ImageableSizeWidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn ImageableSizeHeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn OriginWidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn OriginHeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn ExtentWidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn ExtentHeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaPageImageableSize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>() -> IPrintSchemaPageImageableSize_Vtbl {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizewidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImageableSizeWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulimageablesizewidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulimageablesizeheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImageableSizeHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulimageablesizeheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginWidthInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OriginWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *puloriginwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginHeightInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puloriginheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OriginHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *puloriginheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtentWidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulextentwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulextentheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtentHeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulextentheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            ImageableSizeWidthInMicrons: ImageableSizeWidthInMicrons::<Identity, Impl, OFFSET>,
            ImageableSizeHeightInMicrons: ImageableSizeHeightInMicrons::<Identity, Impl, OFFSET>,
            OriginWidthInMicrons: OriginWidthInMicrons::<Identity, Impl, OFFSET>,
            OriginHeightInMicrons: OriginHeightInMicrons::<Identity, Impl, OFFSET>,
            ExtentWidthInMicrons: ExtentWidthInMicrons::<Identity, Impl, OFFSET>,
            ExtentHeightInMicrons: ExtentHeightInMicrons::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaPageImageableSize as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaPageMediaSizeOption_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl + IPrintSchemaDisplayableElement_Impl + IPrintSchemaOption_Impl {
    fn WidthInMicrons(&mut self) -> ::windows::core::Result<u32>;
    fn HeightInMicrons(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaPageMediaSizeOption_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>() -> IPrintSchemaPageMediaSizeOption_Vtbl {
        unsafe extern "system" fn WidthInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WidthInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeightInMicrons<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HeightInMicrons() {
                ::core::result::Result::Ok(ok__) => {
                    *pulheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaOption_Vtbl::new::<Identity, Impl, OFFSET>(),
            WidthInMicrons: WidthInMicrons::<Identity, Impl, OFFSET>,
            HeightInMicrons: HeightInMicrons::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaPageMediaSizeOption as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaOption as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaParameterDefinition_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl + IPrintSchemaDisplayableElement_Impl {
    fn UserInputRequired(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn UnitType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DataType(&mut self) -> ::windows::core::Result<PrintSchemaParameterDataType>;
    fn RangeMin(&mut self) -> ::windows::core::Result<i32>;
    fn RangeMax(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaParameterDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>() -> IPrintSchemaParameterDefinition_Vtbl {
        unsafe extern "system" fn UserInputRequired<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserInputRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisrequired = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnitType<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrunittype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnitType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrunittype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataType<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMin<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeMin() {
                ::core::result::Result::Ok(ok__) => {
                    *prangemin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMax<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangemax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeMax() {
                ::core::result::Result::Ok(ok__) => {
                    *prangemax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserInputRequired: UserInputRequired::<Identity, Impl, OFFSET>,
            UnitType: UnitType::<Identity, Impl, OFFSET>,
            DataType: DataType::<Identity, Impl, OFFSET>,
            RangeMin: RangeMin::<Identity, Impl, OFFSET>,
            RangeMax: RangeMax::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaParameterDefinition as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaParameterInitializer_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl {
    fn Value(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&mut self, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaParameterInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>() -> IPrintSchemaParameterInitializer_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&pvar)).into()
        }
        Self {
            base: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaParameterInitializer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaTicket_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&mut self, bstrkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn GetFeature(&mut self, bstrname: &super::super::Foundation::BSTR, bstrnamespaceuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaFeature>;
    fn ValidateAsync(&mut self) -> ::windows::core::Result<IPrintSchemaAsyncOperation>;
    fn CommitAsync(&mut self, pprintticketcommit: &::core::option::Option<IPrintSchemaTicket>) -> ::windows::core::Result<IPrintSchemaAsyncOperation>;
    fn NotifyXmlChanged(&mut self) -> ::windows::core::Result<()>;
    fn GetCapabilities(&mut self) -> ::windows::core::Result<IPrintSchemaCapabilities>;
    fn JobCopiesAllDocuments(&mut self) -> ::windows::core::Result<u32>;
    fn SetJobCopiesAllDocuments(&mut self, uljobcopiesalldocuments: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaTicket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>() -> IPrintSchemaTicket_Vtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFeatureByKeyName(::core::mem::transmute_copy(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFeature(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfeature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ValidateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticketcommit: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CommitAsync(::core::mem::transmute(&pprintticketcommit)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyXmlChanged<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyXmlChanged().into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JobCopiesAllDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *puljobcopiesalldocuments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uljobcopiesalldocuments: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJobCopiesAllDocuments(::core::mem::transmute_copy(&uljobcopiesalldocuments)).into()
        }
        Self {
            base: IPrintSchemaElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, Impl, OFFSET>,
            GetFeature: GetFeature::<Identity, Impl, OFFSET>,
            ValidateAsync: ValidateAsync::<Identity, Impl, OFFSET>,
            CommitAsync: CommitAsync::<Identity, Impl, OFFSET>,
            NotifyXmlChanged: NotifyXmlChanged::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            JobCopiesAllDocuments: JobCopiesAllDocuments::<Identity, Impl, OFFSET>,
            SetJobCopiesAllDocuments: SetJobCopiesAllDocuments::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaTicket as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrintSchemaTicket2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrintSchemaElement_Impl + IPrintSchemaTicket_Impl {
    fn GetParameterInitializer(&mut self, bstrname: &super::super::Foundation::BSTR, bstrnamespaceuri: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrintSchemaParameterInitializer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrintSchemaTicket2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket2_Impl, const OFFSET: isize>() -> IPrintSchemaTicket2_Vtbl {
        unsafe extern "system" fn GetParameterInitializer<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSchemaTicket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterinitializer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameterInitializer(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparameterinitializer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPrintSchemaTicket_Vtbl::new::<Identity, Impl, OFFSET>(), GetParameterInitializer: GetParameterInitializer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSchemaTicket2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrintSchemaElement as ::windows::core::Interface>::IID || iid == &<IPrintSchemaTicket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider_Impl: Sized {
    fn GetSupportedVersions(&mut self, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::Result<()>;
    fn BindPrinter(&mut self, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryDeviceNamespace(&mut self, pdefaultnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ConvertPrintTicketToDevMode(&mut self, pprintticket: &::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::Result<()>;
    fn ConvertDevModeToPrintTicket(&mut self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: &::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<()>;
    fn GetPrintCapabilities(&mut self, pprintticket: &::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn ValidatePrintTicket(&mut self, pbaseticket: &::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>() -> IPrintTicketProvider_Vtbl {
        unsafe extern "system" fn GetSupportedVersions<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSupportedVersions(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&ppversions), ::core::mem::transmute_copy(&cversions)).into()
        }
        unsafe extern "system" fn BindPrinter<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BindPrinter(::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&version), ::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&pdevmodeflags), ::core::mem::transmute_copy(&cnamespaces), ::core::mem::transmute_copy(&ppnamespaces)).into()
        }
        unsafe extern "system" fn QueryDeviceNamespace<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdefaultnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryDeviceNamespace(::core::mem::transmute_copy(&pdefaultnamespace)).into()
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertPrintTicketToDevMode(::core::mem::transmute(&pprintticket), ::core::mem::transmute_copy(&cbdevmodein), ::core::mem::transmute_copy(&pdevmodein), ::core::mem::transmute_copy(&pcbdevmodeout), ::core::mem::transmute_copy(&ppdevmodeout)).into()
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertDevModeToPrintTicket(::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute(&pprintticket)).into()
        }
        unsafe extern "system" fn GetPrintCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrintCapabilities(::core::mem::transmute(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidatePrintTicket<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbaseticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidatePrintTicket(::core::mem::transmute(&pbaseticket)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSupportedVersions: GetSupportedVersions::<Identity, Impl, OFFSET>,
            BindPrinter: BindPrinter::<Identity, Impl, OFFSET>,
            QueryDeviceNamespace: QueryDeviceNamespace::<Identity, Impl, OFFSET>,
            ConvertPrintTicketToDevMode: ConvertPrintTicketToDevMode::<Identity, Impl, OFFSET>,
            ConvertDevModeToPrintTicket: ConvertDevModeToPrintTicket::<Identity, Impl, OFFSET>,
            GetPrintCapabilities: GetPrintCapabilities::<Identity, Impl, OFFSET>,
            ValidatePrintTicket: ValidatePrintTicket::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider2_Impl: Sized + IPrintTicketProvider_Impl {
    fn GetPrintDeviceCapabilities(&mut self, pprintticket: &::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn GetPrintDeviceResources(&mut self, pszlocalename: super::super::Foundation::PWSTR, pprintticket: &::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider2_Impl, const OFFSET: isize>() -> IPrintTicketProvider2_Vtbl {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppdevicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrintDeviceCapabilities(::core::mem::transmute(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevicecapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintDeviceResources<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTicketProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalename: super::super::Foundation::PWSTR, pprintticket: ::windows::core::RawPtr, ppdeviceresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrintDeviceResources(::core::mem::transmute_copy(&pszlocalename), ::core::mem::transmute(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeviceresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrintTicketProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPrintDeviceCapabilities: GetPrintDeviceCapabilities::<Identity, Impl, OFFSET>,
            GetPrintDeviceResources: GetPrintDeviceResources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTicketProvider2 as ::windows::core::Interface>::IID || iid == &<IPrintTicketProvider as ::windows::core::Interface>::IID
    }
}
pub trait IPrintUnidiAsyncNotifyRegistration_Impl: Sized + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNotification(&mut self, param0: &::core::option::Option<IAsyncGetSendNotificationCookie>) -> ::windows::core::Result<()>;
}
impl IPrintUnidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: isize>() -> IPrintUnidiAsyncNotifyRegistration_Vtbl {
        unsafe extern "system" fn AsyncGetNotification<Identity: ::windows::core::IUnknownImpl, Impl: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncGetNotification(::core::mem::transmute(&param0)).into()
        }
        Self { base: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, Impl, OFFSET>(), AsyncGetNotification: AsyncGetNotification::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintUnidiAsyncNotifyRegistration as ::windows::core::Interface>::IID || iid == &<IPrintAsyncNotifyRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrinterBidiSetRequestCallback_Impl: Sized {
    fn Completed(&mut self, bstrresponse: &super::super::Foundation::BSTR, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrinterBidiSetRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterBidiSetRequestCallback_Impl, const OFFSET: isize>() -> IPrinterBidiSetRequestCallback_Vtbl {
        unsafe extern "system" fn Completed<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterBidiSetRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Completed(::core::mem::transmute_copy(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Completed: Completed::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterBidiSetRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPrinterExtensionAsyncOperation_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
impl IPrinterExtensionAsyncOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionAsyncOperation_Impl, const OFFSET: isize>() -> IPrinterExtensionAsyncOperation_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionAsyncOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PrinterQueue(&mut self) -> ::windows::core::Result<IPrinterQueue>;
    fn PrintSchemaTicket(&mut self) -> ::windows::core::Result<IPrintSchemaTicket>;
    fn DriverProperties(&mut self) -> ::windows::core::Result<IPrinterPropertyBag>;
    fn UserProperties(&mut self) -> ::windows::core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>() -> IPrinterExtensionContext_Vtbl {
        unsafe extern "system" fn PrinterQueue<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrinterQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintSchemaTicket<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppticket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrintSchemaTicket() {
                ::core::result::Result::Ok(ok__) => {
                    *ppticket = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrinterQueue: PrinterQueue::<Identity, Impl, OFFSET>,
            PrintSchemaTicket: PrintSchemaTicket::<Identity, Impl, OFFSET>,
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionContext as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionContextCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, ulindex: u32) -> ::windows::core::Result<IPrinterExtensionContext>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionContextCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>() -> IPrinterExtensionContextCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionContextCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnDriverEvent(&mut self, peventargs: &::core::option::Option<IPrinterExtensionEventArgs>) -> ::windows::core::Result<()>;
    fn OnPrinterQueuesEnumerated(&mut self, pcontextcollection: &::core::option::Option<IPrinterExtensionContextCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEvent_Impl, const OFFSET: isize>() -> IPrinterExtensionEvent_Vtbl {
        unsafe extern "system" fn OnDriverEvent<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDriverEvent(::core::mem::transmute(&peventargs)).into()
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPrinterQueuesEnumerated(::core::mem::transmute(&pcontextcollection)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnDriverEvent: OnDriverEvent::<Identity, Impl, OFFSET>,
            OnPrinterQueuesEnumerated: OnPrinterQueuesEnumerated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionEventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrinterExtensionContext_Impl {
    fn BidiNotification(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ReasonId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Request(&mut self) -> ::windows::core::Result<IPrinterExtensionRequest>;
    fn SourceApplication(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DetailedReasonId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn WindowModal(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn WindowParent(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>() -> IPrinterExtensionEventArgs_Vtbl {
        unsafe extern "system" fn BidiNotification<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbidinotification: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BidiNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbidinotification = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReasonId<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReasonId() {
                ::core::result::Result::Ok(ok__) => {
                    *preasonid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *pprequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceApplication<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplication: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SourceApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailedReasonId<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetailedreasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DetailedReasonId() {
                ::core::result::Result::Ok(ok__) => {
                    *pdetailedreasonid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowModal<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowModal() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmodal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowParent<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowParent() {
                ::core::result::Result::Ok(ok__) => {
                    *phwndparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrinterExtensionContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            BidiNotification: BidiNotification::<Identity, Impl, OFFSET>,
            ReasonId: ReasonId::<Identity, Impl, OFFSET>,
            Request: Request::<Identity, Impl, OFFSET>,
            SourceApplication: SourceApplication::<Identity, Impl, OFFSET>,
            DetailedReasonId: DetailedReasonId::<Identity, Impl, OFFSET>,
            WindowModal: WindowModal::<Identity, Impl, OFFSET>,
            WindowParent: WindowParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionEventArgs as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrinterExtensionContext as ::windows::core::Interface>::IID
    }
}
pub trait IPrinterExtensionManager_Impl: Sized {
    fn EnableEvents(&mut self, printerdriverid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DisableEvents(&mut self) -> ::windows::core::Result<()>;
}
impl IPrinterExtensionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionManager_Impl, const OFFSET: isize>() -> IPrinterExtensionManager_Vtbl {
        unsafe extern "system" fn EnableEvents<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printerdriverid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableEvents(::core::mem::transmute_copy(&printerdriverid)).into()
        }
        unsafe extern "system" fn DisableEvents<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableEvents().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnableEvents: EnableEvents::<Identity, Impl, OFFSET>,
            DisableEvents: DisableEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterExtensionRequest_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Cancel(&mut self, hrstatus: ::windows::core::HRESULT, bstrlogmessage: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterExtensionRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionRequest_Impl, const OFFSET: isize>() -> IPrinterExtensionRequest_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, bstrlogmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel(::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&bstrlogmessage)).into()
        }
        unsafe extern "system" fn Complete<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterExtensionRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterExtensionRequest as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterPropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetBool(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&mut self, bstrname: &super::super::Foundation::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInt32(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn SetInt32(&mut self, bstrname: &super::super::Foundation::BSTR, nvalue: i32) -> ::windows::core::Result<()>;
    fn GetString(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetString(&mut self, bstrname: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBytes(&mut self, bstrname: &super::super::Foundation::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetBytes(&mut self, bstrname: &super::super::Foundation::BSTR, cbvalue: u32, pvalue: *const u8) -> ::windows::core::Result<()>;
    fn GetReadStream(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn GetWriteStream(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>() -> IPrinterPropertyBag_Vtbl {
        unsafe extern "system" fn GetBool<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBool(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBool(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInt32(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInt32(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetString(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBytes(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&pcbvalue), ::core::mem::transmute_copy(&ppvalue)).into()
        }
        unsafe extern "system" fn SetBytes<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cbvalue: u32, pvalue: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBytes(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&cbvalue), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetReadStream<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReadStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWriteStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBool: GetBool::<Identity, Impl, OFFSET>,
            SetBool: SetBool::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            SetInt32: SetInt32::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            GetBytes: GetBytes::<Identity, Impl, OFFSET>,
            SetBytes: SetBytes::<Identity, Impl, OFFSET>,
            GetReadStream: GetReadStream::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterPropertyBag as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Handle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SendBidiQuery(&mut self, bstrbidiquery: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetProperties(&mut self) -> ::windows::core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue_Impl, const OFFSET: isize>() -> IPrinterQueue_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *phprinter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendBidiQuery<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidiquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendBidiQuery(::core::mem::transmute_copy(&bstrbidiquery)).into()
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SendBidiQuery: SendBidiQuery::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueue as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueue2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrinterQueue_Impl {
    fn SendBidiSetRequestAsync(&mut self, bstrbidirequest: &super::super::Foundation::BSTR, pcallback: &::core::option::Option<IPrinterBidiSetRequestCallback>) -> ::windows::core::Result<IPrinterExtensionAsyncOperation>;
    fn GetPrinterQueueView(&mut self, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::Result<IPrinterQueueView>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueue2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue2_Impl, const OFFSET: isize>() -> IPrinterQueue2_Vtbl {
        unsafe extern "system" fn SendBidiSetRequestAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbidirequest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcallback: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendBidiSetRequestAsync(::core::mem::transmute_copy(&bstrbidirequest), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrinterQueueView<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrinterQueueView(::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppjobview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPrinterQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            SendBidiSetRequestAsync: SendBidiSetRequestAsync::<Identity, Impl, OFFSET>,
            GetPrinterQueueView: GetPrinterQueueView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueue2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrinterQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnBidiResponseReceived(&mut self, bstrresponse: &super::super::Foundation::BSTR, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueEvent_Impl, const OFFSET: isize>() -> IPrinterQueueEvent_Vtbl {
        unsafe extern "system" fn OnBidiResponseReceived<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBidiResponseReceived(::core::mem::transmute_copy(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnBidiResponseReceived: OnBidiResponseReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueView_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetViewRange(&mut self, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueView_Impl, const OFFSET: isize>() -> IPrinterQueueView_Vtbl {
        unsafe extern "system" fn SetViewRange<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewRange(::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), SetViewRange: SetViewRange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueView as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterQueueViewEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnChanged(&mut self, pcollection: &::core::option::Option<IPrintJobCollection>, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterQueueViewEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueViewEvent_Impl, const OFFSET: isize>() -> IPrinterQueueViewEvent_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterQueueViewEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcollection: ::windows::core::RawPtr, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChanged(::core::mem::transmute(&pcollection), ::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize), ::core::mem::transmute_copy(&ulcountjobsinprintqueue)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), OnChanged: OnChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterQueueViewEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DriverProperties(&mut self) -> ::windows::core::Result<IPrinterScriptablePropertyBag>;
    fn QueueProperties(&mut self) -> ::windows::core::Result<IPrinterScriptablePropertyBag>;
    fn UserProperties(&mut self) -> ::windows::core::Result<IPrinterScriptablePropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>() -> IPrinterScriptContext_Vtbl {
        unsafe extern "system" fn DriverProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            QueueProperties: QueueProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptContext as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptablePropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetBool(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&mut self, bstrname: &super::super::Foundation::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInt32(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn SetInt32(&mut self, bstrname: &super::super::Foundation::BSTR, nvalue: i32) -> ::windows::core::Result<()>;
    fn GetString(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetString(&mut self, bstrname: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBytes(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetBytes(&mut self, bstrname: &super::super::Foundation::BSTR, parray: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn GetReadStream(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrinterScriptableStream>;
    fn GetWriteStream(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IPrinterScriptableStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptablePropertyBag_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>() -> IPrinterScriptablePropertyBag_Vtbl {
        unsafe extern "system" fn GetBool<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBool(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBool(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInt32(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInt32(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetString(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBytes(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pparray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytes<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parray: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBytes(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute(&parray)).into()
        }
        unsafe extern "system" fn GetReadStream<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReadStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWriteStream(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBool: GetBool::<Identity, Impl, OFFSET>,
            SetBool: SetBool::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            SetInt32: SetInt32::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            GetBytes: GetBytes::<Identity, Impl, OFFSET>,
            SetBytes: SetBytes::<Identity, Impl, OFFSET>,
            GetReadStream: GetReadStream::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptablePropertyBag2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrinterScriptablePropertyBag_Impl {
    fn GetReadStreamAsXML(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptablePropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag2_Impl, const OFFSET: isize>() -> IPrinterScriptablePropertyBag2_Vtbl {
        unsafe extern "system" fn GetReadStreamAsXML<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptablePropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReadStreamAsXML(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppxmlnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPrinterScriptablePropertyBag_Vtbl::new::<Identity, Impl, OFFSET>(), GetReadStreamAsXML: GetReadStreamAsXML::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrinterScriptablePropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptableSequentialStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Read(&mut self, cbread: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Write(&mut self, parray: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptableSequentialStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>() -> IPrinterScriptableSequentialStream_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbread: i32, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&cbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *pparray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: ::windows::core::RawPtr, pcbwritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Write(::core::mem::transmute(&parray)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptableSequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrinterScriptableStream_Impl: Sized + super::super::System::Com::IDispatch_Impl + IPrinterScriptableSequentialStream_Impl {
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn Seek(&mut self, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<i32>;
    fn SetSize(&mut self, lsize: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrinterScriptableStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>() -> IPrinterScriptableStream_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Seek(::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&streamseek)) {
                ::core::result::Result::Ok(ok__) => {
                    *plposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: IPrinterScriptableStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&lsize)).into()
        }
        Self {
            base: IPrinterScriptableSequentialStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinterScriptableStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IPrinterScriptableSequentialStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory_Impl: Sized {
    fn CreateRasterizer(&mut self, xpspage: &::core::option::Option<super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> ::windows::core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory_Impl, const OFFSET: isize>() -> IXpsRasterizationFactory_Vtbl {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRasterizer(::core::mem::transmute(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppixpsrasterizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory1_Impl: Sized {
    fn CreateRasterizer(&mut self, xpspage: &::core::option::Option<super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> ::windows::core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory1_Impl, const OFFSET: isize>() -> IXpsRasterizationFactory1_Vtbl {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRasterizer(::core::mem::transmute(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppixpsrasterizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory2_Impl: Sized {
    fn CreateRasterizer(&mut self, xpspage: &::core::option::Option<super::super::Storage::Xps::IXpsOMPage>, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> ::windows::core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory2_Impl, const OFFSET: isize>() -> IXpsRasterizationFactory2_Vtbl {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizationFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRasterizer(::core::mem::transmute(&xpspage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&backgroundcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppixpsrasterizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IXpsRasterizer_Impl: Sized {
    fn RasterizeRect(&mut self, x: i32, y: i32, width: i32, height: i32, notificationcallback: &::core::option::Option<IXpsRasterizerNotificationCallback>) -> ::windows::core::Result<super::Imaging::IWICBitmap>;
    fn SetMinimalLineWidth(&mut self, width: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IXpsRasterizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizer_Impl, const OFFSET: isize>() -> IXpsRasterizer_Vtbl {
        unsafe extern "system" fn RasterizeRect<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: ::windows::core::RawPtr, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RasterizeRect(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute(&notificationcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *bitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimalLineWidth<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinimalLineWidth(::core::mem::transmute_copy(&width)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RasterizeRect: RasterizeRect::<Identity, Impl, OFFSET>,
            SetMinimalLineWidth: SetMinimalLineWidth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizer as ::windows::core::Interface>::IID
    }
}
pub trait IXpsRasterizerNotificationCallback_Impl: Sized {
    fn Continue(&mut self) -> ::windows::core::Result<()>;
}
impl IXpsRasterizerNotificationCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizerNotificationCallback_Impl, const OFFSET: isize>() -> IXpsRasterizerNotificationCallback_Vtbl {
        unsafe extern "system" fn Continue<Identity: ::windows::core::IUnknownImpl, Impl: IXpsRasterizerNotificationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Continue().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Continue: Continue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXpsRasterizerNotificationCallback as ::windows::core::Interface>::IID
    }
}
