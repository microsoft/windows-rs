pub trait IAsyncGetSendNotificationCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData();
}
impl ::windows::core::RuntimeName for IAsyncGetSendNotificationCookie {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IAsyncGetSendNotificationCookie";
}
impl IAsyncGetSendNotificationCookieVtbl {
    pub const fn new<Impl: IAsyncGetSendNotificationCookieImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAsyncGetSendNotificationCookieVtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IAsyncGetSendNotificationCookieImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsyncCallWithData(&*(&param0 as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAsyncGetSendNotificationCookie>, base.5, FinishAsyncCallWithData::<Impl, OFFSET>)
    }
}
pub trait IAsyncGetSrvReferralCookieImpl: Sized {
    fn FinishAsyncCall();
    fn CancelAsyncCall();
    fn FinishAsyncCallWithData();
}
impl ::windows::core::RuntimeName for IAsyncGetSrvReferralCookie {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IAsyncGetSrvReferralCookie";
}
impl IAsyncGetSrvReferralCookieVtbl {
    pub const fn new<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAsyncGetSrvReferralCookieVtbl {
        unsafe extern "system" fn FinishAsyncCall<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsyncCall(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelAsyncCall(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IAsyncGetSrvReferralCookieImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsyncCallWithData(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAsyncGetSrvReferralCookie>, base.5, FinishAsyncCall::<Impl, OFFSET>, CancelAsyncCall::<Impl, OFFSET>, FinishAsyncCallWithData::<Impl, OFFSET>)
    }
}
pub trait IBidiAsyncNotifyChannelImpl: Sized + IPrintAsyncNotifyChannelImpl {
    fn CreateNotificationChannel();
    fn GetPrintName();
    fn GetChannelNotificationType();
    fn AsyncGetNotificationSendResponse();
    fn AsyncCloseChannel();
}
impl ::windows::core::RuntimeName for IBidiAsyncNotifyChannel {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IBidiAsyncNotifyChannel";
}
impl IBidiAsyncNotifyChannelVtbl {
    pub const fn new<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBidiAsyncNotifyChannelVtbl {
        unsafe extern "system" fn CreateNotificationChannel<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNotificationChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintName<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrintName(&*(&param0 as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelNotificationType<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChannelNotificationType(&*(&param0 as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncGetNotificationSendResponse(&*(&param0 as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <IAsyncGetSendNotificationCookie as ::windows::core::Abi>::Abi as *const <IAsyncGetSendNotificationCookie as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncCloseChannel<Impl: IBidiAsyncNotifyChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncCloseChannel(&*(&param0 as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <IPrintAsyncCookie as ::windows::core::Abi>::Abi as *const <IPrintAsyncCookie as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBidiAsyncNotifyChannel>, base.5, CreateNotificationChannel::<Impl, OFFSET>, GetPrintName::<Impl, OFFSET>, GetChannelNotificationType::<Impl, OFFSET>, AsyncGetNotificationSendResponse::<Impl, OFFSET>, AsyncCloseChannel::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncCookieImpl: Sized {
    fn FinishAsyncCall();
    fn CancelAsyncCall();
}
impl ::windows::core::RuntimeName for IPrintAsyncCookie {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncCookie";
}
impl IPrintAsyncCookieVtbl {
    pub const fn new<Impl: IPrintAsyncCookieImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncCookieVtbl {
        unsafe extern "system" fn FinishAsyncCall<Impl: IPrintAsyncCookieImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsyncCall(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IPrintAsyncCookieImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelAsyncCall(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncCookie>, base.5, FinishAsyncCall::<Impl, OFFSET>, CancelAsyncCall::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncNewChannelCookieImpl: Sized + IPrintAsyncCookieImpl {
    fn FinishAsyncCallWithData();
}
impl ::windows::core::RuntimeName for IPrintAsyncNewChannelCookie {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncNewChannelCookie";
}
impl IPrintAsyncNewChannelCookieVtbl {
    pub const fn new<Impl: IPrintAsyncNewChannelCookieImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncNewChannelCookieVtbl {
        unsafe extern "system" fn FinishAsyncCallWithData<Impl: IPrintAsyncNewChannelCookieImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::RawPtr, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsyncCallWithData(&*(&param0 as *const <IPrintAsyncNotifyChannel as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyChannel as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncNewChannelCookie>, base.5, FinishAsyncCallWithData::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncNotifyImpl: Sized {
    fn CreatePrintAsyncNotifyChannel();
    fn CreatePrintAsyncNotifyRegistration();
}
impl ::windows::core::RuntimeName for IPrintAsyncNotify {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncNotify";
}
impl IPrintAsyncNotifyVtbl {
    pub const fn new<Impl: IPrintAsyncNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncNotifyVtbl {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Impl: IPrintAsyncNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: ::windows::core::RawPtr, param5: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePrintAsyncNotifyChannel(param0, &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), param2, param3, &*(&param4 as *const <IPrintAsyncNotifyCallback as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param5)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Impl: IPrintAsyncNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: ::windows::core::RawPtr, param4: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePrintAsyncNotifyRegistration(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), param1, param2, &*(&param3 as *const <IPrintAsyncNotifyCallback as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param4)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncNotify>, base.5, CreatePrintAsyncNotifyChannel::<Impl, OFFSET>, CreatePrintAsyncNotifyRegistration::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncNotifyCallbackImpl: Sized {
    fn OnEventNotify();
    fn ChannelClosed();
}
impl ::windows::core::RuntimeName for IPrintAsyncNotifyCallback {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncNotifyCallback";
}
impl IPrintAsyncNotifyCallbackVtbl {
    pub const fn new<Impl: IPrintAsyncNotifyCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncNotifyCallbackVtbl {
        unsafe extern "system" fn OnEventNotify<Impl: IPrintAsyncNotifyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEventNotify(&*(&pchannel as *const <IPrintAsyncNotifyChannel as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyChannel as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelClosed<Impl: IPrintAsyncNotifyCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChannelClosed(&*(&pchannel as *const <IPrintAsyncNotifyChannel as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyChannel as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncNotifyCallback>, base.5, OnEventNotify::<Impl, OFFSET>, ChannelClosed::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncNotifyChannelImpl: Sized {
    fn SendNotification();
    fn CloseChannel();
}
impl ::windows::core::RuntimeName for IPrintAsyncNotifyChannel {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncNotifyChannel";
}
impl IPrintAsyncNotifyChannelVtbl {
    pub const fn new<Impl: IPrintAsyncNotifyChannelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncNotifyChannelVtbl {
        unsafe extern "system" fn SendNotification<Impl: IPrintAsyncNotifyChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendNotification(&*(&pdata as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseChannel<Impl: IPrintAsyncNotifyChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseChannel(&*(&pdata as *const <IPrintAsyncNotifyDataObject as ::windows::core::Abi>::Abi as *const <IPrintAsyncNotifyDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncNotifyChannel>, base.5, SendNotification::<Impl, OFFSET>, CloseChannel::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncNotifyDataObjectImpl: Sized {
    fn AcquireData();
    fn ReleaseData();
}
impl ::windows::core::RuntimeName for IPrintAsyncNotifyDataObject {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncNotifyDataObject";
}
impl IPrintAsyncNotifyDataObjectVtbl {
    pub const fn new<Impl: IPrintAsyncNotifyDataObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncNotifyDataObjectVtbl {
        unsafe extern "system" fn AcquireData<Impl: IPrintAsyncNotifyDataObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquireData(::core::mem::transmute_copy(&ppnotificationdata), ::core::mem::transmute_copy(&psize), ::core::mem::transmute_copy(&ppschema)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseData<Impl: IPrintAsyncNotifyDataObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncNotifyDataObject>, base.5, AcquireData::<Impl, OFFSET>, ReleaseData::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncNotifyRegistrationImpl: Sized {
    fn RegisterForNotifications();
    fn UnregisterForNotifications();
}
impl ::windows::core::RuntimeName for IPrintAsyncNotifyRegistration {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncNotifyRegistration";
}
impl IPrintAsyncNotifyRegistrationVtbl {
    pub const fn new<Impl: IPrintAsyncNotifyRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn RegisterForNotifications<Impl: IPrintAsyncNotifyRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterForNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterForNotifications<Impl: IPrintAsyncNotifyRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterForNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncNotifyRegistration>, base.5, RegisterForNotifications::<Impl, OFFSET>, UnregisterForNotifications::<Impl, OFFSET>)
    }
}
pub trait IPrintAsyncNotifyServerReferralImpl: Sized {
    fn GetServerReferral();
    fn AsyncGetServerReferral();
    fn SetServerReferral();
}
impl ::windows::core::RuntimeName for IPrintAsyncNotifyServerReferral {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintAsyncNotifyServerReferral";
}
impl IPrintAsyncNotifyServerReferralVtbl {
    pub const fn new<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintAsyncNotifyServerReferralVtbl {
        unsafe extern "system" fn GetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetServerReferral(::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncGetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncGetServerReferral(&*(&param0 as *const <IAsyncGetSrvReferralCookie as ::windows::core::Abi>::Abi as *const <IAsyncGetSrvReferralCookie as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerReferral<Impl: IPrintAsyncNotifyServerReferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prmtserverreferral: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServerReferral(&*(&prmtserverreferral as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintAsyncNotifyServerReferral>, base.5, GetServerReferral::<Impl, OFFSET>, AsyncGetServerReferral::<Impl, OFFSET>, SetServerReferral::<Impl, OFFSET>)
    }
}
pub trait IPrintBidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNewChannel();
}
impl ::windows::core::RuntimeName for IPrintBidiAsyncNotifyRegistration {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintBidiAsyncNotifyRegistration";
}
impl IPrintBidiAsyncNotifyRegistrationVtbl {
    pub const fn new<Impl: IPrintBidiAsyncNotifyRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintBidiAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn AsyncGetNewChannel<Impl: IPrintBidiAsyncNotifyRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncGetNewChannel(&*(&param0 as *const <IPrintAsyncNewChannelCookie as ::windows::core::Abi>::Abi as *const <IPrintAsyncNewChannelCookie as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintBidiAsyncNotifyRegistration>, base.5, AsyncGetNewChannel::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPrintCoreHelper {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintCoreHelper";
}
impl IPrintCoreHelperVtbl {
    pub const fn new<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintCoreHelperVtbl {
        unsafe extern "system" fn GetOption<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: super::super::Foundation::PSTR, ppszoption: *mut super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOption(&*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType), cbsize, &*(&pszfeaturerequested as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOptions(
                &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                cbsize,
                &*(&bresolveconflicts as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pfopairs as *const <PRINT_FEATURE_OPTION as ::windows::core::Abi>::Abi as *const <PRINT_FEATURE_OPTION as ::windows::core::DefaultType>::DefaultType),
                cpairs,
                pcpairswritten,
                pdwresult,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pconstrainedoptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumConstrainedOptions(
                &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                cbsize,
                &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pconstrainedoptionlist as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                pdwnumoptions,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhyConstrained<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WhyConstrained(
                &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                cbsize,
                &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszoptionkeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppfoconstraints),
                ::core::mem::transmute_copy(&pdwnumoptions),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFeatures<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfeaturelist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumFeatures(::core::mem::transmute_copy(&pfeaturelist), ::core::mem::transmute_copy(&pdwnumfeatures)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumOptions<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, poptionlist: *mut *mut *mut super::super::Foundation::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumOptions(&*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&poptionlist), ::core::mem::transmute_copy(&pdwnumoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSubstitution<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, ppszdevfontname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFontSubstitution(&*(&psztruetypefontname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&ppszdevfontname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontSubstitution<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztruetypefontname: super::super::Foundation::PWSTR, pszdevfontname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFontSubstitution(&*(&psztruetypefontname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszdevfontname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Impl: IPrintCoreHelperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceOfMSXMLObject(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwclscontext,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&ppv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintCoreHelper>, base.5, GetOption::<Impl, OFFSET>, SetOptions::<Impl, OFFSET>, EnumConstrainedOptions::<Impl, OFFSET>, WhyConstrained::<Impl, OFFSET>, EnumFeatures::<Impl, OFFSET>, EnumOptions::<Impl, OFFSET>, GetFontSubstitution::<Impl, OFFSET>, SetFontSubstitution::<Impl, OFFSET>, CreateInstanceOfMSXMLObject::<Impl, OFFSET>)
    }
}
pub trait IPrintCoreHelperPSImpl: Sized + IPrintCoreHelperImpl {
    fn GetGlobalAttribute();
    fn GetFeatureAttribute();
    fn GetOptionAttribute();
}
impl ::windows::core::RuntimeName for IPrintCoreHelperPS {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintCoreHelperPS";
}
impl IPrintCoreHelperPSVtbl {
    pub const fn new<Impl: IPrintCoreHelperPSImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintCoreHelperPSVtbl {
        unsafe extern "system" fn GetGlobalAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlobalAttribute(&*(&pszattribute as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFeatureAttribute(
                &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszattribute as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwdatatype),
                ::core::mem::transmute_copy(&ppbdata),
                ::core::mem::transmute_copy(&pcbsize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionAttribute<Impl: IPrintCoreHelperPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionAttribute(
                &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszoptionkeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszattribute as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwdatatype),
                ::core::mem::transmute_copy(&ppbdata),
                ::core::mem::transmute_copy(&pcbsize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintCoreHelperPS>, base.5, GetGlobalAttribute::<Impl, OFFSET>, GetFeatureAttribute::<Impl, OFFSET>, GetOptionAttribute::<Impl, OFFSET>)
    }
}
pub trait IPrintCoreHelperUniImpl: Sized + IPrintCoreHelperImpl {
    fn CreateGDLSnapshot();
    fn CreateDefaultGDLSnapshot();
}
impl ::windows::core::RuntimeName for IPrintCoreHelperUni {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintCoreHelperUni";
}
impl IPrintCoreHelperUniVtbl {
    pub const fn new<Impl: IPrintCoreHelperUniImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintCoreHelperUniVtbl {
        unsafe extern "system" fn CreateGDLSnapshot<Impl: IPrintCoreHelperUniImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateGDLSnapshot(&*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType), cbsize, dwflags, ::core::mem::transmute_copy(&ppsnapshotstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Impl: IPrintCoreHelperUniImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDefaultGDLSnapshot(dwflags, ::core::mem::transmute_copy(&ppsnapshotstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintCoreHelperUni>, base.5, CreateGDLSnapshot::<Impl, OFFSET>, CreateDefaultGDLSnapshot::<Impl, OFFSET>)
    }
}
pub trait IPrintCoreHelperUni2Impl: Sized + IPrintCoreHelperUniImpl + IPrintCoreHelperImpl {
    fn GetNamedCommand();
}
impl ::windows::core::RuntimeName for IPrintCoreHelperUni2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintCoreHelperUni2";
}
impl IPrintCoreHelperUni2Vtbl {
    pub const fn new<Impl: IPrintCoreHelperUni2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintCoreHelperUni2Vtbl {
        unsafe extern "system" fn GetNamedCommand<Impl: IPrintCoreHelperUni2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: super::super::Foundation::PWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamedCommand(&*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType), cbsize, &*(&pszcommandname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcommandbytes), ::core::mem::transmute_copy(&pcbcommandsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintCoreHelperUni2>, base.5, GetNamedCommand::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPrintCoreUI2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintCoreUI2";
}
impl IPrintCoreUI2Vtbl {
    pub const fn new<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintCoreUI2Vtbl {
        unsafe extern "system" fn GetOptions<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptions(&*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType), dwflags, pmszfeaturesrequested, cbin, ::core::mem::transmute_copy(&pmszfeatureoptionbuf), cbsize, ::core::mem::transmute_copy(&pcbneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOptions(&*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType), dwflags, pmszfeatureoptionbuf, cbin, ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszconstrainedoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumConstrainedOptions(&*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pmszconstrainedoptionlist), cbsize, ::core::mem::transmute_copy(&pcbneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhyConstrained<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pmszreasonlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WhyConstrained(
                &*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszoptionkeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pmszreasonlist),
                cbsize,
                ::core::mem::transmute_copy(&pcbneeded),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlobalAttribute(&*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&pszattribute as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), cbsize, ::core::mem::transmute_copy(&pcbneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFeatureAttribute(
                &*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszattribute as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwdatatype),
                ::core::mem::transmute_copy(&pbdata),
                cbsize,
                ::core::mem::transmute_copy(&pcbneeded),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionAttribute<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pszoptionkeyword: super::super::Foundation::PSTR, pszattribute: super::super::Foundation::PSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptionAttribute(
                &*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszoptionkeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszattribute as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwdatatype),
                ::core::mem::transmute_copy(&pbdata),
                cbsize,
                ::core::mem::transmute_copy(&pcbneeded),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFeatures<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumFeatures(&*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pmszfeaturelist), cbsize, ::core::mem::transmute_copy(&pcbneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumOptions<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: super::super::Foundation::PSTR, pmszoptionlist: super::super::Foundation::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumOptions(&*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&pszfeaturekeyword as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pmszoptionlist), cbsize, ::core::mem::transmute_copy(&pcbneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySimulationSupport<Impl: IPrintCoreUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuerySimulationSupport(&*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), dwlevel, ::core::mem::transmute_copy(&pcaps), cbsize, ::core::mem::transmute_copy(&pcbneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintCoreUI2>, base.5, GetOptions::<Impl, OFFSET>, SetOptions::<Impl, OFFSET>, EnumConstrainedOptions::<Impl, OFFSET>, WhyConstrained::<Impl, OFFSET>, GetGlobalAttribute::<Impl, OFFSET>, GetFeatureAttribute::<Impl, OFFSET>, GetOptionAttribute::<Impl, OFFSET>, EnumFeatures::<Impl, OFFSET>, EnumOptions::<Impl, OFFSET>, QuerySimulationSupport::<Impl, OFFSET>)
    }
}
pub trait IPrintJobImpl: Sized {
    fn Name();
    fn Id();
    fn PrintedPages();
    fn TotalPages();
    fn Status();
    fn SubmissionTime();
    fn RequestCancel();
}
impl ::windows::core::RuntimeName for IPrintJob {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintJob";
}
impl IPrintJobVtbl {
    pub const fn new<Impl: IPrintJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintJobVtbl {
        unsafe extern "system" fn Name<Impl: IPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pulid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintedPages<Impl: IPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrintedPages(::core::mem::transmute_copy(&pulpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Impl: IPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalPages(::core::mem::transmute_copy(&pulpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut PrintJobStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Impl: IPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubmissionTime(::core::mem::transmute_copy(&psubmissiontime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCancel<Impl: IPrintJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintJob>, base.5, Name::<Impl, OFFSET>, Id::<Impl, OFFSET>, PrintedPages::<Impl, OFFSET>, TotalPages::<Impl, OFFSET>, Status::<Impl, OFFSET>, SubmissionTime::<Impl, OFFSET>, RequestCancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintJobCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintJobCollection {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintJobCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintJobCollectionVtbl {
    pub const fn new<Impl: IPrintJobCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintJobCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrintJobCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pulcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPrintJobCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(ulindex, ::core::mem::transmute_copy(&ppjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrintJobCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintJobCollection>, base.5, Count::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
pub trait IPrintOemCommonImpl: Sized {
    fn GetInfo();
    fn DevMode();
}
impl ::windows::core::RuntimeName for IPrintOemCommon {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintOemCommon";
}
impl IPrintOemCommonVtbl {
    pub const fn new<Impl: IPrintOemCommonImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintOemCommonVtbl {
        unsafe extern "system" fn GetInfo<Impl: IPrintOemCommonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInfo(dwmode, ::core::mem::transmute_copy(&pbuffer), cbsize, ::core::mem::transmute_copy(&pcbneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DevMode<Impl: IPrintOemCommonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DevMode(dwmode, &*(&poemdmparam as *const <OEMDMPARAM as ::windows::core::Abi>::Abi as *const <OEMDMPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintOemCommon>, base.5, GetInfo::<Impl, OFFSET>, DevMode::<Impl, OFFSET>)
    }
}
pub trait IPrintOemDriverUIImpl: Sized {
    fn DrvGetDriverSetting();
    fn DrvUpgradeRegistrySetting();
    fn DrvUpdateUISetting();
}
impl ::windows::core::RuntimeName for IPrintOemDriverUI {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintOemDriverUI";
}
impl IPrintOemDriverUIVtbl {
    pub const fn new<Impl: IPrintOemDriverUIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintOemDriverUIVtbl {
        unsafe extern "system" fn DrvGetDriverSetting<Impl: IPrintOemDriverUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, feature: super::super::Foundation::PSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrvGetDriverSetting(
                &*(&pci as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&feature as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poutput as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbsize,
                pcbneeded,
                pdwoptionsreturned,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Impl: IPrintOemDriverUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: super::super::Foundation::PSTR, poption: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrvUpgradeRegistrySetting(
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&pfeature as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poption as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrvUpdateUISetting<Impl: IPrintOemDriverUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrvUpdateUISetting(&*(&pci as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&poptitem as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwpreviousselection, dwmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintOemDriverUI>, base.5, DrvGetDriverSetting::<Impl, OFFSET>, DrvUpgradeRegistrySetting::<Impl, OFFSET>, DrvUpdateUISetting::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPrintOemUI {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintOemUI";
}
impl IPrintOemUIVtbl {
    pub const fn new<Impl: IPrintOemUIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintOemUIVtbl {
        unsafe extern "system" fn PublishDriverInterface<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PublishDriverInterface(&*(&piunknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonUIProp<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommonUIProp(dwmode, &*(&poemcuipparam as *const <OEMCUIPPARAM as ::windows::core::Abi>::Abi as *const <OEMCUIPPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentPropertySheets<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentPropertySheets(&*(&ppsuiinfo as *const <PROPSHEETUI_INFO as ::windows::core::Abi>::Abi as *const <PROPSHEETUI_INFO as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DevicePropertySheets<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DevicePropertySheets(&*(&ppsuiinfo as *const <PROPSHEETUI_INFO as ::windows::core::Abi>::Abi as *const <PROPSHEETUI_INFO as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DevQueryPrintEx<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DevQueryPrintEx(
                &*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType),
                &*(&pdqpinfo as *const <DEVQUERYPRINT_INFO as ::windows::core::Abi>::Abi as *const <DEVQUERYPRINT_INFO as ::windows::core::DefaultType>::DefaultType),
                &*(&ppublicdm as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                &*(&poemdm as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: super::super::Foundation::PWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceCapabilitiesA(
                &*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType),
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&pdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                wcapability,
                ::core::mem::transmute_copy(&poutput),
                &*(&ppublicdm as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                &*(&poemdm as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwold,
                ::core::mem::transmute_copy(&dwresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpgradePrinter<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpgradePrinter(dwlevel, pdriverupgradeinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrinterEvent<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintername: super::super::Foundation::PWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrinterEvent(&*(&pprintername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), idriverevent, dwflags, &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverEvent<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverEvent(dwdriverevent, dwlevel, pdriverinfo, &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryColorProfile<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryColorProfile(
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&poemuiobj as *const <OEMUIOBJ as ::windows::core::Abi>::Abi as *const <OEMUIOBJ as ::windows::core::DefaultType>::DefaultType),
                &*(&ppublicdm as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                &*(&poemdm as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ulquerymode,
                ::core::mem::transmute_copy(&pvprofiledata),
                pcbprofiledata,
                ::core::mem::transmute_copy(&pflprofiledata),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontInstallerDlgProc<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontInstallerDlgProc(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                usmsg,
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateExternalFonts<Impl: IPrintOemUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateExternalFonts(
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&hheap as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&pwstrcartridges as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPrintOemUI>,
            base.5,
            PublishDriverInterface::<Impl, OFFSET>,
            CommonUIProp::<Impl, OFFSET>,
            DocumentPropertySheets::<Impl, OFFSET>,
            DevicePropertySheets::<Impl, OFFSET>,
            DevQueryPrintEx::<Impl, OFFSET>,
            DeviceCapabilitiesA::<Impl, OFFSET>,
            UpgradePrinter::<Impl, OFFSET>,
            PrinterEvent::<Impl, OFFSET>,
            DriverEvent::<Impl, OFFSET>,
            QueryColorProfile::<Impl, OFFSET>,
            FontInstallerDlgProc::<Impl, OFFSET>,
            UpdateExternalFonts::<Impl, OFFSET>,
        )
    }
}
pub trait IPrintOemUI2Impl: Sized + IPrintOemUIImpl + IPrintOemCommonImpl {
    fn QueryJobAttributes();
    fn HideStandardUI();
    fn DocumentEvent();
}
impl ::windows::core::RuntimeName for IPrintOemUI2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintOemUI2";
}
impl IPrintOemUI2Vtbl {
    pub const fn new<Impl: IPrintOemUI2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintOemUI2Vtbl {
        unsafe extern "system" fn QueryJobAttributes<Impl: IPrintOemUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryJobAttributes(&*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType), dwlevel, lpattributeinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HideStandardUI<Impl: IPrintOemUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HideStandardUI(dwmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentEvent<Impl: IPrintOemUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentEvent(
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&hdc as *const <super::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                iesc,
                cbin,
                &*(&pvin as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbout,
                &*(&pvout as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                piresult,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintOemUI2>, base.5, QueryJobAttributes::<Impl, OFFSET>, HideStandardUI::<Impl, OFFSET>, DocumentEvent::<Impl, OFFSET>)
    }
}
pub trait IPrintOemUIMXDCImpl: Sized {
    fn AdjustImageableArea();
    fn AdjustImageCompression();
    fn AdjustDPI();
}
impl ::windows::core::RuntimeName for IPrintOemUIMXDC {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintOemUIMXDC";
}
impl IPrintOemUIMXDCVtbl {
    pub const fn new<Impl: IPrintOemUIMXDCImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintOemUIMXDCVtbl {
        unsafe extern "system" fn AdjustImageableArea<Impl: IPrintOemUIMXDCImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdjustImageableArea(
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                cbdevmode,
                &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                cboemdm,
                &*(&poemdm as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&prclimageablearea as *const <super::super::Foundation::RECTL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECTL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustImageCompression<Impl: IPrintOemUIMXDCImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdjustImageCompression(
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                cbdevmode,
                &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                cboemdm,
                &*(&poemdm as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                pcompressionmode,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustDPI<Impl: IPrintOemUIMXDCImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdjustDPI(
                &*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                cbdevmode,
                &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                cboemdm,
                &*(&poemdm as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                pdpi,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintOemUIMXDC>, base.5, AdjustImageableArea::<Impl, OFFSET>, AdjustImageCompression::<Impl, OFFSET>, AdjustDPI::<Impl, OFFSET>)
    }
}
pub trait IPrintPreviewDxgiPackageTargetImpl: Sized {
    fn SetJobPageCount();
    fn DrawPage();
    fn InvalidatePreview();
}
impl ::windows::core::RuntimeName for IPrintPreviewDxgiPackageTarget {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintPreviewDxgiPackageTarget";
}
impl IPrintPreviewDxgiPackageTargetVtbl {
    pub const fn new<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintPreviewDxgiPackageTargetVtbl {
        unsafe extern "system" fn SetJobPageCount<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, counttype: PageCountType, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetJobPageCount(counttype, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawPage<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, pageimage: ::windows::core::RawPtr, dpix: f32, dpiy: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DrawPage(jobpagenumber, &*(&pageimage as *const <super::Dxgi::IDXGISurface as ::windows::core::Abi>::Abi as *const <super::Dxgi::IDXGISurface as ::windows::core::DefaultType>::DefaultType), dpix, dpiy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidatePreview<Impl: IPrintPreviewDxgiPackageTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvalidatePreview() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintPreviewDxgiPackageTarget>, base.5, SetJobPageCount::<Impl, OFFSET>, DrawPage::<Impl, OFFSET>, InvalidatePreview::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaAsyncOperationImpl: Sized + IDispatchImpl {
    fn Start();
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaAsyncOperation {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaAsyncOperation";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperationVtbl {
    pub const fn new<Impl: IPrintSchemaAsyncOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaAsyncOperationVtbl {
        unsafe extern "system" fn Start<Impl: IPrintSchemaAsyncOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPrintSchemaAsyncOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaAsyncOperation>, base.5, Start::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaAsyncOperationEventImpl: Sized + IDispatchImpl {
    fn Completed();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaAsyncOperationEvent {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaAsyncOperationEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperationEventVtbl {
    pub const fn new<Impl: IPrintSchemaAsyncOperationEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaAsyncOperationEventVtbl {
        unsafe extern "system" fn Completed<Impl: IPrintSchemaAsyncOperationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pticket: ::windows::core::RawPtr, hroperation: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&pticket as *const <IPrintSchemaTicket as ::windows::core::Abi>::Abi as *const <IPrintSchemaTicket as ::windows::core::DefaultType>::DefaultType), hroperation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaAsyncOperationEvent>, base.5, Completed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaCapabilitiesImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetFeatureByKeyName();
    fn GetFeature();
    fn PageImageableSize();
    fn JobCopiesAllDocumentsMinValue();
    fn JobCopiesAllDocumentsMaxValue();
    fn GetSelectedOptionInPrintTicket();
    fn GetOptions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaCapabilities {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaCapabilities";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilitiesVtbl {
    pub const fn new<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaCapabilitiesVtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFeatureByKeyName(&*(&bstrkeyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFeature(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrnamespaceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageImageableSize<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppageimageablesize: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageImageableSize(::core::mem::transmute_copy(&pppageimageablesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JobCopiesAllDocumentsMinValue(::core::mem::transmute_copy(&puljobcopiesalldocumentsminvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JobCopiesAllDocumentsMaxValue(::core::mem::transmute_copy(&puljobcopiesalldocumentsmaxvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSelectedOptionInPrintTicket(&*(&pfeature as *const <IPrintSchemaFeature as ::windows::core::Abi>::Abi as *const <IPrintSchemaFeature as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptions<Impl: IPrintSchemaCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfeature: ::windows::core::RawPtr, ppoptioncollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOptions(&*(&pfeature as *const <IPrintSchemaFeature as ::windows::core::Abi>::Abi as *const <IPrintSchemaFeature as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoptioncollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaCapabilities>, base.5, GetFeatureByKeyName::<Impl, OFFSET>, GetFeature::<Impl, OFFSET>, PageImageableSize::<Impl, OFFSET>, JobCopiesAllDocumentsMinValue::<Impl, OFFSET>, JobCopiesAllDocumentsMaxValue::<Impl, OFFSET>, GetSelectedOptionInPrintTicket::<Impl, OFFSET>, GetOptions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaCapabilities2Impl: Sized + IPrintSchemaCapabilitiesImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetParameterDefinition();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaCapabilities2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaCapabilities2";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities2Vtbl {
    pub const fn new<Impl: IPrintSchemaCapabilities2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaCapabilities2Vtbl {
        unsafe extern "system" fn GetParameterDefinition<Impl: IPrintSchemaCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParameterDefinition(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrnamespaceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppparameterdefinition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaCapabilities2>, base.5, GetParameterDefinition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaDisplayableElementImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn DisplayName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaDisplayableElement {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaDisplayableElement";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaDisplayableElementVtbl {
    pub const fn new<Impl: IPrintSchemaDisplayableElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaDisplayableElementVtbl {
        unsafe extern "system" fn DisplayName<Impl: IPrintSchemaDisplayableElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName(::core::mem::transmute_copy(&pbstrdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaDisplayableElement>, base.5, DisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaElementImpl: Sized + IDispatchImpl {
    fn XmlNode();
    fn Name();
    fn NamespaceUri();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaElement {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaElement";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaElementVtbl {
    pub const fn new<Impl: IPrintSchemaElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaElementVtbl {
        unsafe extern "system" fn XmlNode<Impl: IPrintSchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XmlNode(::core::mem::transmute_copy(&ppxmlnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IPrintSchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Impl: IPrintSchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnamespaceuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NamespaceUri(::core::mem::transmute_copy(&pbstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaElement>, base.5, XmlNode::<Impl, OFFSET>, Name::<Impl, OFFSET>, NamespaceUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaFeatureImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn SelectedOption();
    fn SetSelectedOption();
    fn SelectionType();
    fn GetOption();
    fn DisplayUI();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaFeature {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaFeature";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaFeatureVtbl {
    pub const fn new<Impl: IPrintSchemaFeatureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaFeatureVtbl {
        unsafe extern "system" fn SelectedOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedOption(::core::mem::transmute_copy(&ppoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poption: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSelectedOption(&*(&poption as *const <IPrintSchemaOption as ::windows::core::Abi>::Abi as *const <IPrintSchemaOption as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionType<Impl: IPrintSchemaFeatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionType(::core::mem::transmute_copy(&pselectiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOption<Impl: IPrintSchemaFeatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOption(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrnamespaceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: IPrintSchemaFeatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayUI(::core::mem::transmute_copy(&pbshow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaFeature>, base.5, SelectedOption::<Impl, OFFSET>, SetSelectedOption::<Impl, OFFSET>, SelectionType::<Impl, OFFSET>, GetOption::<Impl, OFFSET>, DisplayUI::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaNUpOptionImpl: Sized + IPrintSchemaOptionImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn PagesPerSheet();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaNUpOption {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaNUpOption";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaNUpOptionVtbl {
    pub const fn new<Impl: IPrintSchemaNUpOptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaNUpOptionVtbl {
        unsafe extern "system" fn PagesPerSheet<Impl: IPrintSchemaNUpOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulpagespersheet: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PagesPerSheet(::core::mem::transmute_copy(&pulpagespersheet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaNUpOption>, base.5, PagesPerSheet::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaOptionImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn Selected();
    fn Constrained();
    fn GetPropertyValue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaOption {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaOption";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOptionVtbl {
    pub const fn new<Impl: IPrintSchemaOptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaOptionVtbl {
        unsafe extern "system" fn Selected<Impl: IPrintSchemaOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Selected(::core::mem::transmute_copy(&pbisselected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Constrained<Impl: IPrintSchemaOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Constrained(::core::mem::transmute_copy(&psetting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IPrintSchemaOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlvaluenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyValue(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrnamespaceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppxmlvaluenode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaOption>, base.5, Selected::<Impl, OFFSET>, Constrained::<Impl, OFFSET>, GetPropertyValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaOptionCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaOptionCollection {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaOptionCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOptionCollectionVtbl {
    pub const fn new<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaOptionCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pulcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppoption: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(ulindex, ::core::mem::transmute_copy(&ppoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrintSchemaOptionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaOptionCollection>, base.5, Count::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaPageImageableSizeImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn ImageableSizeWidthInMicrons();
    fn ImageableSizeHeightInMicrons();
    fn OriginWidthInMicrons();
    fn OriginHeightInMicrons();
    fn ExtentWidthInMicrons();
    fn ExtentHeightInMicrons();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaPageImageableSize {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaPageImageableSize";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageImageableSizeVtbl {
    pub const fn new<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaPageImageableSizeVtbl {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulimageablesizewidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageableSizeWidthInMicrons(::core::mem::transmute_copy(&pulimageablesizewidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulimageablesizeheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageableSizeHeightInMicrons(::core::mem::transmute_copy(&pulimageablesizeheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puloriginwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OriginWidthInMicrons(::core::mem::transmute_copy(&puloriginwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puloriginheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OriginHeightInMicrons(::core::mem::transmute_copy(&puloriginheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulextentwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtentWidthInMicrons(::core::mem::transmute_copy(&pulextentwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Impl: IPrintSchemaPageImageableSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulextentheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtentHeightInMicrons(::core::mem::transmute_copy(&pulextentheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaPageImageableSize>, base.5, ImageableSizeWidthInMicrons::<Impl, OFFSET>, ImageableSizeHeightInMicrons::<Impl, OFFSET>, OriginWidthInMicrons::<Impl, OFFSET>, OriginHeightInMicrons::<Impl, OFFSET>, ExtentWidthInMicrons::<Impl, OFFSET>, ExtentHeightInMicrons::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaPageMediaSizeOptionImpl: Sized + IPrintSchemaOptionImpl + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn WidthInMicrons();
    fn HeightInMicrons();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaPageMediaSizeOption {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaPageMediaSizeOption";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageMediaSizeOptionVtbl {
    pub const fn new<Impl: IPrintSchemaPageMediaSizeOptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaPageMediaSizeOptionVtbl {
        unsafe extern "system" fn WidthInMicrons<Impl: IPrintSchemaPageMediaSizeOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WidthInMicrons(::core::mem::transmute_copy(&pulwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeightInMicrons<Impl: IPrintSchemaPageMediaSizeOptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeightInMicrons(::core::mem::transmute_copy(&pulheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaPageMediaSizeOption>, base.5, WidthInMicrons::<Impl, OFFSET>, HeightInMicrons::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaParameterDefinitionImpl: Sized + IPrintSchemaDisplayableElementImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn UserInputRequired();
    fn UnitType();
    fn DataType();
    fn RangeMin();
    fn RangeMax();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaParameterDefinition {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaParameterDefinition";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterDefinitionVtbl {
    pub const fn new<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaParameterDefinitionVtbl {
        unsafe extern "system" fn UserInputRequired<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserInputRequired(::core::mem::transmute_copy(&pbisrequired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnitType<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrunittype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnitType(::core::mem::transmute_copy(&pbstrunittype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataType<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataType(::core::mem::transmute_copy(&pdatatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMin<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangemin: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RangeMin(::core::mem::transmute_copy(&prangemin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMax<Impl: IPrintSchemaParameterDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangemax: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RangeMax(::core::mem::transmute_copy(&prangemax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaParameterDefinition>, base.5, UserInputRequired::<Impl, OFFSET>, UnitType::<Impl, OFFSET>, DataType::<Impl, OFFSET>, RangeMin::<Impl, OFFSET>, RangeMax::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaParameterInitializerImpl: Sized + IPrintSchemaElementImpl + IDispatchImpl {
    fn Value();
    fn SetValue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaParameterInitializer {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaParameterInitializer";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterInitializerVtbl {
    pub const fn new<Impl: IPrintSchemaParameterInitializerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaParameterInitializerVtbl {
        unsafe extern "system" fn Value<Impl: IPrintSchemaParameterInitializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPrintSchemaParameterInitializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&pvar as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaParameterInitializer>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaTicket {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaTicket";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicketVtbl {
    pub const fn new<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaTicketVtbl {
        unsafe extern "system" fn GetFeatureByKeyName<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFeatureByKeyName(&*(&bstrkeyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfeature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFeature(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrnamespaceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateAsync<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateAsync(::core::mem::transmute_copy(&ppasyncoperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticketcommit: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitAsync(&*(&pprintticketcommit as *const <IPrintSchemaTicket as ::windows::core::Abi>::Abi as *const <IPrintSchemaTicket as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasyncoperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyXmlChanged<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotifyXmlChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(::core::mem::transmute_copy(&ppcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JobCopiesAllDocuments(::core::mem::transmute_copy(&puljobcopiesalldocuments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Impl: IPrintSchemaTicketImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uljobcopiesalldocuments: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetJobCopiesAllDocuments(uljobcopiesalldocuments) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaTicket>, base.5, GetFeatureByKeyName::<Impl, OFFSET>, GetFeature::<Impl, OFFSET>, ValidateAsync::<Impl, OFFSET>, CommitAsync::<Impl, OFFSET>, NotifyXmlChanged::<Impl, OFFSET>, GetCapabilities::<Impl, OFFSET>, JobCopiesAllDocuments::<Impl, OFFSET>, SetJobCopiesAllDocuments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaTicket2Impl: Sized + IPrintSchemaTicketImpl + IPrintSchemaElementImpl + IDispatchImpl {
    fn GetParameterInitializer();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrintSchemaTicket2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintSchemaTicket2";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket2Vtbl {
    pub const fn new<Impl: IPrintSchemaTicket2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintSchemaTicket2Vtbl {
        unsafe extern "system" fn GetParameterInitializer<Impl: IPrintSchemaTicket2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnamespaceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparameterinitializer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParameterInitializer(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrnamespaceuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppparameterinitializer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintSchemaTicket2>, base.5, GetParameterInitializer::<Impl, OFFSET>)
    }
}
pub trait IPrintTicketProviderImpl: Sized {
    fn GetSupportedVersions();
    fn BindPrinter();
    fn QueryDeviceNamespace();
    fn ConvertPrintTicketToDevMode();
    fn ConvertDevModeToPrintTicket();
    fn GetPrintCapabilities();
    fn ValidatePrintTicket();
}
impl ::windows::core::RuntimeName for IPrintTicketProvider {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintTicketProvider";
}
impl IPrintTicketProviderVtbl {
    pub const fn new<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintTicketProviderVtbl {
        unsafe extern "system" fn GetSupportedVersions<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedVersions(&*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ppversions, cversions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindPrinter<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BindPrinter(&*(&hprinter as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), version, poptions, pdevmodeflags, cnamespaces, &*(&ppnamespaces as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDeviceNamespace<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdefaultnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryDeviceNamespace(&*(&pdefaultnamespace as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertPrintTicketToDevMode(
                &*(&pprintticket as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::DefaultType>::DefaultType),
                cbdevmodein,
                &*(&pdevmodein as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
                pcbdevmodeout,
                &*(&ppdevmodeout as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertDevModeToPrintTicket(cbdevmode, &*(&pdevmode as *const <super::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType), &*(&pprintticket as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintCapabilities<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrintCapabilities(&*(&pprintticket as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidatePrintTicket<Impl: IPrintTicketProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbaseticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidatePrintTicket(&*(&pbaseticket as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintTicketProvider>, base.5, GetSupportedVersions::<Impl, OFFSET>, BindPrinter::<Impl, OFFSET>, QueryDeviceNamespace::<Impl, OFFSET>, ConvertPrintTicketToDevMode::<Impl, OFFSET>, ConvertDevModeToPrintTicket::<Impl, OFFSET>, GetPrintCapabilities::<Impl, OFFSET>, ValidatePrintTicket::<Impl, OFFSET>)
    }
}
pub trait IPrintTicketProvider2Impl: Sized + IPrintTicketProviderImpl {
    fn GetPrintDeviceCapabilities();
    fn GetPrintDeviceResources();
}
impl ::windows::core::RuntimeName for IPrintTicketProvider2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintTicketProvider2";
}
impl IPrintTicketProvider2Vtbl {
    pub const fn new<Impl: IPrintTicketProvider2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintTicketProvider2Vtbl {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Impl: IPrintTicketProvider2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: ::windows::core::RawPtr, ppdevicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrintDeviceCapabilities(&*(&pprintticket as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevicecapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintDeviceResources<Impl: IPrintTicketProvider2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlocalename: super::super::Foundation::PWSTR, pprintticket: ::windows::core::RawPtr, ppdeviceresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrintDeviceResources(&*(&pszlocalename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pprintticket as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::MsXml::IXMLDOMDocument2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdeviceresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintTicketProvider2>, base.5, GetPrintDeviceCapabilities::<Impl, OFFSET>, GetPrintDeviceResources::<Impl, OFFSET>)
    }
}
pub trait IPrintUnidiAsyncNotifyRegistrationImpl: Sized + IPrintAsyncNotifyRegistrationImpl {
    fn AsyncGetNotification();
}
impl ::windows::core::RuntimeName for IPrintUnidiAsyncNotifyRegistration {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrintUnidiAsyncNotifyRegistration";
}
impl IPrintUnidiAsyncNotifyRegistrationVtbl {
    pub const fn new<Impl: IPrintUnidiAsyncNotifyRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintUnidiAsyncNotifyRegistrationVtbl {
        unsafe extern "system" fn AsyncGetNotification<Impl: IPrintUnidiAsyncNotifyRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncGetNotification(&*(&param0 as *const <IAsyncGetSendNotificationCookie as ::windows::core::Abi>::Abi as *const <IAsyncGetSendNotificationCookie as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintUnidiAsyncNotifyRegistration>, base.5, AsyncGetNotification::<Impl, OFFSET>)
    }
}
pub trait IPrinterBidiSetRequestCallbackImpl: Sized {
    fn Completed();
}
impl ::windows::core::RuntimeName for IPrinterBidiSetRequestCallback {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterBidiSetRequestCallback";
}
impl IPrinterBidiSetRequestCallbackVtbl {
    pub const fn new<Impl: IPrinterBidiSetRequestCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterBidiSetRequestCallbackVtbl {
        unsafe extern "system" fn Completed<Impl: IPrinterBidiSetRequestCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&bstrresponse as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), hrstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterBidiSetRequestCallback>, base.5, Completed::<Impl, OFFSET>)
    }
}
pub trait IPrinterExtensionAsyncOperationImpl: Sized {
    fn Cancel();
}
impl ::windows::core::RuntimeName for IPrinterExtensionAsyncOperation {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterExtensionAsyncOperation";
}
impl IPrinterExtensionAsyncOperationVtbl {
    pub const fn new<Impl: IPrinterExtensionAsyncOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterExtensionAsyncOperationVtbl {
        unsafe extern "system" fn Cancel<Impl: IPrinterExtensionAsyncOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterExtensionAsyncOperation>, base.5, Cancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionContextImpl: Sized + IDispatchImpl {
    fn PrinterQueue();
    fn PrintSchemaTicket();
    fn DriverProperties();
    fn UserProperties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterExtensionContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterExtensionContext";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContextVtbl {
    pub const fn new<Impl: IPrinterExtensionContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterExtensionContextVtbl {
        unsafe extern "system" fn PrinterQueue<Impl: IPrinterExtensionContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrinterQueue(::core::mem::transmute_copy(&ppqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintSchemaTicket<Impl: IPrinterExtensionContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppticket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrintSchemaTicket(::core::mem::transmute_copy(&ppticket)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Impl: IPrinterExtensionContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverProperties(::core::mem::transmute_copy(&pppropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Impl: IPrinterExtensionContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserProperties(::core::mem::transmute_copy(&pppropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterExtensionContext>, base.5, PrinterQueue::<Impl, OFFSET>, PrintSchemaTicket::<Impl, OFFSET>, DriverProperties::<Impl, OFFSET>, UserProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionContextCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn GetAt();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterExtensionContextCollection {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterExtensionContextCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContextCollectionVtbl {
    pub const fn new<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterExtensionContextCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pulcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(ulindex, ::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IPrinterExtensionContextCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterExtensionContextCollection>, base.5, Count::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionEventImpl: Sized + IDispatchImpl {
    fn OnDriverEvent();
    fn OnPrinterQueuesEnumerated();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterExtensionEvent {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterExtensionEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEventVtbl {
    pub const fn new<Impl: IPrinterExtensionEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterExtensionEventVtbl {
        unsafe extern "system" fn OnDriverEvent<Impl: IPrinterExtensionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDriverEvent(&*(&peventargs as *const <IPrinterExtensionEventArgs as ::windows::core::Abi>::Abi as *const <IPrinterExtensionEventArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Impl: IPrinterExtensionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontextcollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPrinterQueuesEnumerated(&*(&pcontextcollection as *const <IPrinterExtensionContextCollection as ::windows::core::Abi>::Abi as *const <IPrinterExtensionContextCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterExtensionEvent>, base.5, OnDriverEvent::<Impl, OFFSET>, OnPrinterQueuesEnumerated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionEventArgsImpl: Sized + IPrinterExtensionContextImpl + IDispatchImpl {
    fn BidiNotification();
    fn ReasonId();
    fn Request();
    fn SourceApplication();
    fn DetailedReasonId();
    fn WindowModal();
    fn WindowParent();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterExtensionEventArgs {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterExtensionEventArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEventArgsVtbl {
    pub const fn new<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterExtensionEventArgsVtbl {
        unsafe extern "system" fn BidiNotification<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbidinotification: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BidiNotification(::core::mem::transmute_copy(&pbstrbidinotification)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReasonId<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReasonId(::core::mem::transmute_copy(&preasonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request(::core::mem::transmute_copy(&pprequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceApplication<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplication: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceApplication(::core::mem::transmute_copy(&pbstrapplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailedReasonId<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdetailedreasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetailedReasonId(::core::mem::transmute_copy(&pdetailedreasonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowModal<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WindowModal(::core::mem::transmute_copy(&pbmodal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowParent<Impl: IPrinterExtensionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WindowParent(::core::mem::transmute_copy(&phwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterExtensionEventArgs>, base.5, BidiNotification::<Impl, OFFSET>, ReasonId::<Impl, OFFSET>, Request::<Impl, OFFSET>, SourceApplication::<Impl, OFFSET>, DetailedReasonId::<Impl, OFFSET>, WindowModal::<Impl, OFFSET>, WindowParent::<Impl, OFFSET>)
    }
}
pub trait IPrinterExtensionManagerImpl: Sized {
    fn EnableEvents();
    fn DisableEvents();
}
impl ::windows::core::RuntimeName for IPrinterExtensionManager {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterExtensionManager";
}
impl IPrinterExtensionManagerVtbl {
    pub const fn new<Impl: IPrinterExtensionManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterExtensionManagerVtbl {
        unsafe extern "system" fn EnableEvents<Impl: IPrinterExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printerdriverid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableEvents(&*(&printerdriverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableEvents<Impl: IPrinterExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterExtensionManager>, base.5, EnableEvents::<Impl, OFFSET>, DisableEvents::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionRequestImpl: Sized + IDispatchImpl {
    fn Cancel();
    fn Complete();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterExtensionRequest {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterExtensionRequest";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionRequestVtbl {
    pub const fn new<Impl: IPrinterExtensionRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterExtensionRequestVtbl {
        unsafe extern "system" fn Cancel<Impl: IPrinterExtensionRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, bstrlogmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel(hrstatus, &*(&bstrlogmessage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IPrinterExtensionRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Complete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterExtensionRequest>, base.5, Cancel::<Impl, OFFSET>, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterPropertyBag {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterPropertyBag";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterPropertyBagVtbl {
    pub const fn new<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterPropertyBagVtbl {
        unsafe extern "system" fn GetBool<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBool(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBool(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bvalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt32<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInt32(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInt32(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), nvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetString(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetString(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBytes<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBytes(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcbvalue), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytes<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cbvalue: u32, pvalue: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBytes(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), cbvalue, pvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadStream<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReadStream(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Impl: IPrinterPropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWriteStream(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterPropertyBag>, base.5, GetBool::<Impl, OFFSET>, SetBool::<Impl, OFFSET>, GetInt32::<Impl, OFFSET>, SetInt32::<Impl, OFFSET>, GetString::<Impl, OFFSET>, SetString::<Impl, OFFSET>, GetBytes::<Impl, OFFSET>, SetBytes::<Impl, OFFSET>, GetReadStream::<Impl, OFFSET>, GetWriteStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueImpl: Sized + IDispatchImpl {
    fn Handle();
    fn Name();
    fn SendBidiQuery();
    fn GetProperties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterQueue";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueVtbl {
    pub const fn new<Impl: IPrinterQueueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterQueueVtbl {
        unsafe extern "system" fn Handle<Impl: IPrinterQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&phprinter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IPrinterQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendBidiQuery<Impl: IPrinterQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbidiquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendBidiQuery(&*(&bstrbidiquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: IPrinterQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&pppropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterQueue>, base.5, Handle::<Impl, OFFSET>, Name::<Impl, OFFSET>, SendBidiQuery::<Impl, OFFSET>, GetProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueue2Impl: Sized + IPrinterQueueImpl + IDispatchImpl {
    fn SendBidiSetRequestAsync();
    fn GetPrinterQueueView();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterQueue2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterQueue2";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue2Vtbl {
    pub const fn new<Impl: IPrinterQueue2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterQueue2Vtbl {
        unsafe extern "system" fn SendBidiSetRequestAsync<Impl: IPrinterQueue2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbidirequest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcallback: ::windows::core::RawPtr, ppasyncoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendBidiSetRequestAsync(&*(&bstrbidirequest as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcallback as *const <IPrinterBidiSetRequestCallback as ::windows::core::Abi>::Abi as *const <IPrinterBidiSetRequestCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppasyncoperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrinterQueueView<Impl: IPrinterQueue2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrinterQueueView(ulviewoffset, ulviewsize, ::core::mem::transmute_copy(&ppjobview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterQueue2>, base.5, SendBidiSetRequestAsync::<Impl, OFFSET>, GetPrinterQueueView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueEventImpl: Sized + IDispatchImpl {
    fn OnBidiResponseReceived();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterQueueEvent {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterQueueEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueEventVtbl {
    pub const fn new<Impl: IPrinterQueueEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterQueueEventVtbl {
        unsafe extern "system" fn OnBidiResponseReceived<Impl: IPrinterQueueEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresponse: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnBidiResponseReceived(&*(&bstrresponse as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), hrstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterQueueEvent>, base.5, OnBidiResponseReceived::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueViewImpl: Sized + IDispatchImpl {
    fn SetViewRange();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterQueueView {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterQueueView";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueViewVtbl {
    pub const fn new<Impl: IPrinterQueueViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterQueueViewVtbl {
        unsafe extern "system" fn SetViewRange<Impl: IPrinterQueueViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewRange(ulviewoffset, ulviewsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterQueueView>, base.5, SetViewRange::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueViewEventImpl: Sized + IDispatchImpl {
    fn OnChanged();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterQueueViewEvent {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterQueueViewEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueViewEventVtbl {
    pub const fn new<Impl: IPrinterQueueViewEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterQueueViewEventVtbl {
        unsafe extern "system" fn OnChanged<Impl: IPrinterQueueViewEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcollection: ::windows::core::RawPtr, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChanged(&*(&pcollection as *const <IPrintJobCollection as ::windows::core::Abi>::Abi as *const <IPrintJobCollection as ::windows::core::DefaultType>::DefaultType), ulviewoffset, ulviewsize, ulcountjobsinprintqueue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterQueueViewEvent>, base.5, OnChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptContextImpl: Sized + IDispatchImpl {
    fn DriverProperties();
    fn QueueProperties();
    fn UserProperties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterScriptContext {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterScriptContext";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptContextVtbl {
    pub const fn new<Impl: IPrinterScriptContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterScriptContextVtbl {
        unsafe extern "system" fn DriverProperties<Impl: IPrinterScriptContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverProperties(::core::mem::transmute_copy(&pppropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueProperties<Impl: IPrinterScriptContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueueProperties(::core::mem::transmute_copy(&pppropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Impl: IPrinterScriptContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserProperties(::core::mem::transmute_copy(&pppropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterScriptContext>, base.5, DriverProperties::<Impl, OFFSET>, QueueProperties::<Impl, OFFSET>, UserProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterScriptablePropertyBag {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterScriptablePropertyBag";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBagVtbl {
    pub const fn new<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterScriptablePropertyBagVtbl {
        unsafe extern "system" fn GetBool<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBool(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBool(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bvalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt32<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pnvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInt32(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, nvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInt32(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), nvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetString(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetString(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBytes<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBytes(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pparray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytes<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, parray: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBytes(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&parray as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadStream<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReadStream(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Impl: IPrinterScriptablePropertyBagImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWriteStream(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterScriptablePropertyBag>, base.5, GetBool::<Impl, OFFSET>, SetBool::<Impl, OFFSET>, GetInt32::<Impl, OFFSET>, SetInt32::<Impl, OFFSET>, GetString::<Impl, OFFSET>, SetString::<Impl, OFFSET>, GetBytes::<Impl, OFFSET>, SetBytes::<Impl, OFFSET>, GetReadStream::<Impl, OFFSET>, GetWriteStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptablePropertyBag2Impl: Sized + IPrinterScriptablePropertyBagImpl + IDispatchImpl {
    fn GetReadStreamAsXML();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterScriptablePropertyBag2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterScriptablePropertyBag2";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag2Vtbl {
    pub const fn new<Impl: IPrinterScriptablePropertyBag2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterScriptablePropertyBag2Vtbl {
        unsafe extern "system" fn GetReadStreamAsXML<Impl: IPrinterScriptablePropertyBag2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReadStreamAsXML(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppxmlnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterScriptablePropertyBag2>, base.5, GetReadStreamAsXML::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptableSequentialStreamImpl: Sized + IDispatchImpl {
    fn Read();
    fn Write();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterScriptableSequentialStream {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterScriptableSequentialStream";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableSequentialStreamVtbl {
    pub const fn new<Impl: IPrinterScriptableSequentialStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterScriptableSequentialStreamVtbl {
        unsafe extern "system" fn Read<Impl: IPrinterScriptableSequentialStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbread: i32, pparray: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Read(cbread, ::core::mem::transmute_copy(&pparray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IPrinterScriptableSequentialStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parray: ::windows::core::RawPtr, pcbwritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Write(&*(&parray as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterScriptableSequentialStream>, base.5, Read::<Impl, OFFSET>, Write::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptableStreamImpl: Sized + IPrinterScriptableSequentialStreamImpl + IDispatchImpl {
    fn Commit();
    fn Seek();
    fn SetSize();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPrinterScriptableStream {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IPrinterScriptableStream";
}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableStreamVtbl {
    pub const fn new<Impl: IPrinterScriptableStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinterScriptableStreamVtbl {
        unsafe extern "system" fn Commit<Impl: IPrinterScriptableStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IPrinterScriptableStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Seek(loffset, streamseek, ::core::mem::transmute_copy(&plposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IPrinterScriptableStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSize(lsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinterScriptableStream>, base.5, Commit::<Impl, OFFSET>, Seek::<Impl, OFFSET>, SetSize::<Impl, OFFSET>)
    }
}
pub trait IXpsRasterizationFactoryImpl: Sized {
    fn CreateRasterizer();
}
impl ::windows::core::RuntimeName for IXpsRasterizationFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IXpsRasterizationFactory";
}
impl IXpsRasterizationFactoryVtbl {
    pub const fn new<Impl: IXpsRasterizationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsRasterizationFactoryVtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRasterizer(&*(&xpspage as *const <super::super::Storage::Xps::IXpsOMPage as ::windows::core::Abi>::Abi as *const <super::super::Storage::Xps::IXpsOMPage as ::windows::core::DefaultType>::DefaultType), dpi, nontextrenderingmode, textrenderingmode, ::core::mem::transmute_copy(&ppixpsrasterizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsRasterizationFactory>, base.5, CreateRasterizer::<Impl, OFFSET>)
    }
}
pub trait IXpsRasterizationFactory1Impl: Sized {
    fn CreateRasterizer();
}
impl ::windows::core::RuntimeName for IXpsRasterizationFactory1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IXpsRasterizationFactory1";
}
impl IXpsRasterizationFactory1Vtbl {
    pub const fn new<Impl: IXpsRasterizationFactory1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsRasterizationFactory1Vtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactory1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRasterizer(&*(&xpspage as *const <super::super::Storage::Xps::IXpsOMPage as ::windows::core::Abi>::Abi as *const <super::super::Storage::Xps::IXpsOMPage as ::windows::core::DefaultType>::DefaultType), dpi, nontextrenderingmode, textrenderingmode, pixelformat, ::core::mem::transmute_copy(&ppixpsrasterizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsRasterizationFactory1>, base.5, CreateRasterizer::<Impl, OFFSET>)
    }
}
pub trait IXpsRasterizationFactory2Impl: Sized {
    fn CreateRasterizer();
}
impl ::windows::core::RuntimeName for IXpsRasterizationFactory2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IXpsRasterizationFactory2";
}
impl IXpsRasterizationFactory2Vtbl {
    pub const fn new<Impl: IXpsRasterizationFactory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsRasterizationFactory2Vtbl {
        unsafe extern "system" fn CreateRasterizer<Impl: IXpsRasterizationFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpspage: ::windows::core::RawPtr, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRasterizer(&*(&xpspage as *const <super::super::Storage::Xps::IXpsOMPage as ::windows::core::Abi>::Abi as *const <super::super::Storage::Xps::IXpsOMPage as ::windows::core::DefaultType>::DefaultType), dpix, dpiy, nontextrenderingmode, textrenderingmode, pixelformat, backgroundcolor, ::core::mem::transmute_copy(&ppixpsrasterizer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsRasterizationFactory2>, base.5, CreateRasterizer::<Impl, OFFSET>)
    }
}
pub trait IXpsRasterizerImpl: Sized {
    fn RasterizeRect();
    fn SetMinimalLineWidth();
}
impl ::windows::core::RuntimeName for IXpsRasterizer {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IXpsRasterizer";
}
impl IXpsRasterizerVtbl {
    pub const fn new<Impl: IXpsRasterizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsRasterizerVtbl {
        unsafe extern "system" fn RasterizeRect<Impl: IXpsRasterizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: ::windows::core::RawPtr, bitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RasterizeRect(x, y, width, height, &*(&notificationcallback as *const <IXpsRasterizerNotificationCallback as ::windows::core::Abi>::Abi as *const <IXpsRasterizerNotificationCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimalLineWidth<Impl: IXpsRasterizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMinimalLineWidth(width) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsRasterizer>, base.5, RasterizeRect::<Impl, OFFSET>, SetMinimalLineWidth::<Impl, OFFSET>)
    }
}
pub trait IXpsRasterizerNotificationCallbackImpl: Sized {
    fn Continue();
}
impl ::windows::core::RuntimeName for IXpsRasterizerNotificationCallback {
    const NAME: &'static str = "Windows.Win32.Graphics.Printing.IXpsRasterizerNotificationCallback";
}
impl IXpsRasterizerNotificationCallbackVtbl {
    pub const fn new<Impl: IXpsRasterizerNotificationCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXpsRasterizerNotificationCallbackVtbl {
        unsafe extern "system" fn Continue<Impl: IXpsRasterizerNotificationCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Continue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXpsRasterizerNotificationCallback>, base.5, Continue::<Impl, OFFSET>)
    }
}
