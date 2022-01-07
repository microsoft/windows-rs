#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioBufferImpl: Sized + IClosableImpl + IMemoryBufferImpl {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioBuffer {
    const NAME: &'static str = "Windows.Media.IAudioBuffer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioBufferVtbl {
    pub const fn new<Impl: IAudioBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioBufferVtbl {
        unsafe extern "system" fn Capacity<Impl: IAudioBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IAudioBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: IAudioBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioBuffer>, base.5, Capacity::<Impl, OFFSET>, Length::<Impl, OFFSET>, SetLength::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn LockBuffer(&self, mode: AudioBufferAccessMode) -> ::windows::core::Result<AudioBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioFrame {
    const NAME: &'static str = "Windows.Media.IAudioFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioFrameVtbl {
    pub const fn new<Impl: IAudioFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFrameVtbl {
        unsafe extern "system" fn LockBuffer<Impl: IAudioFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: AudioBufferAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockBuffer(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFrame>, base.5, LockBuffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioFrameFactoryImpl: Sized {
    fn Create(&self, capacity: u32) -> ::windows::core::Result<AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioFrameFactory {
    const NAME: &'static str = "Windows.Media.IAudioFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioFrameFactoryVtbl {
    pub const fn new<Impl: IAudioFrameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFrameFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioFrameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(capacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFrameFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoRepeatModeChangeRequestedEventArgsImpl: Sized {
    fn RequestedAutoRepeatMode(&self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutoRepeatModeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IAutoRepeatModeChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAutoRepeatModeChangeRequestedEventArgsVtbl {
    pub const fn new<Impl: IAutoRepeatModeChangeRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutoRepeatModeChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedAutoRepeatMode<Impl: IAutoRepeatModeChangeRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestedAutoRepeatMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutoRepeatModeChangeRequestedEventArgs>, base.5, RequestedAutoRepeatMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageDisplayPropertiesImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageDisplayProperties {
    const NAME: &'static str = "Windows.Media.IImageDisplayProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IImageDisplayPropertiesVtbl {
    pub const fn new<Impl: IImageDisplayPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageDisplayPropertiesVtbl {
        unsafe extern "system" fn Title<Impl: IImageDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IImageDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtitle<Impl: IImageDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subtitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubtitle<Impl: IImageDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSubtitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageDisplayProperties>, base.5, Title::<Impl, OFFSET>, SetTitle::<Impl, OFFSET>, Subtitle::<Impl, OFFSET>, SetSubtitle::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IMediaControlImpl: Sized {
    fn SoundLevelChanged(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlayPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PausePressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePausePressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StopPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlayPauseTogglePressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlayPauseTogglePressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecordPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRecordPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NextTrackPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveNextTrackPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviousTrackPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePreviousTrackPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FastForwardPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveFastForwardPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RewindPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRewindPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChannelUpPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveChannelUpPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ChannelDownPressed(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveChannelDownPressed(&self, cookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SoundLevel(&self) -> ::windows::core::Result<SoundLevel>;
    fn SetTrackName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TrackName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArtistName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ArtistName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIsPlaying(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlaying(&self) -> ::windows::core::Result<bool>;
    fn SetAlbumArt(&self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AlbumArt(&self) -> ::windows::core::Result<super::Foundation::Uri>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaControl {
    const NAME: &'static str = "Windows.Media.IMediaControl";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IMediaControlVtbl {
    pub const fn new<Impl: IMediaControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaControlVtbl {
        unsafe extern "system" fn SoundLevelChanged<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoundLevelChanged(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSoundLevelChanged<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSoundLevelChanged(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlayPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlayPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePlayPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PausePressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PausePressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePausePressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePausePressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStopPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayPauseTogglePressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlayPauseTogglePressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlayPauseTogglePressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePlayPauseTogglePressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecordPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecordPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecordPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRecordPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NextTrackPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextTrackPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNextTrackPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNextTrackPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviousTrackPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviousTrackPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviousTrackPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePreviousTrackPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FastForwardPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FastForwardPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFastForwardPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFastForwardPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RewindPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RewindPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRewindPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRewindPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChannelUpPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChannelUpPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChannelUpPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveChannelUpPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChannelDownPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChannelDownPressed(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChannelDownPressed<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveChannelDownPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SoundLevel<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoundLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrackName<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTrackName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackName<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrackName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArtistName<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetArtistName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ArtistName<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ArtistName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPlaying<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPlaying(value).into()
        }
        unsafe extern "system" fn IsPlaying<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPlaying() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumArt<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlbumArt(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlbumArt<Impl: IMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlbumArt() {
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
            ::windows::core::GetRuntimeClassName::<IMediaControl>,
            base.5,
            SoundLevelChanged::<Impl, OFFSET>,
            RemoveSoundLevelChanged::<Impl, OFFSET>,
            PlayPressed::<Impl, OFFSET>,
            RemovePlayPressed::<Impl, OFFSET>,
            PausePressed::<Impl, OFFSET>,
            RemovePausePressed::<Impl, OFFSET>,
            StopPressed::<Impl, OFFSET>,
            RemoveStopPressed::<Impl, OFFSET>,
            PlayPauseTogglePressed::<Impl, OFFSET>,
            RemovePlayPauseTogglePressed::<Impl, OFFSET>,
            RecordPressed::<Impl, OFFSET>,
            RemoveRecordPressed::<Impl, OFFSET>,
            NextTrackPressed::<Impl, OFFSET>,
            RemoveNextTrackPressed::<Impl, OFFSET>,
            PreviousTrackPressed::<Impl, OFFSET>,
            RemovePreviousTrackPressed::<Impl, OFFSET>,
            FastForwardPressed::<Impl, OFFSET>,
            RemoveFastForwardPressed::<Impl, OFFSET>,
            RewindPressed::<Impl, OFFSET>,
            RemoveRewindPressed::<Impl, OFFSET>,
            ChannelUpPressed::<Impl, OFFSET>,
            RemoveChannelUpPressed::<Impl, OFFSET>,
            ChannelDownPressed::<Impl, OFFSET>,
            RemoveChannelDownPressed::<Impl, OFFSET>,
            SoundLevel::<Impl, OFFSET>,
            SetTrackName::<Impl, OFFSET>,
            TrackName::<Impl, OFFSET>,
            SetArtistName::<Impl, OFFSET>,
            ArtistName::<Impl, OFFSET>,
            SetIsPlaying::<Impl, OFFSET>,
            IsPlaying::<Impl, OFFSET>,
            SetAlbumArt::<Impl, OFFSET>,
            AlbumArt::<Impl, OFFSET>,
        )
    }
}
pub trait IMediaExtensionImpl: Sized {
    fn SetProperties(&self, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMediaExtension {
    const NAME: &'static str = "Windows.Media.IMediaExtension";
}
impl IMediaExtensionVtbl {
    pub const fn new<Impl: IMediaExtensionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaExtensionVtbl {
        unsafe extern "system" fn SetProperties<Impl: IMediaExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProperties(&*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaExtension>, base.5, SetProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaExtensionManagerImpl: Sized {
    fn RegisterSchemeHandler(&self, activatableclassid: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RegisterSchemeHandlerWithSettings(&self, activatableclassid: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterByteStreamHandler(&self, activatableclassid: &::windows::core::HSTRING, fileextension: &::windows::core::HSTRING, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RegisterByteStreamHandlerWithSettings(&self, activatableclassid: &::windows::core::HSTRING, fileextension: &::windows::core::HSTRING, mimetype: &::windows::core::HSTRING, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterAudioDecoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioDecoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterAudioEncoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioEncoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterVideoDecoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterVideoDecoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn RegisterVideoEncoder(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterVideoEncoderWithSettings(&self, activatableclassid: &::windows::core::HSTRING, inputsubtype: &::windows::core::GUID, outputsubtype: &::windows::core::GUID, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaExtensionManager {
    const NAME: &'static str = "Windows.Media.IMediaExtensionManager";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaExtensionManagerVtbl {
    pub const fn new<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaExtensionManagerVtbl {
        unsafe extern "system" fn RegisterSchemeHandler<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterSchemeHandler(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterSchemeHandlerWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterSchemeHandlerWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterByteStreamHandler<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterByteStreamHandler(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&fileextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterByteStreamHandlerWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterByteStreamHandlerWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&fileextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioDecoder<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioDecoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioDecoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioDecoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioEncoder<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioEncoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioEncoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioEncoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoDecoder<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoDecoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoDecoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoDecoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoEncoder<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoEncoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoEncoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoEncoderWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMediaExtensionManager>,
            base.5,
            RegisterSchemeHandler::<Impl, OFFSET>,
            RegisterSchemeHandlerWithSettings::<Impl, OFFSET>,
            RegisterByteStreamHandler::<Impl, OFFSET>,
            RegisterByteStreamHandlerWithSettings::<Impl, OFFSET>,
            RegisterAudioDecoder::<Impl, OFFSET>,
            RegisterAudioDecoderWithSettings::<Impl, OFFSET>,
            RegisterAudioEncoder::<Impl, OFFSET>,
            RegisterAudioEncoderWithSettings::<Impl, OFFSET>,
            RegisterVideoDecoder::<Impl, OFFSET>,
            RegisterVideoDecoderWithSettings::<Impl, OFFSET>,
            RegisterVideoEncoder::<Impl, OFFSET>,
            RegisterVideoEncoderWithSettings::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaExtensionManager2Impl: Sized + IMediaExtensionManagerImpl {
    fn RegisterMediaExtensionForAppService(&self, extension: &::core::option::Option<IMediaExtension>, connection: &::core::option::Option<super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaExtensionManager2 {
    const NAME: &'static str = "Windows.Media.IMediaExtensionManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaExtensionManager2Vtbl {
    pub const fn new<Impl: IMediaExtensionManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaExtensionManager2Vtbl {
        unsafe extern "system" fn RegisterMediaExtensionForAppService<Impl: IMediaExtensionManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extension: ::windows::core::RawPtr, connection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterMediaExtensionForAppService(&*(&extension as *const <IMediaExtension as ::windows::core::Abi>::Abi as *const <IMediaExtension as ::windows::core::DefaultType>::DefaultType), &*(&connection as *const <super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::Abi>::Abi as *const <super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaExtensionManager2>, base.5, RegisterMediaExtensionForAppService::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IMediaFrameImpl: Sized + IClosableImpl {
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SetRelativeTime(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn RelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetSystemRelativeTime(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SystemRelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetIsDiscontinuous(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDiscontinuous(&self) -> ::windows::core::Result<bool>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IMediaFrame {
    const NAME: &'static str = "Windows.Media.IMediaFrame";
}
#[cfg(feature = "Foundation")]
impl IMediaFrameVtbl {
    pub const fn new<Impl: IMediaFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameVtbl {
        unsafe extern "system" fn Type<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTime<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeTime<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemRelativeTime<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSystemRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDiscontinuous<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsDiscontinuous(value).into()
        }
        unsafe extern "system" fn IsDiscontinuous<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDiscontinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrame>, base.5, Type::<Impl, OFFSET>, IsReadOnly::<Impl, OFFSET>, SetRelativeTime::<Impl, OFFSET>, RelativeTime::<Impl, OFFSET>, SetSystemRelativeTime::<Impl, OFFSET>, SystemRelativeTime::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, Duration::<Impl, OFFSET>, SetIsDiscontinuous::<Impl, OFFSET>, IsDiscontinuous::<Impl, OFFSET>, ExtendedProperties::<Impl, OFFSET>)
    }
}
pub trait IMediaMarkerImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaMarker {
    const NAME: &'static str = "Windows.Media.IMediaMarker";
}
impl IMediaMarkerVtbl {
    pub const fn new<Impl: IMediaMarkerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaMarkerVtbl {
        unsafe extern "system" fn Time<Impl: IMediaMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Time() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaMarkerType<Impl: IMediaMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaMarkerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IMediaMarkerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaMarker>, base.5, Time::<Impl, OFFSET>, MediaMarkerType::<Impl, OFFSET>, Text::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaMarkerTypesStaticsImpl: Sized {
    fn Bookmark(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaMarkerTypesStatics {
    const NAME: &'static str = "Windows.Media.IMediaMarkerTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaMarkerTypesStaticsVtbl {
    pub const fn new<Impl: IMediaMarkerTypesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaMarkerTypesStaticsVtbl {
        unsafe extern "system" fn Bookmark<Impl: IMediaMarkerTypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bookmark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaMarkerTypesStatics>, base.5, Bookmark::<Impl, OFFSET>)
    }
}
pub trait IMediaMarkersImpl: Sized {
    fn Markers(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>>;
}
impl ::windows::core::RuntimeName for IMediaMarkers {
    const NAME: &'static str = "Windows.Media.IMediaMarkers";
}
impl IMediaMarkersVtbl {
    pub const fn new<Impl: IMediaMarkersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaMarkersVtbl {
        unsafe extern "system" fn Markers<Impl: IMediaMarkersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Markers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaMarkers>, base.5, Markers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProcessingTriggerDetailsImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.IMediaProcessingTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaProcessingTriggerDetailsVtbl {
    pub const fn new<Impl: IMediaProcessingTriggerDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaProcessingTriggerDetailsVtbl {
        unsafe extern "system" fn Arguments<Impl: IMediaProcessingTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaProcessingTriggerDetails>, base.5, Arguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTimelineControllerImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ClockRate(&self) -> ::windows::core::Result<f64>;
    fn SetClockRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<MediaTimelineControllerState>;
    fn PositionChanged(&self, positionchangedeventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&self, statechangedeventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTimelineController {
    const NAME: &'static str = "Windows.Media.IMediaTimelineController";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTimelineControllerVtbl {
    pub const fn new<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTimelineControllerVtbl {
        unsafe extern "system" fn Start<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Resume<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Pause<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Position<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClockRate<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClockRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClockRate<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClockRate(value).into()
        }
        unsafe extern "system" fn State<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaTimelineControllerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positionchangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionChanged(&*(&positionchangedeventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&eventcookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StateChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statechangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&statechangedeventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&eventcookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTimelineController>, base.5, Start::<Impl, OFFSET>, Resume::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>, ClockRate::<Impl, OFFSET>, SetClockRate::<Impl, OFFSET>, State::<Impl, OFFSET>, PositionChanged::<Impl, OFFSET>, RemovePositionChanged::<Impl, OFFSET>, StateChanged::<Impl, OFFSET>, RemoveStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTimelineController2Impl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn IsLoopingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Failed(&self, eventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Ended(&self, eventhandler: &::core::option::Option<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTimelineController2 {
    const NAME: &'static str = "Windows.Media.IMediaTimelineController2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTimelineController2Vtbl {
    pub const fn new<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTimelineController2Vtbl {
        unsafe extern "system" fn Duration<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsLoopingEnabled<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLoopingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLoopingEnabled<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsLoopingEnabled(value).into()
        }
        unsafe extern "system" fn Failed<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Failed(&*(&eventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFailed<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFailed(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ended<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ended(&*(&eventhandler as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnded<Impl: IMediaTimelineController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTimelineController2>, base.5, Duration::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, IsLoopingEnabled::<Impl, OFFSET>, SetIsLoopingEnabled::<Impl, OFFSET>, Failed::<Impl, OFFSET>, RemoveFailed::<Impl, OFFSET>, Ended::<Impl, OFFSET>, RemoveEnded::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTimelineControllerFailedEventArgsImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTimelineControllerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.IMediaTimelineControllerFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTimelineControllerFailedEventArgsVtbl {
    pub const fn new<Impl: IMediaTimelineControllerFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTimelineControllerFailedEventArgsVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IMediaTimelineControllerFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTimelineControllerFailedEventArgs>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayPropertiesImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArtist(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMusicDisplayProperties {
    const NAME: &'static str = "Windows.Media.IMusicDisplayProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IMusicDisplayPropertiesVtbl {
    pub const fn new<Impl: IMusicDisplayPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMusicDisplayPropertiesVtbl {
        unsafe extern "system" fn Title<Impl: IMusicDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IMusicDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlbumArtist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlbumArtist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumArtist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlbumArtist(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Artist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Artist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArtist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetArtist(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMusicDisplayProperties>, base.5, Title::<Impl, OFFSET>, SetTitle::<Impl, OFFSET>, AlbumArtist::<Impl, OFFSET>, SetAlbumArtist::<Impl, OFFSET>, Artist::<Impl, OFFSET>, SetArtist::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayProperties2Impl: Sized {
    fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TrackNumber(&self) -> ::windows::core::Result<u32>;
    fn SetTrackNumber(&self, value: u32) -> ::windows::core::Result<()>;
    fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMusicDisplayProperties2 {
    const NAME: &'static str = "Windows.Media.IMusicDisplayProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IMusicDisplayProperties2Vtbl {
    pub const fn new<Impl: IMusicDisplayProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMusicDisplayProperties2Vtbl {
        unsafe extern "system" fn AlbumTitle<Impl: IMusicDisplayProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlbumTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumTitle<Impl: IMusicDisplayProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlbumTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackNumber<Impl: IMusicDisplayProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrackNumber<Impl: IMusicDisplayProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTrackNumber(value).into()
        }
        unsafe extern "system" fn Genres<Impl: IMusicDisplayProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Genres() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMusicDisplayProperties2>, base.5, AlbumTitle::<Impl, OFFSET>, SetAlbumTitle::<Impl, OFFSET>, TrackNumber::<Impl, OFFSET>, SetTrackNumber::<Impl, OFFSET>, Genres::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMusicDisplayProperties3Impl: Sized {
    fn AlbumTrackCount(&self) -> ::windows::core::Result<u32>;
    fn SetAlbumTrackCount(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMusicDisplayProperties3 {
    const NAME: &'static str = "Windows.Media.IMusicDisplayProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl IMusicDisplayProperties3Vtbl {
    pub const fn new<Impl: IMusicDisplayProperties3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMusicDisplayProperties3Vtbl {
        unsafe extern "system" fn AlbumTrackCount<Impl: IMusicDisplayProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlbumTrackCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlbumTrackCount<Impl: IMusicDisplayProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlbumTrackCount(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMusicDisplayProperties3>, base.5, AlbumTrackCount::<Impl, OFFSET>, SetAlbumTrackCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackPositionChangeRequestedEventArgsImpl: Sized {
    fn RequestedPlaybackPosition(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IPlaybackPositionChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaybackPositionChangeRequestedEventArgsVtbl {
    pub const fn new<Impl: IPlaybackPositionChangeRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaybackPositionChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedPlaybackPosition<Impl: IPlaybackPositionChangeRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestedPlaybackPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlaybackPositionChangeRequestedEventArgs>, base.5, RequestedPlaybackPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaybackRateChangeRequestedEventArgsImpl: Sized {
    fn RequestedPlaybackRate(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IPlaybackRateChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaybackRateChangeRequestedEventArgsVtbl {
    pub const fn new<Impl: IPlaybackRateChangeRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaybackRateChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedPlaybackRate<Impl: IPlaybackRateChangeRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestedPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlaybackRateChangeRequestedEventArgs>, base.5, RequestedPlaybackRate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShuffleEnabledChangeRequestedEventArgsImpl: Sized {
    fn RequestedShuffleEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShuffleEnabledChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IShuffleEnabledChangeRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IShuffleEnabledChangeRequestedEventArgsVtbl {
    pub const fn new<Impl: IShuffleEnabledChangeRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShuffleEnabledChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedShuffleEnabled<Impl: IShuffleEnabledChangeRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestedShuffleEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShuffleEnabledChangeRequestedEventArgs>, base.5, RequestedShuffleEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsImpl: Sized {
    fn PlaybackStatus(&self) -> ::windows::core::Result<MediaPlaybackStatus>;
    fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> ::windows::core::Result<()>;
    fn DisplayUpdater(&self) -> ::windows::core::Result<SystemMediaTransportControlsDisplayUpdater>;
    fn SoundLevel(&self) -> ::windows::core::Result<SoundLevel>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPlayEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPlayEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsStopEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsStopEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPauseEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPauseEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRecordEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRecordEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFastForwardEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFastForwardEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRewindEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRewindEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPreviousEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPreviousEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsNextEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsNextEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsChannelUpEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsChannelUpEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsChannelDownEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsChannelDownEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ButtonPressed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveButtonPressed(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertyChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControls";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsVtbl {
    pub const fn new<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControlsVtbl {
        unsafe extern "system" fn PlaybackStatus<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackStatus<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaybackStatus(value).into()
        }
        unsafe extern "system" fn DisplayUpdater<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayUpdater() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoundLevel<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoundLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn IsPlayEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPlayEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPlayEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPlayEnabled(value).into()
        }
        unsafe extern "system" fn IsStopEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStopEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsStopEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsStopEnabled(value).into()
        }
        unsafe extern "system" fn IsPauseEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPauseEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPauseEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPauseEnabled(value).into()
        }
        unsafe extern "system" fn IsRecordEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRecordEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRecordEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsRecordEnabled(value).into()
        }
        unsafe extern "system" fn IsFastForwardEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFastForwardEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsFastForwardEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsFastForwardEnabled(value).into()
        }
        unsafe extern "system" fn IsRewindEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRewindEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRewindEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsRewindEnabled(value).into()
        }
        unsafe extern "system" fn IsPreviousEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPreviousEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPreviousEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPreviousEnabled(value).into()
        }
        unsafe extern "system" fn IsNextEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNextEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsNextEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsNextEnabled(value).into()
        }
        unsafe extern "system" fn IsChannelUpEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsChannelUpEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsChannelUpEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsChannelUpEnabled(value).into()
        }
        unsafe extern "system" fn IsChannelDownEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsChannelDownEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsChannelDownEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsChannelDownEnabled(value).into()
        }
        unsafe extern "system" fn ButtonPressed<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonPressed(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveButtonPressed<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveButtonPressed(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PropertyChanged<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropertyChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChanged<Impl: ISystemMediaTransportControlsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePropertyChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControls>,
            base.5,
            PlaybackStatus::<Impl, OFFSET>,
            SetPlaybackStatus::<Impl, OFFSET>,
            DisplayUpdater::<Impl, OFFSET>,
            SoundLevel::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            SetIsEnabled::<Impl, OFFSET>,
            IsPlayEnabled::<Impl, OFFSET>,
            SetIsPlayEnabled::<Impl, OFFSET>,
            IsStopEnabled::<Impl, OFFSET>,
            SetIsStopEnabled::<Impl, OFFSET>,
            IsPauseEnabled::<Impl, OFFSET>,
            SetIsPauseEnabled::<Impl, OFFSET>,
            IsRecordEnabled::<Impl, OFFSET>,
            SetIsRecordEnabled::<Impl, OFFSET>,
            IsFastForwardEnabled::<Impl, OFFSET>,
            SetIsFastForwardEnabled::<Impl, OFFSET>,
            IsRewindEnabled::<Impl, OFFSET>,
            SetIsRewindEnabled::<Impl, OFFSET>,
            IsPreviousEnabled::<Impl, OFFSET>,
            SetIsPreviousEnabled::<Impl, OFFSET>,
            IsNextEnabled::<Impl, OFFSET>,
            SetIsNextEnabled::<Impl, OFFSET>,
            IsChannelUpEnabled::<Impl, OFFSET>,
            SetIsChannelUpEnabled::<Impl, OFFSET>,
            IsChannelDownEnabled::<Impl, OFFSET>,
            SetIsChannelDownEnabled::<Impl, OFFSET>,
            ButtonPressed::<Impl, OFFSET>,
            RemoveButtonPressed::<Impl, OFFSET>,
            PropertyChanged::<Impl, OFFSET>,
            RemovePropertyChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControls2Impl: Sized {
    fn AutoRepeatMode(&self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode>;
    fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::Result<()>;
    fn ShuffleEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetShuffleEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PlaybackRate(&self) -> ::windows::core::Result<f64>;
    fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn UpdateTimelineProperties(&self, timelineproperties: &::core::option::Option<SystemMediaTransportControlsTimelineProperties>) -> ::windows::core::Result<()>;
    fn PlaybackPositionChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackPositionChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackRateChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackRateChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShuffleEnabledChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShuffleEnabledChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoRepeatModeChangeRequested(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAutoRepeatModeChangeRequested(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControls2 {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControls2";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControls2Vtbl {
    pub const fn new<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControls2Vtbl {
        unsafe extern "system" fn AutoRepeatMode<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoRepeatMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRepeatMode<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoRepeatMode(value).into()
        }
        unsafe extern "system" fn ShuffleEnabled<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShuffleEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShuffleEnabled<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShuffleEnabled(value).into()
        }
        unsafe extern "system" fn PlaybackRate<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackRate<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn UpdateTimelineProperties<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timelineproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateTimelineProperties(&*(&timelineproperties as *const <SystemMediaTransportControlsTimelineProperties as ::windows::core::Abi>::Abi as *const <SystemMediaTransportControlsTimelineProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackPositionChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackPositionChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackPositionChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePlaybackPositionChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackRateChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackRateChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackRateChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePlaybackRateChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShuffleEnabledChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShuffleEnabledChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShuffleEnabledChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveShuffleEnabledChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoRepeatModeChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoRepeatModeChangeRequested(&*(&handler as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutoRepeatModeChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAutoRepeatModeChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControls2>,
            base.5,
            AutoRepeatMode::<Impl, OFFSET>,
            SetAutoRepeatMode::<Impl, OFFSET>,
            ShuffleEnabled::<Impl, OFFSET>,
            SetShuffleEnabled::<Impl, OFFSET>,
            PlaybackRate::<Impl, OFFSET>,
            SetPlaybackRate::<Impl, OFFSET>,
            UpdateTimelineProperties::<Impl, OFFSET>,
            PlaybackPositionChangeRequested::<Impl, OFFSET>,
            RemovePlaybackPositionChangeRequested::<Impl, OFFSET>,
            PlaybackRateChangeRequested::<Impl, OFFSET>,
            RemovePlaybackRateChangeRequested::<Impl, OFFSET>,
            ShuffleEnabledChangeRequested::<Impl, OFFSET>,
            RemoveShuffleEnabledChangeRequested::<Impl, OFFSET>,
            AutoRepeatModeChangeRequested::<Impl, OFFSET>,
            RemoveAutoRepeatModeChangeRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsButtonPressedEventArgsImpl: Sized {
    fn Button(&self) -> ::windows::core::Result<SystemMediaTransportControlsButton>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsButtonPressedEventArgs {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsButtonPressedEventArgsVtbl {
    pub const fn new<Impl: ISystemMediaTransportControlsButtonPressedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControlsButtonPressedEventArgsVtbl {
        unsafe extern "system" fn Button<Impl: ISystemMediaTransportControlsButtonPressedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsButton) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Button() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsButtonPressedEventArgs>, base.5, Button::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsDisplayUpdaterImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<MediaPlaybackType>;
    fn SetType(&self, value: MediaPlaybackType) -> ::windows::core::Result<()>;
    fn AppMediaId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppMediaId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn MusicProperties(&self) -> ::windows::core::Result<MusicDisplayProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<VideoDisplayProperties>;
    fn ImageProperties(&self) -> ::windows::core::Result<ImageDisplayProperties>;
    fn CopyFromFileAsync(&self, r#type: MediaPlaybackType, source: &::core::option::Option<super::Storage::StorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn ClearAll(&self) -> ::windows::core::Result<()>;
    fn Update(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsDisplayUpdater";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsDisplayUpdaterVtbl {
    pub const fn new<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControlsDisplayUpdaterVtbl {
        unsafe extern "system" fn Type<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn AppMediaId<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppMediaId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppMediaId<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppMediaId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MusicProperties<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MusicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProperties<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageProperties<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromFileAsync<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: MediaPlaybackType, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyFromFileAsync(r#type, &*(&source as *const <super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearAll<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearAll().into()
        }
        unsafe extern "system" fn Update<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsDisplayUpdater>, base.5, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, AppMediaId::<Impl, OFFSET>, SetAppMediaId::<Impl, OFFSET>, Thumbnail::<Impl, OFFSET>, SetThumbnail::<Impl, OFFSET>, MusicProperties::<Impl, OFFSET>, VideoProperties::<Impl, OFFSET>, ImageProperties::<Impl, OFFSET>, CopyFromFileAsync::<Impl, OFFSET>, ClearAll::<Impl, OFFSET>, Update::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsPropertyChangedEventArgsImpl: Sized {
    fn Property(&self) -> ::windows::core::Result<SystemMediaTransportControlsProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsPropertyChangedEventArgsVtbl {
    pub const fn new<Impl: ISystemMediaTransportControlsPropertyChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControlsPropertyChangedEventArgsVtbl {
        unsafe extern "system" fn Property<Impl: ISystemMediaTransportControlsPropertyChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsPropertyChangedEventArgs>, base.5, Property::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SystemMediaTransportControls>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsStatics {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsStaticsVtbl {
    pub const fn new<Impl: ISystemMediaTransportControlsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControlsStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISystemMediaTransportControlsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaTransportControlsTimelinePropertiesImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetStartTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetEndTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MinSeekTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetMinSeekTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MaxSeekTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetMaxSeekTime(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetPosition(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsTimelineProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaTransportControlsTimelinePropertiesVtbl {
    pub const fn new<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControlsTimelinePropertiesVtbl {
        unsafe extern "system" fn StartTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinSeekTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinSeekTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSeekTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxSeekTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsTimelineProperties>, base.5, StartTime::<Impl, OFFSET>, SetStartTime::<Impl, OFFSET>, EndTime::<Impl, OFFSET>, SetEndTime::<Impl, OFFSET>, MinSeekTime::<Impl, OFFSET>, SetMinSeekTime::<Impl, OFFSET>, MaxSeekTime::<Impl, OFFSET>, SetMaxSeekTime::<Impl, OFFSET>, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDisplayPropertiesImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoDisplayProperties {
    const NAME: &'static str = "Windows.Media.IVideoDisplayProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoDisplayPropertiesVtbl {
    pub const fn new<Impl: IVideoDisplayPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoDisplayPropertiesVtbl {
        unsafe extern "system" fn Title<Impl: IVideoDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IVideoDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtitle<Impl: IVideoDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subtitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubtitle<Impl: IVideoDisplayPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSubtitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoDisplayProperties>, base.5, Title::<Impl, OFFSET>, SetTitle::<Impl, OFFSET>, Subtitle::<Impl, OFFSET>, SetSubtitle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDisplayProperties2Impl: Sized {
    fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoDisplayProperties2 {
    const NAME: &'static str = "Windows.Media.IVideoDisplayProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoDisplayProperties2Vtbl {
    pub const fn new<Impl: IVideoDisplayProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoDisplayProperties2Vtbl {
        unsafe extern "system" fn Genres<Impl: IVideoDisplayProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Genres() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoDisplayProperties2>, base.5, Genres::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEffectsStaticsImpl: Sized {
    fn VideoStabilization(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEffectsStatics {
    const NAME: &'static str = "Windows.Media.IVideoEffectsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEffectsStaticsVtbl {
    pub const fn new<Impl: IVideoEffectsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEffectsStaticsVtbl {
        unsafe extern "system" fn VideoStabilization<Impl: IVideoEffectsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoStabilization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEffectsStatics>, base.5, VideoStabilization::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVideoFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::Graphics::Imaging::SoftwareBitmap>;
    fn CopyToAsync(&self, frame: &::core::option::Option<VideoFrame>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn Direct3DSurface(&self) -> ::windows::core::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrame {
    const NAME: &'static str = "Windows.Media.IVideoFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVideoFrameVtbl {
    pub const fn new<Impl: IVideoFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoFrameVtbl {
        unsafe extern "system" fn SoftwareBitmap<Impl: IVideoFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoftwareBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToAsync<Impl: IVideoFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyToAsync(&*(&frame as *const <VideoFrame as ::windows::core::Abi>::Abi as *const <VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direct3DSurface<Impl: IVideoFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direct3DSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoFrame>, base.5, SoftwareBitmap::<Impl, OFFSET>, CopyToAsync::<Impl, OFFSET>, Direct3DSurface::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoFrame2Impl: Sized {
    fn CopyToWithBoundsAsync(&self, frame: &::core::option::Option<VideoFrame>, sourcebounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>, destinationbounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoFrame2 {
    const NAME: &'static str = "Windows.Media.IVideoFrame2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoFrame2Vtbl {
    pub const fn new<Impl: IVideoFrame2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoFrame2Vtbl {
        unsafe extern "system" fn CopyToWithBoundsAsync<Impl: IVideoFrame2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, sourcebounds: ::windows::core::RawPtr, destinationbounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoFrame2>, base.5, CopyToWithBoundsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoFrameFactoryImpl: Sized {
    fn Create(&self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithAlpha(&self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoFrameFactory {
    const NAME: &'static str = "Windows.Media.IVideoFrameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoFrameFactoryVtbl {
    pub const fn new<Impl: IVideoFrameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoFrameFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoFrameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAlpha<Impl: IVideoFrameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAlpha(format, width, height, alpha) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoFrameFactory>, base.5, Create::<Impl, OFFSET>, CreateWithAlpha::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoFrameStaticsImpl: Sized {
    fn CreateAsDirect3D11SurfaceBacked(&self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateAsDirect3D11SurfaceBackedWithDevice(&self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithSoftwareBitmap(&self, bitmap: &::core::option::Option<super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithDirect3D11Surface(&self, surface: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoFrameStatics {
    const NAME: &'static str = "Windows.Media.IVideoFrameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoFrameStaticsVtbl {
    pub const fn new<Impl: IVideoFrameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoFrameStaticsVtbl {
        unsafe extern "system" fn CreateAsDirect3D11SurfaceBacked<Impl: IVideoFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsDirect3D11SurfaceBacked(format, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsDirect3D11SurfaceBackedWithDevice<Impl: IVideoFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsDirect3D11SurfaceBackedWithDevice(format, width, height, &*(&device as *const <super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithSoftwareBitmap<Impl: IVideoFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithSoftwareBitmap(&*(&bitmap as *const <super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithDirect3D11Surface<Impl: IVideoFrameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithDirect3D11Surface(&*(&surface as *const <super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoFrameStatics>, base.5, CreateAsDirect3D11SurfaceBacked::<Impl, OFFSET>, CreateAsDirect3D11SurfaceBackedWithDevice::<Impl, OFFSET>, CreateWithSoftwareBitmap::<Impl, OFFSET>, CreateWithDirect3D11Surface::<Impl, OFFSET>)
    }
}
