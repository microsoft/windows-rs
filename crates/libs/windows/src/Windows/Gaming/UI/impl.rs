#[cfg(feature = "implement_exclusive")]
pub trait IGameBarStaticsImpl: Sized {
    fn VisibilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsInputRedirectedChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsInputRedirectedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn IsInputRedirected(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameBarStatics {
    const NAME: &'static str = "Windows.Gaming.UI.IGameBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameBarStaticsVtbl {
    pub const fn new<Impl: IGameBarStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameBarStaticsVtbl {
        unsafe extern "system" fn VisibilityChanged<Impl: IGameBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisibilityChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Impl: IGameBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVisibilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsInputRedirectedChanged<Impl: IGameBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInputRedirectedChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsInputRedirectedChanged<Impl: IGameBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsInputRedirectedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Visible<Impl: IGameBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputRedirected<Impl: IGameBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInputRedirected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameBarStatics>, base.5, VisibilityChanged::<Impl, OFFSET>, RemoveVisibilityChanged::<Impl, OFFSET>, IsInputRedirectedChanged::<Impl, OFFSET>, RemoveIsInputRedirectedChanged::<Impl, OFFSET>, Visible::<Impl, OFFSET>, IsInputRedirected::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameChatMessageReceivedEventArgsVtbl {
        unsafe extern "system" fn AppId<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppDisplayName<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderName<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SenderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Impl: IGameChatMessageReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameChatMessageOrigin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameChatMessageReceivedEventArgs>, base.5, AppId::<Impl, OFFSET>, AppDisplayName::<Impl, OFFSET>, SenderName::<Impl, OFFSET>, Message::<Impl, OFFSET>, Origin::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGameChatOverlayImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameChatOverlayVtbl {
        unsafe extern "system" fn DesiredPosition<Impl: IGameChatOverlayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameChatOverlayPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPosition<Impl: IGameChatOverlayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: GameChatOverlayPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredPosition(value).into()
        }
        unsafe extern "system" fn AddMessage<Impl: IGameChatOverlayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, origin: GameChatMessageOrigin) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddMessage(&*(&sender as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), origin).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameChatOverlay>, base.5, DesiredPosition::<Impl, OFFSET>, SetDesiredPosition::<Impl, OFFSET>, AddMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameChatOverlayMessageSourceImpl: Sized {
    fn MessageReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDelayBeforeClosingAfterMessageReceived(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameChatOverlayMessageSource {
    const NAME: &'static str = "Windows.Gaming.UI.IGameChatOverlayMessageSource";
}
#[cfg(feature = "implement_exclusive")]
impl IGameChatOverlayMessageSourceVtbl {
    pub const fn new<Impl: IGameChatOverlayMessageSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameChatOverlayMessageSourceVtbl {
        unsafe extern "system" fn MessageReceived<Impl: IGameChatOverlayMessageSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MessageReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceived<Impl: IGameChatOverlayMessageSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDelayBeforeClosingAfterMessageReceived<Impl: IGameChatOverlayMessageSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDelayBeforeClosingAfterMessageReceived(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameChatOverlayMessageSource>, base.5, MessageReceived::<Impl, OFFSET>, RemoveMessageReceived::<Impl, OFFSET>, SetDelayBeforeClosingAfterMessageReceived::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGameChatOverlayStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameChatOverlayStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IGameChatOverlayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameChatOverlayStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IGameUIProviderActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl {
    fn GameUIArgs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn ReportCompleted(&self, results: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameUIProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.IGameUIProviderActivatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl IGameUIProviderActivatedEventArgsVtbl {
    pub const fn new<Impl: IGameUIProviderActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGameUIProviderActivatedEventArgsVtbl {
        unsafe extern "system" fn GameUIArgs<Impl: IGameUIProviderActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GameUIArgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IGameUIProviderActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, results: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&results as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGameUIProviderActivatedEventArgs>, base.5, GameUIArgs::<Impl, OFFSET>, ReportCompleted::<Impl, OFFSET>)
    }
}
