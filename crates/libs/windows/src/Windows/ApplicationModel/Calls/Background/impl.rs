#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallBlockedTriggerDetails_Impl: Sized {
    fn PhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CallBlockedReason(&mut self) -> ::windows::core::Result<PhoneCallBlockedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallBlockedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.IPhoneCallBlockedTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallBlockedTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallBlockedTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallBlockedTriggerDetails_Vtbl {
        unsafe extern "system" fn PhoneNumber<Impl: IPhoneCallBlockedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LineId<Impl: IPhoneCallBlockedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CallBlockedReason<Impl: IPhoneCallBlockedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallBlockedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallBlockedReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallBlockedTriggerDetails, BASE_OFFSET>(),
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
            LineId: LineId::<Impl, IMPL_OFFSET>,
            CallBlockedReason: CallBlockedReason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallBlockedTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginDataRequestTriggerDetails_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOriginDataRequestTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.IPhoneCallOriginDataRequestTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOriginDataRequestTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallOriginDataRequestTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallOriginDataRequestTriggerDetails_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IPhoneCallOriginDataRequestTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneNumber<Impl: IPhoneCallOriginDataRequestTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallOriginDataRequestTriggerDetails, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallOriginDataRequestTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneIncomingCallDismissedTriggerDetails_Impl: Sized {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DismissalTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn TextReplyMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reason(&mut self) -> ::windows::core::Result<PhoneIncomingCallDismissedReason>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneIncomingCallDismissedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallDismissedTriggerDetails";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneIncomingCallDismissedTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneIncomingCallDismissedTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneIncomingCallDismissedTriggerDetails_Vtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneIncomingCallDismissedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhoneNumber<Impl: IPhoneIncomingCallDismissedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IPhoneIncomingCallDismissedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DismissalTime<Impl: IPhoneIncomingCallDismissedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DismissalTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextReplyMessage<Impl: IPhoneIncomingCallDismissedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextReplyMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IPhoneIncomingCallDismissedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneIncomingCallDismissedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneIncomingCallDismissedTriggerDetails, BASE_OFFSET>(),
            LineId: LineId::<Impl, IMPL_OFFSET>,
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            DismissalTime: DismissalTime::<Impl, IMPL_OFFSET>,
            TextReplyMessage: TextReplyMessage::<Impl, IMPL_OFFSET>,
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneIncomingCallDismissedTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneIncomingCallNotificationTriggerDetails_Impl: Sized {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CallId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneIncomingCallNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneIncomingCallNotificationTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneIncomingCallNotificationTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneIncomingCallNotificationTriggerDetails_Vtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneIncomingCallNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CallId<Impl: IPhoneIncomingCallNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneIncomingCallNotificationTriggerDetails, BASE_OFFSET>(),
            LineId: LineId::<Impl, IMPL_OFFSET>,
            CallId: CallId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneIncomingCallNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineChangedTriggerDetails_Impl: Sized {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ChangeType(&mut self) -> ::windows::core::Result<PhoneLineChangeKind>;
    fn HasLinePropertyChanged(&mut self, lineproperty: PhoneLineProperties) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineChangedTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.IPhoneLineChangedTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineChangedTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineChangedTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineChangedTriggerDetails_Vtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneLineChangedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChangeType<Impl: IPhoneLineChangedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineChangeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasLinePropertyChanged<Impl: IPhoneLineChangedTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineproperty: PhoneLineProperties, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasLinePropertyChanged(lineproperty) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineChangedTriggerDetails, BASE_OFFSET>(),
            LineId: LineId::<Impl, IMPL_OFFSET>,
            ChangeType: ChangeType::<Impl, IMPL_OFFSET>,
            HasLinePropertyChanged: HasLinePropertyChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineChangedTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNewVoicemailMessageTriggerDetails_Impl: Sized {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn VoicemailCount(&mut self) -> ::windows::core::Result<i32>;
    fn OperatorMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneNewVoicemailMessageTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Background.IPhoneNewVoicemailMessageTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneNewVoicemailMessageTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNewVoicemailMessageTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneNewVoicemailMessageTriggerDetails_Vtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneNewVoicemailMessageTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VoicemailCount<Impl: IPhoneNewVoicemailMessageTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OperatorMessage<Impl: IPhoneNewVoicemailMessageTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperatorMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneNewVoicemailMessageTriggerDetails, BASE_OFFSET>(),
            LineId: LineId::<Impl, IMPL_OFFSET>,
            VoicemailCount: VoicemailCount::<Impl, IMPL_OFFSET>,
            OperatorMessage: OperatorMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneNewVoicemailMessageTriggerDetails as ::windows::core::Interface>::IID
    }
}
