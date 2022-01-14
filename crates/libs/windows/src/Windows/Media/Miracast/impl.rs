#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMiracastReceiver_Impl: Sized {
    fn GetDefaultSettings(&mut self) -> ::windows::core::Result<MiracastReceiverSettings>;
    fn GetCurrentSettings(&mut self) -> ::windows::core::Result<MiracastReceiverSettings>;
    fn GetCurrentSettingsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>>;
    fn DisconnectAllAndApplySettings(&mut self, settings: &::core::option::Option<MiracastReceiverSettings>) -> ::windows::core::Result<MiracastReceiverApplySettingsResult>;
    fn DisconnectAllAndApplySettingsAsync(&mut self, settings: &::core::option::Option<MiracastReceiverSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>>;
    fn GetStatus(&mut self) -> ::windows::core::Result<MiracastReceiverStatus>;
    fn GetStatusAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>>;
    fn StatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateSession(&mut self, view: &::core::option::Option<super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<MiracastReceiverSession>;
    fn CreateSessionAsync(&mut self, view: &::core::option::Option<super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>>;
    fn ClearKnownTransmitters(&mut self) -> ::windows::core::Result<()>;
    fn RemoveKnownTransmitter(&mut self, transmitter: &::core::option::Option<MiracastTransmitter>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiver {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiver";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "implement_exclusive"))]
impl IMiracastReceiver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiver_Vtbl {
        unsafe extern "system" fn GetDefaultSettings<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSettings<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSettingsAsync<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSettingsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectAllAndApplySettings<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectAllAndApplySettings(&*(&settings as *const <MiracastReceiverSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectAllAndApplySettingsAsync<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectAllAndApplySettingsAsync(&*(&settings as *const <MiracastReceiverSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusAsync<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusChanged<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateSession<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession(&*(&view as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionAsync<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionAsync(&*(&view as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearKnownTransmitters<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearKnownTransmitters().into()
        }
        unsafe extern "system" fn RemoveKnownTransmitter<Impl: IMiracastReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transmitter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKnownTransmitter(&*(&transmitter as *const <MiracastTransmitter as ::windows::core::Abi>::Abi as *const <MiracastTransmitter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiver, BASE_OFFSET>(),
            GetDefaultSettings: GetDefaultSettings::<Impl, IMPL_OFFSET>,
            GetCurrentSettings: GetCurrentSettings::<Impl, IMPL_OFFSET>,
            GetCurrentSettingsAsync: GetCurrentSettingsAsync::<Impl, IMPL_OFFSET>,
            DisconnectAllAndApplySettings: DisconnectAllAndApplySettings::<Impl, IMPL_OFFSET>,
            DisconnectAllAndApplySettingsAsync: DisconnectAllAndApplySettingsAsync::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetStatusAsync: GetStatusAsync::<Impl, IMPL_OFFSET>,
            StatusChanged: StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged: RemoveStatusChanged::<Impl, IMPL_OFFSET>,
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
            CreateSessionAsync: CreateSessionAsync::<Impl, IMPL_OFFSET>,
            ClearKnownTransmitters: ClearKnownTransmitters::<Impl, IMPL_OFFSET>,
            RemoveKnownTransmitter: RemoveKnownTransmitter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverApplySettingsResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<MiracastReceiverApplySettingsStatus>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverApplySettingsResult {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverApplySettingsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverApplySettingsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverApplySettingsResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverApplySettingsResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IMiracastReceiverApplySettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverApplySettingsStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IMiracastReceiverApplySettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverApplySettingsResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverApplySettingsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMiracastReceiverConnection_Impl: Sized {
    fn Disconnect(&mut self, reason: MiracastReceiverDisconnectReason) -> ::windows::core::Result<()>;
    fn DisconnectWithMessage(&mut self, reason: MiracastReceiverDisconnectReason, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn PauseAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn ResumeAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Transmitter(&mut self) -> ::windows::core::Result<MiracastTransmitter>;
    fn InputDevices(&mut self) -> ::windows::core::Result<MiracastReceiverInputDevices>;
    fn CursorImageChannel(&mut self) -> ::windows::core::Result<MiracastReceiverCursorImageChannel>;
    fn StreamControl(&mut self) -> ::windows::core::Result<MiracastReceiverStreamControl>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverConnection {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverConnection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMiracastReceiverConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverConnection_Vtbl {
        unsafe extern "system" fn Disconnect<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(reason).into()
        }
        unsafe extern "system" fn DisconnectWithMessage<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectWithMessage(reason, &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pause<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn PauseAsync<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn ResumeAsync<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transmitter<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transmitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputDevices<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CursorImageChannel<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CursorImageChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamControl<Impl: IMiracastReceiverConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverConnection, BASE_OFFSET>(),
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            DisconnectWithMessage: DisconnectWithMessage::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            PauseAsync: PauseAsync::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            ResumeAsync: ResumeAsync::<Impl, IMPL_OFFSET>,
            Transmitter: Transmitter::<Impl, IMPL_OFFSET>,
            InputDevices: InputDevices::<Impl, IMPL_OFFSET>,
            CursorImageChannel: CursorImageChannel::<Impl, IMPL_OFFSET>,
            StreamControl: StreamControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMiracastReceiverConnectionCreatedEventArgs_Impl: Sized {
    fn Connection(&mut self) -> ::windows::core::Result<MiracastReceiverConnection>;
    fn Pin(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverConnectionCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMiracastReceiverConnectionCreatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverConnectionCreatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverConnectionCreatedEventArgs_Vtbl {
        unsafe extern "system" fn Connection<Impl: IMiracastReceiverConnectionCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pin<Impl: IMiracastReceiverConnectionCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMiracastReceiverConnectionCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverConnectionCreatedEventArgs, BASE_OFFSET>(),
            Connection: Connection::<Impl, IMPL_OFFSET>,
            Pin: Pin::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverConnectionCreatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMiracastReceiverCursorImageChannel_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn MaxImageSize(&mut self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Graphics::PointInt32>;
    fn ImageStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn ImageStreamChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageStreamChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverCursorImageChannel {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverCursorImageChannel";
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMiracastReceiverCursorImageChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverCursorImageChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverCursorImageChannel_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxImageSize<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxImageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::PointInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageStream<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageStreamChanged<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageStreamChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageStreamChanged<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveImageStreamChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionChanged<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionChanged<Impl: IMiracastReceiverCursorImageChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverCursorImageChannel, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            MaxImageSize: MaxImageSize::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            ImageStream: ImageStream::<Impl, IMPL_OFFSET>,
            ImageStreamChanged: ImageStreamChanged::<Impl, IMPL_OFFSET>,
            RemoveImageStreamChanged: RemoveImageStreamChanged::<Impl, IMPL_OFFSET>,
            PositionChanged: PositionChanged::<Impl, IMPL_OFFSET>,
            RemovePositionChanged: RemovePositionChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverCursorImageChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait IMiracastReceiverCursorImageChannelSettings_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MaxImageSize(&mut self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn SetMaxImageSize(&mut self, value: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverCursorImageChannelSettings {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl IMiracastReceiverCursorImageChannelSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverCursorImageChannelSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverCursorImageChannelSettings_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMiracastReceiverCursorImageChannelSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IMiracastReceiverCursorImageChannelSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn MaxImageSize<Impl: IMiracastReceiverCursorImageChannelSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxImageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxImageSize<Impl: IMiracastReceiverCursorImageChannelSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxImageSize(&*(&value as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverCursorImageChannelSettings, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            MaxImageSize: MaxImageSize::<Impl, IMPL_OFFSET>,
            SetMaxImageSize: SetMaxImageSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverCursorImageChannelSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverDisconnectedEventArgs_Impl: Sized {
    fn Connection(&mut self) -> ::windows::core::Result<MiracastReceiverConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverDisconnectedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverDisconnectedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverDisconnectedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverDisconnectedEventArgs_Vtbl {
        unsafe extern "system" fn Connection<Impl: IMiracastReceiverDisconnectedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverDisconnectedEventArgs, BASE_OFFSET>(),
            Connection: Connection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverDisconnectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMiracastReceiverGameControllerDevice_Impl: Sized {
    fn TransmitInput(&mut self) -> ::windows::core::Result<bool>;
    fn SetTransmitInput(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestedByTransmitter(&mut self) -> ::windows::core::Result<bool>;
    fn IsTransmittingInput(&mut self) -> ::windows::core::Result<bool>;
    fn Mode(&mut self) -> ::windows::core::Result<MiracastReceiverGameControllerDeviceUsageMode>;
    fn SetMode(&mut self, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::Result<()>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverGameControllerDevice {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverGameControllerDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMiracastReceiverGameControllerDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverGameControllerDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverGameControllerDevice_Vtbl {
        unsafe extern "system" fn TransmitInput<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmitInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmitInput<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransmitInput(value).into()
        }
        unsafe extern "system" fn IsRequestedByTransmitter<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequestedByTransmitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransmittingInput<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransmittingInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Changed<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IMiracastReceiverGameControllerDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverGameControllerDevice, BASE_OFFSET>(),
            TransmitInput: TransmitInput::<Impl, IMPL_OFFSET>,
            SetTransmitInput: SetTransmitInput::<Impl, IMPL_OFFSET>,
            IsRequestedByTransmitter: IsRequestedByTransmitter::<Impl, IMPL_OFFSET>,
            IsTransmittingInput: IsTransmittingInput::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverGameControllerDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverInputDevices_Impl: Sized {
    fn Keyboard(&mut self) -> ::windows::core::Result<MiracastReceiverKeyboardDevice>;
    fn GameController(&mut self) -> ::windows::core::Result<MiracastReceiverGameControllerDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverInputDevices {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverInputDevices";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverInputDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverInputDevices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverInputDevices_Vtbl {
        unsafe extern "system" fn Keyboard<Impl: IMiracastReceiverInputDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GameController<Impl: IMiracastReceiverInputDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverInputDevices, BASE_OFFSET>(),
            Keyboard: Keyboard::<Impl, IMPL_OFFSET>,
            GameController: GameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverInputDevices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMiracastReceiverKeyboardDevice_Impl: Sized {
    fn TransmitInput(&mut self) -> ::windows::core::Result<bool>;
    fn SetTransmitInput(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestedByTransmitter(&mut self) -> ::windows::core::Result<bool>;
    fn IsTransmittingInput(&mut self) -> ::windows::core::Result<bool>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverKeyboardDevice {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverKeyboardDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMiracastReceiverKeyboardDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverKeyboardDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverKeyboardDevice_Vtbl {
        unsafe extern "system" fn TransmitInput<Impl: IMiracastReceiverKeyboardDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmitInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmitInput<Impl: IMiracastReceiverKeyboardDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransmitInput(value).into()
        }
        unsafe extern "system" fn IsRequestedByTransmitter<Impl: IMiracastReceiverKeyboardDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequestedByTransmitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransmittingInput<Impl: IMiracastReceiverKeyboardDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransmittingInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IMiracastReceiverKeyboardDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IMiracastReceiverKeyboardDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverKeyboardDevice, BASE_OFFSET>(),
            TransmitInput: TransmitInput::<Impl, IMPL_OFFSET>,
            SetTransmitInput: SetTransmitInput::<Impl, IMPL_OFFSET>,
            IsRequestedByTransmitter: IsRequestedByTransmitter::<Impl, IMPL_OFFSET>,
            IsTransmittingInput: IsTransmittingInput::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverKeyboardDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMiracastReceiverMediaSourceCreatedEventArgs_Impl: Sized {
    fn Connection(&mut self) -> ::windows::core::Result<MiracastReceiverConnection>;
    fn MediaSource(&mut self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn CursorImageChannelSettings(&mut self) -> ::windows::core::Result<MiracastReceiverCursorImageChannelSettings>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverMediaSourceCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverMediaSourceCreatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl {
        unsafe extern "system" fn Connection<Impl: IMiracastReceiverMediaSourceCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSource<Impl: IMiracastReceiverMediaSourceCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CursorImageChannelSettings<Impl: IMiracastReceiverMediaSourceCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CursorImageChannelSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMiracastReceiverMediaSourceCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverMediaSourceCreatedEventArgs, BASE_OFFSET>(),
            Connection: Connection::<Impl, IMPL_OFFSET>,
            MediaSource: MediaSource::<Impl, IMPL_OFFSET>,
            CursorImageChannelSettings: CursorImageChannelSettings::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverMediaSourceCreatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMiracastReceiverSession_Impl: Sized {
    fn ConnectionCreated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionCreated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaSourceCreated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaSourceCreated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disconnected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllowConnectionTakeover(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowConnectionTakeover(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MaxSimultaneousConnections(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxSimultaneousConnections(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<MiracastReceiverSessionStartResult>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverSession {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMiracastReceiverSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverSession_Vtbl {
        unsafe extern "system" fn ConnectionCreated<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionCreated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionCreated<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaSourceCreated<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSourceCreated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMediaSourceCreated<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaSourceCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnected<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisconnected<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisconnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowConnectionTakeover<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowConnectionTakeover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowConnectionTakeover<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowConnectionTakeover(value).into()
        }
        unsafe extern "system" fn MaxSimultaneousConnections<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSimultaneousConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSimultaneousConnections<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSimultaneousConnections(value).into()
        }
        unsafe extern "system" fn Start<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IMiracastReceiverSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverSession, BASE_OFFSET>(),
            ConnectionCreated: ConnectionCreated::<Impl, IMPL_OFFSET>,
            RemoveConnectionCreated: RemoveConnectionCreated::<Impl, IMPL_OFFSET>,
            MediaSourceCreated: MediaSourceCreated::<Impl, IMPL_OFFSET>,
            RemoveMediaSourceCreated: RemoveMediaSourceCreated::<Impl, IMPL_OFFSET>,
            Disconnected: Disconnected::<Impl, IMPL_OFFSET>,
            RemoveDisconnected: RemoveDisconnected::<Impl, IMPL_OFFSET>,
            AllowConnectionTakeover: AllowConnectionTakeover::<Impl, IMPL_OFFSET>,
            SetAllowConnectionTakeover: SetAllowConnectionTakeover::<Impl, IMPL_OFFSET>,
            MaxSimultaneousConnections: MaxSimultaneousConnections::<Impl, IMPL_OFFSET>,
            SetMaxSimultaneousConnections: SetMaxSimultaneousConnections::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSessionStartResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<MiracastReceiverSessionStartStatus>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverSessionStartResult {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverSessionStartResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverSessionStartResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverSessionStartResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverSessionStartResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IMiracastReceiverSessionStartResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverSessionStartStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IMiracastReceiverSessionStartResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverSessionStartResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverSessionStartResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSettings_Impl: Sized {
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ModelName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ModelNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelNumber(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AuthorizationMethod(&mut self) -> ::windows::core::Result<MiracastReceiverAuthorizationMethod>;
    fn SetAuthorizationMethod(&mut self, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::Result<()>;
    fn RequireAuthorizationFromKnownTransmitters(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequireAuthorizationFromKnownTransmitters(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverSettings {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverSettings_Vtbl {
        unsafe extern "system" fn FriendlyName<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ModelName<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelName<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ModelNumber<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelNumber<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthorizationMethod<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorizationMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorizationMethod<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthorizationMethod(value).into()
        }
        unsafe extern "system" fn RequireAuthorizationFromKnownTransmitters<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequireAuthorizationFromKnownTransmitters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireAuthorizationFromKnownTransmitters<Impl: IMiracastReceiverSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequireAuthorizationFromKnownTransmitters(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverSettings, BASE_OFFSET>(),
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            ModelName: ModelName::<Impl, IMPL_OFFSET>,
            SetModelName: SetModelName::<Impl, IMPL_OFFSET>,
            ModelNumber: ModelNumber::<Impl, IMPL_OFFSET>,
            SetModelNumber: SetModelNumber::<Impl, IMPL_OFFSET>,
            AuthorizationMethod: AuthorizationMethod::<Impl, IMPL_OFFSET>,
            SetAuthorizationMethod: SetAuthorizationMethod::<Impl, IMPL_OFFSET>,
            RequireAuthorizationFromKnownTransmitters: RequireAuthorizationFromKnownTransmitters::<Impl, IMPL_OFFSET>,
            SetRequireAuthorizationFromKnownTransmitters: SetRequireAuthorizationFromKnownTransmitters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMiracastReceiverStatus_Impl: Sized {
    fn ListeningStatus(&mut self) -> ::windows::core::Result<MiracastReceiverListeningStatus>;
    fn WiFiStatus(&mut self) -> ::windows::core::Result<MiracastReceiverWiFiStatus>;
    fn IsConnectionTakeoverSupported(&mut self) -> ::windows::core::Result<bool>;
    fn MaxSimultaneousConnections(&mut self) -> ::windows::core::Result<i32>;
    fn KnownTransmitters(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverStatus {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverStatus";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMiracastReceiverStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverStatus_Vtbl {
        unsafe extern "system" fn ListeningStatus<Impl: IMiracastReceiverStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverListeningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListeningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WiFiStatus<Impl: IMiracastReceiverStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverWiFiStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WiFiStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectionTakeoverSupported<Impl: IMiracastReceiverStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnectionTakeoverSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSimultaneousConnections<Impl: IMiracastReceiverStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSimultaneousConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnownTransmitters<Impl: IMiracastReceiverStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KnownTransmitters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverStatus, BASE_OFFSET>(),
            ListeningStatus: ListeningStatus::<Impl, IMPL_OFFSET>,
            WiFiStatus: WiFiStatus::<Impl, IMPL_OFFSET>,
            IsConnectionTakeoverSupported: IsConnectionTakeoverSupported::<Impl, IMPL_OFFSET>,
            MaxSimultaneousConnections: MaxSimultaneousConnections::<Impl, IMPL_OFFSET>,
            KnownTransmitters: KnownTransmitters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMiracastReceiverStreamControl_Impl: Sized {
    fn GetVideoStreamSettings(&mut self) -> ::windows::core::Result<MiracastReceiverVideoStreamSettings>;
    fn GetVideoStreamSettingsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>>;
    fn SuggestVideoStreamSettings(&mut self, settings: &::core::option::Option<MiracastReceiverVideoStreamSettings>) -> ::windows::core::Result<()>;
    fn SuggestVideoStreamSettingsAsync(&mut self, settings: &::core::option::Option<MiracastReceiverVideoStreamSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MuteAudio(&mut self) -> ::windows::core::Result<bool>;
    fn SetMuteAudio(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverStreamControl {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverStreamControl";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMiracastReceiverStreamControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverStreamControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverStreamControl_Vtbl {
        unsafe extern "system" fn GetVideoStreamSettings<Impl: IMiracastReceiverStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoStreamSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoStreamSettingsAsync<Impl: IMiracastReceiverStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoStreamSettingsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuggestVideoStreamSettings<Impl: IMiracastReceiverStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuggestVideoStreamSettings(&*(&settings as *const <MiracastReceiverVideoStreamSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverVideoStreamSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestVideoStreamSettingsAsync<Impl: IMiracastReceiverStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestVideoStreamSettingsAsync(&*(&settings as *const <MiracastReceiverVideoStreamSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverVideoStreamSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MuteAudio<Impl: IMiracastReceiverStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MuteAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMuteAudio<Impl: IMiracastReceiverStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteAudio(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverStreamControl, BASE_OFFSET>(),
            GetVideoStreamSettings: GetVideoStreamSettings::<Impl, IMPL_OFFSET>,
            GetVideoStreamSettingsAsync: GetVideoStreamSettingsAsync::<Impl, IMPL_OFFSET>,
            SuggestVideoStreamSettings: SuggestVideoStreamSettings::<Impl, IMPL_OFFSET>,
            SuggestVideoStreamSettingsAsync: SuggestVideoStreamSettingsAsync::<Impl, IMPL_OFFSET>,
            MuteAudio: MuteAudio::<Impl, IMPL_OFFSET>,
            SetMuteAudio: SetMuteAudio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverStreamControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
pub trait IMiracastReceiverVideoStreamSettings_Impl: Sized {
    fn Size(&mut self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn SetSize(&mut self, value: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn Bitrate(&mut self) -> ::windows::core::Result<i32>;
    fn SetBitrate(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastReceiverVideoStreamSettings {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings";
}
#[cfg(all(feature = "Graphics", feature = "implement_exclusive"))]
impl IMiracastReceiverVideoStreamSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastReceiverVideoStreamSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastReceiverVideoStreamSettings_Vtbl {
        unsafe extern "system" fn Size<Impl: IMiracastReceiverVideoStreamSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IMiracastReceiverVideoStreamSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bitrate<Impl: IMiracastReceiverVideoStreamSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrate<Impl: IMiracastReceiverVideoStreamSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitrate(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastReceiverVideoStreamSettings, BASE_OFFSET>(),
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            Bitrate: Bitrate::<Impl, IMPL_OFFSET>,
            SetBitrate: SetBitrate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastReceiverVideoStreamSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMiracastTransmitter_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AuthorizationStatus(&mut self) -> ::windows::core::Result<MiracastTransmitterAuthorizationStatus>;
    fn SetAuthorizationStatus(&mut self, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::Result<()>;
    fn GetConnections(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>>;
    fn MacAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastConnectionTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMiracastTransmitter {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastTransmitter";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMiracastTransmitter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMiracastTransmitter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMiracastTransmitter_Vtbl {
        unsafe extern "system" fn Name<Impl: IMiracastTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IMiracastTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthorizationStatus<Impl: IMiracastTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorizationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorizationStatus<Impl: IMiracastTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthorizationStatus(value).into()
        }
        unsafe extern "system" fn GetConnections<Impl: IMiracastTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacAddress<Impl: IMiracastTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MacAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastConnectionTime<Impl: IMiracastTransmitter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastConnectionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMiracastTransmitter, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            AuthorizationStatus: AuthorizationStatus::<Impl, IMPL_OFFSET>,
            SetAuthorizationStatus: SetAuthorizationStatus::<Impl, IMPL_OFFSET>,
            GetConnections: GetConnections::<Impl, IMPL_OFFSET>,
            MacAddress: MacAddress::<Impl, IMPL_OFFSET>,
            LastConnectionTime: LastConnectionTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMiracastTransmitter as ::windows::core::Interface>::IID
    }
}
