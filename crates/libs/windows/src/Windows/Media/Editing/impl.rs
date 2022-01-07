#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundAudioTrackImpl: Sized {
    fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromEnd(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SetDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn Clone(&self) -> ::windows::core::Result<BackgroundAudioTrack>;
    fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.IBackgroundAudioTrack";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundAudioTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundAudioTrackImpl, const OFFSET: isize>() -> IBackgroundAudioTrackVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBackgroundAudioTrack>,
            ::windows::core::GetTrustLevel,
            TrimTimeFromStart::<Impl, OFFSET>,
            SetTrimTimeFromStart::<Impl, OFFSET>,
            TrimTimeFromEnd::<Impl, OFFSET>,
            SetTrimTimeFromEnd::<Impl, OFFSET>,
            OriginalDuration::<Impl, OFFSET>,
            TrimmedDuration::<Impl, OFFSET>,
            UserData::<Impl, OFFSET>,
            SetDelay::<Impl, OFFSET>,
            Delay::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            Volume::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            GetAudioEncodingProperties::<Impl, OFFSET>,
            AudioEffectDefinitions::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundAudioTrackStaticsImpl: Sized {
    fn CreateFromEmbeddedAudioTrack(&self, embeddedaudiotrack: &::core::option::Option<EmbeddedAudioTrack>) -> ::windows::core::Result<BackgroundAudioTrack>;
    fn CreateFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundAudioTrackStatics {
    const NAME: &'static str = "Windows.Media.Editing.IBackgroundAudioTrackStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundAudioTrackStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundAudioTrackStaticsImpl, const OFFSET: isize>() -> IBackgroundAudioTrackStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBackgroundAudioTrackStatics>, ::windows::core::GetTrustLevel, CreateFromEmbeddedAudioTrack::<Impl, OFFSET>, CreateFromFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmbeddedAudioTrackImpl: Sized {
    fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmbeddedAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.IEmbeddedAudioTrack";
}
#[cfg(feature = "implement_exclusive")]
impl IEmbeddedAudioTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmbeddedAudioTrackImpl, const OFFSET: isize>() -> IEmbeddedAudioTrackVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmbeddedAudioTrack>, ::windows::core::GetTrustLevel, GetAudioEncodingProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaClipImpl: Sized {
    fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimTimeFromEnd(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Clone(&self) -> ::windows::core::Result<MediaClip>;
    fn StartTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn EndTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn EmbeddedAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>>;
    fn SelectedEmbeddedAudioTrackIndex(&self) -> ::windows::core::Result<u32>;
    fn SetSelectedEmbeddedAudioTrackIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn GetVideoEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn VideoEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaClip {
    const NAME: &'static str = "Windows.Media.Editing.IMediaClip";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaClipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaClipImpl, const OFFSET: isize>() -> IMediaClipVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaClip>,
            ::windows::core::GetTrustLevel,
            TrimTimeFromStart::<Impl, OFFSET>,
            SetTrimTimeFromStart::<Impl, OFFSET>,
            TrimTimeFromEnd::<Impl, OFFSET>,
            SetTrimTimeFromEnd::<Impl, OFFSET>,
            OriginalDuration::<Impl, OFFSET>,
            TrimmedDuration::<Impl, OFFSET>,
            UserData::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            StartTimeInComposition::<Impl, OFFSET>,
            EndTimeInComposition::<Impl, OFFSET>,
            EmbeddedAudioTracks::<Impl, OFFSET>,
            SelectedEmbeddedAudioTrackIndex::<Impl, OFFSET>,
            SetSelectedEmbeddedAudioTrackIndex::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            Volume::<Impl, OFFSET>,
            GetVideoEncodingProperties::<Impl, OFFSET>,
            AudioEffectDefinitions::<Impl, OFFSET>,
            VideoEffectDefinitions::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaClipStaticsImpl: Sized {
    fn CreateFromColor(&self, color: &super::super::UI::Color, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip>;
    fn CreateFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>;
    fn CreateFromImageFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaClipStatics {
    const NAME: &'static str = "Windows.Media.Editing.IMediaClipStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaClipStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaClipStaticsImpl, const OFFSET: isize>() -> IMediaClipStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaClipStatics>, ::windows::core::GetTrustLevel, CreateFromColor::<Impl, OFFSET>, CreateFromFileAsync::<Impl, OFFSET>, CreateFromImageFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaClipStatics2Impl: Sized {
    fn CreateFromSurface(&self, surface: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, originalduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaClipStatics2 {
    const NAME: &'static str = "Windows.Media.Editing.IMediaClipStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaClipStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaClipStatics2Impl, const OFFSET: isize>() -> IMediaClipStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaClipStatics2>, ::windows::core::GetTrustLevel, CreateFromSurface::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCompositionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Clips(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaClip>>;
    fn BackgroundAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>>;
    fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Clone(&self) -> ::windows::core::Result<MediaComposition>;
    fn SaveAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetThumbnailAsync(&self, timefromstart: &super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>>;
    fn GetThumbnailsAsync(&self, timesfromstart: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan>>, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>>;
    fn RenderToFileAsync(&self, destination: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn RenderToFileWithTrimmingPreferenceAsync(&self, destination: &::core::option::Option<super::super::Storage::IStorageFile>, trimmingpreference: MediaTrimmingPreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn RenderToFileWithProfileAsync(&self, destination: &::core::option::Option<super::super::Storage::IStorageFile>, trimmingpreference: MediaTrimmingPreference, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>;
    fn CreateDefaultEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile>;
    fn GenerateMediaStreamSource(&self) -> ::windows::core::Result<super::Core::MediaStreamSource>;
    fn GenerateMediaStreamSourceWithProfile(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::Core::MediaStreamSource>;
    fn GeneratePreviewMediaStreamSource(&self, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::Core::MediaStreamSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaComposition {
    const NAME: &'static str = "Windows.Media.Editing.IMediaComposition";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCompositionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCompositionImpl, const OFFSET: isize>() -> IMediaCompositionVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaComposition>,
            ::windows::core::GetTrustLevel,
            Duration::<Impl, OFFSET>,
            Clips::<Impl, OFFSET>,
            BackgroundAudioTracks::<Impl, OFFSET>,
            UserData::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            SaveAsync::<Impl, OFFSET>,
            GetThumbnailAsync::<Impl, OFFSET>,
            GetThumbnailsAsync::<Impl, OFFSET>,
            RenderToFileAsync::<Impl, OFFSET>,
            RenderToFileWithTrimmingPreferenceAsync::<Impl, OFFSET>,
            RenderToFileWithProfileAsync::<Impl, OFFSET>,
            CreateDefaultEncodingProfile::<Impl, OFFSET>,
            GenerateMediaStreamSource::<Impl, OFFSET>,
            GenerateMediaStreamSourceWithProfile::<Impl, OFFSET>,
            GeneratePreviewMediaStreamSource::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaComposition2Impl: Sized {
    fn OverlayLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlayLayer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaComposition2 {
    const NAME: &'static str = "Windows.Media.Editing.IMediaComposition2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaComposition2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaComposition2Impl, const OFFSET: isize>() -> IMediaComposition2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaComposition2>, ::windows::core::GetTrustLevel, OverlayLayers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCompositionStaticsImpl: Sized {
    fn LoadAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaComposition>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCompositionStatics {
    const NAME: &'static str = "Windows.Media.Editing.IMediaCompositionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCompositionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCompositionStaticsImpl, const OFFSET: isize>() -> IMediaCompositionStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaCompositionStatics>, ::windows::core::GetTrustLevel, LoadAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetPosition(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<MediaOverlay>;
    fn Clip(&self) -> ::windows::core::Result<MediaClip>;
    fn AudioEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAudioEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaOverlay {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlay";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaOverlayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayImpl, const OFFSET: isize>() -> IMediaOverlayVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaOverlay>,
            ::windows::core::GetTrustLevel,
            Position::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            SetDelay::<Impl, OFFSET>,
            Delay::<Impl, OFFSET>,
            Opacity::<Impl, OFFSET>,
            SetOpacity::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            Clip::<Impl, OFFSET>,
            AudioEnabled::<Impl, OFFSET>,
            SetAudioEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayFactoryImpl: Sized {
    fn Create(&self, clip: &::core::option::Option<MediaClip>) -> ::windows::core::Result<MediaOverlay>;
    fn CreateWithPositionAndOpacity(&self, clip: &::core::option::Option<MediaClip>, position: &super::super::Foundation::Rect, opacity: f64) -> ::windows::core::Result<MediaOverlay>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaOverlayFactory {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlayFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaOverlayFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayFactoryImpl, const OFFSET: isize>() -> IMediaOverlayFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaOverlayFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>, CreateWithPositionAndOpacity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayLayerImpl: Sized {
    fn Clone(&self) -> ::windows::core::Result<MediaOverlayLayer>;
    fn Overlays(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlay>>;
    fn CustomCompositorDefinition(&self) -> ::windows::core::Result<super::Effects::IVideoCompositorDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaOverlayLayer {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlayLayer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaOverlayLayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayLayerImpl, const OFFSET: isize>() -> IMediaOverlayLayerVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaOverlayLayer>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Overlays::<Impl, OFFSET>, CustomCompositorDefinition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaOverlayLayerFactoryImpl: Sized {
    fn CreateWithCompositorDefinition(&self, compositordefinition: &::core::option::Option<super::Effects::IVideoCompositorDefinition>) -> ::windows::core::Result<MediaOverlayLayer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaOverlayLayerFactory {
    const NAME: &'static str = "Windows.Media.Editing.IMediaOverlayLayerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaOverlayLayerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaOverlayLayerFactoryImpl, const OFFSET: isize>() -> IMediaOverlayLayerFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaOverlayLayerFactory>, ::windows::core::GetTrustLevel, CreateWithCompositorDefinition::<Impl, OFFSET>)
    }
}
