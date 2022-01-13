#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICurrentTimeChangeRequestedEventArgsImpl: Sized {
    fn Time(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICurrentTimeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.ICurrentTimeChangeRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICurrentTimeChangeRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentTimeChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentTimeChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn Time<Impl: ICurrentTimeChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Time() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentTimeChangeRequestedEventArgs, BASE_OFFSET>(), Time: Time::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentTimeChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMuteChangeRequestedEventArgsImpl: Sized {
    fn Mute(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMuteChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IMuteChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMuteChangeRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMuteChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMuteChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn Mute<Impl: IMuteChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMuteChangeRequestedEventArgs, BASE_OFFSET>(), Mute: Mute::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMuteChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<PlayToConnectionState>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Transferred(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransferred(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Error(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveError(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToConnection {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToConnection";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToConnectionVtbl {
        unsafe extern "system" fn State<Impl: IPlayToConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StateChanged<Impl: IPlayToConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IPlayToConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Transferred<Impl: IPlayToConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transferred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransferred<Impl: IPlayToConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransferred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Error<Impl: IPlayToConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveError<Impl: IPlayToConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveError(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToConnection, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            Transferred: Transferred::<Impl, IMPL_OFFSET>,
            RemoveTransferred: RemoveTransferred::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            RemoveError: RemoveError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionErrorEventArgsImpl: Sized {
    fn Code(&mut self) -> ::windows::core::Result<PlayToConnectionError>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToConnectionErrorEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToConnectionErrorEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToConnectionErrorEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToConnectionErrorEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToConnectionErrorEventArgsVtbl {
        unsafe extern "system" fn Code<Impl: IPlayToConnectionErrorEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IPlayToConnectionErrorEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToConnectionErrorEventArgs, BASE_OFFSET>(),
            Code: Code::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToConnectionErrorEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionStateChangedEventArgsImpl: Sized {
    fn PreviousState(&mut self) -> ::windows::core::Result<PlayToConnectionState>;
    fn CurrentState(&mut self) -> ::windows::core::Result<PlayToConnectionState>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToConnectionStateChangedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToConnectionStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToConnectionStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToConnectionStateChangedEventArgsVtbl {
        unsafe extern "system" fn PreviousState<Impl: IPlayToConnectionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Impl: IPlayToConnectionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToConnectionStateChangedEventArgs, BASE_OFFSET>(),
            PreviousState: PreviousState::<Impl, IMPL_OFFSET>,
            CurrentState: CurrentState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToConnectionStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToConnectionTransferredEventArgsImpl: Sized {
    fn PreviousSource(&mut self) -> ::windows::core::Result<PlayToSource>;
    fn CurrentSource(&mut self) -> ::windows::core::Result<PlayToSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToConnectionTransferredEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToConnectionTransferredEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToConnectionTransferredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToConnectionTransferredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToConnectionTransferredEventArgsVtbl {
        unsafe extern "system" fn PreviousSource<Impl: IPlayToConnectionTransferredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSource<Impl: IPlayToConnectionTransferredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToConnectionTransferredEventArgs, BASE_OFFSET>(),
            PreviousSource: PreviousSource::<Impl, IMPL_OFFSET>,
            CurrentSource: CurrentSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToConnectionTransferredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToManagerImpl: Sized {
    fn SourceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceSelected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceSelected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDefaultSourceSelection(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DefaultSourceSelection(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToManager {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToManager";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToManagerVtbl {
        unsafe extern "system" fn SourceRequested<Impl: IPlayToManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceRequested<Impl: IPlayToManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceSelected<Impl: IPlayToManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceSelected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceSelected<Impl: IPlayToManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceSelected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDefaultSourceSelection<Impl: IPlayToManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultSourceSelection(value).into()
        }
        unsafe extern "system" fn DefaultSourceSelection<Impl: IPlayToManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultSourceSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToManager, BASE_OFFSET>(),
            SourceRequested: SourceRequested::<Impl, IMPL_OFFSET>,
            RemoveSourceRequested: RemoveSourceRequested::<Impl, IMPL_OFFSET>,
            SourceSelected: SourceSelected::<Impl, IMPL_OFFSET>,
            RemoveSourceSelected: RemoveSourceSelected::<Impl, IMPL_OFFSET>,
            SetDefaultSourceSelection: SetDefaultSourceSelection::<Impl, IMPL_OFFSET>,
            DefaultSourceSelection: DefaultSourceSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToManagerStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<PlayToManager>;
    fn ShowPlayToUI(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToManagerStatics {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToManagerStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IPlayToManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPlayToUI<Impl: IPlayToManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPlayToUI().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToManagerStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            ShowPlayToUI: ShowPlayToUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPlayToReceiverImpl: Sized {
    fn PlayRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PauseRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePauseRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChangeRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChangeRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChangeRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChangeRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentTimeChangeRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentTimeChangeRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MuteChangeRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMuteChangeRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VolumeChangeRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVolumeChangeRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TimeUpdateRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimeUpdateRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StopRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyVolumeChange(&mut self, volume: f64, mute: bool) -> ::windows::core::Result<()>;
    fn NotifyRateChange(&mut self, rate: f64) -> ::windows::core::Result<()>;
    fn NotifyLoadedMetadata(&mut self) -> ::windows::core::Result<()>;
    fn NotifyTimeUpdate(&mut self, currenttime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn NotifyDurationChange(&mut self, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn NotifySeeking(&mut self) -> ::windows::core::Result<()>;
    fn NotifySeeked(&mut self) -> ::windows::core::Result<()>;
    fn NotifyPaused(&mut self) -> ::windows::core::Result<()>;
    fn NotifyPlaying(&mut self) -> ::windows::core::Result<()>;
    fn NotifyEnded(&mut self) -> ::windows::core::Result<()>;
    fn NotifyError(&mut self) -> ::windows::core::Result<()>;
    fn NotifyStopped(&mut self) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetSupportsImage(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsImage(&mut self) -> ::windows::core::Result<bool>;
    fn SetSupportsAudio(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsAudio(&mut self) -> ::windows::core::Result<bool>;
    fn SetSupportsVideo(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsVideo(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToReceiver {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToReceiver";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPlayToReceiverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToReceiverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToReceiverVtbl {
        unsafe extern "system" fn PlayRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlayRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePauseRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePauseRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceChangeRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceChangeRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackRateChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRateChangeRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackRateChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackRateChangeRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentTimeChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTimeChangeRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentTimeChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentTimeChangeRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MuteChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MuteChangeRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMuteChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMuteChangeRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VolumeChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeChangeRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVolumeChangeRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVolumeChangeRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimeUpdateRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeUpdateRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTimeUpdateRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTimeUpdateRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopRequested<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyVolumeChange<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f64, mute: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyVolumeChange(volume, mute).into()
        }
        unsafe extern "system" fn NotifyRateChange<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyRateChange(rate).into()
        }
        unsafe extern "system" fn NotifyLoadedMetadata<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyLoadedMetadata().into()
        }
        unsafe extern "system" fn NotifyTimeUpdate<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currenttime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyTimeUpdate(&*(&currenttime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyDurationChange<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyDurationChange(&*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifySeeking<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySeeking().into()
        }
        unsafe extern "system" fn NotifySeeked<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySeeked().into()
        }
        unsafe extern "system" fn NotifyPaused<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyPaused().into()
        }
        unsafe extern "system" fn NotifyPlaying<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyPlaying().into()
        }
        unsafe extern "system" fn NotifyEnded<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyEnded().into()
        }
        unsafe extern "system" fn NotifyError<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyError().into()
        }
        unsafe extern "system" fn NotifyStopped<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyStopped().into()
        }
        unsafe extern "system" fn FriendlyName<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFriendlyName<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSupportsImage<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportsImage(value).into()
        }
        unsafe extern "system" fn SupportsImage<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportsAudio<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportsAudio(value).into()
        }
        unsafe extern "system" fn SupportsAudio<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportsVideo<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportsVideo(value).into()
        }
        unsafe extern "system" fn SupportsVideo<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StopAsync<Impl: IPlayToReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToReceiver, BASE_OFFSET>(),
            PlayRequested: PlayRequested::<Impl, IMPL_OFFSET>,
            RemovePlayRequested: RemovePlayRequested::<Impl, IMPL_OFFSET>,
            PauseRequested: PauseRequested::<Impl, IMPL_OFFSET>,
            RemovePauseRequested: RemovePauseRequested::<Impl, IMPL_OFFSET>,
            SourceChangeRequested: SourceChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveSourceChangeRequested: RemoveSourceChangeRequested::<Impl, IMPL_OFFSET>,
            PlaybackRateChangeRequested: PlaybackRateChangeRequested::<Impl, IMPL_OFFSET>,
            RemovePlaybackRateChangeRequested: RemovePlaybackRateChangeRequested::<Impl, IMPL_OFFSET>,
            CurrentTimeChangeRequested: CurrentTimeChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveCurrentTimeChangeRequested: RemoveCurrentTimeChangeRequested::<Impl, IMPL_OFFSET>,
            MuteChangeRequested: MuteChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveMuteChangeRequested: RemoveMuteChangeRequested::<Impl, IMPL_OFFSET>,
            VolumeChangeRequested: VolumeChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveVolumeChangeRequested: RemoveVolumeChangeRequested::<Impl, IMPL_OFFSET>,
            TimeUpdateRequested: TimeUpdateRequested::<Impl, IMPL_OFFSET>,
            RemoveTimeUpdateRequested: RemoveTimeUpdateRequested::<Impl, IMPL_OFFSET>,
            StopRequested: StopRequested::<Impl, IMPL_OFFSET>,
            RemoveStopRequested: RemoveStopRequested::<Impl, IMPL_OFFSET>,
            NotifyVolumeChange: NotifyVolumeChange::<Impl, IMPL_OFFSET>,
            NotifyRateChange: NotifyRateChange::<Impl, IMPL_OFFSET>,
            NotifyLoadedMetadata: NotifyLoadedMetadata::<Impl, IMPL_OFFSET>,
            NotifyTimeUpdate: NotifyTimeUpdate::<Impl, IMPL_OFFSET>,
            NotifyDurationChange: NotifyDurationChange::<Impl, IMPL_OFFSET>,
            NotifySeeking: NotifySeeking::<Impl, IMPL_OFFSET>,
            NotifySeeked: NotifySeeked::<Impl, IMPL_OFFSET>,
            NotifyPaused: NotifyPaused::<Impl, IMPL_OFFSET>,
            NotifyPlaying: NotifyPlaying::<Impl, IMPL_OFFSET>,
            NotifyEnded: NotifyEnded::<Impl, IMPL_OFFSET>,
            NotifyError: NotifyError::<Impl, IMPL_OFFSET>,
            NotifyStopped: NotifyStopped::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            SetSupportsImage: SetSupportsImage::<Impl, IMPL_OFFSET>,
            SupportsImage: SupportsImage::<Impl, IMPL_OFFSET>,
            SetSupportsAudio: SetSupportsAudio::<Impl, IMPL_OFFSET>,
            SupportsAudio: SupportsAudio::<Impl, IMPL_OFFSET>,
            SetSupportsVideo: SetSupportsVideo::<Impl, IMPL_OFFSET>,
            SupportsVideo: SupportsVideo::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            StopAsync: StopAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToReceiver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceImpl: Sized {
    fn Connection(&mut self) -> ::windows::core::Result<PlayToConnection>;
    fn Next(&mut self) -> ::windows::core::Result<PlayToSource>;
    fn SetNext(&mut self, value: &::core::option::Option<PlayToSource>) -> ::windows::core::Result<()>;
    fn PlayNext(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToSource {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToSource";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToSourceVtbl {
        unsafe extern "system" fn Connection<Impl: IPlayToSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IPlayToSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNext<Impl: IPlayToSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNext(&*(&value as *const <PlayToSource as ::windows::core::Abi>::Abi as *const <PlayToSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayNext<Impl: IPlayToSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayNext().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToSource, BASE_OFFSET>(),
            Connection: Connection::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            SetNext: SetNext::<Impl, IMPL_OFFSET>,
            PlayNext: PlayNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToSourceDeferral {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToSourceDeferral";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToSourceDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToSourceDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToSourceDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IPlayToSourceDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToSourceDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToSourceDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceRequestImpl: Sized {
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn DisplayErrorString(&mut self, errorstring: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<PlayToSourceDeferral>;
    fn SetSource(&mut self, value: &::core::option::Option<PlayToSource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToSourceRequest {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToSourceRequest";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToSourceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToSourceRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToSourceRequestVtbl {
        unsafe extern "system" fn Deadline<Impl: IPlayToSourceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayErrorString<Impl: IPlayToSourceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayErrorString(&*(&errorstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPlayToSourceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: IPlayToSourceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <PlayToSource as ::windows::core::Abi>::Abi as *const <PlayToSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToSourceRequest, BASE_OFFSET>(),
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            DisplayErrorString: DisplayErrorString::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToSourceRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceRequestedEventArgsImpl: Sized {
    fn SourceRequest(&mut self) -> ::windows::core::Result<PlayToSourceRequest>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToSourceRequestedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToSourceRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToSourceRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToSourceRequestedEventArgsVtbl {
        unsafe extern "system" fn SourceRequest<Impl: IPlayToSourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToSourceRequestedEventArgs, BASE_OFFSET>(),
            SourceRequest: SourceRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToSourceRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceSelectedEventArgsImpl: Sized {
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Icon(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn SupportsImage(&mut self) -> ::windows::core::Result<bool>;
    fn SupportsAudio(&mut self) -> ::windows::core::Result<bool>;
    fn SupportsVideo(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToSourceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToSourceSelectedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToSourceSelectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToSourceSelectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToSourceSelectedEventArgsVtbl {
        unsafe extern "system" fn FriendlyName<Impl: IPlayToSourceSelectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Icon<Impl: IPlayToSourceSelectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Icon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsImage<Impl: IPlayToSourceSelectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsAudio<Impl: IPlayToSourceSelectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsVideo<Impl: IPlayToSourceSelectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToSourceSelectedEventArgs, BASE_OFFSET>(),
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            Icon: Icon::<Impl, IMPL_OFFSET>,
            SupportsImage: SupportsImage::<Impl, IMPL_OFFSET>,
            SupportsAudio: SupportsAudio::<Impl, IMPL_OFFSET>,
            SupportsVideo: SupportsVideo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToSourceSelectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPlayToSourceWithPreferredSourceUriImpl: Sized {
    fn PreferredSourceUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetPreferredSourceUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlayToSourceWithPreferredSourceUri {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlayToSourceWithPreferredSourceUri";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IPlayToSourceWithPreferredSourceUriVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToSourceWithPreferredSourceUriImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToSourceWithPreferredSourceUriVtbl {
        unsafe extern "system" fn PreferredSourceUri<Impl: IPlayToSourceWithPreferredSourceUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredSourceUri<Impl: IPlayToSourceWithPreferredSourceUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredSourceUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToSourceWithPreferredSourceUri, BASE_OFFSET>(),
            PreferredSourceUri: PreferredSourceUri::<Impl, IMPL_OFFSET>,
            SetPreferredSourceUri: SetPreferredSourceUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToSourceWithPreferredSourceUri as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackRateChangeRequestedEventArgsImpl: Sized {
    fn Rate(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IPlaybackRateChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaybackRateChangeRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackRateChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackRateChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn Rate<Impl: IPlaybackRateChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaybackRateChangeRequestedEventArgs, BASE_OFFSET>(), Rate: Rate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackRateChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISourceChangeRequestedEventArgsImpl: Sized {
    fn Stream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Album(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Genre(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Date(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn Rating(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISourceChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.ISourceChangeRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISourceChangeRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISourceChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISourceChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn Stream<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Album<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Album() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Genre<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Genre() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rating<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ISourceChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISourceChangeRequestedEventArgs, BASE_OFFSET>(),
            Stream: Stream::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
            Album: Album::<Impl, IMPL_OFFSET>,
            Genre: Genre::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            Rating: Rating::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISourceChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVolumeChangeRequestedEventArgsImpl: Sized {
    fn Volume(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVolumeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.IVolumeChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVolumeChangeRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVolumeChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVolumeChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn Volume<Impl: IVolumeChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVolumeChangeRequestedEventArgs, BASE_OFFSET>(), Volume: Volume::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVolumeChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
