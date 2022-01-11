#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameBarStaticsImpl: Sized {
    fn VisibilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsInputRedirectedChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsInputRedirectedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn IsInputRedirected(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameBarStatics {
    const NAME: &'static str = "Windows.Gaming.UI.IGameBarStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameBarStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameBarStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameBarStaticsVtbl {
        unsafe extern "system" fn VisibilityChanged<Impl: IGameBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibilityChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Impl: IGameBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisibilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsInputRedirectedChanged<Impl: IGameBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputRedirectedChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsInputRedirectedChanged<Impl: IGameBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsInputRedirectedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Visible<Impl: IGameBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputRedirected<Impl: IGameBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputRedirected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameBarStatics, BASE_OFFSET>(),
            VisibilityChanged: VisibilityChanged::<Impl, IMPL_OFFSET>,
            RemoveVisibilityChanged: RemoveVisibilityChanged::<Impl, IMPL_OFFSET>,
            IsInputRedirectedChanged: IsInputRedirectedChanged::<Impl, IMPL_OFFSET>,
            RemoveIsInputRedirectedChanged: RemoveIsInputRedirectedChanged::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            IsInputRedirected: IsInputRedirected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameBarStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatMessageReceivedEventArgsImpl: Sized {
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SenderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Origin(&self) -> ::windows::core::Result<GameChatMessageOrigin>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameChatMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.IGameChatMessageReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGameChatMessageReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameChatMessageReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameChatMessageReceivedEventArgsVtbl {
        unsafe extern "system" fn AppId<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppDisplayName<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderName<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Origin<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameChatMessageOrigin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameChatMessageReceivedEventArgs, BASE_OFFSET>(),
            AppId: AppId::<Impl, IMPL_OFFSET>,
            AppDisplayName: AppDisplayName::<Impl, IMPL_OFFSET>,
            SenderName: SenderName::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            Origin: Origin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameChatMessageReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatOverlayImpl: Sized {
    fn DesiredPosition(&self) -> ::windows::core::Result<GameChatOverlayPosition>;
    fn SetDesiredPosition(&self, value: GameChatOverlayPosition) -> ::windows::core::Result<()>;
    fn AddMessage(&self, sender: &::windows::core::HSTRING, message: &::windows::core::HSTRING, origin: GameChatMessageOrigin) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameChatOverlay {
    const NAME: &'static str = "Windows.Gaming.UI.IGameChatOverlay";
}
#[cfg(feature = "implement_exclusive")]
impl IGameChatOverlayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameChatOverlayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameChatOverlayVtbl {
        unsafe extern "system" fn DesiredPosition<Impl: IGameChatOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameChatOverlayPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPosition<Impl: IGameChatOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GameChatOverlayPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredPosition(value).into()
        }
        unsafe extern "system" fn AddMessage<Impl: IGameChatOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, origin: GameChatMessageOrigin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMessage(&*(&sender as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), origin).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameChatOverlay, BASE_OFFSET>(),
            DesiredPosition: DesiredPosition::<Impl, IMPL_OFFSET>,
            SetDesiredPosition: SetDesiredPosition::<Impl, IMPL_OFFSET>,
            AddMessage: AddMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameChatOverlay as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameChatOverlayMessageSourceImpl: Sized {
    fn MessageReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDelayBeforeClosingAfterMessageReceived(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameChatOverlayMessageSource {
    const NAME: &'static str = "Windows.Gaming.UI.IGameChatOverlayMessageSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameChatOverlayMessageSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameChatOverlayMessageSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameChatOverlayMessageSourceVtbl {
        unsafe extern "system" fn MessageReceived<Impl: IGameChatOverlayMessageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceived<Impl: IGameChatOverlayMessageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDelayBeforeClosingAfterMessageReceived<Impl: IGameChatOverlayMessageSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelayBeforeClosingAfterMessageReceived(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameChatOverlayMessageSource, BASE_OFFSET>(),
            MessageReceived: MessageReceived::<Impl, IMPL_OFFSET>,
            RemoveMessageReceived: RemoveMessageReceived::<Impl, IMPL_OFFSET>,
            SetDelayBeforeClosingAfterMessageReceived: SetDelayBeforeClosingAfterMessageReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameChatOverlayMessageSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatOverlayStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GameChatOverlay>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameChatOverlayStatics {
    const NAME: &'static str = "Windows.Gaming.UI.IGameChatOverlayStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameChatOverlayStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameChatOverlayStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameChatOverlayStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IGameChatOverlayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGameChatOverlayStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameChatOverlayStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGameUIProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn GameUIArgs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn ReportCompleted(&self, results: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameUIProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.IGameUIProviderActivatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGameUIProviderActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameUIProviderActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameUIProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn GameUIArgs<Impl: IGameUIProviderActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameUIArgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IGameUIProviderActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, results: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&results as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameUIProviderActivatedEventArgs, BASE_OFFSET>(),
            GameUIArgs: GameUIArgs::<Impl, IMPL_OFFSET>,
            ReportCompleted: ReportCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameUIProviderActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
