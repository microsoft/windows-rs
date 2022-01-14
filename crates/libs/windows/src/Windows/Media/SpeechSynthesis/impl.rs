#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInstalledVoicesStatic_Impl: Sized {
    fn AllVoices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VoiceInformation>>;
    fn DefaultVoice(&mut self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInstalledVoicesStatic {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IInstalledVoicesStatic";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInstalledVoicesStatic_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstalledVoicesStatic_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstalledVoicesStatic_Vtbl {
        unsafe extern "system" fn AllVoices<Impl: IInstalledVoicesStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllVoices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultVoice<Impl: IInstalledVoicesStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultVoice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInstalledVoicesStatic, BASE_OFFSET>(),
            AllVoices: AllVoices::<Impl, IMPL_OFFSET>,
            DefaultVoice: DefaultVoice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstalledVoicesStatic as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInstalledVoicesStatic2_Impl: Sized {
    fn TrySetDefaultVoiceAsync(&mut self, voice: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInstalledVoicesStatic2 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInstalledVoicesStatic2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstalledVoicesStatic2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstalledVoicesStatic2_Vtbl {
        unsafe extern "system" fn TrySetDefaultVoiceAsync<Impl: IInstalledVoicesStatic2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetDefaultVoiceAsync(&*(&voice as *const <VoiceInformation as ::windows::core::Abi>::Abi as *const <VoiceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInstalledVoicesStatic2, BASE_OFFSET>(),
            TrySetDefaultVoiceAsync: TrySetDefaultVoiceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstalledVoicesStatic2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISpeechSynthesisStream_Impl: Sized + super::super::Foundation::IClosable_Impl + super::super::Storage::Streams::IContentTypeProvider_Impl + super::super::Storage::Streams::IInputStream_Impl + super::super::Storage::Streams::IOutputStream_Impl + super::super::Storage::Streams::IRandomAccessStream_Impl + super::super::Storage::Streams::IRandomAccessStreamWithContentType_Impl {
    fn Markers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpeechSynthesisStream {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesisStream";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISpeechSynthesisStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesisStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesisStream_Vtbl {
        unsafe extern "system" fn Markers<Impl: ISpeechSynthesisStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechSynthesisStream, BASE_OFFSET>(), Markers: Markers::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechSynthesisStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpeechSynthesizer_Impl: Sized {
    fn SynthesizeTextToStreamAsync(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SynthesizeSsmlToStreamAsync(&mut self, ssml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SetVoice(&mut self, value: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<()>;
    fn Voice(&mut self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpeechSynthesizer {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpeechSynthesizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizer_Vtbl {
        unsafe extern "system" fn SynthesizeTextToStreamAsync<Impl: ISpeechSynthesizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SynthesizeTextToStreamAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SynthesizeSsmlToStreamAsync<Impl: ISpeechSynthesizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ssml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SynthesizeSsmlToStreamAsync(&*(&ssml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoice<Impl: ISpeechSynthesizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVoice(&*(&value as *const <VoiceInformation as ::windows::core::Abi>::Abi as *const <VoiceInformation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Voice<Impl: ISpeechSynthesizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Voice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechSynthesizer, BASE_OFFSET>(),
            SynthesizeTextToStreamAsync: SynthesizeTextToStreamAsync::<Impl, IMPL_OFFSET>,
            SynthesizeSsmlToStreamAsync: SynthesizeSsmlToStreamAsync::<Impl, IMPL_OFFSET>,
            SetVoice: SetVoice::<Impl, IMPL_OFFSET>,
            Voice: Voice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechSynthesizer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizer2_Impl: Sized {
    fn Options(&mut self) -> ::windows::core::Result<SpeechSynthesizerOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizer2 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizer2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizer2_Vtbl {
        unsafe extern "system" fn Options<Impl: ISpeechSynthesizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechSynthesizer2, BASE_OFFSET>(), Options: Options::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechSynthesizer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerOptions_Impl: Sized {
    fn IncludeWordBoundaryMetadata(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncludeWordBoundaryMetadata(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeSentenceBoundaryMetadata(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncludeSentenceBoundaryMetadata(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizerOptions {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizerOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizerOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizerOptions_Vtbl {
        unsafe extern "system" fn IncludeWordBoundaryMetadata<Impl: ISpeechSynthesizerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeWordBoundaryMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeWordBoundaryMetadata<Impl: ISpeechSynthesizerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeWordBoundaryMetadata(value).into()
        }
        unsafe extern "system" fn IncludeSentenceBoundaryMetadata<Impl: ISpeechSynthesizerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeSentenceBoundaryMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeSentenceBoundaryMetadata<Impl: ISpeechSynthesizerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeSentenceBoundaryMetadata(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechSynthesizerOptions, BASE_OFFSET>(),
            IncludeWordBoundaryMetadata: IncludeWordBoundaryMetadata::<Impl, IMPL_OFFSET>,
            SetIncludeWordBoundaryMetadata: SetIncludeWordBoundaryMetadata::<Impl, IMPL_OFFSET>,
            IncludeSentenceBoundaryMetadata: IncludeSentenceBoundaryMetadata::<Impl, IMPL_OFFSET>,
            SetIncludeSentenceBoundaryMetadata: SetIncludeSentenceBoundaryMetadata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechSynthesizerOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerOptions2_Impl: Sized {
    fn AudioVolume(&mut self) -> ::windows::core::Result<f64>;
    fn SetAudioVolume(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SpeakingRate(&mut self) -> ::windows::core::Result<f64>;
    fn SetSpeakingRate(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn AudioPitch(&mut self) -> ::windows::core::Result<f64>;
    fn SetAudioPitch(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizerOptions2 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizerOptions2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizerOptions2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizerOptions2_Vtbl {
        unsafe extern "system" fn AudioVolume<Impl: ISpeechSynthesizerOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioVolume<Impl: ISpeechSynthesizerOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioVolume(value).into()
        }
        unsafe extern "system" fn SpeakingRate<Impl: ISpeechSynthesizerOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakingRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeakingRate<Impl: ISpeechSynthesizerOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeakingRate(value).into()
        }
        unsafe extern "system" fn AudioPitch<Impl: ISpeechSynthesizerOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioPitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioPitch<Impl: ISpeechSynthesizerOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioPitch(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechSynthesizerOptions2, BASE_OFFSET>(),
            AudioVolume: AudioVolume::<Impl, IMPL_OFFSET>,
            SetAudioVolume: SetAudioVolume::<Impl, IMPL_OFFSET>,
            SpeakingRate: SpeakingRate::<Impl, IMPL_OFFSET>,
            SetSpeakingRate: SetSpeakingRate::<Impl, IMPL_OFFSET>,
            AudioPitch: AudioPitch::<Impl, IMPL_OFFSET>,
            SetAudioPitch: SetAudioPitch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechSynthesizerOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerOptions3_Impl: Sized {
    fn AppendedSilence(&mut self) -> ::windows::core::Result<SpeechAppendedSilence>;
    fn SetAppendedSilence(&mut self, value: SpeechAppendedSilence) -> ::windows::core::Result<()>;
    fn PunctuationSilence(&mut self) -> ::windows::core::Result<SpeechPunctuationSilence>;
    fn SetPunctuationSilence(&mut self, value: SpeechPunctuationSilence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizerOptions3 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizerOptions3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizerOptions3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizerOptions3_Vtbl {
        unsafe extern "system" fn AppendedSilence<Impl: ISpeechSynthesizerOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechAppendedSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendedSilence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppendedSilence<Impl: ISpeechSynthesizerOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpeechAppendedSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppendedSilence(value).into()
        }
        unsafe extern "system" fn PunctuationSilence<Impl: ISpeechSynthesizerOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechPunctuationSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PunctuationSilence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPunctuationSilence<Impl: ISpeechSynthesizerOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpeechPunctuationSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPunctuationSilence(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechSynthesizerOptions3, BASE_OFFSET>(),
            AppendedSilence: AppendedSilence::<Impl, IMPL_OFFSET>,
            SetAppendedSilence: SetAppendedSilence::<Impl, IMPL_OFFSET>,
            PunctuationSilence: PunctuationSilence::<Impl, IMPL_OFFSET>,
            SetPunctuationSilence: SetPunctuationSilence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechSynthesizerOptions3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceInformation_Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gender(&mut self) -> ::windows::core::Result<VoiceGender>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceInformation {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IVoiceInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceInformation_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IVoiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IVoiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IVoiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IVoiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gender<Impl: IVoiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceGender) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceInformation, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Gender: Gender::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceInformation as ::windows::core::Interface>::IID
    }
}
