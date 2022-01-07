#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioDeviceInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioDeviceInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioDeviceInputNode";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioDeviceInputNodeVtbl {
    pub const fn new<Impl: IAudioDeviceInputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioDeviceInputNodeVtbl {
        unsafe extern "system" fn Device<Impl: IAudioDeviceInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioDeviceInputNode>, base.5, Device::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioDeviceOutputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioDeviceOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioDeviceOutputNode";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioDeviceOutputNodeVtbl {
    pub const fn new<Impl: IAudioDeviceOutputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioDeviceOutputNodeVtbl {
        unsafe extern "system" fn Device<Impl: IAudioDeviceOutputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioDeviceOutputNode>, base.5, Device::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFileInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Seek(&self, position: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetEndTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetLoopCount(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn FileCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioFileInputNode, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioFileInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioFileInputNode";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioFileInputNodeVtbl {
    pub const fn new<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFileInputNodeVtbl {
        unsafe extern "system" fn SetPlaybackSpeedFactor<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaybackSpeedFactor(value).into()
        }
        unsafe extern "system" fn PlaybackSpeedFactor<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackSpeedFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Seek<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Seek(&*(&position as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStartTime<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndTime<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEndTime<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoopCount<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoopCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoopCount<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLoopCount(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceFile<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileCompleted<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioFileInputNode, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioFileInputNode, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFileCompleted<Impl: IAudioFileInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFileCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAudioFileInputNode>,
            base.5,
            SetPlaybackSpeedFactor::<Impl, OFFSET>,
            PlaybackSpeedFactor::<Impl, OFFSET>,
            Position::<Impl, OFFSET>,
            Seek::<Impl, OFFSET>,
            StartTime::<Impl, OFFSET>,
            SetStartTime::<Impl, OFFSET>,
            EndTime::<Impl, OFFSET>,
            SetEndTime::<Impl, OFFSET>,
            LoopCount::<Impl, OFFSET>,
            SetLoopCount::<Impl, OFFSET>,
            Duration::<Impl, OFFSET>,
            SourceFile::<Impl, OFFSET>,
            FileCompleted::<Impl, OFFSET>,
            RemoveFileCompleted::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFileOutputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn FileEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile>;
    fn FinalizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioFileOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioFileOutputNode";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioFileOutputNodeVtbl {
    pub const fn new<Impl: IAudioFileOutputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFileOutputNodeVtbl {
        unsafe extern "system" fn File<Impl: IAudioFileOutputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileEncodingProfile<Impl: IAudioFileOutputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileEncodingProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalizeAsync<Impl: IAudioFileOutputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinalizeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFileOutputNode>, base.5, File::<Impl, OFFSET>, FileEncodingProfile::<Impl, OFFSET>, FinalizeAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioFrameCompletedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<super::AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioFrameCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.IAudioFrameCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioFrameCompletedEventArgsVtbl {
    pub const fn new<Impl: IAudioFrameCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFrameCompletedEventArgsVtbl {
        unsafe extern "system" fn Frame<Impl: IAudioFrameCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFrameCompletedEventArgs>, base.5, Frame::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFrameInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
    fn AddFrame(&self, frame: &::core::option::Option<super::AudioFrame>) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
    fn QueuedSampleCount(&self) -> ::windows::core::Result<u64>;
    fn AudioFrameCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioFrameCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn QuantumStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuantumStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioFrameInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioFrameInputNode";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioFrameInputNodeVtbl {
    pub const fn new<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFrameInputNodeVtbl {
        unsafe extern "system" fn SetPlaybackSpeedFactor<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaybackSpeedFactor(value).into()
        }
        unsafe extern "system" fn PlaybackSpeedFactor<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackSpeedFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFrame<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddFrame(&*(&frame as *const <super::AudioFrame as ::windows::core::Abi>::Abi as *const <super::AudioFrame as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        unsafe extern "system" fn QueuedSampleCount<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueuedSampleCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFrameCompleted<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFrameCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioFrameCompleted<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAudioFrameCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuantumStarted<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuantumStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveQuantumStarted<Impl: IAudioFrameInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveQuantumStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFrameInputNode>, base.5, SetPlaybackSpeedFactor::<Impl, OFFSET>, PlaybackSpeedFactor::<Impl, OFFSET>, AddFrame::<Impl, OFFSET>, DiscardQueuedFrames::<Impl, OFFSET>, QueuedSampleCount::<Impl, OFFSET>, AudioFrameCompleted::<Impl, OFFSET>, RemoveAudioFrameCompleted::<Impl, OFFSET>, QuantumStarted::<Impl, OFFSET>, RemoveQuantumStarted::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFrameOutputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn GetFrame(&self) -> ::windows::core::Result<super::AudioFrame>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioFrameOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioFrameOutputNode";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioFrameOutputNodeVtbl {
    pub const fn new<Impl: IAudioFrameOutputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFrameOutputNodeVtbl {
        unsafe extern "system" fn GetFrame<Impl: IAudioFrameOutputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFrameOutputNode>, base.5, GetFrame::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioGraphImpl: Sized + IClosableImpl {
    fn CreateFrameInputNode(&self) -> ::windows::core::Result<AudioFrameInputNode>;
    fn CreateFrameInputNodeWithFormat(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioFrameInputNode>;
    fn CreateDeviceInputNodeAsync(&self, category: super::Capture::MediaCategory) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateDeviceInputNodeWithFormatAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateDeviceInputNodeWithFormatOnDeviceAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, device: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateFrameOutputNode(&self) -> ::windows::core::Result<AudioFrameOutputNode>;
    fn CreateFrameOutputNodeWithFormat(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioFrameOutputNode>;
    fn CreateDeviceOutputNodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>>;
    fn CreateFileInputNodeAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>;
    fn CreateFileOutputNodeAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>;
    fn CreateFileOutputNodeWithFileProfileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, fileencodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>;
    fn CreateSubmixNode(&self) -> ::windows::core::Result<AudioSubmixNode>;
    fn CreateSubmixNodeWithFormat(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioSubmixNode>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn ResetAllNodes(&self) -> ::windows::core::Result<()>;
    fn QuantumStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuantumStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn QuantumProcessed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuantumProcessed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnrecoverableErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnrecoverableErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompletedQuantumCount(&self) -> ::windows::core::Result<u64>;
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn LatencyInSamples(&self) -> ::windows::core::Result<i32>;
    fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
    fn RenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
    fn SamplesPerQuantum(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioGraph {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraph";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioGraphVtbl {
    pub const fn new<Impl: IAudioGraphImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraphVtbl {
        unsafe extern "system" fn CreateFrameInputNode<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameInputNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameInputNodeWithFormat<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameInputNodeWithFormat(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceInputNodeAsync<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceInputNodeAsync(category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceInputNodeWithFormatAsync<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceInputNodeWithFormatAsync(category, &*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceInputNodeWithFormatOnDeviceAsync<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceInputNodeWithFormatOnDeviceAsync(category, &*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameOutputNode<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameOutputNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFrameOutputNodeWithFormat<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameOutputNodeWithFormat(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceOutputNodeAsync<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceOutputNodeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileInputNodeAsync<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFileInputNodeAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileOutputNodeAsync<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFileOutputNodeAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileOutputNodeWithFileProfileAsync<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, fileencodingprofile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFileOutputNodeWithFileProfileAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&fileencodingprofile as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::Abi>::Abi as *const <super::MediaProperties::MediaEncodingProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubmixNode<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSubmixNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubmixNodeWithFormat<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSubmixNodeWithFormat(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn ResetAllNodes<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResetAllNodes().into()
        }
        unsafe extern "system" fn QuantumStarted<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuantumStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveQuantumStarted<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveQuantumStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuantumProcessed<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuantumProcessed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveQuantumProcessed<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveQuantumProcessed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnrecoverableErrorOccurred<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnrecoverableErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnrecoverableErrorOccurred<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUnrecoverableErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompletedQuantumCount<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompletedQuantumCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingProperties<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LatencyInSamples<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LatencyInSamples() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryRenderDevice<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryRenderDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderDeviceAudioProcessing<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderDeviceAudioProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SamplesPerQuantum<Impl: IAudioGraphImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SamplesPerQuantum() {
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
            ::windows::core::GetRuntimeClassName::<IAudioGraph>,
            base.5,
            CreateFrameInputNode::<Impl, OFFSET>,
            CreateFrameInputNodeWithFormat::<Impl, OFFSET>,
            CreateDeviceInputNodeAsync::<Impl, OFFSET>,
            CreateDeviceInputNodeWithFormatAsync::<Impl, OFFSET>,
            CreateDeviceInputNodeWithFormatOnDeviceAsync::<Impl, OFFSET>,
            CreateFrameOutputNode::<Impl, OFFSET>,
            CreateFrameOutputNodeWithFormat::<Impl, OFFSET>,
            CreateDeviceOutputNodeAsync::<Impl, OFFSET>,
            CreateFileInputNodeAsync::<Impl, OFFSET>,
            CreateFileOutputNodeAsync::<Impl, OFFSET>,
            CreateFileOutputNodeWithFileProfileAsync::<Impl, OFFSET>,
            CreateSubmixNode::<Impl, OFFSET>,
            CreateSubmixNodeWithFormat::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            ResetAllNodes::<Impl, OFFSET>,
            QuantumStarted::<Impl, OFFSET>,
            RemoveQuantumStarted::<Impl, OFFSET>,
            QuantumProcessed::<Impl, OFFSET>,
            RemoveQuantumProcessed::<Impl, OFFSET>,
            UnrecoverableErrorOccurred::<Impl, OFFSET>,
            RemoveUnrecoverableErrorOccurred::<Impl, OFFSET>,
            CompletedQuantumCount::<Impl, OFFSET>,
            EncodingProperties::<Impl, OFFSET>,
            LatencyInSamples::<Impl, OFFSET>,
            PrimaryRenderDevice::<Impl, OFFSET>,
            RenderDeviceAudioProcessing::<Impl, OFFSET>,
            SamplesPerQuantum::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioGraph2Impl: Sized + IAudioGraphImpl + IClosableImpl {
    fn CreateFrameInputNodeWithFormatAndEmitter(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<AudioFrameInputNode>;
    fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, device: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateFileInputNodeWithEmitterAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>;
    fn CreateSubmixNodeWithFormatAndEmitter(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<AudioSubmixNode>;
    fn CreateBatchUpdater(&self) -> ::windows::core::Result<AudioGraphBatchUpdater>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioGraph2 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraph2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioGraph2Vtbl {
    pub const fn new<Impl: IAudioGraph2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraph2Vtbl {
        unsafe extern "system" fn CreateFrameInputNodeWithFormatAndEmitter<Impl: IAudioGraph2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameInputNodeWithFormatAndEmitter(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&emitter as *const <AudioNodeEmitter as ::windows::core::Abi>::Abi as *const <AudioNodeEmitter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync<Impl: IAudioGraph2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync(
                category,
                &*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType),
                &*(&device as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType),
                &*(&emitter as *const <AudioNodeEmitter as ::windows::core::Abi>::Abi as *const <AudioNodeEmitter as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileInputNodeWithEmitterAsync<Impl: IAudioGraph2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFileInputNodeWithEmitterAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&emitter as *const <AudioNodeEmitter as ::windows::core::Abi>::Abi as *const <AudioNodeEmitter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubmixNodeWithFormatAndEmitter<Impl: IAudioGraph2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSubmixNodeWithFormatAndEmitter(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&emitter as *const <AudioNodeEmitter as ::windows::core::Abi>::Abi as *const <AudioNodeEmitter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBatchUpdater<Impl: IAudioGraph2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBatchUpdater() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioGraph2>, base.5, CreateFrameInputNodeWithFormatAndEmitter::<Impl, OFFSET>, CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync::<Impl, OFFSET>, CreateFileInputNodeWithEmitterAsync::<Impl, OFFSET>, CreateSubmixNodeWithFormatAndEmitter::<Impl, OFFSET>, CreateBatchUpdater::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraph3Impl: Sized {
    fn CreateMediaSourceAudioInputNodeAsync(&self, mediasource: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>;
    fn CreateMediaSourceAudioInputNodeWithEmitterAsync(&self, mediasource: &::core::option::Option<super::Core::MediaSource>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioGraph3 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraph3";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioGraph3Vtbl {
    pub const fn new<Impl: IAudioGraph3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraph3Vtbl {
        unsafe extern "system" fn CreateMediaSourceAudioInputNodeAsync<Impl: IAudioGraph3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMediaSourceAudioInputNodeAsync(&*(&mediasource as *const <super::Core::MediaSource as ::windows::core::Abi>::Abi as *const <super::Core::MediaSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMediaSourceAudioInputNodeWithEmitterAsync<Impl: IAudioGraph3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMediaSourceAudioInputNodeWithEmitterAsync(&*(&mediasource as *const <super::Core::MediaSource as ::windows::core::Abi>::Abi as *const <super::Core::MediaSource as ::windows::core::DefaultType>::DefaultType), &*(&emitter as *const <AudioNodeEmitter as ::windows::core::Abi>::Abi as *const <AudioNodeEmitter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioGraph3>, base.5, CreateMediaSourceAudioInputNodeAsync::<Impl, OFFSET>, CreateMediaSourceAudioInputNodeWithEmitterAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphConnectionImpl: Sized {
    fn Destination(&self) -> ::windows::core::Result<IAudioNode>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn Gain(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioGraphConnection {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraphConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioGraphConnectionVtbl {
    pub const fn new<Impl: IAudioGraphConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraphConnectionVtbl {
        unsafe extern "system" fn Destination<Impl: IAudioGraphConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Impl: IAudioGraphConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGain(value).into()
        }
        unsafe extern "system" fn Gain<Impl: IAudioGraphConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioGraphConnection>, base.5, Destination::<Impl, OFFSET>, SetGain::<Impl, OFFSET>, Gain::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphSettingsImpl: Sized {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn SetEncodingProperties(&self, value: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
    fn SetPrimaryRenderDevice(&self, value: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<()>;
    fn QuantumSizeSelectionMode(&self) -> ::windows::core::Result<QuantumSizeSelectionMode>;
    fn SetQuantumSizeSelectionMode(&self, value: QuantumSizeSelectionMode) -> ::windows::core::Result<()>;
    fn DesiredSamplesPerQuantum(&self) -> ::windows::core::Result<i32>;
    fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows::core::Result<()>;
    fn AudioRenderCategory(&self) -> ::windows::core::Result<super::Render::AudioRenderCategory>;
    fn SetAudioRenderCategory(&self, value: super::Render::AudioRenderCategory) -> ::windows::core::Result<()>;
    fn DesiredRenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
    fn SetDesiredRenderDeviceAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioGraphSettings {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraphSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioGraphSettingsVtbl {
    pub const fn new<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraphSettingsVtbl {
        unsafe extern "system" fn EncodingProperties<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEncodingProperties(&*(&value as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrimaryRenderDevice<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryRenderDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryRenderDevice<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPrimaryRenderDevice(&*(&value as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn QuantumSizeSelectionMode<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut QuantumSizeSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuantumSizeSelectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuantumSizeSelectionMode<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: QuantumSizeSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetQuantumSizeSelectionMode(value).into()
        }
        unsafe extern "system" fn DesiredSamplesPerQuantum<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredSamplesPerQuantum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredSamplesPerQuantum<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredSamplesPerQuantum(value).into()
        }
        unsafe extern "system" fn AudioRenderCategory<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Render::AudioRenderCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioRenderCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioRenderCategory<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Render::AudioRenderCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioRenderCategory(value).into()
        }
        unsafe extern "system" fn DesiredRenderDeviceAudioProcessing<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredRenderDeviceAudioProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRenderDeviceAudioProcessing<Impl: IAudioGraphSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredRenderDeviceAudioProcessing(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAudioGraphSettings>,
            base.5,
            EncodingProperties::<Impl, OFFSET>,
            SetEncodingProperties::<Impl, OFFSET>,
            PrimaryRenderDevice::<Impl, OFFSET>,
            SetPrimaryRenderDevice::<Impl, OFFSET>,
            QuantumSizeSelectionMode::<Impl, OFFSET>,
            SetQuantumSizeSelectionMode::<Impl, OFFSET>,
            DesiredSamplesPerQuantum::<Impl, OFFSET>,
            SetDesiredSamplesPerQuantum::<Impl, OFFSET>,
            AudioRenderCategory::<Impl, OFFSET>,
            SetAudioRenderCategory::<Impl, OFFSET>,
            DesiredRenderDeviceAudioProcessing::<Impl, OFFSET>,
            SetDesiredRenderDeviceAudioProcessing::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphSettings2Impl: Sized {
    fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxPlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioGraphSettings2 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraphSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioGraphSettings2Vtbl {
    pub const fn new<Impl: IAudioGraphSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraphSettings2Vtbl {
        unsafe extern "system" fn SetMaxPlaybackSpeedFactor<Impl: IAudioGraphSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxPlaybackSpeedFactor(value).into()
        }
        unsafe extern "system" fn MaxPlaybackSpeedFactor<Impl: IAudioGraphSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPlaybackSpeedFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioGraphSettings2>, base.5, SetMaxPlaybackSpeedFactor::<Impl, OFFSET>, MaxPlaybackSpeedFactor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphSettingsFactoryImpl: Sized {
    fn Create(&self, audiorendercategory: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioGraphSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioGraphSettingsFactory {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraphSettingsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioGraphSettingsFactoryVtbl {
    pub const fn new<Impl: IAudioGraphSettingsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraphSettingsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioGraphSettingsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiorendercategory: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(audiorendercategory) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioGraphSettingsFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphStaticsImpl: Sized {
    fn CreateAsync(&self, settings: &::core::option::Option<AudioGraphSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioGraphStatics {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraphStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioGraphStaticsVtbl {
    pub const fn new<Impl: IAudioGraphStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraphStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IAudioGraphStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&settings as *const <AudioGraphSettings as ::windows::core::Abi>::Abi as *const <AudioGraphSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioGraphStatics>, base.5, CreateAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphUnrecoverableErrorOccurredEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<AudioGraphUnrecoverableError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.IAudioGraphUnrecoverableErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioGraphUnrecoverableErrorOccurredEventArgsVtbl {
    pub const fn new<Impl: IAudioGraphUnrecoverableErrorOccurredEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioGraphUnrecoverableErrorOccurredEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IAudioGraphUnrecoverableErrorOccurredEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphUnrecoverableError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioGraphUnrecoverableErrorOccurredEventArgs>, base.5, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IAudioInputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>;
    fn AddOutgoingConnection(&self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
    fn AddOutgoingConnectionWithGain(&self, destination: &::core::option::Option<IAudioNode>, gain: f64) -> ::windows::core::Result<()>;
    fn RemoveOutgoingConnection(&self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode";
}
#[cfg(feature = "Foundation")]
impl IAudioInputNodeVtbl {
    pub const fn new<Impl: IAudioInputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioInputNodeVtbl {
        unsafe extern "system" fn OutgoingConnections<Impl: IAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutgoingConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOutgoingConnection<Impl: IAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddOutgoingConnection(&*(&destination as *const <IAudioNode as ::windows::core::Abi>::Abi as *const <IAudioNode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddOutgoingConnectionWithGain<Impl: IAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, gain: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddOutgoingConnectionWithGain(&*(&destination as *const <IAudioNode as ::windows::core::Abi>::Abi as *const <IAudioNode as ::windows::core::DefaultType>::DefaultType), gain).into()
        }
        unsafe extern "system" fn RemoveOutgoingConnection<Impl: IAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOutgoingConnection(&*(&destination as *const <IAudioNode as ::windows::core::Abi>::Abi as *const <IAudioNode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioInputNode>, base.5, OutgoingConnections::<Impl, OFFSET>, AddOutgoingConnection::<Impl, OFFSET>, AddOutgoingConnectionWithGain::<Impl, OFFSET>, RemoveOutgoingConnection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IAudioInputNode2Impl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAudioInputNode2 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode2";
}
#[cfg(feature = "Foundation")]
impl IAudioInputNode2Vtbl {
    pub const fn new<Impl: IAudioInputNode2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioInputNode2Vtbl {
        unsafe extern "system" fn Emitter<Impl: IAudioInputNode2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Emitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioInputNode2>, base.5, Emitter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IAudioNodeImpl: Sized + IClosableImpl {
    fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn OutgoingGain(&self) -> ::windows::core::Result<f64>;
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn ConsumeInput(&self) -> ::windows::core::Result<bool>;
    fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn DisableEffectsByDefinition(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
    fn EnableEffectsByDefinition(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAudioNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNode";
}
#[cfg(feature = "Foundation")]
impl IAudioNodeVtbl {
    pub const fn new<Impl: IAudioNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeVtbl {
        unsafe extern "system" fn EffectDefinitions<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EffectDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingGain<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutgoingGain(value).into()
        }
        unsafe extern "system" fn OutgoingGain<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutgoingGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingProperties<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConsumeInput<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConsumeInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConsumeInput<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetConsumeInput(value).into()
        }
        unsafe extern "system" fn Start<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Reset<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn DisableEffectsByDefinition<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DisableEffectsByDefinition(&*(&definition as *const <super::Effects::IAudioEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IAudioEffectDefinition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableEffectsByDefinition<Impl: IAudioNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnableEffectsByDefinition(&*(&definition as *const <super::Effects::IAudioEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IAudioEffectDefinition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNode>, base.5, EffectDefinitions::<Impl, OFFSET>, SetOutgoingGain::<Impl, OFFSET>, OutgoingGain::<Impl, OFFSET>, EncodingProperties::<Impl, OFFSET>, ConsumeInput::<Impl, OFFSET>, SetConsumeInput::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Reset::<Impl, OFFSET>, DisableEffectsByDefinition::<Impl, OFFSET>, EnableEffectsByDefinition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetPosition(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Shape(&self) -> ::windows::core::Result<AudioNodeEmitterShape>;
    fn DecayModel(&self) -> ::windows::core::Result<AudioNodeEmitterDecayModel>;
    fn Gain(&self) -> ::windows::core::Result<f64>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn DistanceScale(&self) -> ::windows::core::Result<f64>;
    fn SetDistanceScale(&self, value: f64) -> ::windows::core::Result<()>;
    fn DopplerScale(&self) -> ::windows::core::Result<f64>;
    fn SetDopplerScale(&self, value: f64) -> ::windows::core::Result<()>;
    fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDopplerVelocity(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn IsDopplerDisabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitter {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitter";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterVtbl {
    pub const fn new<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterVtbl {
        unsafe extern "system" fn Position<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Direction<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDirection(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Shape<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecayModel<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecayModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gain<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGain(value).into()
        }
        unsafe extern "system" fn DistanceScale<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DistanceScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDistanceScale<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDistanceScale(value).into()
        }
        unsafe extern "system" fn DopplerScale<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DopplerScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDopplerScale<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDopplerScale(value).into()
        }
        unsafe extern "system" fn DopplerVelocity<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DopplerVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDopplerVelocity<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDopplerVelocity(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsDopplerDisabled<Impl: IAudioNodeEmitterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDopplerDisabled() {
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
            ::windows::core::GetRuntimeClassName::<IAudioNodeEmitter>,
            base.5,
            Position::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            Direction::<Impl, OFFSET>,
            SetDirection::<Impl, OFFSET>,
            Shape::<Impl, OFFSET>,
            DecayModel::<Impl, OFFSET>,
            Gain::<Impl, OFFSET>,
            SetGain::<Impl, OFFSET>,
            DistanceScale::<Impl, OFFSET>,
            SetDistanceScale::<Impl, OFFSET>,
            DopplerScale::<Impl, OFFSET>,
            SetDopplerScale::<Impl, OFFSET>,
            DopplerVelocity::<Impl, OFFSET>,
            SetDopplerVelocity::<Impl, OFFSET>,
            IsDopplerDisabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitter2Impl: Sized {
    fn SpatialAudioModel(&self) -> ::windows::core::Result<SpatialAudioModel>;
    fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitter2 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitter2";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitter2Vtbl {
    pub const fn new<Impl: IAudioNodeEmitter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitter2Vtbl {
        unsafe extern "system" fn SpatialAudioModel<Impl: IAudioNodeEmitter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpatialAudioModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpatialAudioModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpatialAudioModel<Impl: IAudioNodeEmitter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SpatialAudioModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpatialAudioModel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitter2>, base.5, SpatialAudioModel::<Impl, OFFSET>, SetSpatialAudioModel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterConePropertiesImpl: Sized {
    fn InnerAngle(&self) -> ::windows::core::Result<f64>;
    fn OuterAngle(&self) -> ::windows::core::Result<f64>;
    fn OuterAngleGain(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitterConeProperties {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitterConeProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterConePropertiesVtbl {
    pub const fn new<Impl: IAudioNodeEmitterConePropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterConePropertiesVtbl {
        unsafe extern "system" fn InnerAngle<Impl: IAudioNodeEmitterConePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InnerAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OuterAngle<Impl: IAudioNodeEmitterConePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OuterAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OuterAngleGain<Impl: IAudioNodeEmitterConePropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OuterAngleGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitterConeProperties>, base.5, InnerAngle::<Impl, OFFSET>, OuterAngle::<Impl, OFFSET>, OuterAngleGain::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterDecayModelImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterDecayKind>;
    fn MinGain(&self) -> ::windows::core::Result<f64>;
    fn MaxGain(&self) -> ::windows::core::Result<f64>;
    fn NaturalProperties(&self) -> ::windows::core::Result<AudioNodeEmitterNaturalDecayModelProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitterDecayModel {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitterDecayModel";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterDecayModelVtbl {
    pub const fn new<Impl: IAudioNodeEmitterDecayModelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterDecayModelVtbl {
        unsafe extern "system" fn Kind<Impl: IAudioNodeEmitterDecayModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterDecayKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinGain<Impl: IAudioNodeEmitterDecayModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxGain<Impl: IAudioNodeEmitterDecayModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NaturalProperties<Impl: IAudioNodeEmitterDecayModelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NaturalProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitterDecayModel>, base.5, Kind::<Impl, OFFSET>, MinGain::<Impl, OFFSET>, MaxGain::<Impl, OFFSET>, NaturalProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterDecayModelStaticsImpl: Sized {
    fn CreateNatural(&self, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel>;
    fn CreateCustom(&self, mingain: f64, maxgain: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitterDecayModelStatics {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitterDecayModelStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterDecayModelStaticsVtbl {
    pub const fn new<Impl: IAudioNodeEmitterDecayModelStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterDecayModelStaticsVtbl {
        unsafe extern "system" fn CreateNatural<Impl: IAudioNodeEmitterDecayModelStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNatural(mingain, maxgain, unitygaindistance, cutoffdistance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustom<Impl: IAudioNodeEmitterDecayModelStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCustom(mingain, maxgain) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitterDecayModelStatics>, base.5, CreateNatural::<Impl, OFFSET>, CreateCustom::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterFactoryImpl: Sized {
    fn CreateAudioNodeEmitter(&self, shape: &::core::option::Option<AudioNodeEmitterShape>, decaymodel: &::core::option::Option<AudioNodeEmitterDecayModel>, settings: AudioNodeEmitterSettings) -> ::windows::core::Result<AudioNodeEmitter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitterFactory {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterFactoryVtbl {
    pub const fn new<Impl: IAudioNodeEmitterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterFactoryVtbl {
        unsafe extern "system" fn CreateAudioNodeEmitter<Impl: IAudioNodeEmitterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, decaymodel: ::windows::core::RawPtr, settings: AudioNodeEmitterSettings, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAudioNodeEmitter(&*(&shape as *const <AudioNodeEmitterShape as ::windows::core::Abi>::Abi as *const <AudioNodeEmitterShape as ::windows::core::DefaultType>::DefaultType), &*(&decaymodel as *const <AudioNodeEmitterDecayModel as ::windows::core::Abi>::Abi as *const <AudioNodeEmitterDecayModel as ::windows::core::DefaultType>::DefaultType), settings) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitterFactory>, base.5, CreateAudioNodeEmitter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterNaturalDecayModelPropertiesImpl: Sized {
    fn UnityGainDistance(&self) -> ::windows::core::Result<f64>;
    fn CutoffDistance(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitterNaturalDecayModelProperties {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitterNaturalDecayModelProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterNaturalDecayModelPropertiesVtbl {
    pub const fn new<Impl: IAudioNodeEmitterNaturalDecayModelPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterNaturalDecayModelPropertiesVtbl {
        unsafe extern "system" fn UnityGainDistance<Impl: IAudioNodeEmitterNaturalDecayModelPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnityGainDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CutoffDistance<Impl: IAudioNodeEmitterNaturalDecayModelPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CutoffDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitterNaturalDecayModelProperties>, base.5, UnityGainDistance::<Impl, OFFSET>, CutoffDistance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterShapeImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterShapeKind>;
    fn ConeProperties(&self) -> ::windows::core::Result<AudioNodeEmitterConeProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitterShape {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitterShape";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterShapeVtbl {
    pub const fn new<Impl: IAudioNodeEmitterShapeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterShapeVtbl {
        unsafe extern "system" fn Kind<Impl: IAudioNodeEmitterShapeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterShapeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConeProperties<Impl: IAudioNodeEmitterShapeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConeProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitterShape>, base.5, Kind::<Impl, OFFSET>, ConeProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterShapeStaticsImpl: Sized {
    fn CreateCone(&self, innerangle: f64, outerangle: f64, outeranglegain: f64) -> ::windows::core::Result<AudioNodeEmitterShape>;
    fn CreateOmnidirectional(&self) -> ::windows::core::Result<AudioNodeEmitterShape>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeEmitterShapeStatics {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeEmitterShapeStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeEmitterShapeStaticsVtbl {
    pub const fn new<Impl: IAudioNodeEmitterShapeStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeEmitterShapeStaticsVtbl {
        unsafe extern "system" fn CreateCone<Impl: IAudioNodeEmitterShapeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, innerangle: f64, outerangle: f64, outeranglegain: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCone(innerangle, outerangle, outeranglegain) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOmnidirectional<Impl: IAudioNodeEmitterShapeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOmnidirectional() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeEmitterShapeStatics>, base.5, CreateCone::<Impl, OFFSET>, CreateOmnidirectional::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeListenerImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetPosition(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn SpeedOfSound(&self) -> ::windows::core::Result<f64>;
    fn SetSpeedOfSound(&self, value: f64) -> ::windows::core::Result<()>;
    fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDopplerVelocity(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioNodeListener {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeListener";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioNodeListenerVtbl {
    pub const fn new<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeListenerVtbl {
        unsafe extern "system" fn Position<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Orientation<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpeedOfSound<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpeedOfSound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeedOfSound<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpeedOfSound(value).into()
        }
        unsafe extern "system" fn DopplerVelocity<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DopplerVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDopplerVelocity<Impl: IAudioNodeListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDopplerVelocity(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeListener>, base.5, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>, Orientation::<Impl, OFFSET>, SetOrientation::<Impl, OFFSET>, SpeedOfSound::<Impl, OFFSET>, SetSpeedOfSound::<Impl, OFFSET>, DopplerVelocity::<Impl, OFFSET>, SetDopplerVelocity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IAudioNodeWithListenerImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn SetListener(&self, value: &::core::option::Option<AudioNodeListener>) -> ::windows::core::Result<()>;
    fn Listener(&self) -> ::windows::core::Result<AudioNodeListener>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAudioNodeWithListener {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeWithListener";
}
#[cfg(feature = "Foundation")]
impl IAudioNodeWithListenerVtbl {
    pub const fn new<Impl: IAudioNodeWithListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioNodeWithListenerVtbl {
        unsafe extern "system" fn SetListener<Impl: IAudioNodeWithListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetListener(&*(&value as *const <AudioNodeListener as ::windows::core::Abi>::Abi as *const <AudioNodeListener as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Listener<Impl: IAudioNodeWithListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Listener() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioNodeWithListener>, base.5, SetListener::<Impl, OFFSET>, Listener::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioPlaybackConnectionImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<AudioPlaybackConnectionState>;
    fn Open(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResult>;
    fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioPlaybackConnection {
    const NAME: &'static str = "Windows.Media.Audio.IAudioPlaybackConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioPlaybackConnectionVtbl {
    pub const fn new<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioPlaybackConnectionVtbl {
        unsafe extern "system" fn Start<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn StartAsync<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Open<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChanged<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IAudioPlaybackConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioPlaybackConnection>, base.5, Start::<Impl, OFFSET>, StartAsync::<Impl, OFFSET>, DeviceId::<Impl, OFFSET>, State::<Impl, OFFSET>, Open::<Impl, OFFSET>, OpenAsync::<Impl, OFFSET>, StateChanged::<Impl, OFFSET>, RemoveStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioPlaybackConnectionOpenResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResultStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioPlaybackConnectionOpenResult {
    const NAME: &'static str = "Windows.Media.Audio.IAudioPlaybackConnectionOpenResult";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioPlaybackConnectionOpenResultVtbl {
    pub const fn new<Impl: IAudioPlaybackConnectionOpenResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioPlaybackConnectionOpenResultVtbl {
        unsafe extern "system" fn Status<Impl: IAudioPlaybackConnectionOpenResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionOpenResultStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IAudioPlaybackConnectionOpenResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioPlaybackConnectionOpenResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioPlaybackConnectionStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryCreateFromId(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<AudioPlaybackConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioPlaybackConnectionStatics {
    const NAME: &'static str = "Windows.Media.Audio.IAudioPlaybackConnectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioPlaybackConnectionStaticsVtbl {
    pub const fn new<Impl: IAudioPlaybackConnectionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioPlaybackConnectionStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IAudioPlaybackConnectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFromId<Impl: IAudioPlaybackConnectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateFromId(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioPlaybackConnectionStatics>, base.5, GetDeviceSelector::<Impl, OFFSET>, TryCreateFromId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStateMonitorImpl: Sized {
    fn SoundLevelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioStateMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SoundLevel(&self) -> ::windows::core::Result<super::SoundLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioStateMonitor {
    const NAME: &'static str = "Windows.Media.Audio.IAudioStateMonitor";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioStateMonitorVtbl {
    pub const fn new<Impl: IAudioStateMonitorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioStateMonitorVtbl {
        unsafe extern "system" fn SoundLevelChanged<Impl: IAudioStateMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoundLevelChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioStateMonitor, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioStateMonitor, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSoundLevelChanged<Impl: IAudioStateMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSoundLevelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SoundLevel<Impl: IAudioStateMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::SoundLevel) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioStateMonitor>, base.5, SoundLevelChanged::<Impl, OFFSET>, RemoveSoundLevelChanged::<Impl, OFFSET>, SoundLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStateMonitorStaticsImpl: Sized {
    fn CreateForRenderMonitoring(&self) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForRenderMonitoringWithCategory(&self, category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForRenderMonitoringWithCategoryAndDeviceRole(&self, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForRenderMonitoringWithCategoryAndDeviceId(&self, category: super::Render::AudioRenderCategory, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoring(&self) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoringWithCategory(&self, category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(&self, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoringWithCategoryAndDeviceId(&self, category: super::Capture::MediaCategory, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioStateMonitor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioStateMonitorStatics {
    const NAME: &'static str = "Windows.Media.Audio.IAudioStateMonitorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioStateMonitorStaticsVtbl {
    pub const fn new<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioStateMonitorStaticsVtbl {
        unsafe extern "system" fn CreateForRenderMonitoring<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForRenderMonitoring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForRenderMonitoringWithCategory<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForRenderMonitoringWithCategory(category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForRenderMonitoringWithCategoryAndDeviceRole<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForRenderMonitoringWithCategoryAndDeviceRole(category, role) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForRenderMonitoringWithCategoryAndDeviceId<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForRenderMonitoringWithCategoryAndDeviceId(category, &*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForCaptureMonitoring<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForCaptureMonitoring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForCaptureMonitoringWithCategory<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForCaptureMonitoringWithCategory(category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForCaptureMonitoringWithCategoryAndDeviceRole<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForCaptureMonitoringWithCategoryAndDeviceRole(category, role) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForCaptureMonitoringWithCategoryAndDeviceId<Impl: IAudioStateMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForCaptureMonitoringWithCategoryAndDeviceId(category, &*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IAudioStateMonitorStatics>,
            base.5,
            CreateForRenderMonitoring::<Impl, OFFSET>,
            CreateForRenderMonitoringWithCategory::<Impl, OFFSET>,
            CreateForRenderMonitoringWithCategoryAndDeviceRole::<Impl, OFFSET>,
            CreateForRenderMonitoringWithCategoryAndDeviceId::<Impl, OFFSET>,
            CreateForCaptureMonitoring::<Impl, OFFSET>,
            CreateForCaptureMonitoringWithCategory::<Impl, OFFSET>,
            CreateForCaptureMonitoringWithCategoryAndDeviceRole::<Impl, OFFSET>,
            CreateForCaptureMonitoringWithCategoryAndDeviceId::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceInputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus>;
    fn DeviceInputNode(&self) -> ::windows::core::Result<AudioDeviceInputNode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioDeviceInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioDeviceInputNodeResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioDeviceInputNodeResultVtbl {
    pub const fn new<Impl: ICreateAudioDeviceInputNodeResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioDeviceInputNodeResultVtbl {
        unsafe extern "system" fn Status<Impl: ICreateAudioDeviceInputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInputNode<Impl: ICreateAudioDeviceInputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceInputNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioDeviceInputNodeResult>, base.5, Status::<Impl, OFFSET>, DeviceInputNode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceInputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioDeviceInputNodeResult2 {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioDeviceInputNodeResult2";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioDeviceInputNodeResult2Vtbl {
    pub const fn new<Impl: ICreateAudioDeviceInputNodeResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioDeviceInputNodeResult2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: ICreateAudioDeviceInputNodeResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioDeviceInputNodeResult2>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceOutputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus>;
    fn DeviceOutputNode(&self) -> ::windows::core::Result<AudioDeviceOutputNode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioDeviceOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioDeviceOutputNodeResultVtbl {
    pub const fn new<Impl: ICreateAudioDeviceOutputNodeResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioDeviceOutputNodeResultVtbl {
        unsafe extern "system" fn Status<Impl: ICreateAudioDeviceOutputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceOutputNode<Impl: ICreateAudioDeviceOutputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceOutputNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioDeviceOutputNodeResult>, base.5, Status::<Impl, OFFSET>, DeviceOutputNode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceOutputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioDeviceOutputNodeResult2 {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult2";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioDeviceOutputNodeResult2Vtbl {
    pub const fn new<Impl: ICreateAudioDeviceOutputNodeResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioDeviceOutputNodeResult2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: ICreateAudioDeviceOutputNodeResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioDeviceOutputNodeResult2>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileInputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus>;
    fn FileInputNode(&self) -> ::windows::core::Result<AudioFileInputNode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioFileInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioFileInputNodeResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioFileInputNodeResultVtbl {
    pub const fn new<Impl: ICreateAudioFileInputNodeResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioFileInputNodeResultVtbl {
        unsafe extern "system" fn Status<Impl: ICreateAudioFileInputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileInputNode<Impl: ICreateAudioFileInputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileInputNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioFileInputNodeResult>, base.5, Status::<Impl, OFFSET>, FileInputNode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileInputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioFileInputNodeResult2 {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioFileInputNodeResult2";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioFileInputNodeResult2Vtbl {
    pub const fn new<Impl: ICreateAudioFileInputNodeResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioFileInputNodeResult2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: ICreateAudioFileInputNodeResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioFileInputNodeResult2>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileOutputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus>;
    fn FileOutputNode(&self) -> ::windows::core::Result<AudioFileOutputNode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioFileOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioFileOutputNodeResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioFileOutputNodeResultVtbl {
    pub const fn new<Impl: ICreateAudioFileOutputNodeResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioFileOutputNodeResultVtbl {
        unsafe extern "system" fn Status<Impl: ICreateAudioFileOutputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileOutputNode<Impl: ICreateAudioFileOutputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileOutputNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioFileOutputNodeResult>, base.5, Status::<Impl, OFFSET>, FileOutputNode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileOutputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioFileOutputNodeResult2 {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioFileOutputNodeResult2";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioFileOutputNodeResult2Vtbl {
    pub const fn new<Impl: ICreateAudioFileOutputNodeResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioFileOutputNodeResult2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: ICreateAudioFileOutputNodeResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioFileOutputNodeResult2>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioGraphResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioGraphCreationStatus>;
    fn Graph(&self) -> ::windows::core::Result<AudioGraph>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioGraphResult {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioGraphResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioGraphResultVtbl {
    pub const fn new<Impl: ICreateAudioGraphResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioGraphResultVtbl {
        unsafe extern "system" fn Status<Impl: ICreateAudioGraphResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Graph<Impl: ICreateAudioGraphResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Graph() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioGraphResult>, base.5, Status::<Impl, OFFSET>, Graph::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioGraphResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateAudioGraphResult2 {
    const NAME: &'static str = "Windows.Media.Audio.ICreateAudioGraphResult2";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateAudioGraphResult2Vtbl {
    pub const fn new<Impl: ICreateAudioGraphResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateAudioGraphResult2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: ICreateAudioGraphResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateAudioGraphResult2>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateMediaSourceAudioInputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MediaSourceAudioInputNodeCreationStatus>;
    fn Node(&self) -> ::windows::core::Result<MediaSourceAudioInputNode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateMediaSourceAudioInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateMediaSourceAudioInputNodeResultVtbl {
    pub const fn new<Impl: ICreateMediaSourceAudioInputNodeResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateMediaSourceAudioInputNodeResultVtbl {
        unsafe extern "system" fn Status<Impl: ICreateMediaSourceAudioInputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceAudioInputNodeCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Node<Impl: ICreateMediaSourceAudioInputNodeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Node() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateMediaSourceAudioInputNodeResult>, base.5, Status::<Impl, OFFSET>, Node::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateMediaSourceAudioInputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateMediaSourceAudioInputNodeResult2 {
    const NAME: &'static str = "Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult2";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateMediaSourceAudioInputNodeResult2Vtbl {
    pub const fn new<Impl: ICreateMediaSourceAudioInputNodeResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateMediaSourceAudioInputNodeResult2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: ICreateMediaSourceAudioInputNodeResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateMediaSourceAudioInputNodeResult2>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IEchoEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()>;
    fn WetDryMix(&self) -> ::windows::core::Result<f64>;
    fn SetFeedback(&self, value: f64) -> ::windows::core::Result<()>;
    fn Feedback(&self) -> ::windows::core::Result<f64>;
    fn SetDelay(&self, value: f64) -> ::windows::core::Result<()>;
    fn Delay(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEchoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.IEchoEffectDefinition";
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl IEchoEffectDefinitionVtbl {
    pub const fn new<Impl: IEchoEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEchoEffectDefinitionVtbl {
        unsafe extern "system" fn SetWetDryMix<Impl: IEchoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWetDryMix(value).into()
        }
        unsafe extern "system" fn WetDryMix<Impl: IEchoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WetDryMix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFeedback<Impl: IEchoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFeedback(value).into()
        }
        unsafe extern "system" fn Feedback<Impl: IEchoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Feedback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelay<Impl: IEchoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDelay(value).into()
        }
        unsafe extern "system" fn Delay<Impl: IEchoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEchoEffectDefinition>, base.5, SetWetDryMix::<Impl, OFFSET>, WetDryMix::<Impl, OFFSET>, SetFeedback::<Impl, OFFSET>, Feedback::<Impl, OFFSET>, SetDelay::<Impl, OFFSET>, Delay::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEchoEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<EchoEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEchoEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Audio.IEchoEffectDefinitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEchoEffectDefinitionFactoryVtbl {
    pub const fn new<Impl: IEchoEffectDefinitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEchoEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IEchoEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&audiograph as *const <AudioGraph as ::windows::core::Abi>::Abi as *const <AudioGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEchoEffectDefinitionFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEqualizerBandImpl: Sized {
    fn Bandwidth(&self) -> ::windows::core::Result<f64>;
    fn SetBandwidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn FrequencyCenter(&self) -> ::windows::core::Result<f64>;
    fn SetFrequencyCenter(&self, value: f64) -> ::windows::core::Result<()>;
    fn Gain(&self) -> ::windows::core::Result<f64>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEqualizerBand {
    const NAME: &'static str = "Windows.Media.Audio.IEqualizerBand";
}
#[cfg(feature = "implement_exclusive")]
impl IEqualizerBandVtbl {
    pub const fn new<Impl: IEqualizerBandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEqualizerBandVtbl {
        unsafe extern "system" fn Bandwidth<Impl: IEqualizerBandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bandwidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBandwidth<Impl: IEqualizerBandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBandwidth(value).into()
        }
        unsafe extern "system" fn FrequencyCenter<Impl: IEqualizerBandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrequencyCenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequencyCenter<Impl: IEqualizerBandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrequencyCenter(value).into()
        }
        unsafe extern "system" fn Gain<Impl: IEqualizerBandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Impl: IEqualizerBandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGain(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEqualizerBand>, base.5, Bandwidth::<Impl, OFFSET>, SetBandwidth::<Impl, OFFSET>, FrequencyCenter::<Impl, OFFSET>, SetFrequencyCenter::<Impl, OFFSET>, Gain::<Impl, OFFSET>, SetGain::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IEqualizerEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn Bands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EqualizerBand>>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEqualizerEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.IEqualizerEffectDefinition";
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl IEqualizerEffectDefinitionVtbl {
    pub const fn new<Impl: IEqualizerEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEqualizerEffectDefinitionVtbl {
        unsafe extern "system" fn Bands<Impl: IEqualizerEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEqualizerEffectDefinition>, base.5, Bands::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEqualizerEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<EqualizerEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEqualizerEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Audio.IEqualizerEffectDefinitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEqualizerEffectDefinitionFactoryVtbl {
    pub const fn new<Impl: IEqualizerEffectDefinitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEqualizerEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IEqualizerEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&audiograph as *const <AudioGraph as ::windows::core::Abi>::Abi as *const <AudioGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEqualizerEffectDefinitionFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameInputNodeQuantumStartedEventArgsImpl: Sized {
    fn RequiredSamples(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameInputNodeQuantumStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.IFrameInputNodeQuantumStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameInputNodeQuantumStartedEventArgsVtbl {
    pub const fn new<Impl: IFrameInputNodeQuantumStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameInputNodeQuantumStartedEventArgsVtbl {
        unsafe extern "system" fn RequiredSamples<Impl: IFrameInputNodeQuantumStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequiredSamples() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameInputNodeQuantumStartedEventArgs>, base.5, RequiredSamples::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait ILimiterEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn SetRelease(&self, value: u32) -> ::windows::core::Result<()>;
    fn Release(&self) -> ::windows::core::Result<u32>;
    fn SetLoudness(&self, value: u32) -> ::windows::core::Result<()>;
    fn Loudness(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILimiterEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.ILimiterEffectDefinition";
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl ILimiterEffectDefinitionVtbl {
    pub const fn new<Impl: ILimiterEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILimiterEffectDefinitionVtbl {
        unsafe extern "system" fn SetRelease<Impl: ILimiterEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRelease(value).into()
        }
        unsafe extern "system" fn Release<Impl: ILimiterEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Release() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoudness<Impl: ILimiterEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLoudness(value).into()
        }
        unsafe extern "system" fn Loudness<Impl: ILimiterEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Loudness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILimiterEffectDefinition>, base.5, SetRelease::<Impl, OFFSET>, Release::<Impl, OFFSET>, SetLoudness::<Impl, OFFSET>, Loudness::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILimiterEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<LimiterEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILimiterEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Audio.ILimiterEffectDefinitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILimiterEffectDefinitionFactoryVtbl {
    pub const fn new<Impl: ILimiterEffectDefinitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILimiterEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILimiterEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&audiograph as *const <AudioGraph as ::windows::core::Abi>::Abi as *const <AudioGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILimiterEffectDefinitionFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaSourceAudioInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioInputNode2Impl + IAudioNodeImpl + IClosableImpl {
    fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Seek(&self, position: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetEndTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetLoopCount(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn MediaSourceCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaSourceCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSourceAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IMediaSourceAudioInputNode";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaSourceAudioInputNodeVtbl {
    pub const fn new<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceAudioInputNodeVtbl {
        unsafe extern "system" fn SetPlaybackSpeedFactor<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaybackSpeedFactor(value).into()
        }
        unsafe extern "system" fn PlaybackSpeedFactor<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackSpeedFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Seek<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Seek(&*(&position as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStartTime<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndTime<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEndTime<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoopCount<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoopCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoopCount<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLoopCount(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaSource<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSourceCompleted<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaSourceCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMediaSourceCompleted<Impl: IMediaSourceAudioInputNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMediaSourceCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMediaSourceAudioInputNode>,
            base.5,
            SetPlaybackSpeedFactor::<Impl, OFFSET>,
            PlaybackSpeedFactor::<Impl, OFFSET>,
            Position::<Impl, OFFSET>,
            Seek::<Impl, OFFSET>,
            StartTime::<Impl, OFFSET>,
            SetStartTime::<Impl, OFFSET>,
            EndTime::<Impl, OFFSET>,
            SetEndTime::<Impl, OFFSET>,
            LoopCount::<Impl, OFFSET>,
            SetLoopCount::<Impl, OFFSET>,
            Duration::<Impl, OFFSET>,
            MediaSource::<Impl, OFFSET>,
            MediaSourceCompleted::<Impl, OFFSET>,
            RemoveMediaSourceCompleted::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IReverbEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()>;
    fn WetDryMix(&self) -> ::windows::core::Result<f64>;
    fn SetReflectionsDelay(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReflectionsDelay(&self) -> ::windows::core::Result<u32>;
    fn SetReverbDelay(&self, value: u8) -> ::windows::core::Result<()>;
    fn ReverbDelay(&self) -> ::windows::core::Result<u8>;
    fn SetRearDelay(&self, value: u8) -> ::windows::core::Result<()>;
    fn RearDelay(&self) -> ::windows::core::Result<u8>;
    fn SetPositionLeft(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionLeft(&self) -> ::windows::core::Result<u8>;
    fn SetPositionRight(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionRight(&self) -> ::windows::core::Result<u8>;
    fn SetPositionMatrixLeft(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionMatrixLeft(&self) -> ::windows::core::Result<u8>;
    fn SetPositionMatrixRight(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionMatrixRight(&self) -> ::windows::core::Result<u8>;
    fn SetEarlyDiffusion(&self, value: u8) -> ::windows::core::Result<()>;
    fn EarlyDiffusion(&self) -> ::windows::core::Result<u8>;
    fn SetLateDiffusion(&self, value: u8) -> ::windows::core::Result<()>;
    fn LateDiffusion(&self) -> ::windows::core::Result<u8>;
    fn SetLowEQGain(&self, value: u8) -> ::windows::core::Result<()>;
    fn LowEQGain(&self) -> ::windows::core::Result<u8>;
    fn SetLowEQCutoff(&self, value: u8) -> ::windows::core::Result<()>;
    fn LowEQCutoff(&self) -> ::windows::core::Result<u8>;
    fn SetHighEQGain(&self, value: u8) -> ::windows::core::Result<()>;
    fn HighEQGain(&self) -> ::windows::core::Result<u8>;
    fn SetHighEQCutoff(&self, value: u8) -> ::windows::core::Result<()>;
    fn HighEQCutoff(&self) -> ::windows::core::Result<u8>;
    fn SetRoomFilterFreq(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomFilterFreq(&self) -> ::windows::core::Result<f64>;
    fn SetRoomFilterMain(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomFilterMain(&self) -> ::windows::core::Result<f64>;
    fn SetRoomFilterHF(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomFilterHF(&self) -> ::windows::core::Result<f64>;
    fn SetReflectionsGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReflectionsGain(&self) -> ::windows::core::Result<f64>;
    fn SetReverbGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReverbGain(&self) -> ::windows::core::Result<f64>;
    fn SetDecayTime(&self, value: f64) -> ::windows::core::Result<()>;
    fn DecayTime(&self) -> ::windows::core::Result<f64>;
    fn SetDensity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Density(&self) -> ::windows::core::Result<f64>;
    fn SetRoomSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomSize(&self) -> ::windows::core::Result<f64>;
    fn SetDisableLateField(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisableLateField(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IReverbEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.IReverbEffectDefinition";
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl IReverbEffectDefinitionVtbl {
    pub const fn new<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReverbEffectDefinitionVtbl {
        unsafe extern "system" fn SetWetDryMix<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWetDryMix(value).into()
        }
        unsafe extern "system" fn WetDryMix<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WetDryMix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReflectionsDelay<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReflectionsDelay(value).into()
        }
        unsafe extern "system" fn ReflectionsDelay<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReflectionsDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReverbDelay<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReverbDelay(value).into()
        }
        unsafe extern "system" fn ReverbDelay<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReverbDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRearDelay<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRearDelay(value).into()
        }
        unsafe extern "system" fn RearDelay<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RearDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionLeft<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionLeft(value).into()
        }
        unsafe extern "system" fn PositionLeft<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionRight<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionRight(value).into()
        }
        unsafe extern "system" fn PositionRight<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionRight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionMatrixLeft<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionMatrixLeft(value).into()
        }
        unsafe extern "system" fn PositionMatrixLeft<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionMatrixLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionMatrixRight<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPositionMatrixRight(value).into()
        }
        unsafe extern "system" fn PositionMatrixRight<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionMatrixRight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEarlyDiffusion<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEarlyDiffusion(value).into()
        }
        unsafe extern "system" fn EarlyDiffusion<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EarlyDiffusion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLateDiffusion<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLateDiffusion(value).into()
        }
        unsafe extern "system" fn LateDiffusion<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LateDiffusion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowEQGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLowEQGain(value).into()
        }
        unsafe extern "system" fn LowEQGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LowEQGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowEQCutoff<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLowEQCutoff(value).into()
        }
        unsafe extern "system" fn LowEQCutoff<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LowEQCutoff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighEQGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHighEQGain(value).into()
        }
        unsafe extern "system" fn HighEQGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighEQGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighEQCutoff<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHighEQCutoff(value).into()
        }
        unsafe extern "system" fn HighEQCutoff<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighEQCutoff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoomFilterFreq<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoomFilterFreq(value).into()
        }
        unsafe extern "system" fn RoomFilterFreq<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoomFilterFreq() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoomFilterMain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoomFilterMain(value).into()
        }
        unsafe extern "system" fn RoomFilterMain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoomFilterMain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoomFilterHF<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoomFilterHF(value).into()
        }
        unsafe extern "system" fn RoomFilterHF<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoomFilterHF() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReflectionsGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReflectionsGain(value).into()
        }
        unsafe extern "system" fn ReflectionsGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReflectionsGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReverbGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReverbGain(value).into()
        }
        unsafe extern "system" fn ReverbGain<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReverbGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecayTime<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDecayTime(value).into()
        }
        unsafe extern "system" fn DecayTime<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecayTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDensity<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDensity(value).into()
        }
        unsafe extern "system" fn Density<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Density() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoomSize<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoomSize(value).into()
        }
        unsafe extern "system" fn RoomSize<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoomSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableLateField<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisableLateField(value).into()
        }
        unsafe extern "system" fn DisableLateField<Impl: IReverbEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisableLateField() {
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
            ::windows::core::GetRuntimeClassName::<IReverbEffectDefinition>,
            base.5,
            SetWetDryMix::<Impl, OFFSET>,
            WetDryMix::<Impl, OFFSET>,
            SetReflectionsDelay::<Impl, OFFSET>,
            ReflectionsDelay::<Impl, OFFSET>,
            SetReverbDelay::<Impl, OFFSET>,
            ReverbDelay::<Impl, OFFSET>,
            SetRearDelay::<Impl, OFFSET>,
            RearDelay::<Impl, OFFSET>,
            SetPositionLeft::<Impl, OFFSET>,
            PositionLeft::<Impl, OFFSET>,
            SetPositionRight::<Impl, OFFSET>,
            PositionRight::<Impl, OFFSET>,
            SetPositionMatrixLeft::<Impl, OFFSET>,
            PositionMatrixLeft::<Impl, OFFSET>,
            SetPositionMatrixRight::<Impl, OFFSET>,
            PositionMatrixRight::<Impl, OFFSET>,
            SetEarlyDiffusion::<Impl, OFFSET>,
            EarlyDiffusion::<Impl, OFFSET>,
            SetLateDiffusion::<Impl, OFFSET>,
            LateDiffusion::<Impl, OFFSET>,
            SetLowEQGain::<Impl, OFFSET>,
            LowEQGain::<Impl, OFFSET>,
            SetLowEQCutoff::<Impl, OFFSET>,
            LowEQCutoff::<Impl, OFFSET>,
            SetHighEQGain::<Impl, OFFSET>,
            HighEQGain::<Impl, OFFSET>,
            SetHighEQCutoff::<Impl, OFFSET>,
            HighEQCutoff::<Impl, OFFSET>,
            SetRoomFilterFreq::<Impl, OFFSET>,
            RoomFilterFreq::<Impl, OFFSET>,
            SetRoomFilterMain::<Impl, OFFSET>,
            RoomFilterMain::<Impl, OFFSET>,
            SetRoomFilterHF::<Impl, OFFSET>,
            RoomFilterHF::<Impl, OFFSET>,
            SetReflectionsGain::<Impl, OFFSET>,
            ReflectionsGain::<Impl, OFFSET>,
            SetReverbGain::<Impl, OFFSET>,
            ReverbGain::<Impl, OFFSET>,
            SetDecayTime::<Impl, OFFSET>,
            DecayTime::<Impl, OFFSET>,
            SetDensity::<Impl, OFFSET>,
            Density::<Impl, OFFSET>,
            SetRoomSize::<Impl, OFFSET>,
            RoomSize::<Impl, OFFSET>,
            SetDisableLateField::<Impl, OFFSET>,
            DisableLateField::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IReverbEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<ReverbEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IReverbEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Audio.IReverbEffectDefinitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IReverbEffectDefinitionFactoryVtbl {
    pub const fn new<Impl: IReverbEffectDefinitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReverbEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IReverbEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&audiograph as *const <AudioGraph as ::windows::core::Abi>::Abi as *const <AudioGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReverbEffectDefinitionFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetDefaultSpatialAudioFormatResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SetDefaultSpatialAudioFormatStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetDefaultSpatialAudioFormatResult {
    const NAME: &'static str = "Windows.Media.Audio.ISetDefaultSpatialAudioFormatResult";
}
#[cfg(feature = "implement_exclusive")]
impl ISetDefaultSpatialAudioFormatResultVtbl {
    pub const fn new<Impl: ISetDefaultSpatialAudioFormatResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISetDefaultSpatialAudioFormatResultVtbl {
        unsafe extern "system" fn Status<Impl: ISetDefaultSpatialAudioFormatResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SetDefaultSpatialAudioFormatStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISetDefaultSpatialAudioFormatResult>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioDeviceConfigurationImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsSpatialAudioSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSpatialAudioFormatSupported(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn ActiveSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultSpatialAudioFormatAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>>;
    fn ConfigurationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConfigurationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAudioDeviceConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.ISpatialAudioDeviceConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAudioDeviceConfigurationVtbl {
    pub const fn new<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAudioDeviceConfigurationVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSpatialAudioSupported<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSpatialAudioSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSpatialAudioFormatSupported<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSpatialAudioFormatSupported(&*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveSpatialAudioFormat<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActiveSpatialAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultSpatialAudioFormat<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultSpatialAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultSpatialAudioFormatAsync<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultSpatialAudioFormatAsync(&*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigurationChanged<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConfigurationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConfigurationChanged<Impl: ISpatialAudioDeviceConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConfigurationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAudioDeviceConfiguration>, base.5, DeviceId::<Impl, OFFSET>, IsSpatialAudioSupported::<Impl, OFFSET>, IsSpatialAudioFormatSupported::<Impl, OFFSET>, ActiveSpatialAudioFormat::<Impl, OFFSET>, DefaultSpatialAudioFormat::<Impl, OFFSET>, SetDefaultSpatialAudioFormatAsync::<Impl, OFFSET>, ConfigurationChanged::<Impl, OFFSET>, RemoveConfigurationChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioDeviceConfigurationStaticsImpl: Sized {
    fn GetForDeviceId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<SpatialAudioDeviceConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAudioDeviceConfigurationStatics {
    const NAME: &'static str = "Windows.Media.Audio.ISpatialAudioDeviceConfigurationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAudioDeviceConfigurationStaticsVtbl {
    pub const fn new<Impl: ISpatialAudioDeviceConfigurationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAudioDeviceConfigurationStaticsVtbl {
        unsafe extern "system" fn GetForDeviceId<Impl: ISpatialAudioDeviceConfigurationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForDeviceId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAudioDeviceConfigurationStatics>, base.5, GetForDeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatConfigurationImpl: Sized {
    fn ReportLicenseChangedAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReportConfigurationChangedAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MixedRealityExclusiveModePolicy(&self) -> ::windows::core::Result<MixedRealitySpatialAudioFormatPolicy>;
    fn SetMixedRealityExclusiveModePolicy(&self, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAudioFormatConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.ISpatialAudioFormatConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAudioFormatConfigurationVtbl {
    pub const fn new<Impl: ISpatialAudioFormatConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAudioFormatConfigurationVtbl {
        unsafe extern "system" fn ReportLicenseChangedAsync<Impl: ISpatialAudioFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportLicenseChangedAsync(&*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportConfigurationChangedAsync<Impl: ISpatialAudioFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportConfigurationChangedAsync(&*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MixedRealityExclusiveModePolicy<Impl: ISpatialAudioFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MixedRealityExclusiveModePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMixedRealityExclusiveModePolicy<Impl: ISpatialAudioFormatConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMixedRealityExclusiveModePolicy(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAudioFormatConfiguration>, base.5, ReportLicenseChangedAsync::<Impl, OFFSET>, ReportConfigurationChangedAsync::<Impl, OFFSET>, MixedRealityExclusiveModePolicy::<Impl, OFFSET>, SetMixedRealityExclusiveModePolicy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatConfigurationStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SpatialAudioFormatConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAudioFormatConfigurationStatics {
    const NAME: &'static str = "Windows.Media.Audio.ISpatialAudioFormatConfigurationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAudioFormatConfigurationStaticsVtbl {
    pub const fn new<Impl: ISpatialAudioFormatConfigurationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAudioFormatConfigurationStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ISpatialAudioFormatConfigurationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAudioFormatConfigurationStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatSubtypeStaticsImpl: Sized {
    fn WindowsSonic(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DolbyAtmosForHeadphones(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DolbyAtmosForHomeTheater(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DolbyAtmosForSpeakers(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DTSHeadphoneX(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DTSXUltra(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAudioFormatSubtypeStatics {
    const NAME: &'static str = "Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAudioFormatSubtypeStaticsVtbl {
    pub const fn new<Impl: ISpatialAudioFormatSubtypeStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAudioFormatSubtypeStaticsVtbl {
        unsafe extern "system" fn WindowsSonic<Impl: ISpatialAudioFormatSubtypeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WindowsSonic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DolbyAtmosForHeadphones<Impl: ISpatialAudioFormatSubtypeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DolbyAtmosForHeadphones() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DolbyAtmosForHomeTheater<Impl: ISpatialAudioFormatSubtypeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DolbyAtmosForHomeTheater() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DolbyAtmosForSpeakers<Impl: ISpatialAudioFormatSubtypeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DolbyAtmosForSpeakers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DTSHeadphoneX<Impl: ISpatialAudioFormatSubtypeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DTSHeadphoneX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DTSXUltra<Impl: ISpatialAudioFormatSubtypeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DTSXUltra() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAudioFormatSubtypeStatics>, base.5, WindowsSonic::<Impl, OFFSET>, DolbyAtmosForHeadphones::<Impl, OFFSET>, DolbyAtmosForHomeTheater::<Impl, OFFSET>, DolbyAtmosForSpeakers::<Impl, OFFSET>, DTSHeadphoneX::<Impl, OFFSET>, DTSXUltra::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatSubtypeStatics2Impl: Sized {
    fn DTSXForHomeTheater(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialAudioFormatSubtypeStatics2 {
    const NAME: &'static str = "Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialAudioFormatSubtypeStatics2Vtbl {
    pub const fn new<Impl: ISpatialAudioFormatSubtypeStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialAudioFormatSubtypeStatics2Vtbl {
        unsafe extern "system" fn DTSXForHomeTheater<Impl: ISpatialAudioFormatSubtypeStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DTSXForHomeTheater() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialAudioFormatSubtypeStatics2>, base.5, DTSXForHomeTheater::<Impl, OFFSET>)
    }
}
