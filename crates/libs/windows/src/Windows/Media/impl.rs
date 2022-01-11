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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioBufferVtbl {
        unsafe extern "system" fn Capacity<Impl: IAudioBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Length<Impl: IAudioBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLength<Impl: IAudioBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioBuffer>, ::windows::core::GetTrustLevel, Capacity::<Impl, IMPL_OFFSET>, Length::<Impl, IMPL_OFFSET>, SetLength::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAudioFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn LockBuffer(&self, mode: AudioBufferAccessMode) -> ::windows::core::Result<AudioBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioFrame {
    const NAME: &'static str = "Windows.Media.IAudioFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAudioFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioFrameVtbl {
        unsafe extern "system" fn LockBuffer<Impl: IAudioFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: AudioBufferAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioFrame>, ::windows::core::GetTrustLevel, LockBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrame as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioFrameFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioFrameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioFrameFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrameFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoRepeatModeChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoRepeatModeChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedAutoRepeatMode<Impl: IAutoRepeatModeChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutoRepeatModeChangeRequestedEventArgs>, ::windows::core::GetTrustLevel, RequestedAutoRepeatMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutoRepeatModeChangeRequestedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageDisplayPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageDisplayPropertiesVtbl {
        unsafe extern "system" fn Title<Impl: IImageDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IImageDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtitle<Impl: IImageDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSubtitle<Impl: IImageDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubtitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImageDisplayProperties>, ::windows::core::GetTrustLevel, Title::<Impl, IMPL_OFFSET>, SetTitle::<Impl, IMPL_OFFSET>, Subtitle::<Impl, IMPL_OFFSET>, SetSubtitle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaControl {
    const NAME: &'static str = "Windows.Media.IMediaControl";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IMediaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaControlVtbl {
        unsafe extern "system" fn SoundLevelChanged<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSoundLevelChanged<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSoundLevelChanged(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlayPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PausePressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePausePressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePausePressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStopPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlayPauseTogglePressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlayPauseTogglePressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlayPauseTogglePressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RecordPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRecordPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecordPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NextTrackPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveNextTrackPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNextTrackPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviousTrackPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePreviousTrackPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviousTrackPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FastForwardPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveFastForwardPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFastForwardPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RewindPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRewindPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRewindPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChannelUpPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChannelUpPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChannelUpPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChannelDownPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChannelDownPressed<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChannelDownPressed(&*(&cookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SoundLevel<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTrackName<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrackName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackName<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetArtistName<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArtistName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ArtistName<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPlaying<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPlaying(value).into()
        }
        unsafe extern "system" fn IsPlaying<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlbumArt<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumArt(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlbumArt<Impl: IMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaControl>,
            ::windows::core::GetTrustLevel,
            SoundLevelChanged::<Impl, IMPL_OFFSET>,
            RemoveSoundLevelChanged::<Impl, IMPL_OFFSET>,
            PlayPressed::<Impl, IMPL_OFFSET>,
            RemovePlayPressed::<Impl, IMPL_OFFSET>,
            PausePressed::<Impl, IMPL_OFFSET>,
            RemovePausePressed::<Impl, IMPL_OFFSET>,
            StopPressed::<Impl, IMPL_OFFSET>,
            RemoveStopPressed::<Impl, IMPL_OFFSET>,
            PlayPauseTogglePressed::<Impl, IMPL_OFFSET>,
            RemovePlayPauseTogglePressed::<Impl, IMPL_OFFSET>,
            RecordPressed::<Impl, IMPL_OFFSET>,
            RemoveRecordPressed::<Impl, IMPL_OFFSET>,
            NextTrackPressed::<Impl, IMPL_OFFSET>,
            RemoveNextTrackPressed::<Impl, IMPL_OFFSET>,
            PreviousTrackPressed::<Impl, IMPL_OFFSET>,
            RemovePreviousTrackPressed::<Impl, IMPL_OFFSET>,
            FastForwardPressed::<Impl, IMPL_OFFSET>,
            RemoveFastForwardPressed::<Impl, IMPL_OFFSET>,
            RewindPressed::<Impl, IMPL_OFFSET>,
            RemoveRewindPressed::<Impl, IMPL_OFFSET>,
            ChannelUpPressed::<Impl, IMPL_OFFSET>,
            RemoveChannelUpPressed::<Impl, IMPL_OFFSET>,
            ChannelDownPressed::<Impl, IMPL_OFFSET>,
            RemoveChannelDownPressed::<Impl, IMPL_OFFSET>,
            SoundLevel::<Impl, IMPL_OFFSET>,
            SetTrackName::<Impl, IMPL_OFFSET>,
            TrackName::<Impl, IMPL_OFFSET>,
            SetArtistName::<Impl, IMPL_OFFSET>,
            ArtistName::<Impl, IMPL_OFFSET>,
            SetIsPlaying::<Impl, IMPL_OFFSET>,
            IsPlaying::<Impl, IMPL_OFFSET>,
            SetAlbumArt::<Impl, IMPL_OFFSET>,
            AlbumArt::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaExtensionImpl: Sized {
    fn SetProperties(&self, configuration: &::core::option::Option<super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaExtension {
    const NAME: &'static str = "Windows.Media.IMediaExtension";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaExtensionVtbl {
        unsafe extern "system" fn SetProperties<Impl: IMediaExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperties(&*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaExtension>, ::windows::core::GetTrustLevel, SetProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaExtensionManager {
    const NAME: &'static str = "Windows.Media.IMediaExtensionManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaExtensionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaExtensionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaExtensionManagerVtbl {
        unsafe extern "system" fn RegisterSchemeHandler<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterSchemeHandler(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterSchemeHandlerWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterSchemeHandlerWithSettings(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&configuration as *const <super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterByteStreamHandler<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterByteStreamHandler(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&fileextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterByteStreamHandlerWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterAudioDecoder<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioDecoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioDecoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterAudioEncoder<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterAudioEncoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterAudioEncoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterVideoDecoder<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoDecoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoDecoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterVideoEncoder<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterVideoEncoder(
                    &*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&inputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                    &*(&outputsubtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RegisterVideoEncoderWithSettings<Impl: IMediaExtensionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaExtensionManager>,
            ::windows::core::GetTrustLevel,
            RegisterSchemeHandler::<Impl, IMPL_OFFSET>,
            RegisterSchemeHandlerWithSettings::<Impl, IMPL_OFFSET>,
            RegisterByteStreamHandler::<Impl, IMPL_OFFSET>,
            RegisterByteStreamHandlerWithSettings::<Impl, IMPL_OFFSET>,
            RegisterAudioDecoder::<Impl, IMPL_OFFSET>,
            RegisterAudioDecoderWithSettings::<Impl, IMPL_OFFSET>,
            RegisterAudioEncoder::<Impl, IMPL_OFFSET>,
            RegisterAudioEncoderWithSettings::<Impl, IMPL_OFFSET>,
            RegisterVideoDecoder::<Impl, IMPL_OFFSET>,
            RegisterVideoDecoderWithSettings::<Impl, IMPL_OFFSET>,
            RegisterVideoEncoder::<Impl, IMPL_OFFSET>,
            RegisterVideoEncoderWithSettings::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaExtensionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaExtensionManager2Impl: Sized + IMediaExtensionManagerImpl {
    fn RegisterMediaExtensionForAppService(&self, extension: &::core::option::Option<IMediaExtension>, connection: &::core::option::Option<super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaExtensionManager2 {
    const NAME: &'static str = "Windows.Media.IMediaExtensionManager2";
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaExtensionManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaExtensionManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaExtensionManager2Vtbl {
        unsafe extern "system" fn RegisterMediaExtensionForAppService<Impl: IMediaExtensionManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extension: ::windows::core::RawPtr, connection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterMediaExtensionForAppService(&*(&extension as *const <IMediaExtension as ::windows::core::Abi>::Abi as *const <IMediaExtension as ::windows::core::DefaultType>::DefaultType), &*(&connection as *const <super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::Abi>::Abi as *const <super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaExtensionManager2>, ::windows::core::GetTrustLevel, RegisterMediaExtensionForAppService::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaExtensionManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IMediaFrame {
    const NAME: &'static str = "Windows.Media.IMediaFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IMediaFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameVtbl {
        unsafe extern "system" fn Type<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsReadOnly<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRelativeTime<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RelativeTime<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSystemRelativeTime<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemRelativeTime(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsDiscontinuous<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDiscontinuous(value).into()
        }
        unsafe extern "system" fn IsDiscontinuous<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedProperties<Impl: IMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaFrame>,
            ::windows::core::GetTrustLevel,
            Type::<Impl, IMPL_OFFSET>,
            IsReadOnly::<Impl, IMPL_OFFSET>,
            SetRelativeTime::<Impl, IMPL_OFFSET>,
            RelativeTime::<Impl, IMPL_OFFSET>,
            SetSystemRelativeTime::<Impl, IMPL_OFFSET>,
            SystemRelativeTime::<Impl, IMPL_OFFSET>,
            SetDuration::<Impl, IMPL_OFFSET>,
            Duration::<Impl, IMPL_OFFSET>,
            SetIsDiscontinuous::<Impl, IMPL_OFFSET>,
            IsDiscontinuous::<Impl, IMPL_OFFSET>,
            ExtendedProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IMediaMarkerImpl: Sized {
    fn Time(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IMediaMarker {
    const NAME: &'static str = "Windows.Media.IMediaMarker";
}
#[cfg(feature = "Foundation")]
impl IMediaMarkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarkerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarkerVtbl {
        unsafe extern "system" fn Time<Impl: IMediaMarkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaMarkerType<Impl: IMediaMarkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IMediaMarkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaMarker>, ::windows::core::GetTrustLevel, Time::<Impl, IMPL_OFFSET>, MediaMarkerType::<Impl, IMPL_OFFSET>, Text::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarker as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarkerTypesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarkerTypesStaticsVtbl {
        unsafe extern "system" fn Bookmark<Impl: IMediaMarkerTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaMarkerTypesStatics>, ::windows::core::GetTrustLevel, Bookmark::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarkerTypesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaMarkersImpl: Sized {
    fn Markers(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaMarkers {
    const NAME: &'static str = "Windows.Media.IMediaMarkers";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaMarkersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaMarkersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaMarkersVtbl {
        unsafe extern "system" fn Markers<Impl: IMediaMarkersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaMarkers>, ::windows::core::GetTrustLevel, Markers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaMarkers as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaProcessingTriggerDetailsImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.IMediaProcessingTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaProcessingTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProcessingTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProcessingTriggerDetailsVtbl {
        unsafe extern "system" fn Arguments<Impl: IMediaProcessingTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaProcessingTriggerDetails>, ::windows::core::GetTrustLevel, Arguments::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProcessingTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTimelineController {
    const NAME: &'static str = "Windows.Media.IMediaTimelineController";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaTimelineControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTimelineControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTimelineControllerVtbl {
        unsafe extern "system" fn Start<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Resume<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Pause<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Position<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClockRate<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClockRate<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClockRate(value).into()
        }
        unsafe extern "system" fn State<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaTimelineControllerState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PositionChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positionchangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePositionChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&eventcookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StateChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statechangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStateChanged<Impl: IMediaTimelineControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&eventcookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaTimelineController>,
            ::windows::core::GetTrustLevel,
            Start::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Position::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            ClockRate::<Impl, IMPL_OFFSET>,
            SetClockRate::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            PositionChanged::<Impl, IMPL_OFFSET>,
            RemovePositionChanged::<Impl, IMPL_OFFSET>,
            StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTimelineController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTimelineController2 {
    const NAME: &'static str = "Windows.Media.IMediaTimelineController2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaTimelineController2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTimelineController2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTimelineController2Vtbl {
        unsafe extern "system" fn Duration<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsLoopingEnabled<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsLoopingEnabled<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLoopingEnabled(value).into()
        }
        unsafe extern "system" fn Failed<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveFailed<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFailed(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ended<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveEnded<Impl: IMediaTimelineController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaTimelineController2>,
            ::windows::core::GetTrustLevel,
            Duration::<Impl, IMPL_OFFSET>,
            SetDuration::<Impl, IMPL_OFFSET>,
            IsLoopingEnabled::<Impl, IMPL_OFFSET>,
            SetIsLoopingEnabled::<Impl, IMPL_OFFSET>,
            Failed::<Impl, IMPL_OFFSET>,
            RemoveFailed::<Impl, IMPL_OFFSET>,
            Ended::<Impl, IMPL_OFFSET>,
            RemoveEnded::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTimelineController2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTimelineControllerFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTimelineControllerFailedEventArgsVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IMediaTimelineControllerFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaTimelineControllerFailedEventArgs>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTimelineControllerFailedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMusicDisplayPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMusicDisplayPropertiesVtbl {
        unsafe extern "system" fn Title<Impl: IMusicDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IMusicDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlbumArtist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlbumArtist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumArtist(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Artist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetArtist<Impl: IMusicDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArtist(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMusicDisplayProperties>, ::windows::core::GetTrustLevel, Title::<Impl, IMPL_OFFSET>, SetTitle::<Impl, IMPL_OFFSET>, AlbumArtist::<Impl, IMPL_OFFSET>, SetAlbumArtist::<Impl, IMPL_OFFSET>, Artist::<Impl, IMPL_OFFSET>, SetArtist::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMusicDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMusicDisplayProperties2Impl: Sized {
    fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlbumTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TrackNumber(&self) -> ::windows::core::Result<u32>;
    fn SetTrackNumber(&self, value: u32) -> ::windows::core::Result<()>;
    fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMusicDisplayProperties2 {
    const NAME: &'static str = "Windows.Media.IMusicDisplayProperties2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMusicDisplayProperties2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMusicDisplayProperties2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMusicDisplayProperties2Vtbl {
        unsafe extern "system" fn AlbumTitle<Impl: IMusicDisplayProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlbumTitle<Impl: IMusicDisplayProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackNumber<Impl: IMusicDisplayProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTrackNumber<Impl: IMusicDisplayProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrackNumber(value).into()
        }
        unsafe extern "system" fn Genres<Impl: IMusicDisplayProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMusicDisplayProperties2>, ::windows::core::GetTrustLevel, AlbumTitle::<Impl, IMPL_OFFSET>, SetAlbumTitle::<Impl, IMPL_OFFSET>, TrackNumber::<Impl, IMPL_OFFSET>, SetTrackNumber::<Impl, IMPL_OFFSET>, Genres::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMusicDisplayProperties2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMusicDisplayProperties3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMusicDisplayProperties3Vtbl {
        unsafe extern "system" fn AlbumTrackCount<Impl: IMusicDisplayProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlbumTrackCount<Impl: IMusicDisplayProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlbumTrackCount(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMusicDisplayProperties3>, ::windows::core::GetTrustLevel, AlbumTrackCount::<Impl, IMPL_OFFSET>, SetAlbumTrackCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMusicDisplayProperties3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPlaybackPositionChangeRequestedEventArgsImpl: Sized {
    fn RequestedPlaybackPosition(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.IPlaybackPositionChangeRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPlaybackPositionChangeRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackPositionChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackPositionChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedPlaybackPosition<Impl: IPlaybackPositionChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaybackPositionChangeRequestedEventArgs>, ::windows::core::GetTrustLevel, RequestedPlaybackPosition::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackPositionChangeRequestedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackRateChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackRateChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedPlaybackRate<Impl: IPlaybackRateChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaybackRateChangeRequestedEventArgs>, ::windows::core::GetTrustLevel, RequestedPlaybackRate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackRateChangeRequestedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShuffleEnabledChangeRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShuffleEnabledChangeRequestedEventArgsVtbl {
        unsafe extern "system" fn RequestedShuffleEnabled<Impl: IShuffleEnabledChangeRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShuffleEnabledChangeRequestedEventArgs>, ::windows::core::GetTrustLevel, RequestedShuffleEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShuffleEnabledChangeRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControls";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemMediaTransportControlsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsVtbl {
        unsafe extern "system" fn PlaybackStatus<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackStatus<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackStatus(value).into()
        }
        unsafe extern "system" fn DisplayUpdater<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SoundLevel<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn IsPlayEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPlayEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPlayEnabled(value).into()
        }
        unsafe extern "system" fn IsStopEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsStopEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsStopEnabled(value).into()
        }
        unsafe extern "system" fn IsPauseEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPauseEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPauseEnabled(value).into()
        }
        unsafe extern "system" fn IsRecordEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRecordEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRecordEnabled(value).into()
        }
        unsafe extern "system" fn IsFastForwardEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsFastForwardEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsFastForwardEnabled(value).into()
        }
        unsafe extern "system" fn IsRewindEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRewindEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRewindEnabled(value).into()
        }
        unsafe extern "system" fn IsPreviousEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPreviousEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPreviousEnabled(value).into()
        }
        unsafe extern "system" fn IsNextEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsNextEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsNextEnabled(value).into()
        }
        unsafe extern "system" fn IsChannelUpEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsChannelUpEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsChannelUpEnabled(value).into()
        }
        unsafe extern "system" fn IsChannelDownEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsChannelDownEnabled<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsChannelDownEnabled(value).into()
        }
        unsafe extern "system" fn ButtonPressed<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveButtonPressed<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveButtonPressed(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PropertyChanged<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePropertyChanged<Impl: ISystemMediaTransportControlsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePropertyChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControls>,
            ::windows::core::GetTrustLevel,
            PlaybackStatus::<Impl, IMPL_OFFSET>,
            SetPlaybackStatus::<Impl, IMPL_OFFSET>,
            DisplayUpdater::<Impl, IMPL_OFFSET>,
            SoundLevel::<Impl, IMPL_OFFSET>,
            IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled::<Impl, IMPL_OFFSET>,
            IsPlayEnabled::<Impl, IMPL_OFFSET>,
            SetIsPlayEnabled::<Impl, IMPL_OFFSET>,
            IsStopEnabled::<Impl, IMPL_OFFSET>,
            SetIsStopEnabled::<Impl, IMPL_OFFSET>,
            IsPauseEnabled::<Impl, IMPL_OFFSET>,
            SetIsPauseEnabled::<Impl, IMPL_OFFSET>,
            IsRecordEnabled::<Impl, IMPL_OFFSET>,
            SetIsRecordEnabled::<Impl, IMPL_OFFSET>,
            IsFastForwardEnabled::<Impl, IMPL_OFFSET>,
            SetIsFastForwardEnabled::<Impl, IMPL_OFFSET>,
            IsRewindEnabled::<Impl, IMPL_OFFSET>,
            SetIsRewindEnabled::<Impl, IMPL_OFFSET>,
            IsPreviousEnabled::<Impl, IMPL_OFFSET>,
            SetIsPreviousEnabled::<Impl, IMPL_OFFSET>,
            IsNextEnabled::<Impl, IMPL_OFFSET>,
            SetIsNextEnabled::<Impl, IMPL_OFFSET>,
            IsChannelUpEnabled::<Impl, IMPL_OFFSET>,
            SetIsChannelUpEnabled::<Impl, IMPL_OFFSET>,
            IsChannelDownEnabled::<Impl, IMPL_OFFSET>,
            SetIsChannelDownEnabled::<Impl, IMPL_OFFSET>,
            ButtonPressed::<Impl, IMPL_OFFSET>,
            RemoveButtonPressed::<Impl, IMPL_OFFSET>,
            PropertyChanged::<Impl, IMPL_OFFSET>,
            RemovePropertyChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControls as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControls2 {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControls2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemMediaTransportControls2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControls2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControls2Vtbl {
        unsafe extern "system" fn AutoRepeatMode<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoRepeatMode<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoRepeatMode(value).into()
        }
        unsafe extern "system" fn ShuffleEnabled<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShuffleEnabled<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShuffleEnabled(value).into()
        }
        unsafe extern "system" fn PlaybackRate<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaybackRate<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaybackRate(value).into()
        }
        unsafe extern "system" fn UpdateTimelineProperties<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timelineproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateTimelineProperties(&*(&timelineproperties as *const <SystemMediaTransportControlsTimelineProperties as ::windows::core::Abi>::Abi as *const <SystemMediaTransportControlsTimelineProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackPositionChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlaybackPositionChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackPositionChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackRateChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlaybackRateChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackRateChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShuffleEnabledChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveShuffleEnabledChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShuffleEnabledChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoRepeatModeChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAutoRepeatModeChangeRequested<Impl: ISystemMediaTransportControls2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutoRepeatModeChangeRequested(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControls2>,
            ::windows::core::GetTrustLevel,
            AutoRepeatMode::<Impl, IMPL_OFFSET>,
            SetAutoRepeatMode::<Impl, IMPL_OFFSET>,
            ShuffleEnabled::<Impl, IMPL_OFFSET>,
            SetShuffleEnabled::<Impl, IMPL_OFFSET>,
            PlaybackRate::<Impl, IMPL_OFFSET>,
            SetPlaybackRate::<Impl, IMPL_OFFSET>,
            UpdateTimelineProperties::<Impl, IMPL_OFFSET>,
            PlaybackPositionChangeRequested::<Impl, IMPL_OFFSET>,
            RemovePlaybackPositionChangeRequested::<Impl, IMPL_OFFSET>,
            PlaybackRateChangeRequested::<Impl, IMPL_OFFSET>,
            RemovePlaybackRateChangeRequested::<Impl, IMPL_OFFSET>,
            ShuffleEnabledChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveShuffleEnabledChangeRequested::<Impl, IMPL_OFFSET>,
            AutoRepeatModeChangeRequested::<Impl, IMPL_OFFSET>,
            RemoveAutoRepeatModeChangeRequested::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControls2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsButtonPressedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsButtonPressedEventArgsVtbl {
        unsafe extern "system" fn Button<Impl: ISystemMediaTransportControlsButtonPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsButton) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsButtonPressedEventArgs>, ::windows::core::GetTrustLevel, Button::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsButtonPressedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsDisplayUpdater";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISystemMediaTransportControlsDisplayUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsDisplayUpdaterVtbl {
        unsafe extern "system" fn Type<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaPlaybackType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn AppMediaId<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppMediaId<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppMediaId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetThumbnail<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::Storage::Streams::RandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::RandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MusicProperties<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VideoProperties<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ImageProperties<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CopyFromFileAsync<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: MediaPlaybackType, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearAll<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAll().into()
        }
        unsafe extern "system" fn Update<Impl: ISystemMediaTransportControlsDisplayUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsDisplayUpdater>,
            ::windows::core::GetTrustLevel,
            Type::<Impl, IMPL_OFFSET>,
            SetType::<Impl, IMPL_OFFSET>,
            AppMediaId::<Impl, IMPL_OFFSET>,
            SetAppMediaId::<Impl, IMPL_OFFSET>,
            Thumbnail::<Impl, IMPL_OFFSET>,
            SetThumbnail::<Impl, IMPL_OFFSET>,
            MusicProperties::<Impl, IMPL_OFFSET>,
            VideoProperties::<Impl, IMPL_OFFSET>,
            ImageProperties::<Impl, IMPL_OFFSET>,
            CopyFromFileAsync::<Impl, IMPL_OFFSET>,
            ClearAll::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsDisplayUpdater as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsPropertyChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsPropertyChangedEventArgsVtbl {
        unsafe extern "system" fn Property<Impl: ISystemMediaTransportControlsPropertyChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsProperty) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsPropertyChangedEventArgs>, ::windows::core::GetTrustLevel, Property::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsPropertyChangedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISystemMediaTransportControlsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.ISystemMediaTransportControlsTimelineProperties";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemMediaTransportControlsTimelinePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsTimelinePropertiesVtbl {
        unsafe extern "system" fn StartTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStartTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEndTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinSeekTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxSeekTime<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSeekTime(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: ISystemMediaTransportControlsTimelinePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsTimelineProperties>,
            ::windows::core::GetTrustLevel,
            StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime::<Impl, IMPL_OFFSET>,
            EndTime::<Impl, IMPL_OFFSET>,
            SetEndTime::<Impl, IMPL_OFFSET>,
            MinSeekTime::<Impl, IMPL_OFFSET>,
            SetMinSeekTime::<Impl, IMPL_OFFSET>,
            MaxSeekTime::<Impl, IMPL_OFFSET>,
            SetMaxSeekTime::<Impl, IMPL_OFFSET>,
            Position::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsTimelineProperties as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoDisplayPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoDisplayPropertiesVtbl {
        unsafe extern "system" fn Title<Impl: IVideoDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IVideoDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtitle<Impl: IVideoDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSubtitle<Impl: IVideoDisplayPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubtitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoDisplayProperties>, ::windows::core::GetTrustLevel, Title::<Impl, IMPL_OFFSET>, SetTitle::<Impl, IMPL_OFFSET>, Subtitle::<Impl, IMPL_OFFSET>, SetSubtitle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoDisplayProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVideoDisplayProperties2Impl: Sized {
    fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoDisplayProperties2 {
    const NAME: &'static str = "Windows.Media.IVideoDisplayProperties2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVideoDisplayProperties2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoDisplayProperties2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoDisplayProperties2Vtbl {
        unsafe extern "system" fn Genres<Impl: IVideoDisplayProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoDisplayProperties2>, ::windows::core::GetTrustLevel, Genres::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoDisplayProperties2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEffectsStaticsVtbl {
        unsafe extern "system" fn VideoStabilization<Impl: IVideoEffectsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoEffectsStatics>, ::windows::core::GetTrustLevel, VideoStabilization::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEffectsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::Graphics::Imaging::SoftwareBitmap>;
    fn CopyToAsync(&self, frame: &::core::option::Option<VideoFrame>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn Direct3DSurface(&self) -> ::windows::core::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrame {
    const NAME: &'static str = "Windows.Media.IVideoFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrameVtbl {
        unsafe extern "system" fn SoftwareBitmap<Impl: IVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CopyToAsync<Impl: IVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Direct3DSurface<Impl: IVideoFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoFrame>, ::windows::core::GetTrustLevel, SoftwareBitmap::<Impl, IMPL_OFFSET>, CopyToAsync::<Impl, IMPL_OFFSET>, Direct3DSurface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrame2Impl: Sized {
    fn CopyToWithBoundsAsync(&self, frame: &::core::option::Option<VideoFrame>, sourcebounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>, destinationbounds: &::core::option::Option<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrame2 {
    const NAME: &'static str = "Windows.Media.IVideoFrame2";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrame2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrame2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrame2Vtbl {
        unsafe extern "system" fn CopyToWithBoundsAsync<Impl: IVideoFrame2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, sourcebounds: ::windows::core::RawPtr, destinationbounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoFrame2>, ::windows::core::GetTrustLevel, CopyToWithBoundsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrame2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrameFactoryImpl: Sized {
    fn Create(&self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithAlpha(&self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<VideoFrame>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrameFactory {
    const NAME: &'static str = "Windows.Media.IVideoFrameFactory";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrameFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrameFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoFrameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithAlpha<Impl: IVideoFrameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoFrameFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithAlpha::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IVideoFrameStaticsImpl: Sized {
    fn CreateAsDirect3D11SurfaceBacked(&self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame>;
    fn CreateAsDirect3D11SurfaceBackedWithDevice(&self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithSoftwareBitmap(&self, bitmap: &::core::option::Option<super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<VideoFrame>;
    fn CreateWithDirect3D11Surface(&self, surface: &::core::option::Option<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<VideoFrame>;
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoFrameStatics {
    const NAME: &'static str = "Windows.Media.IVideoFrameStatics";
}
#[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IVideoFrameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrameStaticsVtbl {
        unsafe extern "system" fn CreateAsDirect3D11SurfaceBacked<Impl: IVideoFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateAsDirect3D11SurfaceBackedWithDevice<Impl: IVideoFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithSoftwareBitmap<Impl: IVideoFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithDirect3D11Surface<Impl: IVideoFrameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVideoFrameStatics>,
            ::windows::core::GetTrustLevel,
            CreateAsDirect3D11SurfaceBacked::<Impl, IMPL_OFFSET>,
            CreateAsDirect3D11SurfaceBackedWithDevice::<Impl, IMPL_OFFSET>,
            CreateWithSoftwareBitmap::<Impl, IMPL_OFFSET>,
            CreateWithDirect3D11Surface::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameStatics as ::windows::core::Interface>::IID
    }
}
