#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioBuffer_Impl: Sized + super::Foundation::IClosable_Impl + super::Foundation::IMemoryBuffer_Impl {
    fn Capacity(&mut self) -> ::windows::core::Result<u32>;
    fn Length(&mut self) -> ::windows::core::Result<u32>;
    fn SetLength(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioBuffer {
    const NAME: &'static str = "Windows.Media.IAudioBuffer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioBuffer_Vtbl {
        unsafe extern "system" fn Capacity<Impl: IAudioBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IAudioBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: IAudioBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioBuffer, BASE_OFFSET>(),
            Capacity: Capacity::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAudioFrame_Impl: Sized + super::Foundation::IClosable_Impl + IMediaFrame_Impl {
    fn LockBuffer(&mut self, mode: AudioBufferAccessMode) -> ::windows::core::Result<AudioBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioFrame {
    const NAME: &'static str = "Windows.Media.IAudioFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAudioFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioFrame_Vtbl {
        unsafe extern "system" fn LockBuffer<Impl: IAudioFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: AudioBufferAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockBuffer(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioFrame, BASE_OFFSET>(), LockBuffer: LockBuffer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioFrameFactory_Impl: Sized {
    fn Create(&mut self, capacity: u32) -> ::windows::core::Result<AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioFrameFactory {
    const NAME: &'static str = "Windows.Media.IAudioFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioFrameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioFrameFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IAudioFrameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(capacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioFrameFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoRepeatModeChangeRequestedEventArgs_Impl: Sized {
    fn RequestedAutoRepeatMode(&mut self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutoRepeatModeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IAutoRepeatModeChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAutoRepeatModeChangeRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoRepeatModeChangeRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoRepeatModeChangeRequestedEventArgs_Vtbl {
        unsafe extern "system" fn RequestedAutoRepeatMode<Impl: IAutoRepeatModeChangeRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedAutoRepeatMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutoRepeatModeChangeRequestedEventArgs, BASE_OFFSET>(),
            RequestedAutoRepeatMode: RequestedAutoRepeatMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutoRepeatModeChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageDisplayProperties_Impl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageDisplayProperties {
    const NAME: &'static str = "Windows.Media.IImageDisplayProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IImageDisplayProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageDisplayProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageDisplayProperties_Vtbl {
        unsafe extern "system" fn Title<Impl: IImageDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IImageDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtitle<Impl: IImageDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubtitle<Impl: IImageDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubtitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageDisplayProperties, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Subtitle: Subtitle::<Impl, IMPL_OFFSET>,
            SetSubtitle: SetSubtitle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IMediaControl_Impl: Sized {
    fn SoundLevelChanged(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlayPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PausePressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePausePressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StopPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayPauseTogglePressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlayPauseTogglePressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecordPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRecordPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NextTrackPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveNextTrackPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviousTrackPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePreviousTrackPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FastForwardPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveFastForwardPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RewindPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRewindPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChannelUpPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveChannelUpPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChannelDownPressed(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveChannelDownPressed(&mut self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SoundLevel(&mut self) -> ::windows::core::Result<SoundLevel>;
    fn SetTrackName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TrackName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArtistName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ArtistName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIsPlaying(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlaying(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlbumArt(&mut self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AlbumArt(&mut self) -> ::windows::core::Result<super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaControl {
    const NAME: &'static str = "Windows.Media.IMediaControl";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IMediaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaControl_Vtbl {
        unsafe extern "system" fn SoundLevelChanged<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoundLevelChanged(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSoundLevelChanged<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSoundLevelChanged(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlayPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PausePressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PausePressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePausePressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePausePressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayPauseTogglePressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayPauseTogglePressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlayPauseTogglePressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayPauseTogglePressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecordPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecordPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecordPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NextTrackPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextTrackPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNextTrackPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNextTrackPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviousTrackPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousTrackPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviousTrackPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviousTrackPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FastForwardPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FastForwardPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFastForwardPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFastForwardPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RewindPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RewindPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRewindPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRewindPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChannelUpPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelUpPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChannelUpPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChannelUpPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChannelDownPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelDownPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChannelDownPressed<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChannelDownPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SoundLevel<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoundLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrackName<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrackName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackName<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArtistName<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArtistName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ArtistName<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArtistName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPlaying<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPlaying(value).into()
        }
        unsafe extern "system" fn IsPlaying<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPlaying() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumArt<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumArt(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlbumArt<Impl: IMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlbumArt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaControl, BASE_OFFSET>(),
            SoundLevelChanged: SoundLevelChanged::<Impl, IMPL_OFFSET>,
            RemoveSoundLevelChanged: RemoveSoundLevelChanged::<Impl, IMPL_OFFSET>,
            PlayPressed: PlayPressed::<Impl, IMPL_OFFSET>,
            RemovePlayPressed: RemovePlayPressed::<Impl, IMPL_OFFSET>,
            PausePressed: PausePressed::<Impl, IMPL_OFFSET>,
            RemovePausePressed: RemovePausePressed::<Impl, IMPL_OFFSET>,
            StopPressed: StopPressed::<Impl, IMPL_OFFSET>,
            RemoveStopPressed: RemoveStopPressed::<Impl, IMPL_OFFSET>,
            PlayPauseTogglePressed: PlayPauseTogglePressed::<Impl, IMPL_OFFSET>,
            RemovePlayPauseTogglePressed: RemovePlayPauseTogglePressed::<Impl, IMPL_OFFSET>,
            RecordPressed: RecordPressed::<Impl, IMPL_OFFSET>,
            RemoveRecordPressed: RemoveRecordPressed::<Impl, IMPL_OFFSET>,
            NextTrackPressed: NextTrackPressed::<Impl, IMPL_OFFSET>,
            RemoveNextTrackPressed: RemoveNextTrackPressed::<Impl, IMPL_OFFSET>,
            PreviousTrackPressed: PreviousTrackPressed::<Impl, IMPL_OFFSET>,
            RemovePreviousTrackPressed: RemovePreviousTrackPressed::<Impl, IMPL_OFFSET>,
            FastForwardPressed: FastForwardPressed::<Impl, IMPL_OFFSET>,
            RemoveFastForwardPressed: RemoveFastForwardPressed::<Impl, IMPL_OFFSET>,
            RewindPressed: RewindPressed::<Impl, IMPL_OFFSET>,
            RemoveRewindPressed: RemoveRewindPressed::<Impl, IMPL_OFFSET>,
            ChannelUpPressed: ChannelUpPressed::<Impl, IMPL_OFFSET>,
            RemoveChannelUpPressed: RemoveChannelUpPressed::<Impl, IMPL_OFFSET>,
            ChannelDownPressed: ChannelDownPressed::<Impl, IMPL_OFFSET>,
            RemoveChannelDownPressed: RemoveChannelDownPressed::<Impl, IMPL_OFFSET>,
            SoundLevel: SoundLevel::<Impl, IMPL_OFFSET>,
            SetTrackName: SetTrackName::<Impl, IMPL_OFFSET>,
            TrackName: TrackName::<Impl, IMPL_OFFSET>,
            SetArtistName: SetArtistName::<Impl, IMPL_OFFSET>,
            ArtistName: ArtistName::<Impl, IMPL_OFFSET>,
            SetIsPlaying: SetIsPlaying::<Impl, IMPL_OFFSET>,
            IsPlaying: IsPlaying::<Impl, IMPL_OFFSET>,
            SetAlbumArt: SetAlbumArt::<Impl, IMPL_OFFSET>,
            AlbumArt: AlbumArt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaExtension_Impl: Sized {
    fn SetProperties(&mut self, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaExtension {
    const NAME: &'static str = "Windows.Media.IMediaExtension";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaExtension_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaExtension_Vtbl {
        unsafe extern "system" fn SetProperties<Impl: IMediaExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperties(&*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaExtension, BASE_OFFSET>(), SetProperties: SetProperties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaExtensionManager_Impl: Sized {
    fn RegisterSchemeHandler(&mut self, activatableclassid: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RegisterSchemeHandlerWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterByteStreamHandler(&mut self, activatableclassid: &::windows::core::HSTRING, fileextension: &::windows::core::HSTRING, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RegisterByteStreamHandlerWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, fileextension: &::windows::core::HSTRING, mimetype: &::windows::core::HSTRING, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterAudioDecoder(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioDecoderWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterAudioEncoder(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioEncoderWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterVideoDecoder(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterVideoDecoderWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterVideoEncoder(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterVideoEncoderWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaExtensionManager {
    const NAME: &'static str = "Windows.Media.IMediaExtensionManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaExtensionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaExtensionManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaExtensionManager_Vtbl {
        unsafe extern "system" fn RegisterSchemeHandler<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterSchemeHandler(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterSchemeHandlerWithSettings<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterSchemeHandlerWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterByteStreamHandler<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterByteStreamHandler(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&fileextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterByteStreamHandlerWithSettings<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterByteStreamHandlerWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&fileextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioDecoder<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioDecoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioDecoderWithSettings<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioDecoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioEncoder<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioEncoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioEncoderWithSettings<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioEncoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoDecoder<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoDecoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoDecoderWithSettings<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoDecoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoEncoder<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoEncoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoEncoderWithSettings<Impl: IMediaExtensionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoEncoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaExtensionManager, BASE_OFFSET>(),
            RegisterSchemeHandler: RegisterSchemeHandler::<Impl, IMPL_OFFSET>,
            RegisterSchemeHandlerWithSettings: RegisterSchemeHandlerWithSettings::<Impl, IMPL_OFFSET>,
            RegisterByteStreamHandler: RegisterByteStreamHandler::<Impl, IMPL_OFFSET>,
            RegisterByteStreamHandlerWithSettings: RegisterByteStreamHandlerWithSettings::<Impl, IMPL_OFFSET>,
            RegisterAudioDecoder: RegisterAudioDecoder::<Impl, IMPL_OFFSET>,
            RegisterAudioDecoderWithSettings: RegisterAudioDecoderWithSettings::<Impl, IMPL_OFFSET>,
            RegisterAudioEncoder: RegisterAudioEncoder::<Impl, IMPL_OFFSET>,
            RegisterAudioEncoderWithSettings: RegisterAudioEncoderWithSettings::<Impl, IMPL_OFFSET>,
            RegisterVideoDecoder: RegisterVideoDecoder::<Impl, IMPL_OFFSET>,
            RegisterVideoDecoderWithSettings: RegisterVideoDecoderWithSettings::<Impl, IMPL_OFFSET>,
            RegisterVideoEncoder: RegisterVideoEncoder::<Impl, IMPL_OFFSET>,
            RegisterVideoEncoderWithSettings: RegisterVideoEncoderWithSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaExtensionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaExtensionManager2_Impl: Sized + IMediaExtensionManager_Impl {
    fn RegisterMediaExtensionForAppService(&mut self, extension: &::core::option::Option<IMediaExtension>, connection: &::core::option::Option<super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaExtensionManager2 {
    const NAME: &'static str = "Windows.Media.IMediaExtensionManager2";
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaExtensionManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaExtensionManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaExtensionManager2_Vtbl {
        unsafe extern "system" fn RegisterMediaExtensionForAppService<Impl: IMediaExtensionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extension: ::windows::core::RawPtr, connection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterMediaExtensionForAppService(&*(&extension as *const <IMediaExtension as ::windows::core::Abi>::Abi as *const <IMediaExtension as ::windows::core::DefaultType>::DefaultType), &*(&connection as *const <super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::Abi>::Abi as *const <super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaExtensionManager2, BASE_OFFSET>(),
            RegisterMediaExtensionForAppService: RegisterMediaExtensionForAppService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaExtensionManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IMediaFrame_Impl: Sized + super::Foundation::IClosable_Impl {
    fn Type(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsReadOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetRelativeTime(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn RelativeTime(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetSystemRelativeTime(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SystemRelativeTime(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetIsDiscontinuous(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDiscontinuous(&mut self) -> ::windows::core::Result<bool>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IMediaFrame {
    const NAME: &'static str = "Windows.Media.IMediaFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IMediaFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrame_Vtbl {
        unsafe extern "system" fn Type<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsReadOnly<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemRelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsDiscontinuous<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDiscontinuous(value).into()
        }
        unsafe extern "system" fn IsDiscontinuous<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDiscontinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IMediaFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrame, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            SetRelativeTime: SetRelativeTime::<Impl, IMPL_OFFSET>,
            RelativeTime: RelativeTime::<Impl, IMPL_OFFSET>,
            SetSystemRelativeTime: SetSystemRelativeTime::<Impl, IMPL_OFFSET>,
            SystemRelativeTime: SystemRelativeTime::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetIsDiscontinuous: SetIsDiscontinuous::<Impl, IMPL_OFFSET>,
            IsDiscontinuous: IsDiscontinuous::<Impl, IMPL_OFFSET>,
            ExtendedProperties: ExtendedProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IMediaMarker_Impl: Sized {
    fn Time(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn MediaMarkerType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IMediaMarker {
    const NAME: &'static str = "Windows.Media.IMediaMarker";
}
#[cfg(feature = "Foundation")]
impl IMediaMarker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarker_Vtbl {
        unsafe extern "system" fn Time<Impl: IMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaMarkerType<Impl: IMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IMediaMarker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaMarker, BASE_OFFSET>(),
            Time: Time::<Impl, IMPL_OFFSET>,
            MediaMarkerType: MediaMarkerType::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaMarkerTypesStatics_Impl: Sized {
    fn Bookmark(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaMarkerTypesStatics {
    const NAME: &'static str = "Windows.Media.IMediaMarkerTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaMarkerTypesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarkerTypesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarkerTypesStatics_Vtbl {
        unsafe extern "system" fn Bookmark<Impl: IMediaMarkerTypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bookmark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaMarkerTypesStatics, BASE_OFFSET>(), Bookmark: Bookmark::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarkerTypesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaMarkers_Impl: Sized {
    fn Markers(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaMarkers {
    const NAME: &'static str = "Windows.Media.IMediaMarkers";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaMarkers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarkers_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarkers_Vtbl {
        unsafe extern "system" fn Markers<Impl: IMediaMarkers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Markers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaMarkers, BASE_OFFSET>(), Markers: Markers::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarkers as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaProcessingTriggerDetails_Impl: Sized {
    fn Arguments(&mut self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.IMediaProcessingTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaProcessingTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProcessingTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProcessingTriggerDetails_Vtbl {
        unsafe extern "system" fn Arguments<Impl: IMediaProcessingTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProcessingTriggerDetails, BASE_OFFSET>(),
            Arguments: Arguments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProcessingTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaTimelineController_Impl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetPosition(&mut self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ClockRate(&mut self) -> ::windows::core::Result<f64>;
    fn SetClockRate(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<MediaTimelineControllerState>;
    fn PositionChanged(&mut self, positionchangedeventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&mut self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&mut self, statechangedeventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTimelineController {
    const NAME: &'static str = "Windows.Media.IMediaTimelineController";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaTimelineController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTimelineController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTimelineController_Vtbl {
        unsafe extern "system" fn Start<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Resume<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Pause<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Position<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClockRate<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClockRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClockRate<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClockRate(value).into()
        }
        unsafe extern "system" fn State<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaTimelineControllerState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionChanged<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positionchangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionChanged(&*(&positionchangedeventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionChanged<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&eventcookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StateChanged<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statechangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&statechangedeventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IMediaTimelineController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&eventcookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTimelineController, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            ClockRate: ClockRate::<Impl, IMPL_OFFSET>,
            SetClockRate: SetClockRate::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            PositionChanged: PositionChanged::<Impl, IMPL_OFFSET>,
            RemovePositionChanged: RemovePositionChanged::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTimelineController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaTimelineController2_Impl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn IsLoopingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsLoopingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Failed(&mut self, eventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Ended(&mut self, eventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnded(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTimelineController2 {
    const NAME: &'static str = "Windows.Media.IMediaTimelineController2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaTimelineController2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTimelineController2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTimelineController2_Vtbl {
        unsafe extern "system" fn Duration<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsLoopingEnabled<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsLoopingEnabled<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLoopingEnabled(value).into()
        }
        unsafe extern "system" fn Failed<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Failed(&*(&eventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFailed<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFailed(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ended<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ended(&*(&eventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnded<Impl: IMediaTimelineController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTimelineController2, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            IsLoopingEnabled: IsLoopingEnabled::<Impl, IMPL_OFFSET>,
            SetIsLoopingEnabled: SetIsLoopingEnabled::<Impl, IMPL_OFFSET>,
            Failed: Failed::<Impl, IMPL_OFFSET>,
            RemoveFailed: RemoveFailed::<Impl, IMPL_OFFSET>,
            Ended: Ended::<Impl, IMPL_OFFSET>,
            RemoveEnded: RemoveEnded::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTimelineController2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTimelineControllerFailedEventArgs_Impl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTimelineControllerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.IMediaTimelineControllerFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTimelineControllerFailedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTimelineControllerFailedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTimelineControllerFailedEventArgs_Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IMediaTimelineControllerFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTimelineControllerFailedEventArgs, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTimelineControllerFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayProperties_Impl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AlbumArtist(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumArtist(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Artist(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArtist(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMusicDisplayProperties {
    const NAME: &'static str = "Windows.Media.IMusicDisplayProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IMusicDisplayProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMusicDisplayProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMusicDisplayProperties_Vtbl {
        unsafe extern "system" fn Title<Impl: IMusicDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IMusicDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlbumArtist<Impl: IMusicDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlbumArtist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumArtist<Impl: IMusicDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumArtist(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Artist<Impl: IMusicDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Artist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArtist<Impl: IMusicDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArtist(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMusicDisplayProperties, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            AlbumArtist: AlbumArtist::<Impl, IMPL_OFFSET>,
            SetAlbumArtist: SetAlbumArtist::<Impl, IMPL_OFFSET>,
            Artist: Artist::<Impl, IMPL_OFFSET>,
            SetArtist: SetArtist::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMusicDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMusicDisplayProperties2_Impl: Sized {
    fn AlbumTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TrackNumber(&mut self) -> ::windows::core::Result<u32>;
    fn SetTrackNumber(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Genres(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMusicDisplayProperties2 {
    const NAME: &'static str = "Windows.Media.IMusicDisplayProperties2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMusicDisplayProperties2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMusicDisplayProperties2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMusicDisplayProperties2_Vtbl {
        unsafe extern "system" fn AlbumTitle<Impl: IMusicDisplayProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlbumTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumTitle<Impl: IMusicDisplayProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackNumber<Impl: IMusicDisplayProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrackNumber<Impl: IMusicDisplayProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrackNumber(value).into()
        }
        unsafe extern "system" fn Genres<Impl: IMusicDisplayProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Genres() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMusicDisplayProperties2, BASE_OFFSET>(),
            AlbumTitle: AlbumTitle::<Impl, IMPL_OFFSET>,
            SetAlbumTitle: SetAlbumTitle::<Impl, IMPL_OFFSET>,
            TrackNumber: TrackNumber::<Impl, IMPL_OFFSET>,
            SetTrackNumber: SetTrackNumber::<Impl, IMPL_OFFSET>,
            Genres: Genres::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMusicDisplayProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayProperties3_Impl: Sized {
    fn AlbumTrackCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetAlbumTrackCount(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMusicDisplayProperties3 {
    const NAME: &'static str = "Windows.Media.IMusicDisplayProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl IMusicDisplayProperties3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMusicDisplayProperties3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMusicDisplayProperties3_Vtbl {
        unsafe extern "system" fn AlbumTrackCount<Impl: IMusicDisplayProperties3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlbumTrackCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumTrackCount<Impl: IMusicDisplayProperties3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumTrackCount(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMusicDisplayProperties3, BASE_OFFSET>(),
            AlbumTrackCount: AlbumTrackCount::<Impl, IMPL_OFFSET>,
            SetAlbumTrackCount: SetAlbumTrackCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMusicDisplayProperties3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPlaybackPositionChangeRequestedEventArgs_Impl: Sized {
    fn RequestedPlaybackPosition(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IPlaybackPositionChangeRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPlaybackPositionChangeRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackPositionChangeRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackPositionChangeRequestedEventArgs_Vtbl {
        unsafe extern "system" fn RequestedPlaybackPosition<Impl: IPlaybackPositionChangeRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedPlaybackPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaybackPositionChangeRequestedEventArgs, BASE_OFFSET>(),
            RequestedPlaybackPosition: RequestedPlaybackPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackPositionChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackRateChangeRequestedEventArgs_Impl: Sized {
    fn RequestedPlaybackRate(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IPlaybackRateChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackRateChangeRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackRateChangeRequestedEventArgs_Vtbl {
        unsafe extern "system" fn RequestedPlaybackRate<Impl: IPlaybackRateChangeRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlaybackRateChangeRequestedEventArgs, BASE_OFFSET>(),
            RequestedPlaybackRate: RequestedPlaybackRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackRateChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShuffleEnabledChangeRequestedEventArgs_Impl: Sized {
    fn RequestedShuffleEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShuffleEnabledChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IShuffleEnabledChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IShuffleEnabledChangeRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShuffleEnabledChangeRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShuffleEnabledChangeRequestedEventArgs_Vtbl {
        unsafe extern "system" fn RequestedShuffleEnabled<Impl: IShuffleEnabledChangeRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedShuffleEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShuffleEnabledChangeRequestedEventArgs, BASE_OFFSET>(),
            RequestedShuffleEnabled: RequestedShuffleEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShuffleEnabledChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemMediaTransportControls_Impl: Sized {
    fn PlaybackStatus(&mut self) -> ::windows::core::Result<MediaPlaybackStatus>;
    fn SetPlaybackStatus(&mut self, value: MediaPlaybackStatus) -> ::windows::core::Result<()>;
    fn DisplayUpdater(&mut self) -> ::windows::core::Result<SystemMediaTransportControlsDisplayUpdater>;
    fn SoundLevel(&mut self) -> ::windows::core::Result<SoundLevel>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlayEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPlayEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsStopEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsStopEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsPauseEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPauseEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRecordEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRecordEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsFastForwardEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsFastForwardEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRewindEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRewindEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsPreviousEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsPreviousEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsNextEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsNextEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsChannelUpEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsChannelUpEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsChannelDownEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsChannelDownEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ButtonPressed(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveButtonPressed(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertyChanged(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControls";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemMediaTransportControls_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControls_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControls_Vtbl {
        unsafe extern "system" fn PlaybackStatus<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackStatus<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackStatus(value).into()
        }
        unsafe extern "system" fn DisplayUpdater<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUpdater() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoundLevel<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoundLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn IsPlayEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPlayEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPlayEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPlayEnabled(value).into()
        }
        unsafe extern "system" fn IsStopEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStopEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStopEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStopEnabled(value).into()
        }
        unsafe extern "system" fn IsPauseEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPauseEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPauseEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPauseEnabled(value).into()
        }
        unsafe extern "system" fn IsRecordEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRecordEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRecordEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRecordEnabled(value).into()
        }
        unsafe extern "system" fn IsFastForwardEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFastForwardEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFastForwardEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFastForwardEnabled(value).into()
        }
        unsafe extern "system" fn IsRewindEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRewindEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRewindEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRewindEnabled(value).into()
        }
        unsafe extern "system" fn IsPreviousEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPreviousEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPreviousEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPreviousEnabled(value).into()
        }
        unsafe extern "system" fn IsNextEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNextEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsNextEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsNextEnabled(value).into()
        }
        unsafe extern "system" fn IsChannelUpEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsChannelUpEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsChannelUpEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsChannelUpEnabled(value).into()
        }
        unsafe extern "system" fn IsChannelDownEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsChannelDownEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsChannelDownEnabled<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsChannelDownEnabled(value).into()
        }
        unsafe extern "system" fn ButtonPressed<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonPressed(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonPressed<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveButtonPressed(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PropertyChanged<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChanged<Impl: ISystemMediaTransportControls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertyChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControls, BASE_OFFSET>(),
            PlaybackStatus: PlaybackStatus::<Impl, IMPL_OFFSET>,
            SetPlaybackStatus: SetPlaybackStatus::<Impl, IMPL_OFFSET>,
            DisplayUpdater: DisplayUpdater::<Impl, IMPL_OFFSET>,
            SoundLevel: SoundLevel::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            IsPlayEnabled: IsPlayEnabled::<Impl, IMPL_OFFSET>,
            SetIsPlayEnabled: SetIsPlayEnabled::<Impl, IMPL_OFFSET>,
            IsStopEnabled: IsStopEnabled::<Impl, IMPL_OFFSET>,
            SetIsStopEnabled: SetIsStopEnabled::<Impl, IMPL_OFFSET>,
            IsPauseEnabled: IsPauseEnabled::<Impl, IMPL_OFFSET>,
            SetIsPauseEnabled: SetIsPauseEnabled::<Impl, IMPL_OFFSET>,
            IsRecordEnabled: IsRecordEnabled::<Impl, IMPL_OFFSET>,
            SetIsRecordEnabled: SetIsRecordEnabled::<Impl, IMPL_OFFSET>,
            IsFastForwardEnabled: IsFastForwardEnabled::<Impl, IMPL_OFFSET>,
            SetIsFastForwardEnabled: SetIsFastForwardEnabled::<Impl, IMPL_OFFSET>,
            IsRewindEnabled: IsRewindEnabled::<Impl, IMPL_OFFSET>,
            SetIsRewindEnabled: SetIsRewindEnabled::<Impl, IMPL_OFFSET>,
            IsPreviousEnabled: IsPreviousEnabled::<Impl, IMPL_OFFSET>,
            SetIsPreviousEnabled: SetIsPreviousEnabled::<Impl, IMPL_OFFSET>,
            IsNextEnabled: IsNextEnabled::<Impl, IMPL_OFFSET>,
            SetIsNextEnabled: SetIsNextEnabled::<Impl, IMPL_OFFSET>,
            IsChannelUpEnabled: IsChannelUpEnabled::<Impl, IMPL_OFFSET>,
            SetIsChannelUpEnabled: SetIsChannelUpEnabled::<Impl, IMPL_OFFSET>,
            IsChannelDownEnabled: IsChannelDownEnabled::<Impl, IMPL_OFFSET>,
            SetIsChannelDownEnabled: SetIsChannelDownEnabled::<Impl, IMPL_OFFSET>,
            ButtonPressed: ButtonPressed::<Impl, IMPL_OFFSET>,
            RemoveButtonPressed: RemoveButtonPressed::<Impl, IMPL_OFFSET>,
            PropertyChanged: PropertyChanged::<Impl, IMPL_OFFSET>,
            RemovePropertyChanged: RemovePropertyChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControls as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemMediaTransportControls2_Impl: Sized {
    fn AutoRepeatMode(&mut self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode>;
    fn SetAutoRepeatMode(&mut self, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::Result<()>;
    fn ShuffleEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetShuffleEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&mut self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn UpdateTimelineProperties(&mut self, timelineproperties: &::core::option::Option<SystemMediaTransportControlsTimelineProperties>) -> ::windows::core::Result<()>;
    fn PlaybackPositionChangeRequested(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackPositionChangeRequested(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChangeRequested(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChangeRequested(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShuffleEnabledChangeRequested(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShuffleEnabledChangeRequested(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoRepeatModeChangeRequested(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAutoRepeatModeChangeRequested(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControls2 {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControls2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemMediaTransportControls2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControls2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControls2_Vtbl {
        unsafe extern "system" fn AutoRepeatMode<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoRepeatMode<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoRepeatMode(value).into()
        }
        unsafe extern "system" fn ShuffleEnabled<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShuffleEnabled<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShuffleEnabled(value).into()
        }
        unsafe extern "system" fn PlaybackRate<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackRate<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn UpdateTimelineProperties<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timelineproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateTimelineProperties(&*(&timelineproperties as *const <SystemMediaTransportControlsTimelineProperties as ::windows::core::Abi>::Abi as *const <SystemMediaTransportControlsTimelineProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackPositionChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackPositionChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackPositionChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackPositionChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackRateChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackRateChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackRateChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackRateChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShuffleEnabledChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShuffleEnabledChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShuffleEnabledChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShuffleEnabledChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoRepeatModeChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoRepeatModeChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutoRepeatModeChangeRequested<Impl: ISystemMediaTransportControls2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutoRepeatModeChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControls2, BASE_OFFSET>(),
            AutoRepeatMode: AutoRepeatMode::<Impl, IMPL_OFFSET>,
            SetAutoRepeatMode: SetAutoRepeatMode::<Impl, IMPL_OFFSET>,
            ShuffleEnabled: ShuffleEnabled::<Impl, IMPL_OFFSET>,
            SetShuffleEnabled: SetShuffleEnabled::<Impl, IMPL_OFFSET>,
            PlaybackRate: PlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate: SetPlaybackRate::<Impl, IMPL_OFFSET>,
            UpdateTimelineProperties: UpdateTimelineProperties::<Impl, IMPL_OFFSET>,
            PlaybackPositionChangeRequested: PlaybackPositionChangeRequested::<Impl, IMPL_OFFSET>,
            RemovePlaybackPositionChangeRequested: RemovePlaybackPositionChangeRequested::<Impl, IMPL_OFFSET>,
            PlaybackRateChangeRequested: PlaybackRateChangeRequested::<Impl, IMPL_OFFSET>,
            RemovePlaybackRateChangeRequested: RemovePlaybackRateChangeRequested::<Impl, IMPL_OFFSET>,
            ShuffleEnabledChangeRequested: ShuffleEnabledChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveShuffleEnabledChangeRequested: RemoveShuffleEnabledChangeRequested::<Impl, IMPL_OFFSET>,
            AutoRepeatModeChangeRequested: AutoRepeatModeChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveAutoRepeatModeChangeRequested: RemoveAutoRepeatModeChangeRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControls2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsButtonPressedEventArgs_Impl: Sized {
    fn Button(&mut self) -> ::windows::core::Result<SystemMediaTransportControlsButton>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsButtonPressedEventArgs {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsButtonPressedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl {
        unsafe extern "system" fn Button<Impl: ISystemMediaTransportControlsButtonPressedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsButton) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Button() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControlsButtonPressedEventArgs, BASE_OFFSET>(),
            Button: Button::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsButtonPressedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISystemMediaTransportControlsDisplayUpdater_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<MediaPlaybackType>;
    fn SetType(&mut self, value: MediaPlaybackType) -> ::windows::core::Result<()>;
    fn AppMediaId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppMediaId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn MusicProperties(&mut self) -> ::windows::core::Result<MusicDisplayProperties>;
    fn VideoProperties(&mut self) -> ::windows::core::Result<VideoDisplayProperties>;
    fn ImageProperties(&mut self) -> ::windows::core::Result<ImageDisplayProperties>;
    fn CopyFromFileAsync(&mut self, r#type: MediaPlaybackType, source: &::core::option::Option<super::Storage::StorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn ClearAll(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsDisplayUpdater";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISystemMediaTransportControlsDisplayUpdater_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsDisplayUpdater_Vtbl {
        unsafe extern "system" fn Type<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn AppMediaId<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMediaId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppMediaId<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppMediaId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MusicProperties<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VideoProperties<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ImageProperties<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromFileAsync<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: MediaPlaybackType, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyFromFileAsync(r#type, &*(&source as *const <super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearAll<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAll().into()
        }
        unsafe extern "system" fn Update<Impl: ISystemMediaTransportControlsDisplayUpdater_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControlsDisplayUpdater, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            AppMediaId: AppMediaId::<Impl, IMPL_OFFSET>,
            SetAppMediaId: SetAppMediaId::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            MusicProperties: MusicProperties::<Impl, IMPL_OFFSET>,
            VideoProperties: VideoProperties::<Impl, IMPL_OFFSET>,
            ImageProperties: ImageProperties::<Impl, IMPL_OFFSET>,
            CopyFromFileAsync: CopyFromFileAsync::<Impl, IMPL_OFFSET>,
            ClearAll: ClearAll::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsDisplayUpdater as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsPropertyChangedEventArgs_Impl: Sized {
    fn Property(&mut self) -> ::windows::core::Result<SystemMediaTransportControlsProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsPropertyChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl {
        unsafe extern "system" fn Property<Impl: ISystemMediaTransportControlsPropertyChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControlsPropertyChangedEventArgs, BASE_OFFSET>(),
            Property: Property::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsPropertyChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<SystemMediaTransportControls>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsStatics {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISystemMediaTransportControlsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControlsStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemMediaTransportControlsTimelineProperties_Impl: Sized {
    fn StartTime(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetStartTime(&mut self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn EndTime(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetEndTime(&mut self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MinSeekTime(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetMinSeekTime(&mut self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MaxSeekTime(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetMaxSeekTime(&mut self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetPosition(&mut self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsTimelineProperties";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemMediaTransportControlsTimelineProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsTimelineProperties_Vtbl {
        unsafe extern "system" fn StartTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStartTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinSeekTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinSeekTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinSeekTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinSeekTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSeekTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSeekTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSeekTime<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSeekTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: ISystemMediaTransportControlsTimelineProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControlsTimelineProperties, BASE_OFFSET>(),
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            EndTime: EndTime::<Impl, IMPL_OFFSET>,
            SetEndTime: SetEndTime::<Impl, IMPL_OFFSET>,
            MinSeekTime: MinSeekTime::<Impl, IMPL_OFFSET>,
            SetMinSeekTime: SetMinSeekTime::<Impl, IMPL_OFFSET>,
            MaxSeekTime: MaxSeekTime::<Impl, IMPL_OFFSET>,
            SetMaxSeekTime: SetMaxSeekTime::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsTimelineProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDisplayProperties_Impl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoDisplayProperties {
    const NAME: &'static str = "Windows.Media.IVideoDisplayProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoDisplayProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoDisplayProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoDisplayProperties_Vtbl {
        unsafe extern "system" fn Title<Impl: IVideoDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IVideoDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtitle<Impl: IVideoDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubtitle<Impl: IVideoDisplayProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubtitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoDisplayProperties, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Subtitle: Subtitle::<Impl, IMPL_OFFSET>,
            SetSubtitle: SetSubtitle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVideoDisplayProperties2_Impl: Sized {
    fn Genres(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoDisplayProperties2 {
    const NAME: &'static str = "Windows.Media.IVideoDisplayProperties2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVideoDisplayProperties2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoDisplayProperties2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoDisplayProperties2_Vtbl {
        unsafe extern "system" fn Genres<Impl: IVideoDisplayProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Genres() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoDisplayProperties2, BASE_OFFSET>(), Genres: Genres::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoDisplayProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEffectsStatics_Impl: Sized {
    fn VideoStabilization(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEffectsStatics {
    const NAME: &'static str = "Windows.Media.IVideoEffectsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEffectsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEffectsStatics_Vtbl {
        unsafe extern "system" fn VideoStabilization<Impl: IVideoEffectsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoStabilization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEffectsStatics, BASE_OFFSET>(),
            VideoStabilization: VideoStabilization::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEffectsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrame_Impl: Sized + super::Foundation::IClosable_Impl + IMediaFrame_Impl {
    fn SoftwareBitmap(&mut self) -> ::windows::core::Result<super::Graphics::Imaging::SoftwareBitmap>;
    fn CopyToAsync(&mut self, frame: &::core::option::Option<VideoFrame>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn Direct3DSurface(&mut self) -> ::windows::core::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrame {
    const NAME: &'static str = "Windows.Media.IVideoFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrame_Vtbl {
        unsafe extern "system" fn SoftwareBitmap<Impl: IVideoFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToAsync<Impl: IVideoFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyToAsync(&*(&frame as *const <VideoFrame as ::windows::core::Abi>::Abi as *const <VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direct3DSurface<Impl: IVideoFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direct3DSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoFrame, BASE_OFFSET>(),
            SoftwareBitmap: SoftwareBitmap::<Impl, IMPL_OFFSET>,
            CopyToAsync: CopyToAsync::<Impl, IMPL_OFFSET>,
            Direct3DSurface: Direct3DSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrame2_Impl: Sized {
    fn CopyToWithBoundsAsync(&mut self, frame: &::core::option::Option<VideoFrame>, sourcebounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>, destinationbounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrame2 {
    const NAME: &'static str = "Windows.Media.IVideoFrame2";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrame2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrame2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrame2_Vtbl {
        unsafe extern "system" fn CopyToWithBoundsAsync<Impl: IVideoFrame2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, sourcebounds: ::windows::core::RawPtr, destinationbounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyToWithBoundsAsync(
                &*(&frame as *const <VideoFrame as ::windows::core::Abi>::Abi as *const <VideoFrame as ::windows::core::DefaultType>::DefaultType),
                &*(&sourcebounds as *const <super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds> as ::windows::core::DefaultType>::DefaultType),
                &*(&destinationbounds as *const <super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds> as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoFrame2, BASE_OFFSET>(),
            CopyToWithBoundsAsync: CopyToWithBoundsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrame2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrameFactory_Impl: Sized {
    fn Create(&mut self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithAlpha(&mut self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<VideoFrame>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrameFactory {
    const NAME: &'static str = "Windows.Media.IVideoFrameFactory";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrameFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IVideoFrameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAlpha<Impl: IVideoFrameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAlpha(format, width, height, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoFrameFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithAlpha: CreateWithAlpha::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrameStatics_Impl: Sized {
    fn CreateAsDirect3D11SurfaceBacked(&mut self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateAsDirect3D11SurfaceBackedWithDevice(&mut self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithSoftwareBitmap(&mut self, bitmap: &::core::option::Option<super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithDirect3D11Surface(&mut self, surface: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<VideoFrame>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrameStatics {
    const NAME: &'static str = "Windows.Media.IVideoFrameStatics";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrameStatics_Vtbl {
        unsafe extern "system" fn CreateAsDirect3D11SurfaceBacked<Impl: IVideoFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsDirect3D11SurfaceBacked(format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsDirect3D11SurfaceBackedWithDevice<Impl: IVideoFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsDirect3D11SurfaceBackedWithDevice(format, width, height, &*(&device as *const <super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithSoftwareBitmap<Impl: IVideoFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithSoftwareBitmap(&*(&bitmap as *const <super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithDirect3D11Surface<Impl: IVideoFrameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithDirect3D11Surface(&*(&surface as *const <super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoFrameStatics, BASE_OFFSET>(),
            CreateAsDirect3D11SurfaceBacked: CreateAsDirect3D11SurfaceBacked::<Impl, IMPL_OFFSET>,
            CreateAsDirect3D11SurfaceBackedWithDevice: CreateAsDirect3D11SurfaceBackedWithDevice::<Impl, IMPL_OFFSET>,
            CreateWithSoftwareBitmap: CreateWithSoftwareBitmap::<Impl, IMPL_OFFSET>,
            CreateWithDirect3D11Surface: CreateWithDirect3D11Surface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameStatics as ::windows::core::Interface>::IID
    }
}
