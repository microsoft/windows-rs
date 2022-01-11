#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInstalledVoicesStaticImpl: Sized {
    fn AllVoices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VoiceInformation>>;
    fn DefaultVoice(&self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInstalledVoicesStatic {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IInstalledVoicesStatic";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInstalledVoicesStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstalledVoicesStaticImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstalledVoicesStaticVtbl {
        unsafe extern "system" fn AllVoices<Impl: IInstalledVoicesStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DefaultVoice<Impl: IInstalledVoicesStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInstalledVoicesStatic2Impl: Sized {
    fn TrySetDefaultVoiceAsync(&self, voice: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInstalledVoicesStatic2 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInstalledVoicesStatic2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstalledVoicesStatic2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstalledVoicesStatic2Vtbl {
        unsafe extern "system" fn TrySetDefaultVoiceAsync<Impl: IInstalledVoicesStatic2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISpeechSynthesisStreamImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl + IRandomAccessStreamWithContentTypeImpl {
    fn Markers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpeechSynthesisStream {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesisStream";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISpeechSynthesisStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesisStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesisStreamVtbl {
        unsafe extern "system" fn Markers<Impl: ISpeechSynthesisStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISpeechSynthesizerImpl: Sized {
    fn SynthesizeTextToStreamAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SynthesizeSsmlToStreamAsync(&self, ssml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SetVoice(&self, value: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<()>;
    fn Voice(&self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpeechSynthesizer {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpeechSynthesizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizerVtbl {
        unsafe extern "system" fn SynthesizeTextToStreamAsync<Impl: ISpeechSynthesizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SynthesizeSsmlToStreamAsync<Impl: ISpeechSynthesizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ssml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVoice<Impl: ISpeechSynthesizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVoice(&*(&value as *const <VoiceInformation as ::windows::core::Abi>::Abi as *const <VoiceInformation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Voice<Impl: ISpeechSynthesizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISpeechSynthesizer2Impl: Sized {
    fn Options(&self) -> ::windows::core::Result<SpeechSynthesizerOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizer2 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizer2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizer2Vtbl {
        unsafe extern "system" fn Options<Impl: ISpeechSynthesizer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISpeechSynthesizerOptionsImpl: Sized {
    fn IncludeWordBoundaryMetadata(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeWordBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeSentenceBoundaryMetadata(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeSentenceBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizerOptions {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizerOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizerOptionsVtbl {
        unsafe extern "system" fn IncludeWordBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncludeWordBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeWordBoundaryMetadata(value).into()
        }
        unsafe extern "system" fn IncludeSentenceBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncludeSentenceBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ISpeechSynthesizerOptions2Impl: Sized {
    fn AudioVolume(&self) -> ::windows::core::Result<f64>;
    fn SetAudioVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn SpeakingRate(&self) -> ::windows::core::Result<f64>;
    fn SetSpeakingRate(&self, value: f64) -> ::windows::core::Result<()>;
    fn AudioPitch(&self) -> ::windows::core::Result<f64>;
    fn SetAudioPitch(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizerOptions2 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizerOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizerOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizerOptions2Vtbl {
        unsafe extern "system" fn AudioVolume<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioVolume<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioVolume(value).into()
        }
        unsafe extern "system" fn SpeakingRate<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSpeakingRate<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeakingRate(value).into()
        }
        unsafe extern "system" fn AudioPitch<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioPitch<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait ISpeechSynthesizerOptions3Impl: Sized {
    fn AppendedSilence(&self) -> ::windows::core::Result<SpeechAppendedSilence>;
    fn SetAppendedSilence(&self, value: SpeechAppendedSilence) -> ::windows::core::Result<()>;
    fn PunctuationSilence(&self) -> ::windows::core::Result<SpeechPunctuationSilence>;
    fn SetPunctuationSilence(&self, value: SpeechPunctuationSilence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizerOptions3 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizerOptions3";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizerOptions3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechSynthesizerOptions3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechSynthesizerOptions3Vtbl {
        unsafe extern "system" fn AppendedSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechAppendedSilence) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppendedSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpeechAppendedSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppendedSilence(value).into()
        }
        unsafe extern "system" fn PunctuationSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechPunctuationSilence) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPunctuationSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpeechPunctuationSilence) -> ::windows::core::HRESULT {
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
pub trait IVoiceInformationImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gender(&self) -> ::windows::core::Result<VoiceGender>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceInformation {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IVoiceInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceInformationVtbl {
        unsafe extern "system" fn DisplayName<Impl: IVoiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IVoiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Language<Impl: IVoiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IVoiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gender<Impl: IVoiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceGender) -> ::windows::core::HRESULT {
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
