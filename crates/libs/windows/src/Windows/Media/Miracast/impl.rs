#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverImpl: Sized {
    fn GetDefaultSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings>;
    fn GetCurrentSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings>;
    fn GetCurrentSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>>;
    fn DisconnectAllAndApplySettings(&self, settings: &::core::option::Option<MiracastReceiverSettings>) -> ::windows::core::Result<MiracastReceiverApplySettingsResult>;
    fn DisconnectAllAndApplySettingsAsync(&self, settings: &::core::option::Option<MiracastReceiverSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>>;
    fn GetStatus(&self) -> ::windows::core::Result<MiracastReceiverStatus>;
    fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateSession(&self, view: &::core::option::Option<super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<MiracastReceiverSession>;
    fn CreateSessionAsync(&self, view: &::core::option::Option<super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>>;
    fn ClearKnownTransmitters(&self) -> ::windows::core::Result<()>;
    fn RemoveKnownTransmitter(&self, transmitter: &::core::option::Option<MiracastTransmitter>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiver {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiver";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverVtbl {
    pub const fn new<Impl: IMiracastReceiverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverVtbl {
        unsafe extern "system" fn GetDefaultSettings<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSettings<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSettingsAsync<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentSettingsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectAllAndApplySettings<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisconnectAllAndApplySettings(&*(&settings as *const <MiracastReceiverSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectAllAndApplySettingsAsync<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisconnectAllAndApplySettingsAsync(&*(&settings as *const <MiracastReceiverSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusAsync<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusChanged<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateSession<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSession(&*(&view as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionAsync<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSessionAsync(&*(&view as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearKnownTransmitters<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearKnownTransmitters().into()
        }
        unsafe extern "system" fn RemoveKnownTransmitter<Impl: IMiracastReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transmitter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveKnownTransmitter(&*(&transmitter as *const <MiracastTransmitter as ::windows::core::Abi>::Abi as *const <MiracastTransmitter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMiracastReceiver>,
            base.5,
            GetDefaultSettings::<Impl, OFFSET>,
            GetCurrentSettings::<Impl, OFFSET>,
            GetCurrentSettingsAsync::<Impl, OFFSET>,
            DisconnectAllAndApplySettings::<Impl, OFFSET>,
            DisconnectAllAndApplySettingsAsync::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetStatusAsync::<Impl, OFFSET>,
            StatusChanged::<Impl, OFFSET>,
            RemoveStatusChanged::<Impl, OFFSET>,
            CreateSession::<Impl, OFFSET>,
            CreateSessionAsync::<Impl, OFFSET>,
            ClearKnownTransmitters::<Impl, OFFSET>,
            RemoveKnownTransmitter::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverApplySettingsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MiracastReceiverApplySettingsStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverApplySettingsResult {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverApplySettingsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverApplySettingsResultVtbl {
    pub const fn new<Impl: IMiracastReceiverApplySettingsResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverApplySettingsResultVtbl {
        unsafe extern "system" fn Status<Impl: IMiracastReceiverApplySettingsResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverApplySettingsStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IMiracastReceiverApplySettingsResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverApplySettingsResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverConnectionImpl: Sized {
    fn Disconnect(&self, reason: MiracastReceiverDisconnectReason) -> ::windows::core::Result<()>;
    fn DisconnectWithMessage(&self, reason: MiracastReceiverDisconnectReason, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Transmitter(&self) -> ::windows::core::Result<MiracastTransmitter>;
    fn InputDevices(&self) -> ::windows::core::Result<MiracastReceiverInputDevices>;
    fn CursorImageChannel(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannel>;
    fn StreamControl(&self) -> ::windows::core::Result<MiracastReceiverStreamControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverConnection {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverConnectionVtbl {
    pub const fn new<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverConnectionVtbl {
        unsafe extern "system" fn Disconnect<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Disconnect(reason).into()
        }
        unsafe extern "system" fn DisconnectWithMessage<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DisconnectWithMessage(reason, &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pause<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn PauseAsync<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn ResumeAsync<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResumeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transmitter<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transmitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputDevices<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CursorImageChannel<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CursorImageChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamControl<Impl: IMiracastReceiverConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverConnection>, base.5, Disconnect::<Impl, OFFSET>, DisconnectWithMessage::<Impl, OFFSET>, Pause::<Impl, OFFSET>, PauseAsync::<Impl, OFFSET>, Resume::<Impl, OFFSET>, ResumeAsync::<Impl, OFFSET>, Transmitter::<Impl, OFFSET>, InputDevices::<Impl, OFFSET>, CursorImageChannel::<Impl, OFFSET>, StreamControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverConnectionCreatedEventArgsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection>;
    fn Pin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverConnectionCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverConnectionCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverConnectionCreatedEventArgsVtbl {
    pub const fn new<Impl: IMiracastReceiverConnectionCreatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverConnectionCreatedEventArgsVtbl {
        unsafe extern "system" fn Connection<Impl: IMiracastReceiverConnectionCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pin<Impl: IMiracastReceiverConnectionCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMiracastReceiverConnectionCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverConnectionCreatedEventArgs>, base.5, Connection::<Impl, OFFSET>, Pin::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverCursorImageChannelImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn Position(&self) -> ::windows::core::Result<super::super::Graphics::PointInt32>;
    fn ImageStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn ImageStreamChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageStreamChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverCursorImageChannel {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverCursorImageChannel";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverCursorImageChannelVtbl {
    pub const fn new<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverCursorImageChannelVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxImageSize<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxImageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::PointInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageStream<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageStreamChanged<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageStreamChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveImageStreamChanged<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveImageStreamChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionChanged<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionChanged<Impl: IMiracastReceiverCursorImageChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverCursorImageChannel>, base.5, IsEnabled::<Impl, OFFSET>, MaxImageSize::<Impl, OFFSET>, Position::<Impl, OFFSET>, ImageStream::<Impl, OFFSET>, ImageStreamChanged::<Impl, OFFSET>, RemoveImageStreamChanged::<Impl, OFFSET>, PositionChanged::<Impl, OFFSET>, RemovePositionChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverCursorImageChannelSettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn SetMaxImageSize(&self, value: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverCursorImageChannelSettings {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverCursorImageChannelSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverCursorImageChannelSettingsVtbl {
    pub const fn new<Impl: IMiracastReceiverCursorImageChannelSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverCursorImageChannelSettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMiracastReceiverCursorImageChannelSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IMiracastReceiverCursorImageChannelSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn MaxImageSize<Impl: IMiracastReceiverCursorImageChannelSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxImageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxImageSize<Impl: IMiracastReceiverCursorImageChannelSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxImageSize(&*(&value as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverCursorImageChannelSettings>, base.5, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>, MaxImageSize::<Impl, OFFSET>, SetMaxImageSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverDisconnectedEventArgsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverDisconnectedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverDisconnectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverDisconnectedEventArgsVtbl {
    pub const fn new<Impl: IMiracastReceiverDisconnectedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverDisconnectedEventArgsVtbl {
        unsafe extern "system" fn Connection<Impl: IMiracastReceiverDisconnectedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverDisconnectedEventArgs>, base.5, Connection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverGameControllerDeviceImpl: Sized {
    fn TransmitInput(&self) -> ::windows::core::Result<bool>;
    fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool>;
    fn IsTransmittingInput(&self) -> ::windows::core::Result<bool>;
    fn Mode(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDeviceUsageMode>;
    fn SetMode(&self, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::Result<()>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverGameControllerDevice {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverGameControllerDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverGameControllerDeviceVtbl {
    pub const fn new<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverGameControllerDeviceVtbl {
        unsafe extern "system" fn TransmitInput<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransmitInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmitInput<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransmitInput(value).into()
        }
        unsafe extern "system" fn IsRequestedByTransmitter<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRequestedByTransmitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransmittingInput<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTransmittingInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mode<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Changed<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IMiracastReceiverGameControllerDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverGameControllerDevice>, base.5, TransmitInput::<Impl, OFFSET>, SetTransmitInput::<Impl, OFFSET>, IsRequestedByTransmitter::<Impl, OFFSET>, IsTransmittingInput::<Impl, OFFSET>, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>, Changed::<Impl, OFFSET>, RemoveChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverInputDevicesImpl: Sized {
    fn Keyboard(&self) -> ::windows::core::Result<MiracastReceiverKeyboardDevice>;
    fn GameController(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverInputDevices {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverInputDevices";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverInputDevicesVtbl {
    pub const fn new<Impl: IMiracastReceiverInputDevicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverInputDevicesVtbl {
        unsafe extern "system" fn Keyboard<Impl: IMiracastReceiverInputDevicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Keyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GameController<Impl: IMiracastReceiverInputDevicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GameController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverInputDevices>, base.5, Keyboard::<Impl, OFFSET>, GameController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverKeyboardDeviceImpl: Sized {
    fn TransmitInput(&self) -> ::windows::core::Result<bool>;
    fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool>;
    fn IsTransmittingInput(&self) -> ::windows::core::Result<bool>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverKeyboardDevice {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverKeyboardDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverKeyboardDeviceVtbl {
    pub const fn new<Impl: IMiracastReceiverKeyboardDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverKeyboardDeviceVtbl {
        unsafe extern "system" fn TransmitInput<Impl: IMiracastReceiverKeyboardDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransmitInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmitInput<Impl: IMiracastReceiverKeyboardDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransmitInput(value).into()
        }
        unsafe extern "system" fn IsRequestedByTransmitter<Impl: IMiracastReceiverKeyboardDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRequestedByTransmitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransmittingInput<Impl: IMiracastReceiverKeyboardDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTransmittingInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IMiracastReceiverKeyboardDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IMiracastReceiverKeyboardDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverKeyboardDevice>, base.5, TransmitInput::<Impl, OFFSET>, SetTransmitInput::<Impl, OFFSET>, IsRequestedByTransmitter::<Impl, OFFSET>, IsTransmittingInput::<Impl, OFFSET>, Changed::<Impl, OFFSET>, RemoveChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverMediaSourceCreatedEventArgsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection>;
    fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn CursorImageChannelSettings(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannelSettings>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverMediaSourceCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverMediaSourceCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverMediaSourceCreatedEventArgsVtbl {
    pub const fn new<Impl: IMiracastReceiverMediaSourceCreatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverMediaSourceCreatedEventArgsVtbl {
        unsafe extern "system" fn Connection<Impl: IMiracastReceiverMediaSourceCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSource<Impl: IMiracastReceiverMediaSourceCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CursorImageChannelSettings<Impl: IMiracastReceiverMediaSourceCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CursorImageChannelSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMiracastReceiverMediaSourceCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverMediaSourceCreatedEventArgs>, base.5, Connection::<Impl, OFFSET>, MediaSource::<Impl, OFFSET>, CursorImageChannelSettings::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSessionImpl: Sized {
    fn ConnectionCreated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaSourceCreated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaSourceCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disconnected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllowConnectionTakeover(&self) -> ::windows::core::Result<bool>;
    fn SetAllowConnectionTakeover(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32>;
    fn SetMaxSimultaneousConnections(&self, value: i32) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<MiracastReceiverSessionStartResult>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverSession {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverSession";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverSessionVtbl {
    pub const fn new<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverSessionVtbl {
        unsafe extern "system" fn ConnectionCreated<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionCreated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionCreated<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConnectionCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaSourceCreated<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaSourceCreated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMediaSourceCreated<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMediaSourceCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnected<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disconnected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisconnected<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDisconnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowConnectionTakeover<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowConnectionTakeover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowConnectionTakeover<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowConnectionTakeover(value).into()
        }
        unsafe extern "system" fn MaxSimultaneousConnections<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSimultaneousConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSimultaneousConnections<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxSimultaneousConnections(value).into()
        }
        unsafe extern "system" fn Start<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAsync<Impl: IMiracastReceiverSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
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
            ::windows::core::GetRuntimeClassName::<IMiracastReceiverSession>,
            base.5,
            ConnectionCreated::<Impl, OFFSET>,
            RemoveConnectionCreated::<Impl, OFFSET>,
            MediaSourceCreated::<Impl, OFFSET>,
            RemoveMediaSourceCreated::<Impl, OFFSET>,
            Disconnected::<Impl, OFFSET>,
            RemoveDisconnected::<Impl, OFFSET>,
            AllowConnectionTakeover::<Impl, OFFSET>,
            SetAllowConnectionTakeover::<Impl, OFFSET>,
            MaxSimultaneousConnections::<Impl, OFFSET>,
            SetMaxSimultaneousConnections::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            StartAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSessionStartResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MiracastReceiverSessionStartStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverSessionStartResult {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverSessionStartResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverSessionStartResultVtbl {
    pub const fn new<Impl: IMiracastReceiverSessionStartResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverSessionStartResultVtbl {
        unsafe extern "system" fn Status<Impl: IMiracastReceiverSessionStartResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverSessionStartStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IMiracastReceiverSessionStartResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverSessionStartResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSettingsImpl: Sized {
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AuthorizationMethod(&self) -> ::windows::core::Result<MiracastReceiverAuthorizationMethod>;
    fn SetAuthorizationMethod(&self, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::Result<()>;
    fn RequireAuthorizationFromKnownTransmitters(&self) -> ::windows::core::Result<bool>;
    fn SetRequireAuthorizationFromKnownTransmitters(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverSettings {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverSettingsVtbl {
    pub const fn new<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverSettingsVtbl {
        unsafe extern "system" fn FriendlyName<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ModelName<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModelName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelName<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetModelName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ModelNumber<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelNumber<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetModelNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthorizationMethod<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthorizationMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorizationMethod<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuthorizationMethod(value).into()
        }
        unsafe extern "system" fn RequireAuthorizationFromKnownTransmitters<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequireAuthorizationFromKnownTransmitters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireAuthorizationFromKnownTransmitters<Impl: IMiracastReceiverSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRequireAuthorizationFromKnownTransmitters(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMiracastReceiverSettings>,
            base.5,
            FriendlyName::<Impl, OFFSET>,
            SetFriendlyName::<Impl, OFFSET>,
            ModelName::<Impl, OFFSET>,
            SetModelName::<Impl, OFFSET>,
            ModelNumber::<Impl, OFFSET>,
            SetModelNumber::<Impl, OFFSET>,
            AuthorizationMethod::<Impl, OFFSET>,
            SetAuthorizationMethod::<Impl, OFFSET>,
            RequireAuthorizationFromKnownTransmitters::<Impl, OFFSET>,
            SetRequireAuthorizationFromKnownTransmitters::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverStatusImpl: Sized {
    fn ListeningStatus(&self) -> ::windows::core::Result<MiracastReceiverListeningStatus>;
    fn WiFiStatus(&self) -> ::windows::core::Result<MiracastReceiverWiFiStatus>;
    fn IsConnectionTakeoverSupported(&self) -> ::windows::core::Result<bool>;
    fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32>;
    fn KnownTransmitters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverStatus {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverStatusVtbl {
    pub const fn new<Impl: IMiracastReceiverStatusImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverStatusVtbl {
        unsafe extern "system" fn ListeningStatus<Impl: IMiracastReceiverStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverListeningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListeningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WiFiStatus<Impl: IMiracastReceiverStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverWiFiStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WiFiStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectionTakeoverSupported<Impl: IMiracastReceiverStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnectionTakeoverSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSimultaneousConnections<Impl: IMiracastReceiverStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSimultaneousConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnownTransmitters<Impl: IMiracastReceiverStatusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KnownTransmitters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverStatus>, base.5, ListeningStatus::<Impl, OFFSET>, WiFiStatus::<Impl, OFFSET>, IsConnectionTakeoverSupported::<Impl, OFFSET>, MaxSimultaneousConnections::<Impl, OFFSET>, KnownTransmitters::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverStreamControlImpl: Sized {
    fn GetVideoStreamSettings(&self) -> ::windows::core::Result<MiracastReceiverVideoStreamSettings>;
    fn GetVideoStreamSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>>;
    fn SuggestVideoStreamSettings(&self, settings: &::core::option::Option<MiracastReceiverVideoStreamSettings>) -> ::windows::core::Result<()>;
    fn SuggestVideoStreamSettingsAsync(&self, settings: &::core::option::Option<MiracastReceiverVideoStreamSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MuteAudio(&self) -> ::windows::core::Result<bool>;
    fn SetMuteAudio(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverStreamControl {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverStreamControl";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverStreamControlVtbl {
    pub const fn new<Impl: IMiracastReceiverStreamControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverStreamControlVtbl {
        unsafe extern "system" fn GetVideoStreamSettings<Impl: IMiracastReceiverStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoStreamSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoStreamSettingsAsync<Impl: IMiracastReceiverStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoStreamSettingsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuggestVideoStreamSettings<Impl: IMiracastReceiverStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SuggestVideoStreamSettings(&*(&settings as *const <MiracastReceiverVideoStreamSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverVideoStreamSettings as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuggestVideoStreamSettingsAsync<Impl: IMiracastReceiverStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestVideoStreamSettingsAsync(&*(&settings as *const <MiracastReceiverVideoStreamSettings as ::windows::core::Abi>::Abi as *const <MiracastReceiverVideoStreamSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MuteAudio<Impl: IMiracastReceiverStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MuteAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMuteAudio<Impl: IMiracastReceiverStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMuteAudio(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverStreamControl>, base.5, GetVideoStreamSettings::<Impl, OFFSET>, GetVideoStreamSettingsAsync::<Impl, OFFSET>, SuggestVideoStreamSettings::<Impl, OFFSET>, SuggestVideoStreamSettingsAsync::<Impl, OFFSET>, MuteAudio::<Impl, OFFSET>, SetMuteAudio::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverVideoStreamSettingsImpl: Sized {
    fn Size(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn SetSize(&self, value: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn Bitrate(&self) -> ::windows::core::Result<i32>;
    fn SetBitrate(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastReceiverVideoStreamSettings {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastReceiverVideoStreamSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastReceiverVideoStreamSettingsVtbl {
    pub const fn new<Impl: IMiracastReceiverVideoStreamSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastReceiverVideoStreamSettingsVtbl {
        unsafe extern "system" fn Size<Impl: IMiracastReceiverVideoStreamSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IMiracastReceiverVideoStreamSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSize(&*(&value as *const <super::super::Graphics::SizeInt32 as ::windows::core::Abi>::Abi as *const <super::super::Graphics::SizeInt32 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bitrate<Impl: IMiracastReceiverVideoStreamSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrate<Impl: IMiracastReceiverVideoStreamSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitrate(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastReceiverVideoStreamSettings>, base.5, Size::<Impl, OFFSET>, SetSize::<Impl, OFFSET>, Bitrate::<Impl, OFFSET>, SetBitrate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastTransmitterImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AuthorizationStatus(&self) -> ::windows::core::Result<MiracastTransmitterAuthorizationStatus>;
    fn SetAuthorizationStatus(&self, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::Result<()>;
    fn GetConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>>;
    fn MacAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastConnectionTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMiracastTransmitter {
    const NAME: &'static str = "Windows.Media.Miracast.IMiracastTransmitter";
}
#[cfg(feature = "implement_exclusive")]
impl IMiracastTransmitterVtbl {
    pub const fn new<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMiracastTransmitterVtbl {
        unsafe extern "system" fn Name<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthorizationStatus<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthorizationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorizationStatus<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuthorizationStatus(value).into()
        }
        unsafe extern "system" fn GetConnections<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacAddress<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MacAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastConnectionTime<Impl: IMiracastTransmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastConnectionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMiracastTransmitter>, base.5, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>, AuthorizationStatus::<Impl, OFFSET>, SetAuthorizationStatus::<Impl, OFFSET>, GetConnections::<Impl, OFFSET>, MacAddress::<Impl, OFFSET>, LastConnectionTime::<Impl, OFFSET>)
    }
}
