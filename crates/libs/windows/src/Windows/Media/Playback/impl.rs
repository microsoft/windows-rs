#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundMediaPlayerStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<MediaPlayer>;
    fn MessageReceivedFromBackground(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceivedFromBackground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MessageReceivedFromForeground(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceivedFromForeground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SendMessageToBackground(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
    fn SendMessageToForeground(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
    fn IsMediaPlaying(&mut self) -> ::windows::core::Result<bool>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundMediaPlayerStatics {
    const NAME: &'static str = "Windows.Media.Playback.IBackgroundMediaPlayerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IBackgroundMediaPlayerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundMediaPlayerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundMediaPlayerStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MessageReceivedFromBackground<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMessageReceivedFromBackground<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceivedFromBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MessageReceivedFromForeground<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMessageReceivedFromForeground<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceivedFromForeground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendMessageToBackground<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessageToBackground(&*(&value as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendMessageToForeground<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessageToForeground(&*(&value as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsMediaPlaying<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Shutdown<Impl: IBackgroundMediaPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundMediaPlayerStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            MessageReceivedFromBackground: MessageReceivedFromBackground::<Impl, IMPL_OFFSET>,
            RemoveMessageReceivedFromBackground: RemoveMessageReceivedFromBackground::<Impl, IMPL_OFFSET>,
            MessageReceivedFromForeground: MessageReceivedFromForeground::<Impl, IMPL_OFFSET>,
            RemoveMessageReceivedFromForeground: RemoveMessageReceivedFromForeground::<Impl, IMPL_OFFSET>,
            SendMessageToBackground: SendMessageToBackground::<Impl, IMPL_OFFSET>,
            SendMessageToForeground: SendMessageToForeground::<Impl, IMPL_OFFSET>,
            IsMediaPlaying: IsMediaPlaying::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundMediaPlayerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentMediaPlaybackItemChangedEventArgs_Impl: Sized {
    fn NewItem(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn OldItem(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrentMediaPlaybackItemChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.ICurrentMediaPlaybackItemChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrentMediaPlaybackItemChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentMediaPlaybackItemChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentMediaPlaybackItemChangedEventArgs_Vtbl {
        unsafe extern "system" fn NewItem<Impl: ICurrentMediaPlaybackItemChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OldItem<Impl: ICurrentMediaPlaybackItemChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentMediaPlaybackItemChangedEventArgs, BASE_OFFSET>(),
            NewItem: NewItem::<Impl, IMPL_OFFSET>,
            OldItem: OldItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentMediaPlaybackItemChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentMediaPlaybackItemChangedEventArgs2_Impl: Sized + ICurrentMediaPlaybackItemChangedEventArgs_Impl {
    fn Reason(&mut self) -> ::windows::core::Result<MediaPlaybackItemChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICurrentMediaPlaybackItemChangedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Playback.ICurrentMediaPlaybackItemChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICurrentMediaPlaybackItemChangedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl {
        unsafe extern "system" fn Reason<Impl: ICurrentMediaPlaybackItemChangedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemChangedReason) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICurrentMediaPlaybackItemChangedEventArgs2, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICurrentMediaPlaybackItemChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaBreak_Impl: Sized {
    fn PlaybackList(&mut self) -> ::windows::core::Result<MediaPlaybackList>;
    fn PresentationPosition(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn InsertionMethod(&mut self) -> ::windows::core::Result<MediaBreakInsertionMethod>;
    fn CustomProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn CanStart(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanStart(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreak {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreak";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaBreak_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreak_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreak_Vtbl {
        unsafe extern "system" fn PlaybackList<Impl: IMediaBreak_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PresentationPosition<Impl: IMediaBreak_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InsertionMethod<Impl: IMediaBreak_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaBreakInsertionMethod) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomProperties<Impl: IMediaBreak_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanStart<Impl: IMediaBreak_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanStart<Impl: IMediaBreak_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanStart(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreak, BASE_OFFSET>(),
            PlaybackList: PlaybackList::<Impl, IMPL_OFFSET>,
            PresentationPosition: PresentationPosition::<Impl, IMPL_OFFSET>,
            InsertionMethod: InsertionMethod::<Impl, IMPL_OFFSET>,
            CustomProperties: CustomProperties::<Impl, IMPL_OFFSET>,
            CanStart: CanStart::<Impl, IMPL_OFFSET>,
            SetCanStart: SetCanStart::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreak as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakEndedEventArgs_Impl: Sized {
    fn MediaBreak(&mut self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBreakEndedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakEndedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBreakEndedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakEndedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakEndedEventArgs_Vtbl {
        unsafe extern "system" fn MediaBreak<Impl: IMediaBreakEndedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreakEndedEventArgs, BASE_OFFSET>(), MediaBreak: MediaBreak::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakEndedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaBreakFactory_Impl: Sized {
    fn Create(&mut self, insertionmethod: MediaBreakInsertionMethod) -> ::windows::core::Result<MediaBreak>;
    fn CreateWithPresentationPosition(&mut self, insertionmethod: MediaBreakInsertionMethod, presentationposition: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakFactory {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaBreakFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IMediaBreakFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithPresentationPosition<Impl: IMediaBreakFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, presentationposition: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreakFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithPresentationPosition: CreateWithPresentationPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaBreakManager_Impl: Sized {
    fn BreaksSeekedOver(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreaksSeekedOver(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakStarted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakStarted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakEnded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakEnded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BreakSkipped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBreakSkipped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentBreak(&mut self) -> ::windows::core::Result<MediaBreak>;
    fn PlaybackSession(&mut self) -> ::windows::core::Result<MediaPlaybackSession>;
    fn PlayBreak(&mut self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn SkipCurrentBreak(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakManager {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaBreakManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakManager_Vtbl {
        unsafe extern "system" fn BreaksSeekedOver<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBreaksSeekedOver<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreaksSeekedOver(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BreakStarted<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBreakStarted<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreakStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BreakEnded<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBreakEnded<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreakEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BreakSkipped<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBreakSkipped<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBreakSkipped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentBreak<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlaybackSession<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlayBreak<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PlayBreak(&*(&value as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SkipCurrentBreak<Impl: IMediaBreakManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SkipCurrentBreak().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreakManager, BASE_OFFSET>(),
            BreaksSeekedOver: BreaksSeekedOver::<Impl, IMPL_OFFSET>,
            RemoveBreaksSeekedOver: RemoveBreaksSeekedOver::<Impl, IMPL_OFFSET>,
            BreakStarted: BreakStarted::<Impl, IMPL_OFFSET>,
            RemoveBreakStarted: RemoveBreakStarted::<Impl, IMPL_OFFSET>,
            BreakEnded: BreakEnded::<Impl, IMPL_OFFSET>,
            RemoveBreakEnded: RemoveBreakEnded::<Impl, IMPL_OFFSET>,
            BreakSkipped: BreakSkipped::<Impl, IMPL_OFFSET>,
            RemoveBreakSkipped: RemoveBreakSkipped::<Impl, IMPL_OFFSET>,
            CurrentBreak: CurrentBreak::<Impl, IMPL_OFFSET>,
            PlaybackSession: PlaybackSession::<Impl, IMPL_OFFSET>,
            PlayBreak: PlayBreak::<Impl, IMPL_OFFSET>,
            SkipCurrentBreak: SkipCurrentBreak::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaBreakSchedule_Impl: Sized {
    fn ScheduleChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBreakSchedule, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScheduleChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InsertMidrollBreak(&mut self, mediabreak: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn RemoveMidrollBreak(&mut self, mediabreak: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn MidrollBreaks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>>;
    fn SetPrerollBreak(&mut self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn PrerollBreak(&mut self) -> ::windows::core::Result<MediaBreak>;
    fn SetPostrollBreak(&mut self, value: &::core::option::Option<MediaBreak>) -> ::windows::core::Result<()>;
    fn PostrollBreak(&mut self) -> ::windows::core::Result<MediaBreak>;
    fn PlaybackItem(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakSchedule {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakSchedule";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaBreakSchedule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakSchedule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakSchedule_Vtbl {
        unsafe extern "system" fn ScheduleChanged<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveScheduleChanged<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScheduleChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertMidrollBreak<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediabreak: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertMidrollBreak(&*(&mediabreak as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveMidrollBreak<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediabreak: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMidrollBreak(&*(&mediabreak as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MidrollBreaks<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPrerollBreak<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrerollBreak(&*(&value as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrerollBreak<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPostrollBreak<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostrollBreak(&*(&value as *const <MediaBreak as ::windows::core::Abi>::Abi as *const <MediaBreak as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PostrollBreak<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlaybackItem<Impl: IMediaBreakSchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreakSchedule, BASE_OFFSET>(),
            ScheduleChanged: ScheduleChanged::<Impl, IMPL_OFFSET>,
            RemoveScheduleChanged: RemoveScheduleChanged::<Impl, IMPL_OFFSET>,
            InsertMidrollBreak: InsertMidrollBreak::<Impl, IMPL_OFFSET>,
            RemoveMidrollBreak: RemoveMidrollBreak::<Impl, IMPL_OFFSET>,
            MidrollBreaks: MidrollBreaks::<Impl, IMPL_OFFSET>,
            SetPrerollBreak: SetPrerollBreak::<Impl, IMPL_OFFSET>,
            PrerollBreak: PrerollBreak::<Impl, IMPL_OFFSET>,
            SetPostrollBreak: SetPostrollBreak::<Impl, IMPL_OFFSET>,
            PostrollBreak: PostrollBreak::<Impl, IMPL_OFFSET>,
            PlaybackItem: PlaybackItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakSchedule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaBreakSeekedOverEventArgs_Impl: Sized {
    fn SeekedOverBreaks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>>;
    fn OldPosition(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn NewPosition(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBreakSeekedOverEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakSeekedOverEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaBreakSeekedOverEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakSeekedOverEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakSeekedOverEventArgs_Vtbl {
        unsafe extern "system" fn SeekedOverBreaks<Impl: IMediaBreakSeekedOverEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OldPosition<Impl: IMediaBreakSeekedOverEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewPosition<Impl: IMediaBreakSeekedOverEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreakSeekedOverEventArgs, BASE_OFFSET>(),
            SeekedOverBreaks: SeekedOverBreaks::<Impl, IMPL_OFFSET>,
            OldPosition: OldPosition::<Impl, IMPL_OFFSET>,
            NewPosition: NewPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakSeekedOverEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakSkippedEventArgs_Impl: Sized {
    fn MediaBreak(&mut self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBreakSkippedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakSkippedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBreakSkippedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakSkippedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakSkippedEventArgs_Vtbl {
        unsafe extern "system" fn MediaBreak<Impl: IMediaBreakSkippedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreakSkippedEventArgs, BASE_OFFSET>(),
            MediaBreak: MediaBreak::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakSkippedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBreakStartedEventArgs_Impl: Sized {
    fn MediaBreak(&mut self) -> ::windows::core::Result<MediaBreak>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBreakStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaBreakStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBreakStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBreakStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBreakStartedEventArgs_Vtbl {
        unsafe extern "system" fn MediaBreak<Impl: IMediaBreakStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBreakStartedEventArgs, BASE_OFFSET>(),
            MediaBreak: MediaBreak::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBreakStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait IMediaEnginePlaybackSource_Impl: Sized {
    fn CurrentItem(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn SetPlaybackSource(&mut self, source: &::core::option::Option<IMediaPlaybackSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IMediaEnginePlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaEnginePlaybackSource";
}
#[cfg(feature = "deprecated")]
impl IMediaEnginePlaybackSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEnginePlaybackSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEnginePlaybackSource_Vtbl {
        unsafe extern "system" fn CurrentItem<Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackSource<Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackSource(&*(&source as *const <IMediaPlaybackSource as ::windows::core::Abi>::Abi as *const <IMediaPlaybackSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEnginePlaybackSource, BASE_OFFSET>(),
            CurrentItem: CurrentItem::<Impl, IMPL_OFFSET>,
            SetPlaybackSource: SetPlaybackSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEnginePlaybackSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaItemDisplayProperties_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<super::MediaPlaybackType>;
    fn SetType(&mut self, value: super::MediaPlaybackType) -> ::windows::core::Result<()>;
    fn MusicProperties(&mut self) -> ::windows::core::Result<super::MusicDisplayProperties>;
    fn VideoProperties(&mut self) -> ::windows::core::Result<super::VideoDisplayProperties>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn ClearAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaItemDisplayProperties {
    const NAME: &'static str = "Windows.Media.Playback.IMediaItemDisplayProperties";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaItemDisplayProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaItemDisplayProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaItemDisplayProperties_Vtbl {
        unsafe extern "system" fn Type<Impl: IMediaItemDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IMediaItemDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaPlaybackType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn MusicProperties<Impl: IMediaItemDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VideoProperties<Impl: IMediaItemDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Thumbnail<Impl: IMediaItemDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: IMediaItemDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearAll<Impl: IMediaItemDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAll().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaItemDisplayProperties, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            MusicProperties: MusicProperties::<Impl, IMPL_OFFSET>,
            VideoProperties: VideoProperties::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            ClearAll: ClearAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaItemDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManager_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MediaPlayer(&mut self) -> ::windows::core::Result<MediaPlayer>;
    fn PlayBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PauseBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn NextBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PreviousBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn FastForwardBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn RewindBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn ShuffleBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn AutoRepeatModeBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PositionBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn RateBehavior(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior>;
    fn PlayReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PauseReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePauseReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NextReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNextReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviousReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviousReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FastForwardReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFastForwardReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RewindReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRewindReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShuffleReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShuffleReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoRepeatModeReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoRepeatModeReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RateReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRateReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManager {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManager_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn MediaPlayer<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlayBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PauseBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NextBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PreviousBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FastForwardBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RewindBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShuffleBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoRepeatModeBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RateBehavior<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlayReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlayReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePauseReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePauseReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NextReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveNextReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNextReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviousReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePreviousReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviousReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FastForwardReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveFastForwardReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFastForwardReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RewindReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRewindReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRewindReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShuffleReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveShuffleReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShuffleReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoRepeatModeReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAutoRepeatModeReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutoRepeatModeReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePositionReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RateReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRateReceived<Impl: IMediaPlaybackCommandManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRateReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManager, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            MediaPlayer: MediaPlayer::<Impl, IMPL_OFFSET>,
            PlayBehavior: PlayBehavior::<Impl, IMPL_OFFSET>,
            PauseBehavior: PauseBehavior::<Impl, IMPL_OFFSET>,
            NextBehavior: NextBehavior::<Impl, IMPL_OFFSET>,
            PreviousBehavior: PreviousBehavior::<Impl, IMPL_OFFSET>,
            FastForwardBehavior: FastForwardBehavior::<Impl, IMPL_OFFSET>,
            RewindBehavior: RewindBehavior::<Impl, IMPL_OFFSET>,
            ShuffleBehavior: ShuffleBehavior::<Impl, IMPL_OFFSET>,
            AutoRepeatModeBehavior: AutoRepeatModeBehavior::<Impl, IMPL_OFFSET>,
            PositionBehavior: PositionBehavior::<Impl, IMPL_OFFSET>,
            RateBehavior: RateBehavior::<Impl, IMPL_OFFSET>,
            PlayReceived: PlayReceived::<Impl, IMPL_OFFSET>,
            RemovePlayReceived: RemovePlayReceived::<Impl, IMPL_OFFSET>,
            PauseReceived: PauseReceived::<Impl, IMPL_OFFSET>,
            RemovePauseReceived: RemovePauseReceived::<Impl, IMPL_OFFSET>,
            NextReceived: NextReceived::<Impl, IMPL_OFFSET>,
            RemoveNextReceived: RemoveNextReceived::<Impl, IMPL_OFFSET>,
            PreviousReceived: PreviousReceived::<Impl, IMPL_OFFSET>,
            RemovePreviousReceived: RemovePreviousReceived::<Impl, IMPL_OFFSET>,
            FastForwardReceived: FastForwardReceived::<Impl, IMPL_OFFSET>,
            RemoveFastForwardReceived: RemoveFastForwardReceived::<Impl, IMPL_OFFSET>,
            RewindReceived: RewindReceived::<Impl, IMPL_OFFSET>,
            RemoveRewindReceived: RemoveRewindReceived::<Impl, IMPL_OFFSET>,
            ShuffleReceived: ShuffleReceived::<Impl, IMPL_OFFSET>,
            RemoveShuffleReceived: RemoveShuffleReceived::<Impl, IMPL_OFFSET>,
            AutoRepeatModeReceived: AutoRepeatModeReceived::<Impl, IMPL_OFFSET>,
            RemoveAutoRepeatModeReceived: RemoveAutoRepeatModeReceived::<Impl, IMPL_OFFSET>,
            PositionReceived: PositionReceived::<Impl, IMPL_OFFSET>,
            RemovePositionReceived: RemovePositionReceived::<Impl, IMPL_OFFSET>,
            RateReceived: RateReceived::<Impl, IMPL_OFFSET>,
            RemoveRateReceived: RemoveRateReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutoRepeatMode(&mut self) -> ::windows::core::Result<super::MediaPlaybackAutoRepeatMode>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn AutoRepeatMode<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            AutoRepeatMode: AutoRepeatMode::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerCommandBehavior_Impl: Sized {
    fn CommandManager(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManager>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn EnablingRule(&mut self) -> ::windows::core::Result<MediaCommandEnablingRule>;
    fn SetEnablingRule(&mut self, value: MediaCommandEnablingRule) -> ::windows::core::Result<()>;
    fn IsEnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsEnabledChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerCommandBehavior {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerCommandBehavior";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerCommandBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerCommandBehavior_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerCommandBehavior_Vtbl {
        unsafe extern "system" fn CommandManager<Impl: IMediaPlaybackCommandManagerCommandBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IMediaPlaybackCommandManagerCommandBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnablingRule<Impl: IMediaPlaybackCommandManagerCommandBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaCommandEnablingRule) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnablingRule<Impl: IMediaPlaybackCommandManagerCommandBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaCommandEnablingRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnablingRule(value).into()
        }
        unsafe extern "system" fn IsEnabledChanged<Impl: IMediaPlaybackCommandManagerCommandBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveIsEnabledChanged<Impl: IMediaPlaybackCommandManagerCommandBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerCommandBehavior, BASE_OFFSET>(),
            CommandManager: CommandManager::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            EnablingRule: EnablingRule::<Impl, IMPL_OFFSET>,
            SetEnablingRule: SetEnablingRule::<Impl, IMPL_OFFSET>,
            IsEnabledChanged: IsEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveIsEnabledChanged: RemoveIsEnabledChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerCommandBehavior as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerFastForwardReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerFastForwardReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerFastForwardReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerNextReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerNextReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerNextReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerNextReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerNextReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerNextReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerNextReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerNextReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPauseReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPauseReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPauseReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerPauseReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPauseReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPlayReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPlayReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPlayReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerPlayReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPlayReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPositionReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPositionReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Position<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPositionReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerPositionReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPositionReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerPreviousReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerPreviousReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerPreviousReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerRateReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&mut self) -> ::windows::core::Result<f64>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerRateReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerRateReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PlaybackRate<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerRateReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerRateReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            PlaybackRate: PlaybackRate::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerRateReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerRewindReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerRewindReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerRewindReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerRewindReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerRewindReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsShuffleRequested(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackCommandManagerShuffleReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn IsShuffleRequested<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackCommandManagerShuffleReceivedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            IsShuffleRequested: IsShuffleRequested::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackCommandManagerShuffleReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItem_Impl: Sized + IMediaPlaybackSource_Impl {
    fn AudioTracksChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioTracksChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoTracksChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoTracksChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TimedMetadataTracksChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTimedMetadataTracksChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn AudioTracks(&mut self) -> ::windows::core::Result<MediaPlaybackAudioTrackList>;
    fn VideoTracks(&mut self) -> ::windows::core::Result<MediaPlaybackVideoTrackList>;
    fn TimedMetadataTracks(&mut self) -> ::windows::core::Result<MediaPlaybackTimedMetadataTrackList>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItem {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItem_Vtbl {
        unsafe extern "system" fn AudioTracksChanged<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAudioTracksChanged<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioTracksChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoTracksChanged<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveVideoTracksChanged<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoTracksChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimedMetadataTracksChanged<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTimedMetadataTracksChanged<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTimedMetadataTracksChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AudioTracks<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VideoTracks<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimedMetadataTracks<Impl: IMediaPlaybackItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItem, BASE_OFFSET>(),
            AudioTracksChanged: AudioTracksChanged::<Impl, IMPL_OFFSET>,
            RemoveAudioTracksChanged: RemoveAudioTracksChanged::<Impl, IMPL_OFFSET>,
            VideoTracksChanged: VideoTracksChanged::<Impl, IMPL_OFFSET>,
            RemoveVideoTracksChanged: RemoveVideoTracksChanged::<Impl, IMPL_OFFSET>,
            TimedMetadataTracksChanged: TimedMetadataTracksChanged::<Impl, IMPL_OFFSET>,
            RemoveTimedMetadataTracksChanged: RemoveTimedMetadataTracksChanged::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            AudioTracks: AudioTracks::<Impl, IMPL_OFFSET>,
            VideoTracks: VideoTracks::<Impl, IMPL_OFFSET>,
            TimedMetadataTracks: TimedMetadataTracks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItem2_Impl: Sized + IMediaPlaybackItem_Impl + IMediaPlaybackSource_Impl {
    fn BreakSchedule(&mut self) -> ::windows::core::Result<MediaBreakSchedule>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn DurationLimit(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn CanSkip(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanSkip(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDisplayProperties(&mut self) -> ::windows::core::Result<MediaItemDisplayProperties>;
    fn ApplyDisplayProperties(&mut self, value: &::core::option::Option<MediaItemDisplayProperties>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItem2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItem2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItem2_Vtbl {
        unsafe extern "system" fn BreakSchedule<Impl: IMediaPlaybackItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartTime<Impl: IMediaPlaybackItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DurationLimit<Impl: IMediaPlaybackItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanSkip<Impl: IMediaPlaybackItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanSkip<Impl: IMediaPlaybackItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanSkip(value).into()
        }
        unsafe extern "system" fn GetDisplayProperties<Impl: IMediaPlaybackItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplyDisplayProperties<Impl: IMediaPlaybackItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyDisplayProperties(&*(&value as *const <MediaItemDisplayProperties as ::windows::core::Abi>::Abi as *const <MediaItemDisplayProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItem2, BASE_OFFSET>(),
            BreakSchedule: BreakSchedule::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            DurationLimit: DurationLimit::<Impl, IMPL_OFFSET>,
            CanSkip: CanSkip::<Impl, IMPL_OFFSET>,
            SetCanSkip: SetCanSkip::<Impl, IMPL_OFFSET>,
            GetDisplayProperties: GetDisplayProperties::<Impl, IMPL_OFFSET>,
            ApplyDisplayProperties: ApplyDisplayProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItem3_Impl: Sized + IMediaPlaybackItem_Impl + IMediaPlaybackItem2_Impl + IMediaPlaybackSource_Impl {
    fn IsDisabledInPlaybackList(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledInPlaybackList(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TotalDownloadProgress(&mut self) -> ::windows::core::Result<f64>;
    fn AutoLoadedDisplayProperties(&mut self) -> ::windows::core::Result<AutoLoadedDisplayPropertyKind>;
    fn SetAutoLoadedDisplayProperties(&mut self, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItem3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItem3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItem3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItem3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItem3_Vtbl {
        unsafe extern "system" fn IsDisabledInPlaybackList<Impl: IMediaPlaybackItem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsDisabledInPlaybackList<Impl: IMediaPlaybackItem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDisabledInPlaybackList(value).into()
        }
        unsafe extern "system" fn TotalDownloadProgress<Impl: IMediaPlaybackItem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoLoadedDisplayProperties<Impl: IMediaPlaybackItem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoLoadedDisplayProperties<Impl: IMediaPlaybackItem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoLoadedDisplayProperties(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItem3, BASE_OFFSET>(),
            IsDisabledInPlaybackList: IsDisabledInPlaybackList::<Impl, IMPL_OFFSET>,
            SetIsDisabledInPlaybackList: SetIsDisabledInPlaybackList::<Impl, IMPL_OFFSET>,
            TotalDownloadProgress: TotalDownloadProgress::<Impl, IMPL_OFFSET>,
            AutoLoadedDisplayProperties: AutoLoadedDisplayProperties::<Impl, IMPL_OFFSET>,
            SetAutoLoadedDisplayProperties: SetAutoLoadedDisplayProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItem3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemError_Impl: Sized {
    fn ErrorCode(&mut self) -> ::windows::core::Result<MediaPlaybackItemErrorCode>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackItemError {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemError";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackItemError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemError_Vtbl {
        unsafe extern "system" fn ErrorCode<Impl: IMediaPlaybackItemError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemErrorCode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IMediaPlaybackItemError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItemError, BASE_OFFSET>(),
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItemFactory_Impl: Sized {
    fn Create(&mut self, source: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItemFactory {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemFactory";
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItemFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IMediaPlaybackItemFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItemFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItemFactory2_Impl: Sized + IMediaPlaybackItemFactory_Impl {
    fn CreateWithStartTime(&mut self, source: &::core::option::Option<super::Core::MediaSource>, starttime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem>;
    fn CreateWithStartTimeAndDurationLimit(&mut self, source: &::core::option::Option<super::Core::MediaSource>, starttime: &super::super::Foundation::TimeSpan, durationlimit: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItemFactory2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemFactory2";
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItemFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemFactory2_Vtbl {
        unsafe extern "system" fn CreateWithStartTime<Impl: IMediaPlaybackItemFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, starttime: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithStartTimeAndDurationLimit<Impl: IMediaPlaybackItemFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, starttime: super::super::Foundation::TimeSpan, durationlimit: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItemFactory2, BASE_OFFSET>(),
            CreateWithStartTime: CreateWithStartTime::<Impl, IMPL_OFFSET>,
            CreateWithStartTimeAndDurationLimit: CreateWithStartTimeAndDurationLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemFailedEventArgs_Impl: Sized {
    fn Item(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn Error(&mut self) -> ::windows::core::Result<MediaPlaybackItemError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackItemFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackItemFailedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemFailedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemFailedEventArgs_Vtbl {
        unsafe extern "system" fn Item<Impl: IMediaPlaybackItemFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Error<Impl: IMediaPlaybackItemFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItemFailedEventArgs, BASE_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackItemOpenedEventArgs_Impl: Sized {
    fn Item(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackItemOpenedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemOpenedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackItemOpenedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemOpenedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemOpenedEventArgs_Vtbl {
        unsafe extern "system" fn Item<Impl: IMediaPlaybackItemOpenedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItemOpenedEventArgs, BASE_OFFSET>(), Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemOpenedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaPlaybackItemStatics_Impl: Sized {
    fn FindFromMediaSource(&mut self, source: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackItemStatics {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackItemStatics";
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaPlaybackItemStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackItemStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackItemStatics_Vtbl {
        unsafe extern "system" fn FindFromMediaSource<Impl: IMediaPlaybackItemStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackItemStatics, BASE_OFFSET>(),
            FindFromMediaSource: FindFromMediaSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackItemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackList_Impl: Sized + IMediaPlaybackSource_Impl {
    fn ItemFailed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemFailed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentItemChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentItemChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemOpened(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemOpened(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Items(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<MediaPlaybackItem>>;
    fn AutoRepeatEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoRepeatEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ShuffleEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetShuffleEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentItem(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn CurrentItemIndex(&mut self) -> ::windows::core::Result<u32>;
    fn MoveNext(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn MovePrevious(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn MoveTo(&mut self, itemindex: u32) -> ::windows::core::Result<MediaPlaybackItem>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackList {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackList";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackList_Vtbl {
        unsafe extern "system" fn ItemFailed<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveItemFailed<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentItemChanged<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCurrentItemChanged<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentItemChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemOpened<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveItemOpened<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Items<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoRepeatEnabled<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoRepeatEnabled<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoRepeatEnabled(value).into()
        }
        unsafe extern "system" fn ShuffleEnabled<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShuffleEnabled<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShuffleEnabled(value).into()
        }
        unsafe extern "system" fn CurrentItem<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentItemIndex<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MoveNext<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MovePrevious<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MoveTo<Impl: IMediaPlaybackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackList, BASE_OFFSET>(),
            ItemFailed: ItemFailed::<Impl, IMPL_OFFSET>,
            RemoveItemFailed: RemoveItemFailed::<Impl, IMPL_OFFSET>,
            CurrentItemChanged: CurrentItemChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentItemChanged: RemoveCurrentItemChanged::<Impl, IMPL_OFFSET>,
            ItemOpened: ItemOpened::<Impl, IMPL_OFFSET>,
            RemoveItemOpened: RemoveItemOpened::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
            AutoRepeatEnabled: AutoRepeatEnabled::<Impl, IMPL_OFFSET>,
            SetAutoRepeatEnabled: SetAutoRepeatEnabled::<Impl, IMPL_OFFSET>,
            ShuffleEnabled: ShuffleEnabled::<Impl, IMPL_OFFSET>,
            SetShuffleEnabled: SetShuffleEnabled::<Impl, IMPL_OFFSET>,
            CurrentItem: CurrentItem::<Impl, IMPL_OFFSET>,
            CurrentItemIndex: CurrentItemIndex::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            MovePrevious: MovePrevious::<Impl, IMPL_OFFSET>,
            MoveTo: MoveTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackList2_Impl: Sized + IMediaPlaybackList_Impl + IMediaPlaybackSource_Impl {
    fn MaxPrefetchTime(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetMaxPrefetchTime(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn StartingItem(&mut self) -> ::windows::core::Result<MediaPlaybackItem>;
    fn SetStartingItem(&mut self, value: &::core::option::Option<MediaPlaybackItem>) -> ::windows::core::Result<()>;
    fn ShuffledItems(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaPlaybackItem>>;
    fn SetShuffledItems(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<MediaPlaybackItem>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackList2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackList2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackList2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackList2_Vtbl {
        unsafe extern "system" fn MaxPrefetchTime<Impl: IMediaPlaybackList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxPrefetchTime<Impl: IMediaPlaybackList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPrefetchTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartingItem<Impl: IMediaPlaybackList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStartingItem<Impl: IMediaPlaybackList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartingItem(&*(&value as *const <MediaPlaybackItem as ::windows::core::Abi>::Abi as *const <MediaPlaybackItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShuffledItems<Impl: IMediaPlaybackList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShuffledItems<Impl: IMediaPlaybackList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShuffledItems(&*(&value as *const <super::super::Foundation::Collections::IIterable<MediaPlaybackItem> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<MediaPlaybackItem> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackList2, BASE_OFFSET>(),
            MaxPrefetchTime: MaxPrefetchTime::<Impl, IMPL_OFFSET>,
            SetMaxPrefetchTime: SetMaxPrefetchTime::<Impl, IMPL_OFFSET>,
            StartingItem: StartingItem::<Impl, IMPL_OFFSET>,
            SetStartingItem: SetStartingItem::<Impl, IMPL_OFFSET>,
            ShuffledItems: ShuffledItems::<Impl, IMPL_OFFSET>,
            SetShuffledItems: SetShuffledItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackList3_Impl: Sized + IMediaPlaybackList_Impl + IMediaPlaybackList2_Impl + IMediaPlaybackSource_Impl {
    fn MaxPlayedItemsToKeepOpen(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetMaxPlayedItemsToKeepOpen(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackList3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackList3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackList3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackList3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackList3_Vtbl {
        unsafe extern "system" fn MaxPlayedItemsToKeepOpen<Impl: IMediaPlaybackList3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxPlayedItemsToKeepOpen<Impl: IMediaPlaybackList3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPlayedItemsToKeepOpen(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackList3, BASE_OFFSET>(),
            MaxPlayedItemsToKeepOpen: MaxPlayedItemsToKeepOpen::<Impl, IMPL_OFFSET>,
            SetMaxPlayedItemsToKeepOpen: SetMaxPlayedItemsToKeepOpen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackList3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSession_Impl: Sized {
    fn PlaybackStateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekCompleted(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingStarted(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingStarted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingEnded(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingEnded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingProgressChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingProgressChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadProgressChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadProgressChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NaturalDurationChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNaturalDurationChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NaturalVideoSizeChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNaturalVideoSizeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaPlayer(&mut self) -> ::windows::core::Result<MediaPlayer>;
    fn NaturalDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPosition(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PlaybackState(&mut self) -> ::windows::core::Result<MediaPlaybackState>;
    fn CanSeek(&mut self) -> ::windows::core::Result<bool>;
    fn CanPause(&mut self) -> ::windows::core::Result<bool>;
    fn IsProtected(&mut self) -> ::windows::core::Result<bool>;
    fn PlaybackRate(&mut self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn BufferingProgress(&mut self) -> ::windows::core::Result<f64>;
    fn DownloadProgress(&mut self) -> ::windows::core::Result<f64>;
    fn NaturalVideoHeight(&mut self) -> ::windows::core::Result<u32>;
    fn NaturalVideoWidth(&mut self) -> ::windows::core::Result<u32>;
    fn NormalizedSourceRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetNormalizedSourceRect(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn StereoscopicVideoPackingMode(&mut self) -> ::windows::core::Result<super::MediaProperties::StereoscopicVideoPackingMode>;
    fn SetStereoscopicVideoPackingMode(&mut self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSession {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSession";
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaPlaybackSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSession_Vtbl {
        unsafe extern "system" fn PlaybackStateChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlaybackStateChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackRateChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlaybackRateChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackRateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SeekCompleted<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSeekCompleted<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSeekCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingStarted<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBufferingStarted<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingEnded<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBufferingEnded<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingProgressChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBufferingProgressChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingProgressChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadProgressChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDownloadProgressChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadProgressChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalDurationChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveNaturalDurationChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNaturalDurationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PositionChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePositionChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NaturalVideoSizeChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveNaturalVideoSizeChanged<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNaturalVideoSizeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaPlayer<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NaturalDuration<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackState<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanSeek<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanPause<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsProtected<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlaybackRate<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackRate<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn BufferingProgress<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DownloadProgress<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NaturalVideoHeight<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NaturalVideoWidth<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NormalizedSourceRect<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNormalizedSourceRect<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalizedSourceRect(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StereoscopicVideoPackingMode<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStereoscopicVideoPackingMode<Impl: IMediaPlaybackSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStereoscopicVideoPackingMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSession, BASE_OFFSET>(),
            PlaybackStateChanged: PlaybackStateChanged::<Impl, IMPL_OFFSET>,
            RemovePlaybackStateChanged: RemovePlaybackStateChanged::<Impl, IMPL_OFFSET>,
            PlaybackRateChanged: PlaybackRateChanged::<Impl, IMPL_OFFSET>,
            RemovePlaybackRateChanged: RemovePlaybackRateChanged::<Impl, IMPL_OFFSET>,
            SeekCompleted: SeekCompleted::<Impl, IMPL_OFFSET>,
            RemoveSeekCompleted: RemoveSeekCompleted::<Impl, IMPL_OFFSET>,
            BufferingStarted: BufferingStarted::<Impl, IMPL_OFFSET>,
            RemoveBufferingStarted: RemoveBufferingStarted::<Impl, IMPL_OFFSET>,
            BufferingEnded: BufferingEnded::<Impl, IMPL_OFFSET>,
            RemoveBufferingEnded: RemoveBufferingEnded::<Impl, IMPL_OFFSET>,
            BufferingProgressChanged: BufferingProgressChanged::<Impl, IMPL_OFFSET>,
            RemoveBufferingProgressChanged: RemoveBufferingProgressChanged::<Impl, IMPL_OFFSET>,
            DownloadProgressChanged: DownloadProgressChanged::<Impl, IMPL_OFFSET>,
            RemoveDownloadProgressChanged: RemoveDownloadProgressChanged::<Impl, IMPL_OFFSET>,
            NaturalDurationChanged: NaturalDurationChanged::<Impl, IMPL_OFFSET>,
            RemoveNaturalDurationChanged: RemoveNaturalDurationChanged::<Impl, IMPL_OFFSET>,
            PositionChanged: PositionChanged::<Impl, IMPL_OFFSET>,
            RemovePositionChanged: RemovePositionChanged::<Impl, IMPL_OFFSET>,
            NaturalVideoSizeChanged: NaturalVideoSizeChanged::<Impl, IMPL_OFFSET>,
            RemoveNaturalVideoSizeChanged: RemoveNaturalVideoSizeChanged::<Impl, IMPL_OFFSET>,
            MediaPlayer: MediaPlayer::<Impl, IMPL_OFFSET>,
            NaturalDuration: NaturalDuration::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            PlaybackState: PlaybackState::<Impl, IMPL_OFFSET>,
            CanSeek: CanSeek::<Impl, IMPL_OFFSET>,
            CanPause: CanPause::<Impl, IMPL_OFFSET>,
            IsProtected: IsProtected::<Impl, IMPL_OFFSET>,
            PlaybackRate: PlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate: SetPlaybackRate::<Impl, IMPL_OFFSET>,
            BufferingProgress: BufferingProgress::<Impl, IMPL_OFFSET>,
            DownloadProgress: DownloadProgress::<Impl, IMPL_OFFSET>,
            NaturalVideoHeight: NaturalVideoHeight::<Impl, IMPL_OFFSET>,
            NaturalVideoWidth: NaturalVideoWidth::<Impl, IMPL_OFFSET>,
            NormalizedSourceRect: NormalizedSourceRect::<Impl, IMPL_OFFSET>,
            SetNormalizedSourceRect: SetNormalizedSourceRect::<Impl, IMPL_OFFSET>,
            StereoscopicVideoPackingMode: StereoscopicVideoPackingMode::<Impl, IMPL_OFFSET>,
            SetStereoscopicVideoPackingMode: SetStereoscopicVideoPackingMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSession2_Impl: Sized {
    fn BufferedRangesChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferedRangesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayedRangesChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlayedRangesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekableRangesChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekableRangesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SupportedPlaybackRatesChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSupportedPlaybackRatesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SphericalVideoProjection(&mut self) -> ::windows::core::Result<MediaPlaybackSphericalVideoProjection>;
    fn IsMirroring(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsMirroring(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetBufferedRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn GetPlayedRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn GetSeekableRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>>;
    fn IsSupportedPlaybackRateRange(&mut self, rate1: f64, rate2: f64) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSession2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSession2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSession2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSession2_Vtbl {
        unsafe extern "system" fn BufferedRangesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBufferedRangesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferedRangesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayedRangesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlayedRangesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayedRangesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SeekableRangesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSeekableRangesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSeekableRangesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedPlaybackRatesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSupportedPlaybackRatesChanged<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSupportedPlaybackRatesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SphericalVideoProjection<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsMirroring<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsMirroring<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMirroring(value).into()
        }
        unsafe extern "system" fn GetBufferedRanges<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPlayedRanges<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSeekableRanges<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSupportedPlaybackRateRange<Impl: IMediaPlaybackSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate1: f64, rate2: f64, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSession2, BASE_OFFSET>(),
            BufferedRangesChanged: BufferedRangesChanged::<Impl, IMPL_OFFSET>,
            RemoveBufferedRangesChanged: RemoveBufferedRangesChanged::<Impl, IMPL_OFFSET>,
            PlayedRangesChanged: PlayedRangesChanged::<Impl, IMPL_OFFSET>,
            RemovePlayedRangesChanged: RemovePlayedRangesChanged::<Impl, IMPL_OFFSET>,
            SeekableRangesChanged: SeekableRangesChanged::<Impl, IMPL_OFFSET>,
            RemoveSeekableRangesChanged: RemoveSeekableRangesChanged::<Impl, IMPL_OFFSET>,
            SupportedPlaybackRatesChanged: SupportedPlaybackRatesChanged::<Impl, IMPL_OFFSET>,
            RemoveSupportedPlaybackRatesChanged: RemoveSupportedPlaybackRatesChanged::<Impl, IMPL_OFFSET>,
            SphericalVideoProjection: SphericalVideoProjection::<Impl, IMPL_OFFSET>,
            IsMirroring: IsMirroring::<Impl, IMPL_OFFSET>,
            SetIsMirroring: SetIsMirroring::<Impl, IMPL_OFFSET>,
            GetBufferedRanges: GetBufferedRanges::<Impl, IMPL_OFFSET>,
            GetPlayedRanges: GetPlayedRanges::<Impl, IMPL_OFFSET>,
            GetSeekableRanges: GetSeekableRanges::<Impl, IMPL_OFFSET>,
            IsSupportedPlaybackRateRange: IsSupportedPlaybackRateRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSession3_Impl: Sized {
    fn PlaybackRotation(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaRotation>;
    fn SetPlaybackRotation(&mut self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()>;
    fn GetOutputDegradationPolicyState(&mut self) -> ::windows::core::Result<MediaPlaybackSessionOutputDegradationPolicyState>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSession3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSession3";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaPlaybackSession3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSession3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSession3_Vtbl {
        unsafe extern "system" fn PlaybackRotation<Impl: IMediaPlaybackSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackRotation<Impl: IMediaPlaybackSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRotation(value).into()
        }
        unsafe extern "system" fn GetOutputDegradationPolicyState<Impl: IMediaPlaybackSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSession3, BASE_OFFSET>(),
            PlaybackRotation: PlaybackRotation::<Impl, IMPL_OFFSET>,
            SetPlaybackRotation: SetPlaybackRotation::<Impl, IMPL_OFFSET>,
            GetOutputDegradationPolicyState: GetOutputDegradationPolicyState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSession3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSessionBufferingStartedEventArgs_Impl: Sized {
    fn IsPlaybackInterruption(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackSessionBufferingStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSessionBufferingStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSessionBufferingStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl {
        unsafe extern "system" fn IsPlaybackInterruption<Impl: IMediaPlaybackSessionBufferingStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSessionBufferingStartedEventArgs, BASE_OFFSET>(),
            IsPlaybackInterruption: IsPlaybackInterruption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSessionBufferingStartedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlaybackSessionOutputDegradationPolicyState_Impl: Sized {
    fn VideoConstrictionReason(&mut self) -> ::windows::core::Result<MediaPlaybackSessionVideoConstrictionReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlaybackSessionOutputDegradationPolicyState {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSessionOutputDegradationPolicyState";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSessionOutputDegradationPolicyState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl {
        unsafe extern "system" fn VideoConstrictionReason<Impl: IMediaPlaybackSessionOutputDegradationPolicyState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackSessionVideoConstrictionReason) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSessionOutputDegradationPolicyState, BASE_OFFSET>(),
            VideoConstrictionReason: VideoConstrictionReason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSessionOutputDegradationPolicyState as ::windows::core::Interface>::IID
    }
}
pub trait IMediaPlaybackSource_Impl: Sized {}
impl ::windows::core::RuntimeName for IMediaPlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSource";
}
impl IMediaPlaybackSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaPlaybackSphericalVideoProjection_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn FrameFormat(&mut self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat>;
    fn SetFrameFormat(&mut self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()>;
    fn HorizontalFieldOfViewInDegrees(&mut self) -> ::windows::core::Result<f64>;
    fn SetHorizontalFieldOfViewInDegrees(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ViewOrientation(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetViewOrientation(&mut self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn ProjectionMode(&mut self) -> ::windows::core::Result<SphericalVideoProjectionMode>;
    fn SetProjectionMode(&mut self, value: SphericalVideoProjectionMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackSphericalVideoProjection {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSphericalVideoProjection";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaPlaybackSphericalVideoProjection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackSphericalVideoProjection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackSphericalVideoProjection_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn FrameFormat<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrameFormat<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameFormat(value).into()
        }
        unsafe extern "system" fn HorizontalFieldOfViewInDegrees<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHorizontalFieldOfViewInDegrees<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalFieldOfViewInDegrees(value).into()
        }
        unsafe extern "system" fn ViewOrientation<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewOrientation<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProjectionMode<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProjectionMode<Impl: IMediaPlaybackSphericalVideoProjection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProjectionMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackSphericalVideoProjection, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            FrameFormat: FrameFormat::<Impl, IMPL_OFFSET>,
            SetFrameFormat: SetFrameFormat::<Impl, IMPL_OFFSET>,
            HorizontalFieldOfViewInDegrees: HorizontalFieldOfViewInDegrees::<Impl, IMPL_OFFSET>,
            SetHorizontalFieldOfViewInDegrees: SetHorizontalFieldOfViewInDegrees::<Impl, IMPL_OFFSET>,
            ViewOrientation: ViewOrientation::<Impl, IMPL_OFFSET>,
            SetViewOrientation: SetViewOrientation::<Impl, IMPL_OFFSET>,
            ProjectionMode: ProjectionMode::<Impl, IMPL_OFFSET>,
            SetProjectionMode: SetProjectionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackSphericalVideoProjection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlaybackTimedMetadataTrackList_Impl: Sized {
    fn PresentationModeChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePresentationModeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetPresentationMode(&mut self, index: u32) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
    fn SetPresentationMode(&mut self, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlaybackTimedMetadataTrackList {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackTimedMetadataTrackList";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlaybackTimedMetadataTrackList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlaybackTimedMetadataTrackList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlaybackTimedMetadataTrackList_Vtbl {
        unsafe extern "system" fn PresentationModeChanged<Impl: IMediaPlaybackTimedMetadataTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePresentationModeChanged<Impl: IMediaPlaybackTimedMetadataTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePresentationModeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPresentationMode<Impl: IMediaPlaybackTimedMetadataTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPresentationMode<Impl: IMediaPlaybackTimedMetadataTrackList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentationMode(index, value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlaybackTimedMetadataTrackList, BASE_OFFSET>(),
            PresentationModeChanged: PresentationModeChanged::<Impl, IMPL_OFFSET>,
            RemovePresentationModeChanged: RemovePresentationModeChanged::<Impl, IMPL_OFFSET>,
            GetPresentationMode: GetPresentationMode::<Impl, IMPL_OFFSET>,
            SetPresentationMode: SetPresentationMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlaybackTimedMetadataTrackList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaPlayer_Impl: Sized {
    fn AutoPlay(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn NaturalDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPosition(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BufferingProgress(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentState(&mut self) -> ::windows::core::Result<MediaPlayerState>;
    fn CanSeek(&mut self) -> ::windows::core::Result<bool>;
    fn CanPause(&mut self) -> ::windows::core::Result<bool>;
    fn IsLoopingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsLoopingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsProtected(&mut self) -> ::windows::core::Result<bool>;
    fn IsMuted(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsMuted(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&mut self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&mut self) -> ::windows::core::Result<f64>;
    fn SetVolume(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackMediaMarkers(&mut self) -> ::windows::core::Result<PlaybackMediaMarkerSequence>;
    fn MediaOpened(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaOpened(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaEnded(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaEnded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaFailed(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaFailed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackMediaMarkerReached(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackMediaMarkerReached(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaPlayerRateChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaPlayerRateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VolumeChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVolumeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SeekCompleted(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSeekCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingStarted(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingStarted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BufferingEnded(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBufferingEnded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Play(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn SetUriSource(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaPlayer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer_Vtbl {
        unsafe extern "system" fn AutoPlay<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoPlay<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoPlay(value).into()
        }
        unsafe extern "system" fn NaturalDuration<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingProgress<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentState<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanSeek<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanPause<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsLoopingEnabled<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsLoopingEnabled<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLoopingEnabled(value).into()
        }
        unsafe extern "system" fn IsProtected<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsMuted<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsMuted<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsMuted(value).into()
        }
        unsafe extern "system" fn PlaybackRate<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackRate<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn Volume<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVolume<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        unsafe extern "system" fn PlaybackMediaMarkers<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaOpened<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMediaOpened<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaEnded<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMediaEnded<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaFailed<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMediaFailed<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentStateChanged<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCurrentStateChanged<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackMediaMarkerReached<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlaybackMediaMarkerReached<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackMediaMarkerReached(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaPlayerRateChanged<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMediaPlayerRateChanged<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMediaPlayerRateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VolumeChanged<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveVolumeChanged<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVolumeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SeekCompleted<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSeekCompleted<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSeekCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingStarted<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBufferingStarted<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferingEnded<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveBufferingEnded<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBufferingEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Play<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Pause<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn SetUriSource<Impl: IMediaPlayer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUriSource(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayer, BASE_OFFSET>(),
            AutoPlay: AutoPlay::<Impl, IMPL_OFFSET>,
            SetAutoPlay: SetAutoPlay::<Impl, IMPL_OFFSET>,
            NaturalDuration: NaturalDuration::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            BufferingProgress: BufferingProgress::<Impl, IMPL_OFFSET>,
            CurrentState: CurrentState::<Impl, IMPL_OFFSET>,
            CanSeek: CanSeek::<Impl, IMPL_OFFSET>,
            CanPause: CanPause::<Impl, IMPL_OFFSET>,
            IsLoopingEnabled: IsLoopingEnabled::<Impl, IMPL_OFFSET>,
            SetIsLoopingEnabled: SetIsLoopingEnabled::<Impl, IMPL_OFFSET>,
            IsProtected: IsProtected::<Impl, IMPL_OFFSET>,
            IsMuted: IsMuted::<Impl, IMPL_OFFSET>,
            SetIsMuted: SetIsMuted::<Impl, IMPL_OFFSET>,
            PlaybackRate: PlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate: SetPlaybackRate::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            PlaybackMediaMarkers: PlaybackMediaMarkers::<Impl, IMPL_OFFSET>,
            MediaOpened: MediaOpened::<Impl, IMPL_OFFSET>,
            RemoveMediaOpened: RemoveMediaOpened::<Impl, IMPL_OFFSET>,
            MediaEnded: MediaEnded::<Impl, IMPL_OFFSET>,
            RemoveMediaEnded: RemoveMediaEnded::<Impl, IMPL_OFFSET>,
            MediaFailed: MediaFailed::<Impl, IMPL_OFFSET>,
            RemoveMediaFailed: RemoveMediaFailed::<Impl, IMPL_OFFSET>,
            CurrentStateChanged: CurrentStateChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentStateChanged: RemoveCurrentStateChanged::<Impl, IMPL_OFFSET>,
            PlaybackMediaMarkerReached: PlaybackMediaMarkerReached::<Impl, IMPL_OFFSET>,
            RemovePlaybackMediaMarkerReached: RemovePlaybackMediaMarkerReached::<Impl, IMPL_OFFSET>,
            MediaPlayerRateChanged: MediaPlayerRateChanged::<Impl, IMPL_OFFSET>,
            RemoveMediaPlayerRateChanged: RemoveMediaPlayerRateChanged::<Impl, IMPL_OFFSET>,
            VolumeChanged: VolumeChanged::<Impl, IMPL_OFFSET>,
            RemoveVolumeChanged: RemoveVolumeChanged::<Impl, IMPL_OFFSET>,
            SeekCompleted: SeekCompleted::<Impl, IMPL_OFFSET>,
            RemoveSeekCompleted: RemoveSeekCompleted::<Impl, IMPL_OFFSET>,
            BufferingStarted: BufferingStarted::<Impl, IMPL_OFFSET>,
            RemoveBufferingStarted: RemoveBufferingStarted::<Impl, IMPL_OFFSET>,
            BufferingEnded: BufferingEnded::<Impl, IMPL_OFFSET>,
            RemoveBufferingEnded: RemoveBufferingEnded::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            SetUriSource: SetUriSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayer2_Impl: Sized {
    fn SystemMediaTransportControls(&mut self) -> ::windows::core::Result<super::SystemMediaTransportControls>;
    fn AudioCategory(&mut self) -> ::windows::core::Result<MediaPlayerAudioCategory>;
    fn SetAudioCategory(&mut self, value: MediaPlayerAudioCategory) -> ::windows::core::Result<()>;
    fn AudioDeviceType(&mut self) -> ::windows::core::Result<MediaPlayerAudioDeviceType>;
    fn SetAudioDeviceType(&mut self, value: MediaPlayerAudioDeviceType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayer2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer2_Vtbl {
        unsafe extern "system" fn SystemMediaTransportControls<Impl: IMediaPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AudioCategory<Impl: IMediaPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioCategory) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioCategory<Impl: IMediaPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioCategory(value).into()
        }
        unsafe extern "system" fn AudioDeviceType<Impl: IMediaPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioDeviceType<Impl: IMediaPlayer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioDeviceType(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayer2, BASE_OFFSET>(),
            SystemMediaTransportControls: SystemMediaTransportControls::<Impl, IMPL_OFFSET>,
            AudioCategory: AudioCategory::<Impl, IMPL_OFFSET>,
            SetAudioCategory: SetAudioCategory::<Impl, IMPL_OFFSET>,
            AudioDeviceType: AudioDeviceType::<Impl, IMPL_OFFSET>,
            SetAudioDeviceType: SetAudioDeviceType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Casting", feature = "implement_exclusive"))]
pub trait IMediaPlayer3_Impl: Sized {
    fn IsMutedChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsMutedChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioBalance(&mut self) -> ::windows::core::Result<f64>;
    fn SetAudioBalance(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn RealTimePlayback(&mut self) -> ::windows::core::Result<bool>;
    fn SetRealTimePlayback(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StereoscopicVideoRenderMode(&mut self) -> ::windows::core::Result<StereoscopicVideoRenderMode>;
    fn SetStereoscopicVideoRenderMode(&mut self, value: StereoscopicVideoRenderMode) -> ::windows::core::Result<()>;
    fn BreakManager(&mut self) -> ::windows::core::Result<MediaBreakManager>;
    fn CommandManager(&mut self) -> ::windows::core::Result<MediaPlaybackCommandManager>;
    fn AudioDevice(&mut self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
    fn SetAudioDevice(&mut self, value: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<()>;
    fn TimelineController(&mut self) -> ::windows::core::Result<super::MediaTimelineController>;
    fn SetTimelineController(&mut self, value: &::core::option::Option<super::MediaTimelineController>) -> ::windows::core::Result<()>;
    fn TimelineControllerPositionOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimelineControllerPositionOffset(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PlaybackSession(&mut self) -> ::windows::core::Result<MediaPlaybackSession>;
    fn StepForwardOneFrame(&mut self) -> ::windows::core::Result<()>;
    fn StepBackwardOneFrame(&mut self) -> ::windows::core::Result<()>;
    fn GetAsCastingSource(&mut self) -> ::windows::core::Result<super::Casting::CastingSource>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Casting", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer3";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Casting", feature = "implement_exclusive"))]
impl IMediaPlayer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer3_Vtbl {
        unsafe extern "system" fn IsMutedChanged<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveIsMutedChanged<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsMutedChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceChanged<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSourceChanged<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioBalance<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioBalance<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioBalance(value).into()
        }
        unsafe extern "system" fn RealTimePlayback<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRealTimePlayback<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRealTimePlayback(value).into()
        }
        unsafe extern "system" fn StereoscopicVideoRenderMode<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoRenderMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStereoscopicVideoRenderMode<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StereoscopicVideoRenderMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStereoscopicVideoRenderMode(value).into()
        }
        unsafe extern "system" fn BreakManager<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CommandManager<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AudioDevice<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioDevice<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioDevice(&*(&value as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimelineController<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTimelineController<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimelineController(&*(&value as *const <super::MediaTimelineController as ::windows::core::Abi>::Abi as *const <super::MediaTimelineController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimelineControllerPositionOffset<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTimelineControllerPositionOffset<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimelineControllerPositionOffset(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackSession<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StepForwardOneFrame<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StepForwardOneFrame().into()
        }
        unsafe extern "system" fn StepBackwardOneFrame<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StepBackwardOneFrame().into()
        }
        unsafe extern "system" fn GetAsCastingSource<Impl: IMediaPlayer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayer3, BASE_OFFSET>(),
            IsMutedChanged: IsMutedChanged::<Impl, IMPL_OFFSET>,
            RemoveIsMutedChanged: RemoveIsMutedChanged::<Impl, IMPL_OFFSET>,
            SourceChanged: SourceChanged::<Impl, IMPL_OFFSET>,
            RemoveSourceChanged: RemoveSourceChanged::<Impl, IMPL_OFFSET>,
            AudioBalance: AudioBalance::<Impl, IMPL_OFFSET>,
            SetAudioBalance: SetAudioBalance::<Impl, IMPL_OFFSET>,
            RealTimePlayback: RealTimePlayback::<Impl, IMPL_OFFSET>,
            SetRealTimePlayback: SetRealTimePlayback::<Impl, IMPL_OFFSET>,
            StereoscopicVideoRenderMode: StereoscopicVideoRenderMode::<Impl, IMPL_OFFSET>,
            SetStereoscopicVideoRenderMode: SetStereoscopicVideoRenderMode::<Impl, IMPL_OFFSET>,
            BreakManager: BreakManager::<Impl, IMPL_OFFSET>,
            CommandManager: CommandManager::<Impl, IMPL_OFFSET>,
            AudioDevice: AudioDevice::<Impl, IMPL_OFFSET>,
            SetAudioDevice: SetAudioDevice::<Impl, IMPL_OFFSET>,
            TimelineController: TimelineController::<Impl, IMPL_OFFSET>,
            SetTimelineController: SetTimelineController::<Impl, IMPL_OFFSET>,
            TimelineControllerPositionOffset: TimelineControllerPositionOffset::<Impl, IMPL_OFFSET>,
            SetTimelineControllerPositionOffset: SetTimelineControllerPositionOffset::<Impl, IMPL_OFFSET>,
            PlaybackSession: PlaybackSession::<Impl, IMPL_OFFSET>,
            StepForwardOneFrame: StepForwardOneFrame::<Impl, IMPL_OFFSET>,
            StepBackwardOneFrame: StepBackwardOneFrame::<Impl, IMPL_OFFSET>,
            GetAsCastingSource: GetAsCastingSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IMediaPlayer4_Impl: Sized {
    fn SetSurfaceSize(&mut self, size: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn GetSurface(&mut self, compositor: &::core::option::Option<super::super::UI::Composition::Compositor>) -> ::windows::core::Result<MediaPlayerSurface>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer4 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer4";
}
#[cfg(all(feature = "Foundation", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IMediaPlayer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer4_Vtbl {
        unsafe extern "system" fn SetSurfaceSize<Impl: IMediaPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSurfaceSize(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSurface<Impl: IMediaPlayer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayer4, BASE_OFFSET>(),
            SetSurfaceSize: SetSurfaceSize::<Impl, IMPL_OFFSET>,
            GetSurface: GetSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaPlayer5_Impl: Sized {
    fn VideoFrameAvailable(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameAvailable(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsVideoFrameServerEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsVideoFrameServerEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CopyFrameToVideoSurface(&mut self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
    fn CopyFrameToVideoSurfaceWithTargetRectangle(&mut self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, targetrectangle: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn CopyFrameToStereoscopicVideoSurfaces(&mut self, destinationlefteye: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, destinationrighteye: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer5 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer5";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaPlayer5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer5_Vtbl {
        unsafe extern "system" fn VideoFrameAvailable<Impl: IMediaPlayer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveVideoFrameAvailable<Impl: IMediaPlayer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVideoFrameAvailable(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsVideoFrameServerEnabled<Impl: IMediaPlayer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsVideoFrameServerEnabled<Impl: IMediaPlayer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVideoFrameServerEnabled(value).into()
        }
        unsafe extern "system" fn CopyFrameToVideoSurface<Impl: IMediaPlayer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyFrameToVideoSurface(&*(&destination as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyFrameToVideoSurfaceWithTargetRectangle<Impl: IMediaPlayer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, targetrectangle: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyFrameToVideoSurfaceWithTargetRectangle(&*(&destination as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), &*(&targetrectangle as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CopyFrameToStereoscopicVideoSurfaces<Impl: IMediaPlayer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationlefteye: ::windows::core::RawPtr, destinationrighteye: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CopyFrameToStereoscopicVideoSurfaces(
                    &*(&destinationlefteye as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType),
                    &*(&destinationrighteye as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayer5, BASE_OFFSET>(),
            VideoFrameAvailable: VideoFrameAvailable::<Impl, IMPL_OFFSET>,
            RemoveVideoFrameAvailable: RemoveVideoFrameAvailable::<Impl, IMPL_OFFSET>,
            IsVideoFrameServerEnabled: IsVideoFrameServerEnabled::<Impl, IMPL_OFFSET>,
            SetIsVideoFrameServerEnabled: SetIsVideoFrameServerEnabled::<Impl, IMPL_OFFSET>,
            CopyFrameToVideoSurface: CopyFrameToVideoSurface::<Impl, IMPL_OFFSET>,
            CopyFrameToVideoSurfaceWithTargetRectangle: CopyFrameToVideoSurfaceWithTargetRectangle::<Impl, IMPL_OFFSET>,
            CopyFrameToStereoscopicVideoSurfaces: CopyFrameToStereoscopicVideoSurfaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaPlayer6_Impl: Sized {
    fn SubtitleFrameChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubtitleFrameChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RenderSubtitlesToSurface(&mut self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<bool>;
    fn RenderSubtitlesToSurfaceWithTargetRectangle(&mut self, destination: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, targetrectangle: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer6 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer6";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaPlayer6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer6_Vtbl {
        unsafe extern "system" fn SubtitleFrameChanged<Impl: IMediaPlayer6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSubtitleFrameChanged<Impl: IMediaPlayer6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubtitleFrameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RenderSubtitlesToSurface<Impl: IMediaPlayer6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenderSubtitlesToSurfaceWithTargetRectangle<Impl: IMediaPlayer6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, targetrectangle: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayer6, BASE_OFFSET>(),
            SubtitleFrameChanged: SubtitleFrameChanged::<Impl, IMPL_OFFSET>,
            RemoveSubtitleFrameChanged: RemoveSubtitleFrameChanged::<Impl, IMPL_OFFSET>,
            RenderSubtitlesToSurface: RenderSubtitlesToSurface::<Impl, IMPL_OFFSET>,
            RenderSubtitlesToSurfaceWithTargetRectangle: RenderSubtitlesToSurfaceWithTargetRectangle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Audio", feature = "implement_exclusive"))]
pub trait IMediaPlayer7_Impl: Sized {
    fn AudioStateMonitor(&mut self) -> ::windows::core::Result<super::Audio::AudioStateMonitor>;
}
#[cfg(all(feature = "Media_Audio", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayer7 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer7";
}
#[cfg(all(feature = "Media_Audio", feature = "implement_exclusive"))]
impl IMediaPlayer7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayer7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayer7_Vtbl {
        unsafe extern "system" fn AudioStateMonitor<Impl: IMediaPlayer7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayer7, BASE_OFFSET>(),
            AudioStateMonitor: AudioStateMonitor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayer7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlayerDataReceivedEventArgs_Impl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerDataReceivedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlayerDataReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerDataReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerDataReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Data<Impl: IMediaPlayerDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerDataReceivedEventArgs, BASE_OFFSET>(), Data: Data::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlayerEffects_Impl: Sized {
    fn AddAudioEffect(&mut self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RemoveAllEffects(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerEffects {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerEffects";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlayerEffects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerEffects_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerEffects_Vtbl {
        unsafe extern "system" fn AddAudioEffect<Impl: IMediaPlayerEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectoptional: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAudioEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectoptional, &*(&configuration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAllEffects<Impl: IMediaPlayerEffects_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllEffects().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerEffects, BASE_OFFSET>(),
            AddAudioEffect: AddAudioEffect::<Impl, IMPL_OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaPlayerEffects2_Impl: Sized {
    fn AddVideoEffect(&mut self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, effectconfiguration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerEffects2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerEffects2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaPlayerEffects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerEffects2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerEffects2_Vtbl {
        unsafe extern "system" fn AddVideoEffect<Impl: IMediaPlayerEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectoptional: bool, effectconfiguration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddVideoEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectoptional, &*(&effectconfiguration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerEffects2, BASE_OFFSET>(),
            AddVideoEffect: AddVideoEffect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerEffects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerFailedEventArgs_Impl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<MediaPlayerError>;
    fn ExtendedErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ErrorMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerFailedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerFailedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerFailedEventArgs_Vtbl {
        unsafe extern "system" fn Error<Impl: IMediaPlayerFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerError) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedErrorCode<Impl: IMediaPlayerFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ErrorMessage<Impl: IMediaPlayerFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerFailedEventArgs, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            ExtendedErrorCode: ExtendedErrorCode::<Impl, IMPL_OFFSET>,
            ErrorMessage: ErrorMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerRateChangedEventArgs_Impl: Sized {
    fn NewRate(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerRateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerRateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerRateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerRateChangedEventArgs_Vtbl {
        unsafe extern "system" fn NewRate<Impl: IMediaPlayerRateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerRateChangedEventArgs, BASE_OFFSET>(), NewRate: NewRate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerRateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "Media_Protection", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaPlayerSource_Impl: Sized {
    fn ProtectionManager(&mut self) -> ::windows::core::Result<super::Protection::MediaProtectionManager>;
    fn SetProtectionManager(&mut self, value: &::core::option::Option<super::Protection::MediaProtectionManager>) -> ::windows::core::Result<()>;
    fn SetFileSource(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
    fn SetStreamSource(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetMediaSource(&mut self, source: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_Core", feature = "Media_Protection", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSource";
}
#[cfg(all(feature = "Media_Core", feature = "Media_Protection", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaPlayerSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerSource_Vtbl {
        unsafe extern "system" fn ProtectionManager<Impl: IMediaPlayerSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProtectionManager<Impl: IMediaPlayerSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectionManager(&*(&value as *const <super::Protection::MediaProtectionManager as ::windows::core::Abi>::Abi as *const <super::Protection::MediaProtectionManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFileSource<Impl: IMediaPlayerSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileSource(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStreamSource<Impl: IMediaPlayerSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamSource(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMediaSource<Impl: IMediaPlayerSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaSource(&*(&source as *const <super::Core::IMediaSource as ::windows::core::Abi>::Abi as *const <super::Core::IMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerSource, BASE_OFFSET>(),
            ProtectionManager: ProtectionManager::<Impl, IMPL_OFFSET>,
            SetProtectionManager: SetProtectionManager::<Impl, IMPL_OFFSET>,
            SetFileSource: SetFileSource::<Impl, IMPL_OFFSET>,
            SetStreamSource: SetStreamSource::<Impl, IMPL_OFFSET>,
            SetMediaSource: SetMediaSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerSource2_Impl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<IMediaPlaybackSource>;
    fn SetSource(&mut self, value: &::core::option::Option<IMediaPlaybackSource>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerSource2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerSource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerSource2_Vtbl {
        unsafe extern "system" fn Source<Impl: IMediaPlayerSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: IMediaPlayerSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <IMediaPlaybackSource as ::windows::core::Abi>::Abi as *const <IMediaPlaybackSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerSource2, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IMediaPlayerSurface_Impl: Sized {
    fn CompositionSurface(&mut self) -> ::windows::core::Result<super::super::UI::Composition::ICompositionSurface>;
    fn Compositor(&mut self) -> ::windows::core::Result<super::super::UI::Composition::Compositor>;
    fn MediaPlayer(&mut self) -> ::windows::core::Result<MediaPlayer>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerSurface {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSurface";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IMediaPlayerSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerSurface_Vtbl {
        unsafe extern "system" fn CompositionSurface<Impl: IMediaPlayerSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Compositor<Impl: IMediaPlayerSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaPlayer<Impl: IMediaPlayerSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerSurface, BASE_OFFSET>(),
            CompositionSurface: CompositionSurface::<Impl, IMPL_OFFSET>,
            Compositor: Compositor::<Impl, IMPL_OFFSET>,
            MediaPlayer: MediaPlayer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerSurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPlaybackMediaMarker_Impl: Sized {
    fn Time(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MediaMarkerType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackMediaMarker {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarker";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPlaybackMediaMarker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarker_Vtbl {
        unsafe extern "system" fn Time<Impl: IPlaybackMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaMarkerType<Impl: IPlaybackMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IPlaybackMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaybackMediaMarker, BASE_OFFSET>(),
            Time: Time::<Impl, IMPL_OFFSET>,
            MediaMarkerType: MediaMarkerType::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPlaybackMediaMarkerFactory_Impl: Sized {
    fn CreateFromTime(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<PlaybackMediaMarker>;
    fn Create(&mut self, value: &super::super::Foundation::TimeSpan, mediamarkettype: &::windows::core::HSTRING, text: &::windows::core::HSTRING) -> ::windows::core::Result<PlaybackMediaMarker>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackMediaMarkerFactory {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarkerFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPlaybackMediaMarkerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarkerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarkerFactory_Vtbl {
        unsafe extern "system" fn CreateFromTime<Impl: IPlaybackMediaMarkerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Create<Impl: IPlaybackMediaMarkerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan, mediamarkettype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaybackMediaMarkerFactory, BASE_OFFSET>(),
            CreateFromTime: CreateFromTime::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarkerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackMediaMarkerReachedEventArgs_Impl: Sized {
    fn PlaybackMediaMarker(&mut self) -> ::windows::core::Result<PlaybackMediaMarker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarkerReachedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaybackMediaMarkerReachedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarkerReachedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarkerReachedEventArgs_Vtbl {
        unsafe extern "system" fn PlaybackMediaMarker<Impl: IPlaybackMediaMarkerReachedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaybackMediaMarkerReachedEventArgs, BASE_OFFSET>(),
            PlaybackMediaMarker: PlaybackMediaMarker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarkerReachedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPlaybackMediaMarkerSequence_Impl: Sized + super::super::Foundation::Collections::IIterable_Impl<PlaybackMediaMarker> {
    fn Size(&mut self) -> ::windows::core::Result<u32>;
    fn Insert(&mut self, value: &::core::option::Option<PlaybackMediaMarker>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackMediaMarkerSequence {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarkerSequence";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPlaybackMediaMarkerSequence_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackMediaMarkerSequence_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackMediaMarkerSequence_Vtbl {
        unsafe extern "system" fn Size<Impl: IPlaybackMediaMarkerSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Insert<Impl: IPlaybackMediaMarkerSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert(&*(&value as *const <PlaybackMediaMarker as ::windows::core::Abi>::Abi as *const <PlaybackMediaMarker as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPlaybackMediaMarkerSequence_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaybackMediaMarkerSequence, BASE_OFFSET>(),
            Size: Size::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackMediaMarkerSequence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait ITimedMetadataPresentationModeChangedEventArgs_Impl: Sized {
    fn Track(&mut self) -> ::windows::core::Result<super::Core::TimedMetadataTrack>;
    fn OldPresentationMode(&mut self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
    fn NewPresentationMode(&mut self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode>;
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedMetadataPresentationModeChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.ITimedMetadataPresentationModeChangedEventArgs";
}
#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
impl ITimedMetadataPresentationModeChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataPresentationModeChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataPresentationModeChangedEventArgs_Vtbl {
        unsafe extern "system" fn Track<Impl: ITimedMetadataPresentationModeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OldPresentationMode<Impl: ITimedMetadataPresentationModeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewPresentationMode<Impl: ITimedMetadataPresentationModeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataPresentationModeChangedEventArgs, BASE_OFFSET>(),
            Track: Track::<Impl, IMPL_OFFSET>,
            OldPresentationMode: OldPresentationMode::<Impl, IMPL_OFFSET>,
            NewPresentationMode: NewPresentationMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataPresentationModeChangedEventArgs as ::windows::core::Interface>::IID
    }
}
