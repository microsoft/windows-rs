#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundMediaPlayerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<MediaPlayer>;
    fn MessageReceivedFromBackground(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceivedFromBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MessageReceivedFromForeground(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceivedFromForeground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SendMessageToBackground(&self, value: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
    fn SendMessageToForeground(&self, value: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
    fn IsMediaPlaying(&self) -> ::windows::core::Result<bool>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundMediaPlayerStatics {
    const NAME: &'static str = "Windows.Media.Playback.IBackgroundMediaPlayerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundMediaPlayerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundMediaPlayerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundMediaPlayerStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageReceivedFromBackground<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageReceivedFromBackground(&*(&value as *const <super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceivedFromBackground<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceivedFromBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MessageReceivedFromForeground<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageReceivedFromForeground(&*(&value as *const <super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceivedFromForeground<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceivedFromForeground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendMessageToBackground<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessageToBackground(&*(&value as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendMessageToForeground<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessageToForeground(&*(&value as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsMediaPlaying<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMediaPlaying() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IBackgroundMediaPlayerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBackgroundMediaPlayerStatics>,
            ::windows::core::GetTrustLevel,
            Current::<Impl, IMPL_OFFSET>,
            MessageReceivedFromBackground::<Impl, IMPL_OFFSET>,
            RemoveMessageReceivedFromBackground::<Impl, IMPL_OFFSET>,
            MessageReceivedFromForeground::<Impl, IMPL_OFFSET>,
            RemoveMessageReceivedFromForeground::<Impl, IMPL_OFFSET>,
            SendMessageToBackground::<Impl, IMPL_OFFSET>,
            SendMessageToForeground::<Impl, IMPL_OFFSET>,
            IsMediaPlaying::<Impl, IMPL_OFFSET>,
            Shutdown::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundMediaPlayerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentMediaPlaybackItemChangedEventArgsImpl: Sized {
    fn NewItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn OldItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrentMediaPlaybackItemChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.ICurrentMediaPlaybackItemChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrentMediaPlaybackItemChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentMediaPlaybackItemChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentMediaPlaybackItemChangedEventArgsVtbl {
        unsafe extern "system" fn NewItem<Impl: ICurrentMediaPlaybackItemChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldItem<Impl: ICurrentMediaPlaybackItemChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICurrentMediaPlaybackItemChangedEventArgs>, ::windows::core::GetTrustLevel, NewItem::<Impl, IMPL_OFFSET>, OldItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentMediaPlaybackItemChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentMediaPlaybackItemChangedEventArgs2Impl: Sized + ICurrentMediaPlaybackItemChangedEventArgsImpl {
    fn Reason(&self) -> ::windows::core::Result<MediaPlaybackItemChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrentMediaPlaybackItemChangedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Playback.ICurrentMediaPlaybackItemChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrentMediaPlaybackItemChangedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentMediaPlaybackItemChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentMediaPlaybackItemChangedEventArgs2Vtbl {
        unsafe extern "system" fn Reason<Impl: ICurrentMediaPlaybackItemChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemChangedReason) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICurrentMediaPlaybackItemChangedEventArgs2>, ::windows::core::GetTrustLevel, Reason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentMediaPlaybackItemChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaBreakImpl: Sized {
    fn PlaybackList(&self) -> ::windows::core::Result<MediaPlaybackList>;
    fn PresentationPosition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn InsertionMethod(&self) -> ::windows::core::Result<MediaBreakInsertionMethod>;
    fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn CanStart(&self) -> ::windows::core::Result<bool>;
    fn SetCanStart(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreak {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreak";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaBreakVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakVtbl {
        unsafe extern "system" fn PlaybackList<Impl: IMediaBreakImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationPosition<Impl: IMediaBreakImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertionMethod<Impl: IMediaBreakImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaBreakInsertionMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertionMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomProperties<Impl: IMediaBreakImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanStart<Impl: IMediaBreakImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanStart<Impl: IMediaBreakImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanStart(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaBreak>,
            ::windows::core::GetTrustLevel,
            PlaybackList::<Impl, IMPL_OFFSET>,
            PresentationPosition::<Impl, IMPL_OFFSET>,
            InsertionMethod::<Impl, IMPL_OFFSET>,
            CustomProperties::<Impl, IMPL_OFFSET>,
            CanStart::<Impl, IMPL_OFFSET>,
            SetCanStart::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreak as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakEndedEventArgsImpl: Sized {
    fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBreakEndedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakEndedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBreakEndedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakEndedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakEndedEventArgsVtbl {
        unsafe extern "system" fn MediaBreak<Impl: IMediaBreakEndedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaBreak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaBreakEndedEventArgs>, ::windows::core::GetTrustLevel, MediaBreak::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakEndedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaBreakFactoryImpl: Sized {
    fn Create(&self, insertionmethod: MediaBreakInsertionMethod) -> ::windows::core::Result<MediaBreak>;
    fn CreateWithPresentationPosition(&self, insertionmethod: MediaBreakInsertionMethod, presentationposition: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakFactory {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaBreakFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMediaBreakFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(insertionmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPresentationPosition<Impl: IMediaBreakFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, presentationposition: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithPresentationPosition(insertionmethod, &*(&presentationposition as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaBreakFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithPresentationPosition::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaBreakManagerImpl: Sized {
    fn BreaksSeekedOver(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreaksSeekedOver(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakEnded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakSkipped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakSkipped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentBreak(&self) -> ::windows::core::Result<MediaBreak>;
    fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession>;
    fn PlayBreak(&self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn SkipCurrentBreak(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakManager {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaBreakManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakManagerVtbl {
        unsafe extern "system" fn BreaksSeekedOver<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreaksSeekedOver(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBreaksSeekedOver<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreaksSeekedOver(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BreakStarted<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBreakStarted<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreakStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BreakEnded<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakEnded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBreakEnded<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreakEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BreakSkipped<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakSkipped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBreakSkipped<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreakSkipped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentBreak<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentBreak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaybackSession<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayBreak<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayBreak(&*(&value as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SkipCurrentBreak<Impl: IMediaBreakManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SkipCurrentBreak().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaBreakManager>,
            ::windows::core::GetTrustLevel,
            BreaksSeekedOver::<Impl, IMPL_OFFSET>,
            RemoveBreaksSeekedOver::<Impl, IMPL_OFFSET>,
            BreakStarted::<Impl, IMPL_OFFSET>,
            RemoveBreakStarted::<Impl, IMPL_OFFSET>,
            BreakEnded::<Impl, IMPL_OFFSET>,
            RemoveBreakEnded::<Impl, IMPL_OFFSET>,
            BreakSkipped::<Impl, IMPL_OFFSET>,
            RemoveBreakSkipped::<Impl, IMPL_OFFSET>,
            CurrentBreak::<Impl, IMPL_OFFSET>,
            PlaybackSession::<Impl, IMPL_OFFSET>,
            PlayBreak::<Impl, IMPL_OFFSET>,
            SkipCurrentBreak::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaBreakScheduleImpl: Sized {
    fn ScheduleChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakSchedule, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScheduleChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InsertMidrollBreak(&self, mediabreak: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn RemoveMidrollBreak(&self, mediabreak: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn MidrollBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>>;
    fn SetPrerollBreak(&self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn PrerollBreak(&self) -> ::windows::core::Result<MediaBreak>;
    fn SetPostrollBreak(&self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn PostrollBreak(&self) -> ::windows::core::Result<MediaBreak>;
    fn PlaybackItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakSchedule {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakSchedule";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaBreakScheduleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakScheduleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakScheduleVtbl {
        unsafe extern "system" fn ScheduleChanged<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduleChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBreakSchedule, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBreakSchedule, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScheduleChanged<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScheduleChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertMidrollBreak<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediabreak: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertMidrollBreak(&*(&mediabreak as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveMidrollBreak<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediabreak: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMidrollBreak(&*(&mediabreak as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MidrollBreaks<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MidrollBreaks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrerollBreak<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrerollBreak(&*(&value as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrerollBreak<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrerollBreak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostrollBreak<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostrollBreak(&*(&value as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PostrollBreak<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostrollBreak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaybackItem<Impl: IMediaBreakScheduleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaBreakSchedule>,
            ::windows::core::GetTrustLevel,
            ScheduleChanged::<Impl, IMPL_OFFSET>,
            RemoveScheduleChanged::<Impl, IMPL_OFFSET>,
            InsertMidrollBreak::<Impl, IMPL_OFFSET>,
            RemoveMidrollBreak::<Impl, IMPL_OFFSET>,
            MidrollBreaks::<Impl, IMPL_OFFSET>,
            SetPrerollBreak::<Impl, IMPL_OFFSET>,
            PrerollBreak::<Impl, IMPL_OFFSET>,
            SetPostrollBreak::<Impl, IMPL_OFFSET>,
            PostrollBreak::<Impl, IMPL_OFFSET>,
            PlaybackItem::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakSchedule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaBreakSeekedOverEventArgsImpl: Sized {
    fn SeekedOverBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>>;
    fn OldPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn NewPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakSeekedOverEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakSeekedOverEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaBreakSeekedOverEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakSeekedOverEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakSeekedOverEventArgsVtbl {
        unsafe extern "system" fn SeekedOverBreaks<Impl: IMediaBreakSeekedOverEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekedOverBreaks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPosition<Impl: IMediaBreakSeekedOverEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewPosition<Impl: IMediaBreakSeekedOverEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaBreakSeekedOverEventArgs>, ::windows::core::GetTrustLevel, SeekedOverBreaks::<Impl, IMPL_OFFSET>, OldPosition::<Impl, IMPL_OFFSET>, NewPosition::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakSeekedOverEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakSkippedEventArgsImpl: Sized {
    fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBreakSkippedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakSkippedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBreakSkippedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakSkippedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakSkippedEventArgsVtbl {
        unsafe extern "system" fn MediaBreak<Impl: IMediaBreakSkippedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaBreak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaBreakSkippedEventArgs>, ::windows::core::GetTrustLevel, MediaBreak::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakSkippedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakStartedEventArgsImpl: Sized {
    fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBreakStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBreakStartedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakStartedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakStartedEventArgsVtbl {
        unsafe extern "system" fn MediaBreak<Impl: IMediaBreakStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaBreak() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaBreakStartedEventArgs>, ::windows::core::GetTrustLevel, MediaBreak::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait IMediaEnginePlaybackSourceImpl: Sized {
    fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn SetPlaybackSource(&self, source: &::core::option::Option<IMediaPlaybackSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IMediaEnginePlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaEnginePlaybackSource";
}
#[cfg(feature = "deprecated")]
impl IMediaEnginePlaybackSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEnginePlaybackSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEnginePlaybackSourceVtbl {
        unsafe extern "system" fn CurrentItem<Impl: IMediaEnginePlaybackSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackSource<Impl: IMediaEnginePlaybackSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackSource(&*(&source as *const <IMediaPlaybackSource as ::windows::core::Abi>::Abi as *const <IMediaPlaybackSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaEnginePlaybackSource>, ::windows::core::GetTrustLevel, CurrentItem::<Impl, IMPL_OFFSET>, SetPlaybackSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEnginePlaybackSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaItemDisplayPropertiesImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<super::MediaPlaybackType>;
    fn SetType(&self, value: super::MediaPlaybackType) -> ::windows::core::Result<()>;
    fn MusicProperties(&self) -> ::windows::core::Result<super::MusicDisplayProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<super::VideoDisplayProperties>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn ClearAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaItemDisplayProperties {
    const NAME: &'static str = "Windows.Media.Playback.IMediaItemDisplayProperties";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaItemDisplayPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaItemDisplayPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaItemDisplayPropertiesVtbl {
        unsafe extern "system" fn Type<Impl: IMediaItemDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IMediaItemDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaPlaybackType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn MusicProperties<Impl: IMediaItemDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MusicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProperties<Impl: IMediaItemDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Impl: IMediaItemDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: IMediaItemDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearAll<Impl: IMediaItemDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAll().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaItemDisplayProperties>,
            ::windows::core::GetTrustLevel,
            Type::<Impl, IMPL_OFFSET>,
            SetType::<Impl, IMPL_OFFSET>,
            MusicProperties::<Impl, IMPL_OFFSET>,
            VideoProperties::<Impl, IMPL_OFFSET>,
            Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail::<Impl, IMPL_OFFSET>,
            ClearAll::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaItemDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer>;
    fn PlayBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PauseBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn NextBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PreviousBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn FastForwardBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn RewindBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn ShuffleBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn AutoRepeatModeBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PositionBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn RateBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PlayReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PauseReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePauseReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NextReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNextReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviousReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviousReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FastForwardReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFastForwardReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RewindReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRewindReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShuffleReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShuffleReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoRepeatModeReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoRepeatModeReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RateReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRateReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManager {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn MediaPlayer<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlayer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FastForwardBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FastForwardBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RewindBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RewindBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShuffleBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShuffleBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRepeatModeBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRepeatModeBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RateBehavior<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RateBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlayReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePauseReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePauseReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NextReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNextReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNextReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviousReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviousReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviousReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FastForwardReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FastForwardReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFastForwardReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFastForwardReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RewindReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RewindReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRewindReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRewindReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShuffleReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShuffleReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShuffleReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShuffleReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoRepeatModeReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRepeatModeReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutoRepeatModeReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutoRepeatModeReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RateReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RateReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRateReceived<Impl: IMediaPlaybackCommandManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRateReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManager>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled::<Impl, IMPL_OFFSET>,
            MediaPlayer::<Impl, IMPL_OFFSET>,
            PlayBehavior::<Impl, IMPL_OFFSET>,
            PauseBehavior::<Impl, IMPL_OFFSET>,
            NextBehavior::<Impl, IMPL_OFFSET>,
            PreviousBehavior::<Impl, IMPL_OFFSET>,
            FastForwardBehavior::<Impl, IMPL_OFFSET>,
            RewindBehavior::<Impl, IMPL_OFFSET>,
            ShuffleBehavior::<Impl, IMPL_OFFSET>,
            AutoRepeatModeBehavior::<Impl, IMPL_OFFSET>,
            PositionBehavior::<Impl, IMPL_OFFSET>,
            RateBehavior::<Impl, IMPL_OFFSET>,
            PlayReceived::<Impl, IMPL_OFFSET>,
            RemovePlayReceived::<Impl, IMPL_OFFSET>,
            PauseReceived::<Impl, IMPL_OFFSET>,
            RemovePauseReceived::<Impl, IMPL_OFFSET>,
            NextReceived::<Impl, IMPL_OFFSET>,
            RemoveNextReceived::<Impl, IMPL_OFFSET>,
            PreviousReceived::<Impl, IMPL_OFFSET>,
            RemovePreviousReceived::<Impl, IMPL_OFFSET>,
            FastForwardReceived::<Impl, IMPL_OFFSET>,
            RemoveFastForwardReceived::<Impl, IMPL_OFFSET>,
            RewindReceived::<Impl, IMPL_OFFSET>,
            RemoveRewindReceived::<Impl, IMPL_OFFSET>,
            ShuffleReceived::<Impl, IMPL_OFFSET>,
            RemoveShuffleReceived::<Impl, IMPL_OFFSET>,
            AutoRepeatModeReceived::<Impl, IMPL_OFFSET>,
            RemoveAutoRepeatModeReceived::<Impl, IMPL_OFFSET>,
            PositionReceived::<Impl, IMPL_OFFSET>,
            RemovePositionReceived::<Impl, IMPL_OFFSET>,
            RateReceived::<Impl, IMPL_OFFSET>,
            RemoveRateReceived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoRepeatMode(&self) -> ::windows::core::Result<super::MediaPlaybackAutoRepeatMode>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn AutoRepeatMode<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRepeatMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, AutoRepeatMode::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerCommandBehaviorImpl: Sized {
    fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnablingRule(&self) -> ::windows::core::Result<MediaCommandEnablingRule>;
    fn SetEnablingRule(&self, value: MediaCommandEnablingRule) -> ::windows::core::Result<()>;
    fn IsEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerCommandBehavior {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerCommandBehavior";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerCommandBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerCommandBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerCommandBehaviorVtbl {
        unsafe extern "system" fn CommandManager<Impl: IMediaPlaybackCommandManagerCommandBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IMediaPlaybackCommandManagerCommandBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnablingRule<Impl: IMediaPlaybackCommandManagerCommandBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCommandEnablingRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnablingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnablingRule<Impl: IMediaPlaybackCommandManagerCommandBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaCommandEnablingRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnablingRule(value).into()
        }
        unsafe extern "system" fn IsEnabledChanged<Impl: IMediaPlaybackCommandManagerCommandBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsEnabledChanged<Impl: IMediaPlaybackCommandManagerCommandBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerCommandBehavior>,
            ::windows::core::GetTrustLevel,
            CommandManager::<Impl, IMPL_OFFSET>,
            IsEnabled::<Impl, IMPL_OFFSET>,
            EnablingRule::<Impl, IMPL_OFFSET>,
            SetEnablingRule::<Impl, IMPL_OFFSET>,
            IsEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveIsEnabledChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerCommandBehavior as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerFastForwardReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerFastForwardReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerFastForwardReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerFastForwardReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerFastForwardReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerFastForwardReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerNextReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerNextReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerNextReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerNextReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerNextReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerNextReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerNextReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerNextReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerNextReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerNextReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPauseReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPauseReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPauseReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPauseReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerPauseReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPauseReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPlayReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPlayReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPlayReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPlayReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerPlayReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPlayReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPositionReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPositionReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPositionReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPositionReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Position<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerPositionReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, Position::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPositionReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPreviousReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPreviousReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPreviousReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPreviousReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerPreviousReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPreviousReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerRateReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerRateReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerRateReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerRateReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerRateReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PlaybackRate<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerRateReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, PlaybackRate::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerRateReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerRewindReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerRewindReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerRewindReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerRewindReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerRewindReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerRewindReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerShuffleReceivedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsShuffleRequested(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerShuffleReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerShuffleReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerShuffleReceivedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn IsShuffleRequested<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShuffleRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackCommandManagerShuffleReceivedEventArgs>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, SetHandled::<Impl, IMPL_OFFSET>, IsShuffleRequested::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerShuffleReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItemImpl: Sized + IMediaPlaybackSourceImpl {
    fn AudioTracksChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioTracksChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoTracksChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoTracksChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TimedMetadataTracksChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimedMetadataTracksChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn AudioTracks(&self) -> ::windows::core::Result<MediaPlaybackAudioTrackList>;
    fn VideoTracks(&self) -> ::windows::core::Result<MediaPlaybackVideoTrackList>;
    fn TimedMetadataTracks(&self) -> ::windows::core::Result<MediaPlaybackTimedMetadataTrackList>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItem {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemVtbl {
        unsafe extern "system" fn AudioTracksChanged<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioTracksChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioTracksChanged<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioTracksChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoTracksChanged<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoTracksChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoTracksChanged<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoTracksChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimedMetadataTracksChanged<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimedMetadataTracksChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTimedMetadataTracksChanged<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTimedMetadataTracksChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioTracks<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoTracks<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimedMetadataTracks<Impl: IMediaPlaybackItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackItem>,
            ::windows::core::GetTrustLevel,
            AudioTracksChanged::<Impl, IMPL_OFFSET>,
            RemoveAudioTracksChanged::<Impl, IMPL_OFFSET>,
            VideoTracksChanged::<Impl, IMPL_OFFSET>,
            RemoveVideoTracksChanged::<Impl, IMPL_OFFSET>,
            TimedMetadataTracksChanged::<Impl, IMPL_OFFSET>,
            RemoveTimedMetadataTracksChanged::<Impl, IMPL_OFFSET>,
            Source::<Impl, IMPL_OFFSET>,
            AudioTracks::<Impl, IMPL_OFFSET>,
            VideoTracks::<Impl, IMPL_OFFSET>,
            TimedMetadataTracks::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItem2Impl: Sized + IMediaPlaybackItemImpl + IMediaPlaybackSourceImpl {
    fn BreakSchedule(&self) -> ::windows::core::Result<MediaBreakSchedule>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn DurationLimit(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn CanSkip(&self) -> ::windows::core::Result<bool>;
    fn SetCanSkip(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDisplayProperties(&self) -> ::windows::core::Result<MediaItemDisplayProperties>;
    fn ApplyDisplayProperties(&self, value: &::core::option::Option<MediaItemDisplayProperties>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItem2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItem2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItem2Vtbl {
        unsafe extern "system" fn BreakSchedule<Impl: IMediaPlaybackItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: IMediaPlaybackItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DurationLimit<Impl: IMediaPlaybackItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DurationLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSkip<Impl: IMediaPlaybackItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSkip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanSkip<Impl: IMediaPlaybackItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanSkip(value).into()
        }
        unsafe extern "system" fn GetDisplayProperties<Impl: IMediaPlaybackItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyDisplayProperties<Impl: IMediaPlaybackItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyDisplayProperties(&*(&value as *const <MediaItemDisplayProperties as ::windows::core::Abi>::Abi as *const <MediaItemDisplayProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackItem2>,
            ::windows::core::GetTrustLevel,
            BreakSchedule::<Impl, IMPL_OFFSET>,
            StartTime::<Impl, IMPL_OFFSET>,
            DurationLimit::<Impl, IMPL_OFFSET>,
            CanSkip::<Impl, IMPL_OFFSET>,
            SetCanSkip::<Impl, IMPL_OFFSET>,
            GetDisplayProperties::<Impl, IMPL_OFFSET>,
            ApplyDisplayProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItem3Impl: Sized + IMediaPlaybackItemImpl + IMediaPlaybackItem2Impl + IMediaPlaybackSourceImpl {
    fn IsDisabledInPlaybackList(&self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledInPlaybackList(&self, value: bool) -> ::windows::core::Result<()>;
    fn TotalDownloadProgress(&self) -> ::windows::core::Result<f64>;
    fn AutoLoadedDisplayProperties(&self) -> ::windows::core::Result<AutoLoadedDisplayPropertyKind>;
    fn SetAutoLoadedDisplayProperties(&self, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItem3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItem3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItem3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItem3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItem3Vtbl {
        unsafe extern "system" fn IsDisabledInPlaybackList<Impl: IMediaPlaybackItem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabledInPlaybackList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDisabledInPlaybackList<Impl: IMediaPlaybackItem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDisabledInPlaybackList(value).into()
        }
        unsafe extern "system" fn TotalDownloadProgress<Impl: IMediaPlaybackItem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalDownloadProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoLoadedDisplayProperties<Impl: IMediaPlaybackItem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoLoadedDisplayProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoLoadedDisplayProperties<Impl: IMediaPlaybackItem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoLoadedDisplayProperties(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackItem3>,
            ::windows::core::GetTrustLevel,
            IsDisabledInPlaybackList::<Impl, IMPL_OFFSET>,
            SetIsDisabledInPlaybackList::<Impl, IMPL_OFFSET>,
            TotalDownloadProgress::<Impl, IMPL_OFFSET>,
            AutoLoadedDisplayProperties::<Impl, IMPL_OFFSET>,
            SetAutoLoadedDisplayProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItem3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemErrorImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<MediaPlaybackItemErrorCode>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackItemError {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemError";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackItemErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemErrorVtbl {
        unsafe extern "system" fn ErrorCode<Impl: IMediaPlaybackItemErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IMediaPlaybackItemErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackItemError>, ::windows::core::GetTrustLevel, ErrorCode::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItemFactoryImpl: Sized {
    fn Create(&self, source: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItemFactory {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemFactory";
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItemFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMediaPlaybackItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&source as *const <super::Core::MediaSource as ::windows::core::Abi>::Abi as *const <super::Core::MediaSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackItemFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItemFactory2Impl: Sized + IMediaPlaybackItemFactoryImpl {
    fn CreateWithStartTime(&self, source: &::core::option::Option<super::Core::MediaSource>, starttime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem>;
    fn CreateWithStartTimeAndDurationLimit(&self, source: &::core::option::Option<super::Core::MediaSource>, starttime: &super::super::Foundation::TimeSpan, durationlimit: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItemFactory2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemFactory2";
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItemFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemFactory2Vtbl {
        unsafe extern "system" fn CreateWithStartTime<Impl: IMediaPlaybackItemFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, starttime: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithStartTime(&*(&source as *const <super::Core::MediaSource as ::windows::core::Abi>::Abi as *const <super::Core::MediaSource as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithStartTimeAndDurationLimit<Impl: IMediaPlaybackItemFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, starttime: super::super::Foundation::TimeSpan, durationlimit: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithStartTimeAndDurationLimit(
                &*(&source as *const <super::Core::MediaSource as ::windows::core::Abi>::Abi as *const <super::Core::MediaSource as ::windows::core::DefaultType>::DefaultType),
                &*(&starttime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&durationlimit as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackItemFactory2>, ::windows::core::GetTrustLevel, CreateWithStartTime::<Impl, IMPL_OFFSET>, CreateWithStartTimeAndDurationLimit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemFailedEventArgsImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn Error(&self) -> ::windows::core::Result<MediaPlaybackItemError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackItemFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackItemFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemFailedEventArgsVtbl {
        unsafe extern "system" fn Item<Impl: IMediaPlaybackItemFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IMediaPlaybackItemFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackItemFailedEventArgs>, ::windows::core::GetTrustLevel, Item::<Impl, IMPL_OFFSET>, Error::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemOpenedEventArgsImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackItemOpenedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemOpenedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackItemOpenedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemOpenedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemOpenedEventArgsVtbl {
        unsafe extern "system" fn Item<Impl: IMediaPlaybackItemOpenedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackItemOpenedEventArgs>, ::windows::core::GetTrustLevel, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemOpenedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItemStaticsImpl: Sized {
    fn FindFromMediaSource(&self, source: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItemStatics {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemStatics";
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemStaticsVtbl {
        unsafe extern "system" fn FindFromMediaSource<Impl: IMediaPlaybackItemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFromMediaSource(&*(&source as *const <super::Core::MediaSource as ::windows::core::Abi>::Abi as *const <super::Core::MediaSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackItemStatics>, ::windows::core::GetTrustLevel, FindFromMediaSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackListImpl: Sized + IMediaPlaybackSourceImpl {
    fn ItemFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentItemChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentItemChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemOpened(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<MediaPlaybackItem>>;
    fn AutoRepeatEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoRepeatEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShuffleEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetShuffleEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn CurrentItemIndex(&self) -> ::windows::core::Result<u32>;
    fn MoveNext(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn MovePrevious(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn MoveTo(&self, itemindex: u32) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackList {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackList";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackListVtbl {
        unsafe extern "system" fn ItemFailed<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemFailed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemFailed<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentItemChanged<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentItemChanged<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentItemChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemOpened<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemOpened(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemOpened<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Items<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoRepeatEnabled<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRepeatEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRepeatEnabled<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoRepeatEnabled(value).into()
        }
        unsafe extern "system" fn ShuffleEnabled<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShuffleEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShuffleEnabled<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShuffleEnabled(value).into()
        }
        unsafe extern "system" fn CurrentItem<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemIndex<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePrevious<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePrevious() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveTo<Impl: IMediaPlaybackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveTo(itemindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackList>,
            ::windows::core::GetTrustLevel,
            ItemFailed::<Impl, IMPL_OFFSET>,
            RemoveItemFailed::<Impl, IMPL_OFFSET>,
            CurrentItemChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentItemChanged::<Impl, IMPL_OFFSET>,
            ItemOpened::<Impl, IMPL_OFFSET>,
            RemoveItemOpened::<Impl, IMPL_OFFSET>,
            Items::<Impl, IMPL_OFFSET>,
            AutoRepeatEnabled::<Impl, IMPL_OFFSET>,
            SetAutoRepeatEnabled::<Impl, IMPL_OFFSET>,
            ShuffleEnabled::<Impl, IMPL_OFFSET>,
            SetShuffleEnabled::<Impl, IMPL_OFFSET>,
            CurrentItem::<Impl, IMPL_OFFSET>,
            CurrentItemIndex::<Impl, IMPL_OFFSET>,
            MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious::<Impl, IMPL_OFFSET>,
            MoveTo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackList2Impl: Sized + IMediaPlaybackListImpl + IMediaPlaybackSourceImpl {
    fn MaxPrefetchTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetMaxPrefetchTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn StartingItem(&self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn SetStartingItem(&self, value: &::core::option::Option<MediaPlaybackItem>) -> ::windows::core::Result<()>;
    fn ShuffledItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaPlaybackItem>>;
    fn SetShuffledItems(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<MediaPlaybackItem>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackList2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackList2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackList2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackList2Vtbl {
        unsafe extern "system" fn MaxPrefetchTime<Impl: IMediaPlaybackList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPrefetchTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPrefetchTime<Impl: IMediaPlaybackList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPrefetchTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartingItem<Impl: IMediaPlaybackList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartingItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingItem<Impl: IMediaPlaybackList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartingItem(&*(&value as *const <MediaPlaybackItem as ::windows::core::Abi>::Abi as *const <MediaPlaybackItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShuffledItems<Impl: IMediaPlaybackList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShuffledItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShuffledItems<Impl: IMediaPlaybackList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShuffledItems(&*(&value as *const <super::super::Foundation::Collections::IIterable<MediaPlaybackItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<MediaPlaybackItem> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackList2>,
            ::windows::core::GetTrustLevel,
            MaxPrefetchTime::<Impl, IMPL_OFFSET>,
            SetMaxPrefetchTime::<Impl, IMPL_OFFSET>,
            StartingItem::<Impl, IMPL_OFFSET>,
            SetStartingItem::<Impl, IMPL_OFFSET>,
            ShuffledItems::<Impl, IMPL_OFFSET>,
            SetShuffledItems::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackList3Impl: Sized + IMediaPlaybackListImpl + IMediaPlaybackList2Impl + IMediaPlaybackSourceImpl {
    fn MaxPlayedItemsToKeepOpen(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetMaxPlayedItemsToKeepOpen(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackList3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackList3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackList3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackList3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackList3Vtbl {
        unsafe extern "system" fn MaxPlayedItemsToKeepOpen<Impl: IMediaPlaybackList3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPlayedItemsToKeepOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPlayedItemsToKeepOpen<Impl: IMediaPlaybackList3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPlayedItemsToKeepOpen(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackList3>, ::windows::core::GetTrustLevel, MaxPlayedItemsToKeepOpen::<Impl, IMPL_OFFSET>, SetMaxPlayedItemsToKeepOpen::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackList3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSessionImpl: Sized {
    fn PlaybackStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekCompleted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingStarted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingEnded(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingProgressChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingProgressChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadProgressChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadProgressChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NaturalDurationChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNaturalDurationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NaturalVideoSizeChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNaturalVideoSizeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer>;
    fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PlaybackState(&self) -> ::windows::core::Result<MediaPlaybackState>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn CanPause(&self) -> ::windows::core::Result<bool>;
    fn IsProtected(&self) -> ::windows::core::Result<bool>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn BufferingProgress(&self) -> ::windows::core::Result<f64>;
    fn DownloadProgress(&self) -> ::windows::core::Result<f64>;
    fn NaturalVideoHeight(&self) -> ::windows::core::Result<u32>;
    fn NaturalVideoWidth(&self) -> ::windows::core::Result<u32>;
    fn NormalizedSourceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetNormalizedSourceRect(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn StereoscopicVideoPackingMode(&self) -> ::windows::core::Result<super::MediaProperties::StereoscopicVideoPackingMode>;
    fn SetStereoscopicVideoPackingMode(&self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSession {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSession";
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaPlaybackSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSessionVtbl {
        unsafe extern "system" fn PlaybackStateChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackStateChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackRateChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackRateChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackRateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SeekCompleted<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekCompleted(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSeekCompleted<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSeekCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingStarted<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingStarted(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBufferingStarted<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingEnded<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingEnded(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBufferingEnded<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingProgressChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingProgressChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBufferingProgressChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingProgressChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadProgressChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadProgressChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadProgressChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadProgressChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalDurationChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalDurationChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNaturalDurationChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNaturalDurationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalVideoSizeChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalVideoSizeChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNaturalVideoSizeChanged<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNaturalVideoSizeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaPlayer<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlayer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalDuration<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackState<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSeek<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPause<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsProtected<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaybackRate<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackRate<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn BufferingProgress<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadProgress<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalVideoHeight<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalVideoHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalVideoWidth<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalVideoWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizedSourceRect<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizedSourceRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalizedSourceRect<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedSourceRect(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StereoscopicVideoPackingMode<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoscopicVideoPackingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStereoscopicVideoPackingMode<Impl: IMediaPlaybackSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStereoscopicVideoPackingMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackSession>,
            ::windows::core::GetTrustLevel,
            PlaybackStateChanged::<Impl, IMPL_OFFSET>,
            RemovePlaybackStateChanged::<Impl, IMPL_OFFSET>,
            PlaybackRateChanged::<Impl, IMPL_OFFSET>,
            RemovePlaybackRateChanged::<Impl, IMPL_OFFSET>,
            SeekCompleted::<Impl, IMPL_OFFSET>,
            RemoveSeekCompleted::<Impl, IMPL_OFFSET>,
            BufferingStarted::<Impl, IMPL_OFFSET>,
            RemoveBufferingStarted::<Impl, IMPL_OFFSET>,
            BufferingEnded::<Impl, IMPL_OFFSET>,
            RemoveBufferingEnded::<Impl, IMPL_OFFSET>,
            BufferingProgressChanged::<Impl, IMPL_OFFSET>,
            RemoveBufferingProgressChanged::<Impl, IMPL_OFFSET>,
            DownloadProgressChanged::<Impl, IMPL_OFFSET>,
            RemoveDownloadProgressChanged::<Impl, IMPL_OFFSET>,
            NaturalDurationChanged::<Impl, IMPL_OFFSET>,
            RemoveNaturalDurationChanged::<Impl, IMPL_OFFSET>,
            PositionChanged::<Impl, IMPL_OFFSET>,
            RemovePositionChanged::<Impl, IMPL_OFFSET>,
            NaturalVideoSizeChanged::<Impl, IMPL_OFFSET>,
            RemoveNaturalVideoSizeChanged::<Impl, IMPL_OFFSET>,
            MediaPlayer::<Impl, IMPL_OFFSET>,
            NaturalDuration::<Impl, IMPL_OFFSET>,
            Position::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            PlaybackState::<Impl, IMPL_OFFSET>,
            CanSeek::<Impl, IMPL_OFFSET>,
            CanPause::<Impl, IMPL_OFFSET>,
            IsProtected::<Impl, IMPL_OFFSET>,
            PlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate::<Impl, IMPL_OFFSET>,
            BufferingProgress::<Impl, IMPL_OFFSET>,
            DownloadProgress::<Impl, IMPL_OFFSET>,
            NaturalVideoHeight::<Impl, IMPL_OFFSET>,
            NaturalVideoWidth::<Impl, IMPL_OFFSET>,
            NormalizedSourceRect::<Impl, IMPL_OFFSET>,
            SetNormalizedSourceRect::<Impl, IMPL_OFFSET>,
            StereoscopicVideoPackingMode::<Impl, IMPL_OFFSET>,
            SetStereoscopicVideoPackingMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSession2Impl: Sized {
    fn BufferedRangesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferedRangesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayedRangesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayedRangesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekableRangesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekableRangesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SupportedPlaybackRatesChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSupportedPlaybackRatesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SphericalVideoProjection(&self) -> ::windows::core::Result<MediaPlaybackSphericalVideoProjection>;
    fn IsMirroring(&self) -> ::windows::core::Result<bool>;
    fn SetIsMirroring(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetBufferedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn GetPlayedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn GetSeekableRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn IsSupportedPlaybackRateRange(&self, rate1: f64, rate2: f64) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSession2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSession2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSession2Vtbl {
        unsafe extern "system" fn BufferedRangesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferedRangesChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBufferedRangesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferedRangesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayedRangesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayedRangesChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlayedRangesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayedRangesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SeekableRangesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekableRangesChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSeekableRangesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSeekableRangesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedPlaybackRatesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPlaybackRatesChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSupportedPlaybackRatesChanged<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSupportedPlaybackRatesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SphericalVideoProjection<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SphericalVideoProjection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMirroring<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMirroring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMirroring<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMirroring(value).into()
        }
        unsafe extern "system" fn GetBufferedRanges<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlayedRanges<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSeekableRanges<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSeekableRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedPlaybackRateRange<Impl: IMediaPlaybackSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate1: f64, rate2: f64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedPlaybackRateRange(rate1, rate2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackSession2>,
            ::windows::core::GetTrustLevel,
            BufferedRangesChanged::<Impl, IMPL_OFFSET>,
            RemoveBufferedRangesChanged::<Impl, IMPL_OFFSET>,
            PlayedRangesChanged::<Impl, IMPL_OFFSET>,
            RemovePlayedRangesChanged::<Impl, IMPL_OFFSET>,
            SeekableRangesChanged::<Impl, IMPL_OFFSET>,
            RemoveSeekableRangesChanged::<Impl, IMPL_OFFSET>,
            SupportedPlaybackRatesChanged::<Impl, IMPL_OFFSET>,
            RemoveSupportedPlaybackRatesChanged::<Impl, IMPL_OFFSET>,
            SphericalVideoProjection::<Impl, IMPL_OFFSET>,
            IsMirroring::<Impl, IMPL_OFFSET>,
            SetIsMirroring::<Impl, IMPL_OFFSET>,
            GetBufferedRanges::<Impl, IMPL_OFFSET>,
            GetPlayedRanges::<Impl, IMPL_OFFSET>,
            GetSeekableRanges::<Impl, IMPL_OFFSET>,
            IsSupportedPlaybackRateRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSession3Impl: Sized {
    fn PlaybackRotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation>;
    fn SetPlaybackRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()>;
    fn GetOutputDegradationPolicyState(&self) -> ::windows::core::Result<MediaPlaybackSessionOutputDegradationPolicyState>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSession3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSession3";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaPlaybackSession3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSession3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSession3Vtbl {
        unsafe extern "system" fn PlaybackRotation<Impl: IMediaPlaybackSession3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackRotation<Impl: IMediaPlaybackSession3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRotation(value).into()
        }
        unsafe extern "system" fn GetOutputDegradationPolicyState<Impl: IMediaPlaybackSession3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputDegradationPolicyState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackSession3>, ::windows::core::GetTrustLevel, PlaybackRotation::<Impl, IMPL_OFFSET>, SetPlaybackRotation::<Impl, IMPL_OFFSET>, GetOutputDegradationPolicyState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSession3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSessionBufferingStartedEventArgsImpl: Sized {
    fn IsPlaybackInterruption(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackSessionBufferingStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSessionBufferingStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackSessionBufferingStartedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSessionBufferingStartedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSessionBufferingStartedEventArgsVtbl {
        unsafe extern "system" fn IsPlaybackInterruption<Impl: IMediaPlaybackSessionBufferingStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPlaybackInterruption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackSessionBufferingStartedEventArgs>, ::windows::core::GetTrustLevel, IsPlaybackInterruption::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSessionBufferingStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSessionOutputDegradationPolicyStateImpl: Sized {
    fn VideoConstrictionReason(&self) -> ::windows::core::Result<MediaPlaybackSessionVideoConstrictionReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackSessionOutputDegradationPolicyState {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSessionOutputDegradationPolicyState";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackSessionOutputDegradationPolicyStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSessionOutputDegradationPolicyStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSessionOutputDegradationPolicyStateVtbl {
        unsafe extern "system" fn VideoConstrictionReason<Impl: IMediaPlaybackSessionOutputDegradationPolicyStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackSessionVideoConstrictionReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoConstrictionReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackSessionOutputDegradationPolicyState>, ::windows::core::GetTrustLevel, VideoConstrictionReason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSessionOutputDegradationPolicyState as ::windows::core::Interface>::IID
    }
}
pub trait IMediaPlaybackSourceImpl: Sized {}
impl ::windows::core::RuntimeName for IMediaPlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSource";
}
impl IMediaPlaybackSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSourceVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackSource>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSphericalVideoProjectionImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat>;
    fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()>;
    fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetViewOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn ProjectionMode(&self) -> ::windows::core::Result<SphericalVideoProjectionMode>;
    fn SetProjectionMode(&self, value: SphericalVideoProjectionMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSphericalVideoProjection {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSphericalVideoProjection";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaPlaybackSphericalVideoProjectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSphericalVideoProjectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSphericalVideoProjectionVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn FrameFormat<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameFormat<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameFormat(value).into()
        }
        unsafe extern "system" fn HorizontalFieldOfViewInDegrees<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalFieldOfViewInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalFieldOfViewInDegrees<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalFieldOfViewInDegrees(value).into()
        }
        unsafe extern "system" fn ViewOrientation<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewOrientation<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProjectionMode<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProjectionMode<Impl: IMediaPlaybackSphericalVideoProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProjectionMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlaybackSphericalVideoProjection>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled::<Impl, IMPL_OFFSET>,
            FrameFormat::<Impl, IMPL_OFFSET>,
            SetFrameFormat::<Impl, IMPL_OFFSET>,
            HorizontalFieldOfViewInDegrees::<Impl, IMPL_OFFSET>,
            SetHorizontalFieldOfViewInDegrees::<Impl, IMPL_OFFSET>,
            ViewOrientation::<Impl, IMPL_OFFSET>,
            SetViewOrientation::<Impl, IMPL_OFFSET>,
            ProjectionMode::<Impl, IMPL_OFFSET>,
            SetProjectionMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSphericalVideoProjection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackTimedMetadataTrackListImpl: Sized {
    fn PresentationModeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePresentationModeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetPresentationMode(&self, index: u32) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
    fn SetPresentationMode(&self, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackTimedMetadataTrackList {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackTimedMetadataTrackList";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackTimedMetadataTrackListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackTimedMetadataTrackListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackTimedMetadataTrackListVtbl {
        unsafe extern "system" fn PresentationModeChanged<Impl: IMediaPlaybackTimedMetadataTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationModeChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePresentationModeChanged<Impl: IMediaPlaybackTimedMetadataTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePresentationModeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPresentationMode<Impl: IMediaPlaybackTimedMetadataTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresentationMode(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentationMode<Impl: IMediaPlaybackTimedMetadataTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentationMode(index, value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlaybackTimedMetadataTrackList>, ::windows::core::GetTrustLevel, PresentationModeChanged::<Impl, IMPL_OFFSET>, RemovePresentationModeChanged::<Impl, IMPL_OFFSET>, GetPresentationMode::<Impl, IMPL_OFFSET>, SetPresentationMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackTimedMetadataTrackList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlayerImpl: Sized {
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BufferingProgress(&self) -> ::windows::core::Result<f64>;
    fn CurrentState(&self) -> ::windows::core::Result<MediaPlayerState>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn CanPause(&self) -> ::windows::core::Result<bool>;
    fn IsLoopingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsProtected(&self) -> ::windows::core::Result<bool>;
    fn IsMuted(&self) -> ::windows::core::Result<bool>;
    fn SetIsMuted(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackMediaMarkers(&self) -> ::windows::core::Result<PlaybackMediaMarkerSequence>;
    fn MediaOpened(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaEnded(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaFailed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackMediaMarkerReached(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackMediaMarkerReached(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaPlayerRateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaPlayerRateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VolumeChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVolumeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekCompleted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingStarted(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingEnded(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn SetUriSource(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerVtbl {
        unsafe extern "system" fn AutoPlay<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoPlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoPlay<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoPlay(value).into()
        }
        unsafe extern "system" fn NaturalDuration<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NaturalDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingProgress<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanSeek<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPause<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLoopingEnabled<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLoopingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLoopingEnabled<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLoopingEnabled(value).into()
        }
        unsafe extern "system" fn IsProtected<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMuted<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsMuted<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMuted(value).into()
        }
        unsafe extern "system" fn PlaybackRate<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackRate<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn Volume<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVolume<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        unsafe extern "system" fn PlaybackMediaMarkers<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackMediaMarkers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaOpened<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaOpened(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMediaOpened<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaEnded<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaEnded(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMediaEnded<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaFailed<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaFailed(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMediaFailed<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentStateChanged<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentStateChanged<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackMediaMarkerReached<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackMediaMarkerReached(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackMediaMarkerReached<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackMediaMarkerReached(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaPlayerRateChanged<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlayerRateChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMediaPlayerRateChanged<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaPlayerRateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VolumeChanged<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVolumeChanged<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVolumeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SeekCompleted<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekCompleted(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSeekCompleted<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSeekCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingStarted<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingStarted(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBufferingStarted<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingEnded<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingEnded(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBufferingEnded<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Play<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Pause<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn SetUriSource<Impl: IMediaPlayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriSource(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlayer>,
            ::windows::core::GetTrustLevel,
            AutoPlay::<Impl, IMPL_OFFSET>,
            SetAutoPlay::<Impl, IMPL_OFFSET>,
            NaturalDuration::<Impl, IMPL_OFFSET>,
            Position::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            BufferingProgress::<Impl, IMPL_OFFSET>,
            CurrentState::<Impl, IMPL_OFFSET>,
            CanSeek::<Impl, IMPL_OFFSET>,
            CanPause::<Impl, IMPL_OFFSET>,
            IsLoopingEnabled::<Impl, IMPL_OFFSET>,
            SetIsLoopingEnabled::<Impl, IMPL_OFFSET>,
            IsProtected::<Impl, IMPL_OFFSET>,
            IsMuted::<Impl, IMPL_OFFSET>,
            SetIsMuted::<Impl, IMPL_OFFSET>,
            PlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate::<Impl, IMPL_OFFSET>,
            Volume::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
            PlaybackMediaMarkers::<Impl, IMPL_OFFSET>,
            MediaOpened::<Impl, IMPL_OFFSET>,
            RemoveMediaOpened::<Impl, IMPL_OFFSET>,
            MediaEnded::<Impl, IMPL_OFFSET>,
            RemoveMediaEnded::<Impl, IMPL_OFFSET>,
            MediaFailed::<Impl, IMPL_OFFSET>,
            RemoveMediaFailed::<Impl, IMPL_OFFSET>,
            CurrentStateChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentStateChanged::<Impl, IMPL_OFFSET>,
            PlaybackMediaMarkerReached::<Impl, IMPL_OFFSET>,
            RemovePlaybackMediaMarkerReached::<Impl, IMPL_OFFSET>,
            MediaPlayerRateChanged::<Impl, IMPL_OFFSET>,
            RemoveMediaPlayerRateChanged::<Impl, IMPL_OFFSET>,
            VolumeChanged::<Impl, IMPL_OFFSET>,
            RemoveVolumeChanged::<Impl, IMPL_OFFSET>,
            SeekCompleted::<Impl, IMPL_OFFSET>,
            RemoveSeekCompleted::<Impl, IMPL_OFFSET>,
            BufferingStarted::<Impl, IMPL_OFFSET>,
            RemoveBufferingStarted::<Impl, IMPL_OFFSET>,
            BufferingEnded::<Impl, IMPL_OFFSET>,
            RemoveBufferingEnded::<Impl, IMPL_OFFSET>,
            Play::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            SetUriSource::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer2Impl: Sized {
    fn SystemMediaTransportControls(&self) -> ::windows::core::Result<super::SystemMediaTransportControls>;
    fn AudioCategory(&self) -> ::windows::core::Result<MediaPlayerAudioCategory>;
    fn SetAudioCategory(&self, value: MediaPlayerAudioCategory) -> ::windows::core::Result<()>;
    fn AudioDeviceType(&self) -> ::windows::core::Result<MediaPlayerAudioDeviceType>;
    fn SetAudioDeviceType(&self, value: MediaPlayerAudioDeviceType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayer2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer2Vtbl {
        unsafe extern "system" fn SystemMediaTransportControls<Impl: IMediaPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemMediaTransportControls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioCategory<Impl: IMediaPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioCategory<Impl: IMediaPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioCategory(value).into()
        }
        unsafe extern "system" fn AudioDeviceType<Impl: IMediaPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioDeviceType<Impl: IMediaPlayer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioDeviceType(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayer2>, ::windows::core::GetTrustLevel, SystemMediaTransportControls::<Impl, IMPL_OFFSET>, AudioCategory::<Impl, IMPL_OFFSET>, SetAudioCategory::<Impl, IMPL_OFFSET>, AudioDeviceType::<Impl, IMPL_OFFSET>, SetAudioDeviceType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Casting", feature = "implement_exclusive"))]
pub trait IMediaPlayer3Impl: Sized {
    fn IsMutedChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsMutedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioBalance(&self) -> ::windows::core::Result<f64>;
    fn SetAudioBalance(&self, value: f64) -> ::windows::core::Result<()>;
    fn RealTimePlayback(&self) -> ::windows::core::Result<bool>;
    fn SetRealTimePlayback(&self, value: bool) -> ::windows::core::Result<()>;
    fn StereoscopicVideoRenderMode(&self) -> ::windows::core::Result<StereoscopicVideoRenderMode>;
    fn SetStereoscopicVideoRenderMode(&self, value: StereoscopicVideoRenderMode) -> ::windows::core::Result<()>;
    fn BreakManager(&self) -> ::windows::core::Result<MediaBreakManager>;
    fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager>;
    fn AudioDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
    fn SetAudioDevice(&self, value: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<()>;
    fn TimelineController(&self) -> ::windows::core::Result<super::MediaTimelineController>;
    fn SetTimelineController(&self, value: &::core::option::Option<super::MediaTimelineController>) -> ::windows::core::Result<()>;
    fn TimelineControllerPositionOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimelineControllerPositionOffset(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession>;
    fn StepForwardOneFrame(&self) -> ::windows::core::Result<()>;
    fn StepBackwardOneFrame(&self) -> ::windows::core::Result<()>;
    fn GetAsCastingSource(&self) -> ::windows::core::Result<super::Casting::CastingSource>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Casting", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer3";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Casting", feature = "implement_exclusive"))]
impl IMediaPlayer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer3Vtbl {
        unsafe extern "system" fn IsMutedChanged<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMutedChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsMutedChanged<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsMutedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceChanged<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceChanged<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioBalance<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioBalance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioBalance<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioBalance(value).into()
        }
        unsafe extern "system" fn RealTimePlayback<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RealTimePlayback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealTimePlayback<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRealTimePlayback(value).into()
        }
        unsafe extern "system" fn StereoscopicVideoRenderMode<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoscopicVideoRenderMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStereoscopicVideoRenderMode<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StereoscopicVideoRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStereoscopicVideoRenderMode(value).into()
        }
        unsafe extern "system" fn BreakManager<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandManager<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioDevice<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioDevice<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioDevice(&*(&value as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimelineController<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimelineController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimelineController<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimelineController(&*(&value as *const <super::MediaTimelineController as ::windows::core::Abi>::Abi as *const <super::MediaTimelineController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimelineControllerPositionOffset<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimelineControllerPositionOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimelineControllerPositionOffset<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimelineControllerPositionOffset(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackSession<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StepForwardOneFrame<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StepForwardOneFrame().into()
        }
        unsafe extern "system" fn StepBackwardOneFrame<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StepBackwardOneFrame().into()
        }
        unsafe extern "system" fn GetAsCastingSource<Impl: IMediaPlayer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsCastingSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlayer3>,
            ::windows::core::GetTrustLevel,
            IsMutedChanged::<Impl, IMPL_OFFSET>,
            RemoveIsMutedChanged::<Impl, IMPL_OFFSET>,
            SourceChanged::<Impl, IMPL_OFFSET>,
            RemoveSourceChanged::<Impl, IMPL_OFFSET>,
            AudioBalance::<Impl, IMPL_OFFSET>,
            SetAudioBalance::<Impl, IMPL_OFFSET>,
            RealTimePlayback::<Impl, IMPL_OFFSET>,
            SetRealTimePlayback::<Impl, IMPL_OFFSET>,
            StereoscopicVideoRenderMode::<Impl, IMPL_OFFSET>,
            SetStereoscopicVideoRenderMode::<Impl, IMPL_OFFSET>,
            BreakManager::<Impl, IMPL_OFFSET>,
            CommandManager::<Impl, IMPL_OFFSET>,
            AudioDevice::<Impl, IMPL_OFFSET>,
            SetAudioDevice::<Impl, IMPL_OFFSET>,
            TimelineController::<Impl, IMPL_OFFSET>,
            SetTimelineController::<Impl, IMPL_OFFSET>,
            TimelineControllerPositionOffset::<Impl, IMPL_OFFSET>,
            SetTimelineControllerPositionOffset::<Impl, IMPL_OFFSET>,
            PlaybackSession::<Impl, IMPL_OFFSET>,
            StepForwardOneFrame::<Impl, IMPL_OFFSET>,
            StepBackwardOneFrame::<Impl, IMPL_OFFSET>,
            GetAsCastingSource::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IMediaPlayer4Impl: Sized {
    fn SetSurfaceSize(&self, size: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn GetSurface(&self, compositor: &::core::option::Option<super::super::UI::Composition::Compositor>) -> ::windows::core::Result<MediaPlayerSurface>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer4 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer4";
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IMediaPlayer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer4Vtbl {
        unsafe extern "system" fn SetSurfaceSize<Impl: IMediaPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurfaceSize(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSurface<Impl: IMediaPlayer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSurface(&*(&compositor as *const <super::super::UI::Composition::Compositor as ::windows::core::Abi>::Abi as *const <super::super::UI::Composition::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayer4>, ::windows::core::GetTrustLevel, SetSurfaceSize::<Impl, IMPL_OFFSET>, GetSurface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaPlayer5Impl: Sized {
    fn VideoFrameAvailable(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameAvailable(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsVideoFrameServerEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsVideoFrameServerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CopyFrameToVideoSurface(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
    fn CopyFrameToVideoSurfaceWithTargetRectangle(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, targetrectangle: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn CopyFrameToStereoscopicVideoSurfaces(&self, destinationlefteye: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, destinationrighteye: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer5 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer5";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaPlayer5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer5Vtbl {
        unsafe extern "system" fn VideoFrameAvailable<Impl: IMediaPlayer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFrameAvailable(&*(&value as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVideoFrameAvailable<Impl: IMediaPlayer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoFrameAvailable(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsVideoFrameServerEnabled<Impl: IMediaPlayer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVideoFrameServerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsVideoFrameServerEnabled<Impl: IMediaPlayer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVideoFrameServerEnabled(value).into()
        }
        unsafe extern "system" fn CopyFrameToVideoSurface<Impl: IMediaPlayer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyFrameToVideoSurface(&*(&destination as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyFrameToVideoSurfaceWithTargetRectangle<Impl: IMediaPlayer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, targetrectangle: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyFrameToVideoSurfaceWithTargetRectangle(&*(&destination as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), &*(&targetrectangle as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyFrameToStereoscopicVideoSurfaces<Impl: IMediaPlayer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationlefteye: ::windows::core::RawPtr, destinationrighteye: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopyFrameToStereoscopicVideoSurfaces(
                    &*(&destinationlefteye as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType),
                    &*(&destinationrighteye as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaPlayer5>,
            ::windows::core::GetTrustLevel,
            VideoFrameAvailable::<Impl, IMPL_OFFSET>,
            RemoveVideoFrameAvailable::<Impl, IMPL_OFFSET>,
            IsVideoFrameServerEnabled::<Impl, IMPL_OFFSET>,
            SetIsVideoFrameServerEnabled::<Impl, IMPL_OFFSET>,
            CopyFrameToVideoSurface::<Impl, IMPL_OFFSET>,
            CopyFrameToVideoSurfaceWithTargetRectangle::<Impl, IMPL_OFFSET>,
            CopyFrameToStereoscopicVideoSurfaces::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaPlayer6Impl: Sized {
    fn SubtitleFrameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubtitleFrameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RenderSubtitlesToSurface(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<bool>;
    fn RenderSubtitlesToSurfaceWithTargetRectangle(&self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, targetrectangle: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer6 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer6";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaPlayer6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer6Vtbl {
        unsafe extern "system" fn SubtitleFrameChanged<Impl: IMediaPlayer6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubtitleFrameChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubtitleFrameChanged<Impl: IMediaPlayer6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubtitleFrameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RenderSubtitlesToSurface<Impl: IMediaPlayer6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderSubtitlesToSurface(&*(&destination as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderSubtitlesToSurfaceWithTargetRectangle<Impl: IMediaPlayer6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, targetrectangle: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderSubtitlesToSurfaceWithTargetRectangle(&*(&destination as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), &*(&targetrectangle as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayer6>, ::windows::core::GetTrustLevel, SubtitleFrameChanged::<Impl, IMPL_OFFSET>, RemoveSubtitleFrameChanged::<Impl, IMPL_OFFSET>, RenderSubtitlesToSurface::<Impl, IMPL_OFFSET>, RenderSubtitlesToSurfaceWithTargetRectangle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Audio", feature = "implement_exclusive"))]
pub trait IMediaPlayer7Impl: Sized {
    fn AudioStateMonitor(&self) -> ::windows::core::Result<super::Audio::AudioStateMonitor>;
}
#[cfg(all(feature = "Media_Audio", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer7 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer7";
}
#[cfg(all(feature = "Media_Audio", feature = "implement_exclusive"))]
impl IMediaPlayer7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer7Vtbl {
        unsafe extern "system" fn AudioStateMonitor<Impl: IMediaPlayer7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioStateMonitor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayer7>, ::windows::core::GetTrustLevel, AudioStateMonitor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlayerDataReceivedEventArgsImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerDataReceivedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlayerDataReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerDataReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerDataReceivedEventArgsVtbl {
        unsafe extern "system" fn Data<Impl: IMediaPlayerDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerDataReceivedEventArgs>, ::windows::core::GetTrustLevel, Data::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlayerEffectsImpl: Sized {
    fn AddAudioEffect(&self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RemoveAllEffects(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerEffects {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerEffects";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlayerEffectsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerEffectsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerEffectsVtbl {
        unsafe extern "system" fn AddAudioEffect<Impl: IMediaPlayerEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectoptional: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAudioEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectoptional, &*(&configuration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAllEffects<Impl: IMediaPlayerEffectsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllEffects().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerEffects>, ::windows::core::GetTrustLevel, AddAudioEffect::<Impl, IMPL_OFFSET>, RemoveAllEffects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlayerEffects2Impl: Sized {
    fn AddVideoEffect(&self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, effectconfiguration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerEffects2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerEffects2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlayerEffects2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerEffects2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerEffects2Vtbl {
        unsafe extern "system" fn AddVideoEffect<Impl: IMediaPlayerEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectoptional: bool, effectconfiguration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddVideoEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectoptional, &*(&effectconfiguration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerEffects2>, ::windows::core::GetTrustLevel, AddVideoEffect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerEffects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerFailedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<MediaPlayerError>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerFailedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IMediaPlayerFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedErrorCode<Impl: IMediaPlayerFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorMessage<Impl: IMediaPlayerFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerFailedEventArgs>, ::windows::core::GetTrustLevel, Error::<Impl, IMPL_OFFSET>, ExtendedErrorCode::<Impl, IMPL_OFFSET>, ErrorMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerRateChangedEventArgsImpl: Sized {
    fn NewRate(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerRateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerRateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerRateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerRateChangedEventArgsVtbl {
        unsafe extern "system" fn NewRate<Impl: IMediaPlayerRateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerRateChangedEventArgs>, ::windows::core::GetTrustLevel, NewRate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerRateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "Media_Protection", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaPlayerSourceImpl: Sized {
    fn ProtectionManager(&self) -> ::windows::core::Result<super::Protection::MediaProtectionManager>;
    fn SetProtectionManager(&self, value: &::core::option::Option<super::Protection::MediaProtectionManager>) -> ::windows::core::Result<()>;
    fn SetFileSource(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
    fn SetStreamSource(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetMediaSource(&self, source: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_Core", feature = "Media_Protection", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSource";
}
#[cfg(all(feature = "Media_Core", feature = "Media_Protection", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaPlayerSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerSourceVtbl {
        unsafe extern "system" fn ProtectionManager<Impl: IMediaPlayerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectionManager<Impl: IMediaPlayerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectionManager(&*(&value as *const <super::Protection::MediaProtectionManager as ::windows::core::Abi>::Abi as *const <super::Protection::MediaProtectionManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFileSource<Impl: IMediaPlayerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileSource(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStreamSource<Impl: IMediaPlayerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamSource(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMediaSource<Impl: IMediaPlayerSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaSource(&*(&source as *const <super::Core::IMediaSource as ::windows::core::Abi>::Abi as *const <super::Core::IMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerSource>, ::windows::core::GetTrustLevel, ProtectionManager::<Impl, IMPL_OFFSET>, SetProtectionManager::<Impl, IMPL_OFFSET>, SetFileSource::<Impl, IMPL_OFFSET>, SetStreamSource::<Impl, IMPL_OFFSET>, SetMediaSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerSource2Impl: Sized {
    fn Source(&self) -> ::windows::core::Result<IMediaPlaybackSource>;
    fn SetSource(&self, value: &::core::option::Option<IMediaPlaybackSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerSource2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerSource2Vtbl {
        unsafe extern "system" fn Source<Impl: IMediaPlayerSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IMediaPlayerSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <IMediaPlaybackSource as ::windows::core::Abi>::Abi as *const <IMediaPlaybackSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerSource2>, ::windows::core::GetTrustLevel, Source::<Impl, IMPL_OFFSET>, SetSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IMediaPlayerSurfaceImpl: Sized {
    fn CompositionSurface(&self) -> ::windows::core::Result<super::super::UI::Composition::ICompositionSurface>;
    fn Compositor(&self) -> ::windows::core::Result<super::super::UI::Composition::Compositor>;
    fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerSurface {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSurface";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IMediaPlayerSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerSurfaceVtbl {
        unsafe extern "system" fn CompositionSurface<Impl: IMediaPlayerSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositionSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compositor<Impl: IMediaPlayerSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compositor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPlayer<Impl: IMediaPlayerSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlayer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerSurface>, ::windows::core::GetTrustLevel, CompositionSurface::<Impl, IMPL_OFFSET>, Compositor::<Impl, IMPL_OFFSET>, MediaPlayer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPlaybackMediaMarkerImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackMediaMarker {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarker";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPlaybackMediaMarkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarkerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarkerVtbl {
        unsafe extern "system" fn Time<Impl: IPlaybackMediaMarkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaMarkerType<Impl: IPlaybackMediaMarkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaMarkerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IPlaybackMediaMarkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaybackMediaMarker>, ::windows::core::GetTrustLevel, Time::<Impl, IMPL_OFFSET>, MediaMarkerType::<Impl, IMPL_OFFSET>, Text::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPlaybackMediaMarkerFactoryImpl: Sized {
    fn CreateFromTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<PlaybackMediaMarker>;
    fn Create(&self, value: &super::super::Foundation::TimeSpan, mediamarkettype: &::windows::core::HSTRING, text: &::windows::core::HSTRING) -> ::windows::core::Result<PlaybackMediaMarker>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackMediaMarkerFactory {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarkerFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPlaybackMediaMarkerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarkerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarkerFactoryVtbl {
        unsafe extern "system" fn CreateFromTime<Impl: IPlaybackMediaMarkerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IPlaybackMediaMarkerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan, mediamarkettype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&mediamarkettype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaybackMediaMarkerFactory>, ::windows::core::GetTrustLevel, CreateFromTime::<Impl, IMPL_OFFSET>, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarkerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackMediaMarkerReachedEventArgsImpl: Sized {
    fn PlaybackMediaMarker(&self) -> ::windows::core::Result<PlaybackMediaMarker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarkerReachedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaybackMediaMarkerReachedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarkerReachedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarkerReachedEventArgsVtbl {
        unsafe extern "system" fn PlaybackMediaMarker<Impl: IPlaybackMediaMarkerReachedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackMediaMarker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaybackMediaMarkerReachedEventArgs>, ::windows::core::GetTrustLevel, PlaybackMediaMarker::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarkerReachedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPlaybackMediaMarkerSequenceImpl: Sized + IIterableImpl<PlaybackMediaMarker> {
    fn Size(&self) -> ::windows::core::Result<u32>;
    fn Insert(&self, value: &::core::option::Option<PlaybackMediaMarker>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackMediaMarkerSequence {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarkerSequence";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPlaybackMediaMarkerSequenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarkerSequenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarkerSequenceVtbl {
        unsafe extern "system" fn Size<Impl: IPlaybackMediaMarkerSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Insert<Impl: IPlaybackMediaMarkerSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert(&*(&value as *const <PlaybackMediaMarker as ::windows::core::Abi>::Abi as *const <PlaybackMediaMarker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPlaybackMediaMarkerSequenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaybackMediaMarkerSequence>, ::windows::core::GetTrustLevel, Size::<Impl, IMPL_OFFSET>, Insert::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarkerSequence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait ITimedMetadataPresentationModeChangedEventArgsImpl: Sized {
    fn Track(&self) -> ::windows::core::Result<super::Core::TimedMetadataTrack>;
    fn OldPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
    fn NewPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedMetadataPresentationModeChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.ITimedMetadataPresentationModeChangedEventArgs";
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ITimedMetadataPresentationModeChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataPresentationModeChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataPresentationModeChangedEventArgsVtbl {
        unsafe extern "system" fn Track<Impl: ITimedMetadataPresentationModeChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPresentationMode<Impl: ITimedMetadataPresentationModeChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldPresentationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewPresentationMode<Impl: ITimedMetadataPresentationModeChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewPresentationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimedMetadataPresentationModeChangedEventArgs>, ::windows::core::GetTrustLevel, Track::<Impl, IMPL_OFFSET>, OldPresentationMode::<Impl, IMPL_OFFSET>, NewPresentationMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataPresentationModeChangedEventArgs as ::windows::core::Interface>::IID
    }
}
