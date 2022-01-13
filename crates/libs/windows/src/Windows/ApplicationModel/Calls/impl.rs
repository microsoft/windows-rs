#[cfg(feature = "implement_exclusive")]
pub trait ICallAnswerEventArgsImpl: Sized {
    fn AcceptedMedia(&mut self) -> ::windows::core::Result<VoipPhoneCallMedia>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICallAnswerEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.ICallAnswerEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICallAnswerEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallAnswerEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallAnswerEventArgsVtbl {
        unsafe extern "system" fn AcceptedMedia<Impl: ICallAnswerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptedMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICallAnswerEventArgs, BASE_OFFSET>(), AcceptedMedia: AcceptedMedia::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallAnswerEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallRejectEventArgsImpl: Sized {
    fn RejectReason(&mut self) -> ::windows::core::Result<VoipPhoneCallRejectReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICallRejectEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.ICallRejectEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICallRejectEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallRejectEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallRejectEventArgsVtbl {
        unsafe extern "system" fn RejectReason<Impl: ICallRejectEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallRejectReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RejectReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICallRejectEventArgs, BASE_OFFSET>(), RejectReason: RejectReason::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallRejectEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallStateChangeEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<VoipPhoneCallState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICallStateChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.ICallStateChangeEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICallStateChangeEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallStateChangeEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallStateChangeEventArgsVtbl {
        unsafe extern "system" fn State<Impl: ICallStateChangeEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICallStateChangeEventArgs, BASE_OFFSET>(), State: State::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallStateChangeEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenCallEndCallDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenCallEndCallDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.ILockScreenCallEndCallDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenCallEndCallDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenCallEndCallDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenCallEndCallDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ILockScreenCallEndCallDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenCallEndCallDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenCallEndCallDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILockScreenCallEndRequestedEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<LockScreenCallEndCallDeferral>;
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenCallEndRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.ILockScreenCallEndRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILockScreenCallEndRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenCallEndRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenCallEndRequestedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: ILockScreenCallEndRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: ILockScreenCallEndRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenCallEndRequestedEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenCallEndRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILockScreenCallUIImpl: Sized {
    fn Dismiss(&mut self) -> ::windows::core::Result<()>;
    fn EndRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEndRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CallTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCallTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenCallUI {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.ILockScreenCallUI";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILockScreenCallUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenCallUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenCallUIVtbl {
        unsafe extern "system" fn Dismiss<Impl: ILockScreenCallUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dismiss().into()
        }
        unsafe extern "system" fn EndRequested<Impl: ILockScreenCallUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEndRequested<Impl: ILockScreenCallUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEndRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: ILockScreenCallUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LockScreenCallUI, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: ILockScreenCallUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CallTitle<Impl: ILockScreenCallUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallTitle<Impl: ILockScreenCallUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenCallUI, BASE_OFFSET>(),
            Dismiss: Dismiss::<Impl, IMPL_OFFSET>,
            EndRequested: EndRequested::<Impl, IMPL_OFFSET>,
            RemoveEndRequested: RemoveEndRequested::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            CallTitle: CallTitle::<Impl, IMPL_OFFSET>,
            SetCallTitle: SetCallTitle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenCallUI as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMuteChangeEventArgsImpl: Sized {
    fn Muted(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMuteChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IMuteChangeEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMuteChangeEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMuteChangeEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMuteChangeEventArgsVtbl {
        unsafe extern "system" fn Muted<Impl: IMuteChangeEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Muted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMuteChangeEventArgs, BASE_OFFSET>(), Muted: Muted::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMuteChangeEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallImpl: Sized {
    fn StatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioDeviceChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioDeviceChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsMutedChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsMutedChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CallId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsMuted(&mut self) -> ::windows::core::Result<bool>;
    fn Status(&mut self) -> ::windows::core::Result<PhoneCallStatus>;
    fn AudioDevice(&mut self) -> ::windows::core::Result<PhoneCallAudioDevice>;
    fn GetPhoneCallInfo(&mut self) -> ::windows::core::Result<PhoneCallInfo>;
    fn GetPhoneCallInfoAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallInfo>>;
    fn End(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn EndAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn SendDtmfKey(&mut self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn SendDtmfKeyAsync(&mut self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn AcceptIncoming(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn AcceptIncomingAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn Hold(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn HoldAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn ResumeFromHold(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn ResumeFromHoldAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn Mute(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn MuteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn Unmute(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn UnmuteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn RejectIncoming(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn RejectIncomingAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
    fn ChangeAudioDevice(&mut self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn ChangeAudioDeviceAsync(&mut self, endpoint: PhoneCallAudioDevice) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCall";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallVtbl {
        unsafe extern "system" fn StatusChanged<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioDeviceChanged<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDeviceChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioDeviceChanged<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioDeviceChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsMutedChanged<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMutedChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneCall, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsMutedChanged<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsMutedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CallId<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsMuted<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMuted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioDevice<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallAudioDevice) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCallInfo<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhoneCallInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCallInfoAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhoneCallInfoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDtmfKey<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDtmfKey(key, dtmftoneaudioplayback) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDtmfKeyAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDtmfKeyAsync(key, dtmftoneaudioplayback) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptIncoming<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptIncomingAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptIncomingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hold<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeFromHold<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeFromHold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeFromHoldAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeFromHoldAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mute<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MuteAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MuteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmute<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unmute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmuteAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnmuteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RejectIncoming<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RejectIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RejectIncomingAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RejectIncomingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeAudioDevice<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioDevice, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeAudioDevice(endpoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeAudioDeviceAsync<Impl: IPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: PhoneCallAudioDevice, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeAudioDeviceAsync(endpoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCall, BASE_OFFSET>(),
            StatusChanged: StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged: RemoveStatusChanged::<Impl, IMPL_OFFSET>,
            AudioDeviceChanged: AudioDeviceChanged::<Impl, IMPL_OFFSET>,
            RemoveAudioDeviceChanged: RemoveAudioDeviceChanged::<Impl, IMPL_OFFSET>,
            IsMutedChanged: IsMutedChanged::<Impl, IMPL_OFFSET>,
            RemoveIsMutedChanged: RemoveIsMutedChanged::<Impl, IMPL_OFFSET>,
            CallId: CallId::<Impl, IMPL_OFFSET>,
            IsMuted: IsMuted::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            AudioDevice: AudioDevice::<Impl, IMPL_OFFSET>,
            GetPhoneCallInfo: GetPhoneCallInfo::<Impl, IMPL_OFFSET>,
            GetPhoneCallInfoAsync: GetPhoneCallInfoAsync::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            EndAsync: EndAsync::<Impl, IMPL_OFFSET>,
            SendDtmfKey: SendDtmfKey::<Impl, IMPL_OFFSET>,
            SendDtmfKeyAsync: SendDtmfKeyAsync::<Impl, IMPL_OFFSET>,
            AcceptIncoming: AcceptIncoming::<Impl, IMPL_OFFSET>,
            AcceptIncomingAsync: AcceptIncomingAsync::<Impl, IMPL_OFFSET>,
            Hold: Hold::<Impl, IMPL_OFFSET>,
            HoldAsync: HoldAsync::<Impl, IMPL_OFFSET>,
            ResumeFromHold: ResumeFromHold::<Impl, IMPL_OFFSET>,
            ResumeFromHoldAsync: ResumeFromHoldAsync::<Impl, IMPL_OFFSET>,
            Mute: Mute::<Impl, IMPL_OFFSET>,
            MuteAsync: MuteAsync::<Impl, IMPL_OFFSET>,
            Unmute: Unmute::<Impl, IMPL_OFFSET>,
            UnmuteAsync: UnmuteAsync::<Impl, IMPL_OFFSET>,
            RejectIncoming: RejectIncoming::<Impl, IMPL_OFFSET>,
            RejectIncomingAsync: RejectIncomingAsync::<Impl, IMPL_OFFSET>,
            ChangeAudioDevice: ChangeAudioDevice::<Impl, IMPL_OFFSET>,
            ChangeAudioDeviceAsync: ChangeAudioDeviceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCall as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhoneCallBlockingStaticsImpl: Sized {
    fn BlockUnknownNumbers(&mut self) -> ::windows::core::Result<bool>;
    fn SetBlockUnknownNumbers(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BlockPrivateNumbers(&mut self) -> ::windows::core::Result<bool>;
    fn SetBlockPrivateNumbers(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SetCallBlockingListAsync(&mut self, phonenumberlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallBlockingStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallBlockingStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhoneCallBlockingStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallBlockingStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallBlockingStaticsVtbl {
        unsafe extern "system" fn BlockUnknownNumbers<Impl: IPhoneCallBlockingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockUnknownNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockUnknownNumbers<Impl: IPhoneCallBlockingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlockUnknownNumbers(value).into()
        }
        unsafe extern "system" fn BlockPrivateNumbers<Impl: IPhoneCallBlockingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockPrivateNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockPrivateNumbers<Impl: IPhoneCallBlockingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlockPrivateNumbers(value).into()
        }
        unsafe extern "system" fn SetCallBlockingListAsync<Impl: IPhoneCallBlockingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumberlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCallBlockingListAsync(&*(&phonenumberlist as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallBlockingStatics, BASE_OFFSET>(),
            BlockUnknownNumbers: BlockUnknownNumbers::<Impl, IMPL_OFFSET>,
            SetBlockUnknownNumbers: SetBlockUnknownNumbers::<Impl, IMPL_OFFSET>,
            BlockPrivateNumbers: BlockPrivateNumbers::<Impl, IMPL_OFFSET>,
            SetBlockPrivateNumbers: SetBlockPrivateNumbers::<Impl, IMPL_OFFSET>,
            SetCallBlockingListAsync: SetCallBlockingListAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallBlockingStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallHistoryEntryImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&mut self) -> ::windows::core::Result<PhoneCallHistoryEntryAddress>;
    fn SetAddress(&mut self, value: &::core::option::Option<PhoneCallHistoryEntryAddress>) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetDuration(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn IsCallerIdBlocked(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCallerIdBlocked(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsEmergency(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEmergency(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsIncoming(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsIncoming(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsMissed(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsMissed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRinging(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRinging(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeen(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSeen(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsSuppressed(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSuppressed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsVoicemail(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsVoicemail(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Media(&mut self) -> ::windows::core::Result<PhoneCallHistoryEntryMedia>;
    fn SetMedia(&mut self, value: PhoneCallHistoryEntryMedia) -> ::windows::core::Result<()>;
    fn OtherAppReadAccess(&mut self) -> ::windows::core::Result<PhoneCallHistoryEntryOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&mut self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn RemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSourceId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceIdKind(&mut self) -> ::windows::core::Result<PhoneCallHistorySourceIdKind>;
    fn SetSourceIdKind(&mut self, value: PhoneCallHistorySourceIdKind) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallHistoryEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryEntry";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallHistoryEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryEntryVtbl {
        unsafe extern "system" fn Id<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(&*(&value as *const <PhoneCallHistoryEntryAddress as ::windows::core::Abi>::Abi as *const <PhoneCallHistoryEntryAddress as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCallerIdBlocked<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallerIdBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCallerIdBlocked<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCallerIdBlocked(value).into()
        }
        unsafe extern "system" fn IsEmergency<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEmergency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEmergency<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEmergency(value).into()
        }
        unsafe extern "system" fn IsIncoming<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsIncoming<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsIncoming(value).into()
        }
        unsafe extern "system" fn IsMissed<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMissed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMissed<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMissed(value).into()
        }
        unsafe extern "system" fn IsRinging<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRinging() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRinging<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRinging(value).into()
        }
        unsafe extern "system" fn IsSeen<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSeen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSeen<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSeen(value).into()
        }
        unsafe extern "system" fn IsSuppressed<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSuppressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSuppressed<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSuppressed(value).into()
        }
        unsafe extern "system" fn IsVoicemail<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVoicemail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsVoicemail<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVoicemail(value).into()
        }
        unsafe extern "system" fn Media<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Media() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMedia<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMedia(value).into()
        }
        unsafe extern "system" fn OtherAppReadAccess<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAppReadAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherAppReadAccess<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppReadAccess(value).into()
        }
        unsafe extern "system" fn RemoteId<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceDisplayName<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceId<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceId<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceIdKind<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistorySourceIdKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceIdKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceIdKind<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneCallHistorySourceIdKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceIdKind(value).into()
        }
        unsafe extern "system" fn StartTime<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IPhoneCallHistoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryEntry, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            SetAddress: SetAddress::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            IsCallerIdBlocked: IsCallerIdBlocked::<Impl, IMPL_OFFSET>,
            SetIsCallerIdBlocked: SetIsCallerIdBlocked::<Impl, IMPL_OFFSET>,
            IsEmergency: IsEmergency::<Impl, IMPL_OFFSET>,
            SetIsEmergency: SetIsEmergency::<Impl, IMPL_OFFSET>,
            IsIncoming: IsIncoming::<Impl, IMPL_OFFSET>,
            SetIsIncoming: SetIsIncoming::<Impl, IMPL_OFFSET>,
            IsMissed: IsMissed::<Impl, IMPL_OFFSET>,
            SetIsMissed: SetIsMissed::<Impl, IMPL_OFFSET>,
            IsRinging: IsRinging::<Impl, IMPL_OFFSET>,
            SetIsRinging: SetIsRinging::<Impl, IMPL_OFFSET>,
            IsSeen: IsSeen::<Impl, IMPL_OFFSET>,
            SetIsSeen: SetIsSeen::<Impl, IMPL_OFFSET>,
            IsSuppressed: IsSuppressed::<Impl, IMPL_OFFSET>,
            SetIsSuppressed: SetIsSuppressed::<Impl, IMPL_OFFSET>,
            IsVoicemail: IsVoicemail::<Impl, IMPL_OFFSET>,
            SetIsVoicemail: SetIsVoicemail::<Impl, IMPL_OFFSET>,
            Media: Media::<Impl, IMPL_OFFSET>,
            SetMedia: SetMedia::<Impl, IMPL_OFFSET>,
            OtherAppReadAccess: OtherAppReadAccess::<Impl, IMPL_OFFSET>,
            SetOtherAppReadAccess: SetOtherAppReadAccess::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
            SourceDisplayName: SourceDisplayName::<Impl, IMPL_OFFSET>,
            SourceId: SourceId::<Impl, IMPL_OFFSET>,
            SetSourceId: SetSourceId::<Impl, IMPL_OFFSET>,
            SourceIdKind: SourceIdKind::<Impl, IMPL_OFFSET>,
            SetSourceIdKind: SetSourceIdKind::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryEntryAddressImpl: Sized {
    fn ContactId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RawAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRawAddress(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RawAddressKind(&mut self) -> ::windows::core::Result<PhoneCallHistoryEntryRawAddressKind>;
    fn SetRawAddressKind(&mut self, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallHistoryEntryAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddress";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallHistoryEntryAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryEntryAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryEntryAddressVtbl {
        unsafe extern "system" fn ContactId<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContactId<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RawAddress<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRawAddress<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRawAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RawAddressKind<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawAddressKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRawAddressKind<Impl: IPhoneCallHistoryEntryAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRawAddressKind(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryEntryAddress, BASE_OFFSET>(),
            ContactId: ContactId::<Impl, IMPL_OFFSET>,
            SetContactId: SetContactId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            RawAddress: RawAddress::<Impl, IMPL_OFFSET>,
            SetRawAddress: SetRawAddress::<Impl, IMPL_OFFSET>,
            RawAddressKind: RawAddressKind::<Impl, IMPL_OFFSET>,
            SetRawAddressKind: SetRawAddressKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryEntryAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallHistoryEntryAddressFactoryImpl: Sized {
    fn Create(&mut self, rawaddress: &::windows::core::HSTRING, rawaddresskind: PhoneCallHistoryEntryRawAddressKind) -> ::windows::core::Result<PhoneCallHistoryEntryAddress>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallHistoryEntryAddressFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddressFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallHistoryEntryAddressFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryEntryAddressFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryEntryAddressFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPhoneCallHistoryEntryAddressFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rawaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rawaddresskind: PhoneCallHistoryEntryRawAddressKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&rawaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), rawaddresskind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryEntryAddressFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryEntryAddressFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhoneCallHistoryEntryQueryOptionsImpl: Sized {
    fn DesiredMedia(&mut self) -> ::windows::core::Result<PhoneCallHistoryEntryQueryDesiredMedia>;
    fn SetDesiredMedia(&mut self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::Result<()>;
    fn SourceIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallHistoryEntryQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryQueryOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhoneCallHistoryEntryQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryEntryQueryOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryEntryQueryOptionsVtbl {
        unsafe extern "system" fn DesiredMedia<Impl: IPhoneCallHistoryEntryQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredMedia<Impl: IPhoneCallHistoryEntryQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredMedia(value).into()
        }
        unsafe extern "system" fn SourceIds<Impl: IPhoneCallHistoryEntryQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryEntryQueryOptions, BASE_OFFSET>(),
            DesiredMedia: DesiredMedia::<Impl, IMPL_OFFSET>,
            SetDesiredMedia: SetDesiredMedia::<Impl, IMPL_OFFSET>,
            SourceIds: SourceIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryEntryQueryOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhoneCallHistoryEntryReaderImpl: Sized {
    fn ReadBatchAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallHistoryEntryReader {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhoneCallHistoryEntryReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryEntryReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryEntryReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IPhoneCallHistoryEntryReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryEntryReader, BASE_OFFSET>(),
            ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryEntryReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IPhoneCallHistoryManagerForUserImpl: Sized {
    fn RequestStoreAsync(&mut self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>;
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallHistoryManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerForUser";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IPhoneCallHistoryManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryManagerForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryManagerForUserVtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IPhoneCallHistoryManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IPhoneCallHistoryManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryManagerForUser, BASE_OFFSET>(),
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallHistoryManagerStaticsImpl: Sized {
    fn RequestStoreAsync(&mut self, accesstype: PhoneCallHistoryStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallHistoryManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallHistoryManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryManagerStaticsVtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IPhoneCallHistoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryManagerStatics, BASE_OFFSET>(),
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IPhoneCallHistoryManagerStatics2Impl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<PhoneCallHistoryManagerForUser>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallHistoryManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IPhoneCallHistoryManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IPhoneCallHistoryManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryManagerStatics2, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhoneCallHistoryStoreImpl: Sized {
    fn GetEntryAsync(&mut self, callhistoryentryid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>>;
    fn GetEntryReader(&mut self) -> ::windows::core::Result<PhoneCallHistoryEntryReader>;
    fn GetEntryReaderWithOptions(&mut self, queryoptions: &::core::option::Option<PhoneCallHistoryEntryQueryOptions>) -> ::windows::core::Result<PhoneCallHistoryEntryReader>;
    fn SaveEntryAsync(&mut self, callhistoryentry: &::core::option::Option<PhoneCallHistoryEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteEntryAsync(&mut self, callhistoryentry: &::core::option::Option<PhoneCallHistoryEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteEntriesAsync(&mut self, callhistoryentries: &::core::option::Option<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkEntryAsSeenAsync(&mut self, callhistoryentry: &::core::option::Option<PhoneCallHistoryEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkEntriesAsSeenAsync(&mut self, callhistoryentries: &::core::option::Option<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetUnseenCountAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn MarkAllAsSeenAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetSourcesUnseenCountAsync(&mut self, sourceids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn MarkSourcesAsSeenAsync(&mut self, sourceids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallHistoryStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallHistoryStore";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhoneCallHistoryStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallHistoryStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallHistoryStoreVtbl {
        unsafe extern "system" fn GetEntryAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callhistoryentryid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntryAsync(&*(&callhistoryentryid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntryReader<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntryReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntryReaderWithOptions<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntryReaderWithOptions(&*(&queryoptions as *const <PhoneCallHistoryEntryQueryOptions as ::windows::core::Abi>::Abi as *const <PhoneCallHistoryEntryQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveEntryAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveEntryAsync(&*(&callhistoryentry as *const <PhoneCallHistoryEntry as ::windows::core::Abi>::Abi as *const <PhoneCallHistoryEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntryAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteEntryAsync(&*(&callhistoryentry as *const <PhoneCallHistoryEntry as ::windows::core::Abi>::Abi as *const <PhoneCallHistoryEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntriesAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callhistoryentries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteEntriesAsync(&*(&callhistoryentries as *const <super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkEntryAsSeenAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callhistoryentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkEntryAsSeenAsync(&*(&callhistoryentry as *const <PhoneCallHistoryEntry as ::windows::core::Abi>::Abi as *const <PhoneCallHistoryEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkEntriesAsSeenAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callhistoryentries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkEntriesAsSeenAsync(&*(&callhistoryentries as *const <super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnseenCountAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnseenCountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAllAsSeenAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkAllAsSeenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourcesUnseenCountAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourcesUnseenCountAsync(&*(&sourceids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkSourcesAsSeenAsync<Impl: IPhoneCallHistoryStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarkSourcesAsSeenAsync(&*(&sourceids as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallHistoryStore, BASE_OFFSET>(),
            GetEntryAsync: GetEntryAsync::<Impl, IMPL_OFFSET>,
            GetEntryReader: GetEntryReader::<Impl, IMPL_OFFSET>,
            GetEntryReaderWithOptions: GetEntryReaderWithOptions::<Impl, IMPL_OFFSET>,
            SaveEntryAsync: SaveEntryAsync::<Impl, IMPL_OFFSET>,
            DeleteEntryAsync: DeleteEntryAsync::<Impl, IMPL_OFFSET>,
            DeleteEntriesAsync: DeleteEntriesAsync::<Impl, IMPL_OFFSET>,
            MarkEntryAsSeenAsync: MarkEntryAsSeenAsync::<Impl, IMPL_OFFSET>,
            MarkEntriesAsSeenAsync: MarkEntriesAsSeenAsync::<Impl, IMPL_OFFSET>,
            GetUnseenCountAsync: GetUnseenCountAsync::<Impl, IMPL_OFFSET>,
            MarkAllAsSeenAsync: MarkAllAsSeenAsync::<Impl, IMPL_OFFSET>,
            GetSourcesUnseenCountAsync: GetSourcesUnseenCountAsync::<Impl, IMPL_OFFSET>,
            MarkSourcesAsSeenAsync: MarkSourcesAsSeenAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallHistoryStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallInfoImpl: Sized {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsHoldSupported(&mut self) -> ::windows::core::Result<bool>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CallDirection(&mut self) -> ::windows::core::Result<PhoneCallDirection>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallInfoVtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsHoldSupported<Impl: IPhoneCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHoldSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: IPhoneCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneNumber<Impl: IPhoneCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IPhoneCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CallDirection<Impl: IPhoneCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallInfo, BASE_OFFSET>(),
            LineId: LineId::<Impl, IMPL_OFFSET>,
            IsHoldSupported: IsHoldSupported::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            CallDirection: CallDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallManagerStaticsImpl: Sized {
    fn ShowPhoneCallUI(&mut self, phonenumber: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallManagerStaticsVtbl {
        unsafe extern "system" fn ShowPhoneCallUI<Impl: IPhoneCallManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPhoneCallUI(&*(&phonenumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallManagerStatics, BASE_OFFSET>(),
            ShowPhoneCallUI: ShowPhoneCallUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallManagerStatics2Impl: Sized {
    fn CallStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCallStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsCallActive(&mut self) -> ::windows::core::Result<bool>;
    fn IsCallIncoming(&mut self) -> ::windows::core::Result<bool>;
    fn ShowPhoneCallSettingsUI(&mut self) -> ::windows::core::Result<()>;
    fn RequestStoreAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallStore>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallManagerStatics2Vtbl {
        unsafe extern "system" fn CallStateChanged<Impl: IPhoneCallManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallStateChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCallStateChanged<Impl: IPhoneCallManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCallStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCallActive<Impl: IPhoneCallManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallIncoming<Impl: IPhoneCallManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPhoneCallSettingsUI<Impl: IPhoneCallManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPhoneCallSettingsUI().into()
        }
        unsafe extern "system" fn RequestStoreAsync<Impl: IPhoneCallManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallManagerStatics2, BASE_OFFSET>(),
            CallStateChanged: CallStateChanged::<Impl, IMPL_OFFSET>,
            RemoveCallStateChanged: RemoveCallStateChanged::<Impl, IMPL_OFFSET>,
            IsCallActive: IsCallActive::<Impl, IMPL_OFFSET>,
            IsCallIncoming: IsCallIncoming::<Impl, IMPL_OFFSET>,
            ShowPhoneCallSettingsUI: ShowPhoneCallSettingsUI::<Impl, IMPL_OFFSET>,
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallStaticsImpl: Sized {
    fn GetFromId(&mut self, callid: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneCall>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallStaticsVtbl {
        unsafe extern "system" fn GetFromId<Impl: IPhoneCallStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromId(&*(&callid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallStatics, BASE_OFFSET>(), GetFromId: GetFromId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallStoreImpl: Sized {
    fn IsEmergencyPhoneNumberAsync(&mut self, number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetDefaultLineAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::GUID>>;
    fn RequestLineWatcher(&mut self) -> ::windows::core::Result<PhoneLineWatcher>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallStore";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallStoreVtbl {
        unsafe extern "system" fn IsEmergencyPhoneNumberAsync<Impl: IPhoneCallStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEmergencyPhoneNumberAsync(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultLineAsync<Impl: IPhoneCallStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultLineAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestLineWatcher<Impl: IPhoneCallStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLineWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallStore, BASE_OFFSET>(),
            IsEmergencyPhoneNumberAsync: IsEmergencyPhoneNumberAsync::<Impl, IMPL_OFFSET>,
            GetDefaultLineAsync: GetDefaultLineAsync::<Impl, IMPL_OFFSET>,
            RequestLineWatcher: RequestLineWatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallVideoCapabilitiesImpl: Sized {
    fn IsVideoCallingCapable(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallVideoCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallVideoCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallVideoCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallVideoCapabilitiesVtbl {
        unsafe extern "system" fn IsVideoCallingCapable<Impl: IPhoneCallVideoCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoCallingCapable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallVideoCapabilities, BASE_OFFSET>(),
            IsVideoCallingCapable: IsVideoCallingCapable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallVideoCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallVideoCapabilitiesManagerStaticsImpl: Sized {
    fn GetCapabilitiesAsync(&mut self, phonenumber: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallVideoCapabilitiesManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilitiesManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallVideoCapabilitiesManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallVideoCapabilitiesManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallVideoCapabilitiesManagerStaticsVtbl {
        unsafe extern "system" fn GetCapabilitiesAsync<Impl: IPhoneCallVideoCapabilitiesManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonenumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilitiesAsync(&*(&phonenumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallVideoCapabilitiesManagerStatics, BASE_OFFSET>(),
            GetCapabilitiesAsync: GetCapabilitiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallVideoCapabilitiesManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhoneCallsResultImpl: Sized {
    fn OperationStatus(&mut self) -> ::windows::core::Result<PhoneLineOperationStatus>;
    fn AllActivePhoneCalls(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhoneCall>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallsResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneCallsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhoneCallsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallsResultVtbl {
        unsafe extern "system" fn OperationStatus<Impl: IPhoneCallsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllActivePhoneCalls<Impl: IPhoneCallsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllActivePhoneCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallsResult, BASE_OFFSET>(),
            OperationStatus: OperationStatus::<Impl, IMPL_OFFSET>,
            AllActivePhoneCalls: AllActivePhoneCalls::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Contacts", feature = "implement_exclusive"))]
pub trait IPhoneDialOptionsImpl: Sized {
    fn Number(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumber(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Contact(&mut self) -> ::windows::core::Result<super::Contacts::Contact>;
    fn SetContact(&mut self, value: &::core::option::Option<super::Contacts::Contact>) -> ::windows::core::Result<()>;
    fn ContactPhone(&mut self) -> ::windows::core::Result<super::Contacts::ContactPhone>;
    fn SetContactPhone(&mut self, value: &::core::option::Option<super::Contacts::ContactPhone>) -> ::windows::core::Result<()>;
    fn Media(&mut self) -> ::windows::core::Result<PhoneCallMedia>;
    fn SetMedia(&mut self, value: PhoneCallMedia) -> ::windows::core::Result<()>;
    fn AudioEndpoint(&mut self) -> ::windows::core::Result<PhoneAudioRoutingEndpoint>;
    fn SetAudioEndpoint(&mut self, value: PhoneAudioRoutingEndpoint) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Contacts", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneDialOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneDialOptions";
}
#[cfg(all(feature = "ApplicationModel_Contacts", feature = "implement_exclusive"))]
impl IPhoneDialOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneDialOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneDialOptionsVtbl {
        unsafe extern "system" fn Number<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Number() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumber<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Contact<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContact<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContact(&*(&value as *const <super::Contacts::Contact as ::windows::core::Abi>::Abi as *const <super::Contacts::Contact as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContactPhone<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactPhone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContactPhone<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactPhone(&*(&value as *const <super::Contacts::ContactPhone as ::windows::core::Abi>::Abi as *const <super::Contacts::ContactPhone as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Media<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Media() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMedia<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneCallMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMedia(value).into()
        }
        unsafe extern "system" fn AudioEndpoint<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneAudioRoutingEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEndpoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEndpoint<Impl: IPhoneDialOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PhoneAudioRoutingEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioEndpoint(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneDialOptions, BASE_OFFSET>(),
            Number: Number::<Impl, IMPL_OFFSET>,
            SetNumber: SetNumber::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Contact: Contact::<Impl, IMPL_OFFSET>,
            SetContact: SetContact::<Impl, IMPL_OFFSET>,
            ContactPhone: ContactPhone::<Impl, IMPL_OFFSET>,
            SetContactPhone: SetContactPhone::<Impl, IMPL_OFFSET>,
            Media: Media::<Impl, IMPL_OFFSET>,
            SetMedia: SetMedia::<Impl, IMPL_OFFSET>,
            AudioEndpoint: AudioEndpoint::<Impl, IMPL_OFFSET>,
            SetAudioEndpoint: SetAudioEndpoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneDialOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
pub trait IPhoneLineImpl: Sized {
    fn LineChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLine, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DisplayColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn NetworkState(&mut self) -> ::windows::core::Result<PhoneNetworkState>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Voicemail(&mut self) -> ::windows::core::Result<PhoneVoicemail>;
    fn NetworkName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularDetails(&mut self) -> ::windows::core::Result<PhoneLineCellularDetails>;
    fn Transport(&mut self) -> ::windows::core::Result<PhoneLineTransport>;
    fn CanDial(&mut self) -> ::windows::core::Result<bool>;
    fn SupportsTile(&mut self) -> ::windows::core::Result<bool>;
    fn VideoCallingCapabilities(&mut self) -> ::windows::core::Result<PhoneCallVideoCapabilities>;
    fn LineConfiguration(&mut self) -> ::windows::core::Result<PhoneLineConfiguration>;
    fn IsImmediateDialNumberAsync(&mut self, number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Dial(&mut self, number: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DialWithOptions(&mut self, options: &::core::option::Option<PhoneDialOptions>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneLine {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLine";
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl IPhoneLineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineVtbl {
        unsafe extern "system" fn LineChanged<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLine, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLine, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLineChanged<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLineChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayColor<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkState<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneNetworkState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Voicemail<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Voicemail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkName<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularDetails<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellularDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transport<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineTransport) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDial<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsTile<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsTile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoCallingCapabilities<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoCallingCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineConfiguration<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsImmediateDialNumberAsync<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsImmediateDialNumberAsync(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dial<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dial(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DialWithOptions<Impl: IPhoneLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DialWithOptions(&*(&options as *const <PhoneDialOptions as ::windows::core::Abi>::Abi as *const <PhoneDialOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLine, BASE_OFFSET>(),
            LineChanged: LineChanged::<Impl, IMPL_OFFSET>,
            RemoveLineChanged: RemoveLineChanged::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayColor: DisplayColor::<Impl, IMPL_OFFSET>,
            NetworkState: NetworkState::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Voicemail: Voicemail::<Impl, IMPL_OFFSET>,
            NetworkName: NetworkName::<Impl, IMPL_OFFSET>,
            CellularDetails: CellularDetails::<Impl, IMPL_OFFSET>,
            Transport: Transport::<Impl, IMPL_OFFSET>,
            CanDial: CanDial::<Impl, IMPL_OFFSET>,
            SupportsTile: SupportsTile::<Impl, IMPL_OFFSET>,
            VideoCallingCapabilities: VideoCallingCapabilities::<Impl, IMPL_OFFSET>,
            LineConfiguration: LineConfiguration::<Impl, IMPL_OFFSET>,
            IsImmediateDialNumberAsync: IsImmediateDialNumberAsync::<Impl, IMPL_OFFSET>,
            Dial: Dial::<Impl, IMPL_OFFSET>,
            DialWithOptions: DialWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLine2Impl: Sized {
    fn EnableTextReply(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TransportDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLine2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLine2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLine2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLine2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLine2Vtbl {
        unsafe extern "system" fn EnableTextReply<Impl: IPhoneLine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTextReply(value).into()
        }
        unsafe extern "system" fn TransportDeviceId<Impl: IPhoneLine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLine2, BASE_OFFSET>(),
            EnableTextReply: EnableTextReply::<Impl, IMPL_OFFSET>,
            TransportDeviceId: TransportDeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLine2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneLine3Impl: Sized {
    fn DialWithResult(&mut self, number: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneLineDialResult>;
    fn DialWithResultAsync(&mut self, number: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>>;
    fn GetAllActivePhoneCalls(&mut self) -> ::windows::core::Result<PhoneCallsResult>;
    fn GetAllActivePhoneCallsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneCallsResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneLine3 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLine3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneLine3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLine3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLine3Vtbl {
        unsafe extern "system" fn DialWithResult<Impl: IPhoneLine3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialWithResult(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialWithResultAsync<Impl: IPhoneLine3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialWithResultAsync(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllActivePhoneCalls<Impl: IPhoneLine3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllActivePhoneCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllActivePhoneCallsAsync<Impl: IPhoneLine3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllActivePhoneCallsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLine3, BASE_OFFSET>(),
            DialWithResult: DialWithResult::<Impl, IMPL_OFFSET>,
            DialWithResultAsync: DialWithResultAsync::<Impl, IMPL_OFFSET>,
            GetAllActivePhoneCalls: GetAllActivePhoneCalls::<Impl, IMPL_OFFSET>,
            GetAllActivePhoneCallsAsync: GetAllActivePhoneCallsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLine3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineCellularDetailsImpl: Sized {
    fn SimState(&mut self) -> ::windows::core::Result<PhoneSimState>;
    fn SimSlotIndex(&mut self) -> ::windows::core::Result<i32>;
    fn IsModemOn(&mut self) -> ::windows::core::Result<bool>;
    fn RegistrationRejectCode(&mut self) -> ::windows::core::Result<i32>;
    fn GetNetworkOperatorDisplayText(&mut self, location: PhoneLineNetworkOperatorDisplayTextLocation) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineCellularDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineCellularDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineCellularDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineCellularDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineCellularDetailsVtbl {
        unsafe extern "system" fn SimState<Impl: IPhoneLineCellularDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneSimState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimSlotIndex<Impl: IPhoneLineCellularDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimSlotIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsModemOn<Impl: IPhoneLineCellularDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsModemOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistrationRejectCode<Impl: IPhoneLineCellularDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationRejectCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkOperatorDisplayText<Impl: IPhoneLineCellularDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: PhoneLineNetworkOperatorDisplayTextLocation, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkOperatorDisplayText(location) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineCellularDetails, BASE_OFFSET>(),
            SimState: SimState::<Impl, IMPL_OFFSET>,
            SimSlotIndex: SimSlotIndex::<Impl, IMPL_OFFSET>,
            IsModemOn: IsModemOn::<Impl, IMPL_OFFSET>,
            RegistrationRejectCode: RegistrationRejectCode::<Impl, IMPL_OFFSET>,
            GetNetworkOperatorDisplayText: GetNetworkOperatorDisplayText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineCellularDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPhoneLineConfigurationImpl: Sized {
    fn IsVideoCallingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneLineConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineConfiguration";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPhoneLineConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineConfigurationVtbl {
        unsafe extern "system" fn IsVideoCallingEnabled<Impl: IPhoneLineConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoCallingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IPhoneLineConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineConfiguration, BASE_OFFSET>(),
            IsVideoCallingEnabled: IsVideoCallingEnabled::<Impl, IMPL_OFFSET>,
            ExtendedProperties: ExtendedProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineDialResultImpl: Sized {
    fn DialCallStatus(&mut self) -> ::windows::core::Result<PhoneCallOperationStatus>;
    fn DialedCall(&mut self) -> ::windows::core::Result<PhoneCall>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineDialResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineDialResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineDialResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineDialResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineDialResultVtbl {
        unsafe extern "system" fn DialCallStatus<Impl: IPhoneLineDialResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneCallOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialCallStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialedCall<Impl: IPhoneLineDialResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialedCall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineDialResult, BASE_OFFSET>(),
            DialCallStatus: DialCallStatus::<Impl, IMPL_OFFSET>,
            DialedCall: DialedCall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineDialResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneLineStaticsImpl: Sized {
    fn FromIdAsync(&mut self, lineid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhoneLine>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneLineStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneLineStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IPhoneLineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&lineid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineStatics, BASE_OFFSET>(), FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IPhoneLineTransportDeviceImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transport(&mut self) -> ::windows::core::Result<PhoneLineTransport>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>>;
    fn RegisterApp(&mut self) -> ::windows::core::Result<()>;
    fn RegisterAppForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<()>;
    fn UnregisterApp(&mut self) -> ::windows::core::Result<()>;
    fn UnregisterAppForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<()>;
    fn IsRegistered(&mut self) -> ::windows::core::Result<bool>;
    fn Connect(&mut self) -> ::windows::core::Result<bool>;
    fn ConnectAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneLineTransportDevice {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineTransportDevice";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IPhoneLineTransportDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineTransportDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineTransportDeviceVtbl {
        unsafe extern "system" fn DeviceId<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transport<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineTransport) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterApp<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterApp().into()
        }
        unsafe extern "system" fn RegisterAppForUser<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterAppForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterApp<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterApp().into()
        }
        unsafe extern "system" fn UnregisterAppForUser<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAppForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsRegistered<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRegistered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Impl: IPhoneLineTransportDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineTransportDevice, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Transport: Transport::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            RegisterApp: RegisterApp::<Impl, IMPL_OFFSET>,
            RegisterAppForUser: RegisterAppForUser::<Impl, IMPL_OFFSET>,
            UnregisterApp: UnregisterApp::<Impl, IMPL_OFFSET>,
            UnregisterAppForUser: UnregisterAppForUser::<Impl, IMPL_OFFSET>,
            IsRegistered: IsRegistered::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineTransportDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneLineTransportDevice2Impl: Sized {
    fn AudioRoutingStatus(&mut self) -> ::windows::core::Result<TransportDeviceAudioRoutingStatus>;
    fn AudioRoutingStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioRoutingStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InBandRingingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn InBandRingingEnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInBandRingingEnabledChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneLineTransportDevice2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineTransportDevice2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneLineTransportDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineTransportDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineTransportDevice2Vtbl {
        unsafe extern "system" fn AudioRoutingStatus<Impl: IPhoneLineTransportDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TransportDeviceAudioRoutingStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioRoutingStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioRoutingStatusChanged<Impl: IPhoneLineTransportDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioRoutingStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioRoutingStatusChanged<Impl: IPhoneLineTransportDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioRoutingStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InBandRingingEnabled<Impl: IPhoneLineTransportDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InBandRingingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InBandRingingEnabledChanged<Impl: IPhoneLineTransportDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InBandRingingEnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInBandRingingEnabledChanged<Impl: IPhoneLineTransportDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInBandRingingEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineTransportDevice2, BASE_OFFSET>(),
            AudioRoutingStatus: AudioRoutingStatus::<Impl, IMPL_OFFSET>,
            AudioRoutingStatusChanged: AudioRoutingStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveAudioRoutingStatusChanged: RemoveAudioRoutingStatusChanged::<Impl, IMPL_OFFSET>,
            InBandRingingEnabled: InBandRingingEnabled::<Impl, IMPL_OFFSET>,
            InBandRingingEnabledChanged: InBandRingingEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveInBandRingingEnabledChanged: RemoveInBandRingingEnabledChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineTransportDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineTransportDeviceStaticsImpl: Sized {
    fn FromId(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneLineTransportDevice>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForPhoneLineTransport(&mut self, transport: PhoneLineTransport) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineTransportDeviceStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineTransportDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineTransportDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineTransportDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineTransportDeviceStaticsVtbl {
        unsafe extern "system" fn FromId<Impl: IPhoneLineTransportDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IPhoneLineTransportDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForPhoneLineTransport<Impl: IPhoneLineTransportDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: PhoneLineTransport, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForPhoneLineTransport(transport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineTransportDeviceStatics, BASE_OFFSET>(),
            FromId: FromId::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForPhoneLineTransport: GetDeviceSelectorForPhoneLineTransport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineTransportDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneLineWatcherImpl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn LineAdded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LineRemoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LineUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLineUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<PhoneLineWatcherStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneLineWatcher {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneLineWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineWatcherVtbl {
        unsafe extern "system" fn Start<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn LineAdded<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLineAdded<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLineAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineRemoved<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLineRemoved<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLineRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineUpdated<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLineUpdated<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLineUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PhoneLineWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPhoneLineWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneLineWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineWatcher, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            LineAdded: LineAdded::<Impl, IMPL_OFFSET>,
            RemoveLineAdded: RemoveLineAdded::<Impl, IMPL_OFFSET>,
            LineRemoved: LineRemoved::<Impl, IMPL_OFFSET>,
            RemoveLineRemoved: RemoveLineRemoved::<Impl, IMPL_OFFSET>,
            LineUpdated: LineUpdated::<Impl, IMPL_OFFSET>,
            RemoveLineUpdated: RemoveLineUpdated::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneLineWatcherEventArgsImpl: Sized {
    fn LineId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneLineWatcherEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneLineWatcherEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneLineWatcherEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneLineWatcherEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneLineWatcherEventArgsVtbl {
        unsafe extern "system" fn LineId<Impl: IPhoneLineWatcherEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneLineWatcherEventArgs, BASE_OFFSET>(), LineId: LineId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneLineWatcherEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneVoicemailImpl: Sized {
    fn Number(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MessageCount(&mut self) -> ::windows::core::Result<i32>;
    fn Type(&mut self) -> ::windows::core::Result<PhoneVoicemailType>;
    fn DialVoicemailAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneVoicemail {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IPhoneVoicemail";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneVoicemailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneVoicemailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneVoicemailVtbl {
        unsafe extern "system" fn Number<Impl: IPhoneVoicemailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Number() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Impl: IPhoneVoicemailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IPhoneVoicemailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PhoneVoicemailType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialVoicemailAsync<Impl: IPhoneVoicemailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialVoicemailAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneVoicemail, BASE_OFFSET>(),
            Number: Number::<Impl, IMPL_OFFSET>,
            MessageCount: MessageCount::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            DialVoicemailAsync: DialVoicemailAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneVoicemail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVoipCallCoordinatorImpl: Sized {
    fn ReserveCallResourcesAsync(&mut self, taskentrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>;
    fn MuteStateChanged(&mut self, mutechangehandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMuteStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestNewIncomingCall(&mut self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, contactimage: &::core::option::Option<super::super::Foundation::Uri>, servicename: &::windows::core::HSTRING, brandingimage: &::core::option::Option<super::super::Foundation::Uri>, calldetails: &::windows::core::HSTRING, ringtone: &::core::option::Option<super::super::Foundation::Uri>, media: VoipPhoneCallMedia, ringtimeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<VoipPhoneCall>;
    fn RequestNewOutgoingCall(&mut self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall>;
    fn NotifyMuted(&mut self) -> ::windows::core::Result<()>;
    fn NotifyUnmuted(&mut self) -> ::windows::core::Result<()>;
    fn RequestOutgoingUpgradeToVideoCall(&mut self, callupgradeguid: &::windows::core::GUID, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<VoipPhoneCall>;
    fn RequestIncomingUpgradeToVideoCall(&mut self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, contactimage: &::core::option::Option<super::super::Foundation::Uri>, servicename: &::windows::core::HSTRING, brandingimage: &::core::option::Option<super::super::Foundation::Uri>, calldetails: &::windows::core::HSTRING, ringtone: &::core::option::Option<super::super::Foundation::Uri>, ringtimeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<VoipPhoneCall>;
    fn TerminateCellularCall(&mut self, callupgradeguid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CancelUpgrade(&mut self, callupgradeguid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoipCallCoordinator {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipCallCoordinator";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVoipCallCoordinatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipCallCoordinatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipCallCoordinatorVtbl {
        unsafe extern "system" fn ReserveCallResourcesAsync<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskentrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReserveCallResourcesAsync(&*(&taskentrypoint as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MuteStateChanged<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mutechangehandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MuteStateChanged(&*(&mutechangehandler as *const <super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMuteStateChanged<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMuteStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestNewIncomingCall<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactimage: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brandingimage: ::windows::core::RawPtr, calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ringtone: ::windows::core::RawPtr, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestNewIncomingCall(
                &*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactnumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactimage as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&brandingimage as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&calldetails as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&ringtone as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                media,
                &*(&ringtimeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestNewOutgoingCall<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestNewOutgoingCall(
                &*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                media,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyMuted<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyMuted().into()
        }
        unsafe extern "system" fn NotifyUnmuted<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyUnmuted().into()
        }
        unsafe extern "system" fn RequestOutgoingUpgradeToVideoCall<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows::core::GUID, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestOutgoingUpgradeToVideoCall(
                &*(&callupgradeguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestIncomingUpgradeToVideoCall<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactimage: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, brandingimage: ::windows::core::RawPtr, calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ringtone: ::windows::core::RawPtr, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestIncomingUpgradeToVideoCall(
                &*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactnumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactimage as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&brandingimage as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&calldetails as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&ringtone as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&ringtimeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateCellularCall<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateCellularCall(&*(&callupgradeguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelUpgrade<Impl: IVoipCallCoordinatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callupgradeguid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelUpgrade(&*(&callupgradeguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipCallCoordinator, BASE_OFFSET>(),
            ReserveCallResourcesAsync: ReserveCallResourcesAsync::<Impl, IMPL_OFFSET>,
            MuteStateChanged: MuteStateChanged::<Impl, IMPL_OFFSET>,
            RemoveMuteStateChanged: RemoveMuteStateChanged::<Impl, IMPL_OFFSET>,
            RequestNewIncomingCall: RequestNewIncomingCall::<Impl, IMPL_OFFSET>,
            RequestNewOutgoingCall: RequestNewOutgoingCall::<Impl, IMPL_OFFSET>,
            NotifyMuted: NotifyMuted::<Impl, IMPL_OFFSET>,
            NotifyUnmuted: NotifyUnmuted::<Impl, IMPL_OFFSET>,
            RequestOutgoingUpgradeToVideoCall: RequestOutgoingUpgradeToVideoCall::<Impl, IMPL_OFFSET>,
            RequestIncomingUpgradeToVideoCall: RequestIncomingUpgradeToVideoCall::<Impl, IMPL_OFFSET>,
            TerminateCellularCall: TerminateCellularCall::<Impl, IMPL_OFFSET>,
            CancelUpgrade: CancelUpgrade::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipCallCoordinator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVoipCallCoordinator2Impl: Sized + IVoipCallCoordinatorImpl {
    fn SetupNewAcceptedCall(&mut self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoipCallCoordinator2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipCallCoordinator2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVoipCallCoordinator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipCallCoordinator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipCallCoordinator2Vtbl {
        unsafe extern "system" fn SetupNewAcceptedCall<Impl: IVoipCallCoordinator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetupNewAcceptedCall(
                &*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactnumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                media,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipCallCoordinator2, BASE_OFFSET>(),
            SetupNewAcceptedCall: SetupNewAcceptedCall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipCallCoordinator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVoipCallCoordinator3Impl: Sized + IVoipCallCoordinatorImpl {
    fn RequestNewAppInitiatedCall(&mut self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, servicename: &::windows::core::HSTRING, media: VoipPhoneCallMedia) -> ::windows::core::Result<VoipPhoneCall>;
    fn RequestNewIncomingCallWithContactRemoteId(&mut self, context: &::windows::core::HSTRING, contactname: &::windows::core::HSTRING, contactnumber: &::windows::core::HSTRING, contactimage: &::core::option::Option<super::super::Foundation::Uri>, servicename: &::windows::core::HSTRING, brandingimage: &::core::option::Option<super::super::Foundation::Uri>, calldetails: &::windows::core::HSTRING, ringtone: &::core::option::Option<super::super::Foundation::Uri>, media: VoipPhoneCallMedia, ringtimeout: &super::super::Foundation::TimeSpan, contactremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<VoipPhoneCall>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoipCallCoordinator3 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipCallCoordinator3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVoipCallCoordinator3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipCallCoordinator3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipCallCoordinator3Vtbl {
        unsafe extern "system" fn RequestNewAppInitiatedCall<Impl: IVoipCallCoordinator3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, media: VoipPhoneCallMedia, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestNewAppInitiatedCall(
                &*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactnumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                media,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestNewIncomingCallWithContactRemoteId<Impl: IVoipCallCoordinator3Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            context: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            contactname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            contactnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            contactimage: ::windows::core::RawPtr,
            servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            brandingimage: ::windows::core::RawPtr,
            calldetails: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            ringtone: ::windows::core::RawPtr,
            media: VoipPhoneCallMedia,
            ringtimeout: super::super::Foundation::TimeSpan,
            contactremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestNewIncomingCallWithContactRemoteId(
                &*(&context as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactnumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&contactimage as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&brandingimage as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&calldetails as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&ringtone as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                media,
                &*(&ringtimeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&contactremoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipCallCoordinator3, BASE_OFFSET>(),
            RequestNewAppInitiatedCall: RequestNewAppInitiatedCall::<Impl, IMPL_OFFSET>,
            RequestNewIncomingCallWithContactRemoteId: RequestNewIncomingCallWithContactRemoteId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipCallCoordinator3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVoipCallCoordinator4Impl: Sized + IVoipCallCoordinatorImpl {
    fn ReserveOneProcessCallResourcesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoipCallCoordinator4 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipCallCoordinator4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVoipCallCoordinator4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipCallCoordinator4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipCallCoordinator4Vtbl {
        unsafe extern "system" fn ReserveOneProcessCallResourcesAsync<Impl: IVoipCallCoordinator4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReserveOneProcessCallResourcesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipCallCoordinator4, BASE_OFFSET>(),
            ReserveOneProcessCallResourcesAsync: ReserveOneProcessCallResourcesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipCallCoordinator4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoipCallCoordinatorStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<VoipCallCoordinator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoipCallCoordinatorStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVoipCallCoordinatorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipCallCoordinatorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipCallCoordinatorStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IVoipCallCoordinatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipCallCoordinatorStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipCallCoordinatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVoipPhoneCallImpl: Sized {
    fn EndRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEndRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResumeRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResumeRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AnswerRequested(&mut self, accepthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnswerRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RejectRequested(&mut self, rejecthandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRejectRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyCallHeld(&mut self) -> ::windows::core::Result<()>;
    fn NotifyCallActive(&mut self) -> ::windows::core::Result<()>;
    fn NotifyCallEnded(&mut self) -> ::windows::core::Result<()>;
    fn ContactName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn CallMedia(&mut self) -> ::windows::core::Result<VoipPhoneCallMedia>;
    fn SetCallMedia(&mut self, value: VoipPhoneCallMedia) -> ::windows::core::Result<()>;
    fn NotifyCallReady(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoipPhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipPhoneCall";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVoipPhoneCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipPhoneCallImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipPhoneCallVtbl {
        unsafe extern "system" fn EndRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEndRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEndRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HoldRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHoldRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHoldRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResumeRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResumeRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResumeRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AnswerRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accepthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnswerRequested(&*(&accepthandler as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAnswerRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAnswerRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RejectRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rejecthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RejectRequested(&*(&rejecthandler as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRejectRequested<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRejectRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyCallHeld<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCallHeld().into()
        }
        unsafe extern "system" fn NotifyCallActive<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCallActive().into()
        }
        unsafe extern "system" fn NotifyCallEnded<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCallEnded().into()
        }
        unsafe extern "system" fn ContactName<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContactName<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CallMedia<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoipPhoneCallMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallMedia<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VoipPhoneCallMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallMedia(value).into()
        }
        unsafe extern "system" fn NotifyCallReady<Impl: IVoipPhoneCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCallReady().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipPhoneCall, BASE_OFFSET>(),
            EndRequested: EndRequested::<Impl, IMPL_OFFSET>,
            RemoveEndRequested: RemoveEndRequested::<Impl, IMPL_OFFSET>,
            HoldRequested: HoldRequested::<Impl, IMPL_OFFSET>,
            RemoveHoldRequested: RemoveHoldRequested::<Impl, IMPL_OFFSET>,
            ResumeRequested: ResumeRequested::<Impl, IMPL_OFFSET>,
            RemoveResumeRequested: RemoveResumeRequested::<Impl, IMPL_OFFSET>,
            AnswerRequested: AnswerRequested::<Impl, IMPL_OFFSET>,
            RemoveAnswerRequested: RemoveAnswerRequested::<Impl, IMPL_OFFSET>,
            RejectRequested: RejectRequested::<Impl, IMPL_OFFSET>,
            RemoveRejectRequested: RemoveRejectRequested::<Impl, IMPL_OFFSET>,
            NotifyCallHeld: NotifyCallHeld::<Impl, IMPL_OFFSET>,
            NotifyCallActive: NotifyCallActive::<Impl, IMPL_OFFSET>,
            NotifyCallEnded: NotifyCallEnded::<Impl, IMPL_OFFSET>,
            ContactName: ContactName::<Impl, IMPL_OFFSET>,
            SetContactName: SetContactName::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            CallMedia: CallMedia::<Impl, IMPL_OFFSET>,
            SetCallMedia: SetCallMedia::<Impl, IMPL_OFFSET>,
            NotifyCallReady: NotifyCallReady::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipPhoneCall as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVoipPhoneCall2Impl: Sized + IVoipPhoneCallImpl {
    fn TryShowAppUI(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoipPhoneCall2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipPhoneCall2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVoipPhoneCall2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipPhoneCall2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipPhoneCall2Vtbl {
        unsafe extern "system" fn TryShowAppUI<Impl: IVoipPhoneCall2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryShowAppUI().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipPhoneCall2, BASE_OFFSET>(), TryShowAppUI: TryShowAppUI::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipPhoneCall2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVoipPhoneCall3Impl: Sized + IVoipPhoneCallImpl + IVoipPhoneCall2Impl {
    fn NotifyCallAccepted(&mut self, media: VoipPhoneCallMedia) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoipPhoneCall3 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.IVoipPhoneCall3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVoipPhoneCall3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoipPhoneCall3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoipPhoneCall3Vtbl {
        unsafe extern "system" fn NotifyCallAccepted<Impl: IVoipPhoneCall3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, media: VoipPhoneCallMedia) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCallAccepted(media).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoipPhoneCall3, BASE_OFFSET>(),
            NotifyCallAccepted: NotifyCallAccepted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoipPhoneCall3 as ::windows::core::Interface>::IID
    }
}
