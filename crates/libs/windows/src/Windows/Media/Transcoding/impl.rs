#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaTranscoder_Impl: Sized {
    fn SetTrimStartTime(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimStartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTrimStopTime(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TrimStopTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAlwaysReencode(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysReencode(&mut self) -> ::windows::core::Result<bool>;
    fn SetHardwareAccelerationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn HardwareAccelerationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn AddAudioEffect(&mut self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddAudioEffectWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn AddVideoEffect(&mut self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddVideoEffectWithSettings(&mut self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn ClearEffects(&mut self) -> ::windows::core::Result<()>;
    fn PrepareFileTranscodeAsync(&mut self, source: &::core::option::Option<super::super::Storage::IStorageFile>, destination: &::core::option::Option<super::super::Storage::IStorageFile>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
    fn PrepareStreamTranscodeAsync(&mut self, source: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, destination: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTranscoder {
    const NAME: &'static str = "Windows.Media.Transcoding.IMediaTranscoder";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaTranscoder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTranscoder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTranscoder_Vtbl {
        unsafe extern "system" fn SetTrimStartTime<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimStartTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrimStartTime<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimStopTime<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimStopTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrimStopTime<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrimStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysReencode<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysReencode(value).into()
        }
        unsafe extern "system" fn AlwaysReencode<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysReencode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHardwareAccelerationEnabled<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHardwareAccelerationEnabled(value).into()
        }
        unsafe extern "system" fn HardwareAccelerationEnabled<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareAccelerationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAudioEffect<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAudioEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddAudioEffectWithSettings<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAudioEffectWithSettings(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectrequired, &*(&configuration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddVideoEffect<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddVideoEffect(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddVideoEffectWithSettings<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddVideoEffectWithSettings(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), effectrequired, &*(&configuration as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearEffects<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearEffects().into()
        }
        unsafe extern "system" fn PrepareFileTranscodeAsync<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn PrepareStreamTranscodeAsync<Impl: IMediaTranscoder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTranscoder, BASE_OFFSET>(),
            SetTrimStartTime: SetTrimStartTime::<Impl, IMPL_OFFSET>,
            TrimStartTime: TrimStartTime::<Impl, IMPL_OFFSET>,
            SetTrimStopTime: SetTrimStopTime::<Impl, IMPL_OFFSET>,
            TrimStopTime: TrimStopTime::<Impl, IMPL_OFFSET>,
            SetAlwaysReencode: SetAlwaysReencode::<Impl, IMPL_OFFSET>,
            AlwaysReencode: AlwaysReencode::<Impl, IMPL_OFFSET>,
            SetHardwareAccelerationEnabled: SetHardwareAccelerationEnabled::<Impl, IMPL_OFFSET>,
            HardwareAccelerationEnabled: HardwareAccelerationEnabled::<Impl, IMPL_OFFSET>,
            AddAudioEffect: AddAudioEffect::<Impl, IMPL_OFFSET>,
            AddAudioEffectWithSettings: AddAudioEffectWithSettings::<Impl, IMPL_OFFSET>,
            AddVideoEffect: AddVideoEffect::<Impl, IMPL_OFFSET>,
            AddVideoEffectWithSettings: AddVideoEffectWithSettings::<Impl, IMPL_OFFSET>,
            ClearEffects: ClearEffects::<Impl, IMPL_OFFSET>,
            PrepareFileTranscodeAsync: PrepareFileTranscodeAsync::<Impl, IMPL_OFFSET>,
            PrepareStreamTranscodeAsync: PrepareStreamTranscodeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTranscoder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaTranscoder2_Impl: Sized {
    fn PrepareMediaStreamSourceTranscodeAsync(&mut self, source: &::core::option::Option<super::Core::IMediaSource>, destination: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, profile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>;
    fn SetVideoProcessingAlgorithm(&mut self, value: MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()>;
    fn VideoProcessingAlgorithm(&mut self) -> ::windows::core::Result<MediaVideoProcessingAlgorithm>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTranscoder2 {
    const NAME: &'static str = "Windows.Media.Transcoding.IMediaTranscoder2";
}
#[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaTranscoder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTranscoder2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTranscoder2_Vtbl {
        unsafe extern "system" fn PrepareMediaStreamSourceTranscodeAsync<Impl: IMediaTranscoder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetVideoProcessingAlgorithm<Impl: IMediaTranscoder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoProcessingAlgorithm(value).into()
        }
        unsafe extern "system" fn VideoProcessingAlgorithm<Impl: IMediaTranscoder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProcessingAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTranscoder2, BASE_OFFSET>(),
            PrepareMediaStreamSourceTranscodeAsync: PrepareMediaStreamSourceTranscodeAsync::<Impl, IMPL_OFFSET>,
            SetVideoProcessingAlgorithm: SetVideoProcessingAlgorithm::<Impl, IMPL_OFFSET>,
            VideoProcessingAlgorithm: VideoProcessingAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTranscoder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrepareTranscodeResult_Impl: Sized {
    fn CanTranscode(&mut self) -> ::windows::core::Result<bool>;
    fn FailureReason(&mut self) -> ::windows::core::Result<TranscodeFailureReason>;
    fn TranscodeAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrepareTranscodeResult {
    const NAME: &'static str = "Windows.Media.Transcoding.IPrepareTranscodeResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrepareTranscodeResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrepareTranscodeResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrepareTranscodeResult_Vtbl {
        unsafe extern "system" fn CanTranscode<Impl: IPrepareTranscodeResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanTranscode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FailureReason<Impl: IPrepareTranscodeResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TranscodeFailureReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FailureReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranscodeAsync<Impl: IPrepareTranscodeResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranscodeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrepareTranscodeResult, BASE_OFFSET>(),
            CanTranscode: CanTranscode::<Impl, IMPL_OFFSET>,
            FailureReason: FailureReason::<Impl, IMPL_OFFSET>,
            TranscodeAsync: TranscodeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrepareTranscodeResult as ::windows::core::Interface>::IID
    }
}
