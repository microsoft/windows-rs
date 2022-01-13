#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IBackgroundAudioTrackImpl: Sized {
    fn TrimTimeFromStart(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromStart(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimTimeFromEnd(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromEnd(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn OriginalDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrimmedDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SetDelay(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Delay(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetVolume(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&mut self) -> ::windows::core::Result<f64>;
    fn Clone(&mut self) -> ::windows::core::Result<BackgroundAudioTrack>;
    fn GetAudioEncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn AudioEffectDefinitions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.IBackgroundAudioTrack";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IBackgroundAudioTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundAudioTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundAudioTrackVtbl {
        unsafe extern "system" fn TrimTimeFromStart<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimTimeFromStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimTimeFromStart<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimTimeFromStart(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrimTimeFromEnd<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimTimeFromEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimTimeFromEnd<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimTimeFromEnd(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OriginalDuration<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrimmedDuration<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimmedDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserData<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelay<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Delay<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        unsafe extern "system" fn Volume<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioEncodingProperties<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioEffectDefinitions<Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEffectDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundAudioTrack, BASE_OFFSET>(),
            TrimTimeFromStart: TrimTimeFromStart::<Impl, IMPL_OFFSET>,
            SetTrimTimeFromStart: SetTrimTimeFromStart::<Impl, IMPL_OFFSET>,
            TrimTimeFromEnd: TrimTimeFromEnd::<Impl, IMPL_OFFSET>,
            SetTrimTimeFromEnd: SetTrimTimeFromEnd::<Impl, IMPL_OFFSET>,
            OriginalDuration: OriginalDuration::<Impl, IMPL_OFFSET>,
            TrimmedDuration: TrimmedDuration::<Impl, IMPL_OFFSET>,
            UserData: UserData::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
            Delay: Delay::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetAudioEncodingProperties: GetAudioEncodingProperties::<Impl, IMPL_OFFSET>,
            AudioEffectDefinitions: AudioEffectDefinitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundAudioTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IBackgroundAudioTrackStaticsImpl: Sized {
    fn CreateFromEmbeddedAudioTrack(&mut self, embeddedaudiotrack: &::core::option::Option<EmbeddedAudioTrack>) -> ::windows::core::Result<BackgroundAudioTrack>;
    fn CreateFromFileAsync(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundAudioTrackStatics {
    const NAME: &'static str = "Windows.Media.Editing.IBackgroundAudioTrackStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IBackgroundAudioTrackStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundAudioTrackStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundAudioTrackStaticsVtbl {
        unsafe extern "system" fn CreateFromEmbeddedAudioTrack<Impl: IBackgroundAudioTrackStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, embeddedaudiotrack: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromEmbeddedAudioTrack(&*(&embeddedaudiotrack as *const <EmbeddedAudioTrack as ::windows::core::Abi>::Abi as *const <EmbeddedAudioTrack as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromFileAsync<Impl: IBackgroundAudioTrackStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromFileAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundAudioTrackStatics, BASE_OFFSET>(),
            CreateFromEmbeddedAudioTrack: CreateFromEmbeddedAudioTrack::<Impl, IMPL_OFFSET>,
            CreateFromFileAsync: CreateFromFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundAudioTrackStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IEmbeddedAudioTrackImpl: Sized {
    fn GetAudioEncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEmbeddedAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.IEmbeddedAudioTrack";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IEmbeddedAudioTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmbeddedAudioTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmbeddedAudioTrackVtbl {
        unsafe extern "system" fn GetAudioEncodingProperties<Impl: IEmbeddedAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEmbeddedAudioTrack, BASE_OFFSET>(),
            GetAudioEncodingProperties: GetAudioEncodingProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmbeddedAudioTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaClipImpl: Sized {
    fn TrimTimeFromStart(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromStart(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimTimeFromEnd(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromEnd(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn OriginalDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrimmedDuration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Clone(&mut self) -> ::windows::core::Result<MediaClip>;
    fn StartTimeInComposition(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn EndTimeInComposition(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn EmbeddedAudioTracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>>;
    fn SelectedEmbeddedAudioTrackIndex(&mut self) -> ::windows::core::Result<u32>;
    fn SetSelectedEmbeddedAudioTrackIndex(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn SetVolume(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&mut self) -> ::windows::core::Result<f64>;
    fn GetVideoEncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn AudioEffectDefinitions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn VideoEffectDefinitions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaClip {
    const NAME: &'static str = "Windows.Media.Editing.IMediaClip";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaClipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaClipVtbl {
        unsafe extern "system" fn TrimTimeFromStart<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimTimeFromStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimTimeFromStart<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimTimeFromStart(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrimTimeFromEnd<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimTimeFromEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimTimeFromEnd<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimTimeFromEnd(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OriginalDuration<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrimmedDuration<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimmedDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserData<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTimeInComposition<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTimeInComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndTimeInComposition<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTimeInComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmbeddedAudioTracks<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmbeddedAudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedEmbeddedAudioTrackIndex<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedEmbeddedAudioTrackIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedEmbeddedAudioTrackIndex<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedEmbeddedAudioTrackIndex(value).into()
        }
        unsafe extern "system" fn SetVolume<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        unsafe extern "system" fn Volume<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetVideoEncodingProperties<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioEffectDefinitions<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEffectDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoEffectDefinitions<Impl: IMediaClipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoEffectDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaClip, BASE_OFFSET>(),
            TrimTimeFromStart: TrimTimeFromStart::<Impl, IMPL_OFFSET>,
            SetTrimTimeFromStart: SetTrimTimeFromStart::<Impl, IMPL_OFFSET>,
            TrimTimeFromEnd: TrimTimeFromEnd::<Impl, IMPL_OFFSET>,
            SetTrimTimeFromEnd: SetTrimTimeFromEnd::<Impl, IMPL_OFFSET>,
            OriginalDuration: OriginalDuration::<Impl, IMPL_OFFSET>,
            TrimmedDuration: TrimmedDuration::<Impl, IMPL_OFFSET>,
            UserData: UserData::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            StartTimeInComposition: StartTimeInComposition::<Impl, IMPL_OFFSET>,
            EndTimeInComposition: EndTimeInComposition::<Impl, IMPL_OFFSET>,
            EmbeddedAudioTracks: EmbeddedAudioTracks::<Impl, IMPL_OFFSET>,
            SelectedEmbeddedAudioTrackIndex: SelectedEmbeddedAudioTrackIndex::<Impl, IMPL_OFFSET>,
            SetSelectedEmbeddedAudioTrackIndex: SetSelectedEmbeddedAudioTrackIndex::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            GetVideoEncodingProperties: GetVideoEncodingProperties::<Impl, IMPL_OFFSET>,
            AudioEffectDefinitions: AudioEffectDefinitions::<Impl, IMPL_OFFSET>,
            VideoEffectDefinitions: VideoEffectDefinitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaClip as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "UI", feature = "implement_exclusive"))]
pub trait IMediaClipStaticsImpl: Sized {
    fn CreateFromColor(&mut self, color: &super::super::UI::Color, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip>;
    fn CreateFromFileAsync(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>;
    fn CreateFromImageFileAsync(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaClipStatics {
    const NAME: &'static str = "Windows.Media.Editing.IMediaClipStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "UI", feature = "implement_exclusive"))]
impl IMediaClipStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaClipStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaClipStaticsVtbl {
        unsafe extern "system" fn CreateFromColor<Impl: IMediaClipStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: super::super::UI::Color, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromColor(&*(&color as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType), &*(&originalduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromFileAsync<Impl: IMediaClipStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromFileAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromImageFileAsync<Impl: IMediaClipStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromImageFileAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&originalduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaClipStatics, BASE_OFFSET>(),
            CreateFromColor: CreateFromColor::<Impl, IMPL_OFFSET>,
            CreateFromFileAsync: CreateFromFileAsync::<Impl, IMPL_OFFSET>,
            CreateFromImageFileAsync: CreateFromImageFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaClipStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaClipStatics2Impl: Sized {
    fn CreateFromSurface(&mut self, surface: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaClipStatics2 {
    const NAME: &'static str = "Windows.Media.Editing.IMediaClipStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaClipStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaClipStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaClipStatics2Vtbl {
        unsafe extern "system" fn CreateFromSurface<Impl: IMediaClipStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromSurface(&*(&surface as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), &*(&originalduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaClipStatics2, BASE_OFFSET>(),
            CreateFromSurface: CreateFromSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaClipStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaCompositionImpl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Clips(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaClip>>;
    fn BackgroundAudioTracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>>;
    fn UserData(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Clone(&mut self) -> ::windows::core::Result<MediaComposition>;
    fn SaveAsync(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetThumbnailAsync(&mut self, timefromstart: &super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>>;
    fn GetThumbnailsAsync(&mut self, timesfromstart: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan>>, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>>;
    fn RenderToFileAsync(&mut self, destination: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn RenderToFileWithTrimmingPreferenceAsync(&mut self, destination: &::core::option::Option<super::super::Storage::IStorageFile>, trimmingpreference: MediaTrimmingPreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn RenderToFileWithProfileAsync(&mut self, destination: &::core::option::Option<super::super::Storage::IStorageFile>, trimmingpreference: MediaTrimmingPreference, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn CreateDefaultEncodingProfile(&mut self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile>;
    fn GenerateMediaStreamSource(&mut self) -> ::windows::core::Result<super::Core::MediaStreamSource>;
    fn GenerateMediaStreamSourceWithProfile(&mut self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::Core::MediaStreamSource>;
    fn GeneratePreviewMediaStreamSource(&mut self, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::Core::MediaStreamSource>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaComposition {
    const NAME: &'static str = "Windows.Media.Editing.IMediaComposition";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaCompositionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCompositionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCompositionVtbl {
        unsafe extern "system" fn Duration<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clips<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clips() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundAudioTracks<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundAudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserData<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailAsync<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timefromstart: super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsync(&*(&timefromstart as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), scaledwidth, scaledheight, frameprecision) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailsAsync<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timesfromstart: ::windows::core::RawPtr, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailsAsync(&*(&timesfromstart as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType), scaledwidth, scaledheight, frameprecision) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderToFileAsync<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderToFileAsync(&*(&destination as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderToFileWithTrimmingPreferenceAsync<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, trimmingpreference: MediaTrimmingPreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderToFileWithTrimmingPreferenceAsync(&*(&destination as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), trimmingpreference) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderToFileWithProfileAsync<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, trimmingpreference: MediaTrimmingPreference, encodingprofile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderToFileWithProfileAsync(&*(&destination as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), trimmingpreference, &*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultEncodingProfile<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDefaultEncodingProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateMediaStreamSource<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateMediaStreamSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateMediaStreamSourceWithProfile<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateMediaStreamSourceWithProfile(&*(&encodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeneratePreviewMediaStreamSource<Impl: IMediaCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaledwidth: i32, scaledheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeneratePreviewMediaStreamSource(scaledwidth, scaledheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaComposition, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            Clips: Clips::<Impl, IMPL_OFFSET>,
            BackgroundAudioTracks: BackgroundAudioTracks::<Impl, IMPL_OFFSET>,
            UserData: UserData::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            GetThumbnailAsync: GetThumbnailAsync::<Impl, IMPL_OFFSET>,
            GetThumbnailsAsync: GetThumbnailsAsync::<Impl, IMPL_OFFSET>,
            RenderToFileAsync: RenderToFileAsync::<Impl, IMPL_OFFSET>,
            RenderToFileWithTrimmingPreferenceAsync: RenderToFileWithTrimmingPreferenceAsync::<Impl, IMPL_OFFSET>,
            RenderToFileWithProfileAsync: RenderToFileWithProfileAsync::<Impl, IMPL_OFFSET>,
            CreateDefaultEncodingProfile: CreateDefaultEncodingProfile::<Impl, IMPL_OFFSET>,
            GenerateMediaStreamSource: GenerateMediaStreamSource::<Impl, IMPL_OFFSET>,
            GenerateMediaStreamSourceWithProfile: GenerateMediaStreamSourceWithProfile::<Impl, IMPL_OFFSET>,
            GeneratePreviewMediaStreamSource: GeneratePreviewMediaStreamSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaComposition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaComposition2Impl: Sized {
    fn OverlayLayers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlayLayer>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaComposition2 {
    const NAME: &'static str = "Windows.Media.Editing.IMediaComposition2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaComposition2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaComposition2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaComposition2Vtbl {
        unsafe extern "system" fn OverlayLayers<Impl: IMediaComposition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverlayLayers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaComposition2, BASE_OFFSET>(), OverlayLayers: OverlayLayers::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaComposition2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait IMediaCompositionStaticsImpl: Sized {
    fn LoadAsync(&mut self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaComposition>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaCompositionStatics {
    const NAME: &'static str = "Windows.Media.Editing.IMediaCompositionStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl IMediaCompositionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCompositionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCompositionStaticsVtbl {
        unsafe extern "system" fn LoadAsync<Impl: IMediaCompositionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadAsync(&*(&file as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCompositionStatics, BASE_OFFSET>(), LoadAsync: LoadAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCompositionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaOverlayImpl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetPosition(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Delay(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Opacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<MediaOverlay>;
    fn Clip(&mut self) -> ::windows::core::Result<MediaClip>;
    fn AudioEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAudioEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaOverlay {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlay";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaOverlayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaOverlayVtbl {
        unsafe extern "system" fn Position<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDelay<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Delay<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Opacity<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn Clone<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioEnabled<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEnabled<Impl: IMediaOverlayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaOverlay, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
            Delay: Delay::<Impl, IMPL_OFFSET>,
            Opacity: Opacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Clip: Clip::<Impl, IMPL_OFFSET>,
            AudioEnabled: AudioEnabled::<Impl, IMPL_OFFSET>,
            SetAudioEnabled: SetAudioEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaOverlay as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaOverlayFactoryImpl: Sized {
    fn Create(&mut self, clip: &::core::option::Option<MediaClip>) -> ::windows::core::Result<MediaOverlay>;
    fn CreateWithPositionAndOpacity(&mut self, clip: &::core::option::Option<MediaClip>, position: &super::super::Foundation::Rect, opacity: f64) -> ::windows::core::Result<MediaOverlay>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaOverlayFactory {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlayFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaOverlayFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaOverlayFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMediaOverlayFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&clip as *const <MediaClip as ::windows::core::Abi>::Abi as *const <MediaClip as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPositionAndOpacity<Impl: IMediaOverlayFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr, position: super::super::Foundation::Rect, opacity: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithPositionAndOpacity(&*(&clip as *const <MediaClip as ::windows::core::Abi>::Abi as *const <MediaClip as ::windows::core::DefaultType>::DefaultType), &*(&position as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), opacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaOverlayFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithPositionAndOpacity: CreateWithPositionAndOpacity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaOverlayFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IMediaOverlayLayerImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<MediaOverlayLayer>;
    fn Overlays(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlay>>;
    fn CustomCompositorDefinition(&mut self) -> ::windows::core::Result<super::Effects::IVideoCompositorDefinition>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaOverlayLayer {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlayLayer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "implement_exclusive"))]
impl IMediaOverlayLayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayLayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaOverlayLayerVtbl {
        unsafe extern "system" fn Clone<Impl: IMediaOverlayLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Overlays<Impl: IMediaOverlayLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Overlays() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomCompositorDefinition<Impl: IMediaOverlayLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomCompositorDefinition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaOverlayLayer, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Overlays: Overlays::<Impl, IMPL_OFFSET>,
            CustomCompositorDefinition: CustomCompositorDefinition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaOverlayLayer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IMediaOverlayLayerFactoryImpl: Sized {
    fn CreateWithCompositorDefinition(&mut self, compositordefinition: &::core::option::Option<super::Effects::IVideoCompositorDefinition>) -> ::windows::core::Result<MediaOverlayLayer>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaOverlayLayerFactory {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlayLayerFactory";
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl IMediaOverlayLayerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayLayerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaOverlayLayerFactoryVtbl {
        unsafe extern "system" fn CreateWithCompositorDefinition<Impl: IMediaOverlayLayerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositordefinition: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithCompositorDefinition(&*(&compositordefinition as *const <super::Effects::IVideoCompositorDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IVideoCompositorDefinition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaOverlayLayerFactory, BASE_OFFSET>(),
            CreateWithCompositorDefinition: CreateWithCompositorDefinition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaOverlayLayerFactory as ::windows::core::Interface>::IID
    }
}
