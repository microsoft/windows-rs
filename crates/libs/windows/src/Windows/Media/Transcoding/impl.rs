#[cfg(feature = "implement_exclusive")]
pub trait IMediaTranscoderImpl: Sized {
    fn SetTrimStartTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimStopTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimStopTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAlwaysReencode(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysReencode(&self) -> ::windows::core::Result<bool>;
    fn SetHardwareAccelerationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn HardwareAccelerationEnabled(&self) -> ::windows::core::Result<bool>;
    fn AddAudioEffect(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddAudioEffectWithSettings(&self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn AddVideoEffect(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddVideoEffectWithSettings(&self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn ClearEffects(&self) -> ::windows::core::Result<()>;
    fn PrepareFileTranscodeAsync(&self, source: &::core::option::Option<super::super::Storage::IStorageFile>, destination: &::core::option::Option<super::super::Storage::IStorageFile>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
    fn PrepareStreamTranscodeAsync(&self, source: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, destination: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTranscoder {
    const NAME: &'static str = "Windows.Media.Transcoding.IMediaTranscoder";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTranscoderVtbl {
    pub const fn new<Impl: IMediaTranscoderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTranscoderVtbl {
        unsafe extern "system" fn SetTrimStartTime<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTrimStartTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrimStartTime<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrimStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimStopTime<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTrimStopTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrimStopTime<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrimStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysReencode<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlwaysReencode(value).into()
        }
        unsafe extern "system" fn AlwaysReencode<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlwaysReencode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHardwareAccelerationEnabled<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHardwareAccelerationEnabled(value).into()
        }
        unsafe extern "system" fn HardwareAccelerationEnabled<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HardwareAccelerationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAudioEffect<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddAudioEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddAudioEffectWithSettings<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddAudioEffectWithSettings(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectrequired, &*(&configuration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddVideoEffect<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddVideoEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddVideoEffectWithSettings<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddVideoEffectWithSettings(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectrequired, &*(&configuration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearEffects<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearEffects().into()
        }
        unsafe extern "system" fn PrepareFileTranscodeAsync<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareFileTranscodeAsync(
                &*(&source as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&destination as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&profile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareStreamTranscodeAsync<Impl: IMediaTranscoderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareStreamTranscodeAsync(
                &*(&source as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&destination as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&profile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IMediaTranscoder>,
            base.5,
            SetTrimStartTime::<Impl, OFFSET>,
            TrimStartTime::<Impl, OFFSET>,
            SetTrimStopTime::<Impl, OFFSET>,
            TrimStopTime::<Impl, OFFSET>,
            SetAlwaysReencode::<Impl, OFFSET>,
            AlwaysReencode::<Impl, OFFSET>,
            SetHardwareAccelerationEnabled::<Impl, OFFSET>,
            HardwareAccelerationEnabled::<Impl, OFFSET>,
            AddAudioEffect::<Impl, OFFSET>,
            AddAudioEffectWithSettings::<Impl, OFFSET>,
            AddVideoEffect::<Impl, OFFSET>,
            AddVideoEffectWithSettings::<Impl, OFFSET>,
            ClearEffects::<Impl, OFFSET>,
            PrepareFileTranscodeAsync::<Impl, OFFSET>,
            PrepareStreamTranscodeAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTranscoder2Impl: Sized {
    fn PrepareMediaStreamSourceTranscodeAsync(&self, source: &::core::option::Option<super::Core::IMediaSource>, destination: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
    fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()>;
    fn VideoProcessingAlgorithm(&self) -> ::windows::core::Result<MediaVideoProcessingAlgorithm>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTranscoder2 {
    const NAME: &'static str = "Windows.Media.Transcoding.IMediaTranscoder2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTranscoder2Vtbl {
    pub const fn new<Impl: IMediaTranscoder2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTranscoder2Vtbl {
        unsafe extern "system" fn PrepareMediaStreamSourceTranscodeAsync<Impl: IMediaTranscoder2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrepareMediaStreamSourceTranscodeAsync(
                &*(&source as *const <super::Core::IMediaSource as ::windows::core::Abi>::Abi as *const <super::Core::IMediaSource as ::windows::core::DefaultType>::DefaultType),
                &*(&destination as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&profile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoProcessingAlgorithm<Impl: IMediaTranscoder2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoProcessingAlgorithm(value).into()
        }
        unsafe extern "system" fn VideoProcessingAlgorithm<Impl: IMediaTranscoder2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoProcessingAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTranscoder2>, base.5, PrepareMediaStreamSourceTranscodeAsync::<Impl, OFFSET>, SetVideoProcessingAlgorithm::<Impl, OFFSET>, VideoProcessingAlgorithm::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrepareTranscodeResultImpl: Sized {
    fn CanTranscode(&self) -> ::windows::core::Result<bool>;
    fn FailureReason(&self) -> ::windows::core::Result<TranscodeFailureReason>;
    fn TranscodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrepareTranscodeResult {
    const NAME: &'static str = "Windows.Media.Transcoding.IPrepareTranscodeResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPrepareTranscodeResultVtbl {
    pub const fn new<Impl: IPrepareTranscodeResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrepareTranscodeResultVtbl {
        unsafe extern "system" fn CanTranscode<Impl: IPrepareTranscodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanTranscode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FailureReason<Impl: IPrepareTranscodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TranscodeFailureReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FailureReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranscodeAsync<Impl: IPrepareTranscodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranscodeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrepareTranscodeResult>, base.5, CanTranscode::<Impl, OFFSET>, FailureReason::<Impl, OFFSET>, TranscodeAsync::<Impl, OFFSET>)
    }
}
