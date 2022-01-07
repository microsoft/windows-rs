#[cfg(feature = "implement_exclusive")]
pub trait IInstalledVoicesStaticImpl: Sized {
    fn AllVoices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VoiceInformation>>;
    fn DefaultVoice(&self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInstalledVoicesStatic {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IInstalledVoicesStatic";
}
#[cfg(feature = "implement_exclusive")]
impl IInstalledVoicesStaticVtbl {
    pub const fn new<Impl: IInstalledVoicesStaticImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstalledVoicesStaticVtbl {
        unsafe extern "system" fn AllVoices<Impl: IInstalledVoicesStaticImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllVoices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultVoice<Impl: IInstalledVoicesStaticImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultVoice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstalledVoicesStatic>, base.5, AllVoices::<Impl, OFFSET>, DefaultVoice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstalledVoicesStatic2Impl: Sized {
    fn TrySetDefaultVoiceAsync(&self, voice: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInstalledVoicesStatic2 {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.IInstalledVoicesStatic2";
}
#[cfg(feature = "implement_exclusive")]
impl IInstalledVoicesStatic2Vtbl {
    pub const fn new<Impl: IInstalledVoicesStatic2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstalledVoicesStatic2Vtbl {
        unsafe extern "system" fn TrySetDefaultVoiceAsync<Impl: IInstalledVoicesStatic2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetDefaultVoiceAsync(&*(&voice as *const <VoiceInformation as ::windows::core::Abi>::Abi as *const <VoiceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstalledVoicesStatic2>, base.5, TrySetDefaultVoiceAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISpeechSynthesisStreamImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl + IRandomAccessStreamWithContentTypeImpl {
    fn Markers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpeechSynthesisStream {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesisStream";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISpeechSynthesisStreamVtbl {
    pub const fn new<Impl: ISpeechSynthesisStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechSynthesisStreamVtbl {
        unsafe extern "system" fn Markers<Impl: ISpeechSynthesisStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechSynthesisStream>, base.5, Markers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechSynthesizerImpl: Sized {
    fn SynthesizeTextToStreamAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SynthesizeSsmlToStreamAsync(&self, ssml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>;
    fn SetVoice(&self, value: &::core::option::Option<VoiceInformation>) -> ::windows::core::Result<()>;
    fn Voice(&self) -> ::windows::core::Result<VoiceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechSynthesizer {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.ISpeechSynthesizer";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechSynthesizerVtbl {
    pub const fn new<Impl: ISpeechSynthesizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechSynthesizerVtbl {
        unsafe extern "system" fn SynthesizeTextToStreamAsync<Impl: ISpeechSynthesizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SynthesizeTextToStreamAsync(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SynthesizeSsmlToStreamAsync<Impl: ISpeechSynthesizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ssml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SynthesizeSsmlToStreamAsync(&*(&ssml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoice<Impl: ISpeechSynthesizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVoice(&*(&value as *const <VoiceInformation as ::windows::core::Abi>::Abi as *const <VoiceInformation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Voice<Impl: ISpeechSynthesizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Voice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechSynthesizer>, base.5, SynthesizeTextToStreamAsync::<Impl, OFFSET>, SynthesizeSsmlToStreamAsync::<Impl, OFFSET>, SetVoice::<Impl, OFFSET>, Voice::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISpeechSynthesizer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechSynthesizer2Vtbl {
        unsafe extern "system" fn Options<Impl: ISpeechSynthesizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechSynthesizer2>, base.5, Options::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechSynthesizerOptionsVtbl {
        unsafe extern "system" fn IncludeWordBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludeWordBoundaryMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeWordBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIncludeWordBoundaryMetadata(value).into()
        }
        unsafe extern "system" fn IncludeSentenceBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludeSentenceBoundaryMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeSentenceBoundaryMetadata<Impl: ISpeechSynthesizerOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIncludeSentenceBoundaryMetadata(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechSynthesizerOptions>, base.5, IncludeWordBoundaryMetadata::<Impl, OFFSET>, SetIncludeWordBoundaryMetadata::<Impl, OFFSET>, IncludeSentenceBoundaryMetadata::<Impl, OFFSET>, SetIncludeSentenceBoundaryMetadata::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechSynthesizerOptions2Vtbl {
        unsafe extern "system" fn AudioVolume<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioVolume<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioVolume(value).into()
        }
        unsafe extern "system" fn SpeakingRate<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpeakingRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeakingRate<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpeakingRate(value).into()
        }
        unsafe extern "system" fn AudioPitch<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioPitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioPitch<Impl: ISpeechSynthesizerOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioPitch(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechSynthesizerOptions2>, base.5, AudioVolume::<Impl, OFFSET>, SetAudioVolume::<Impl, OFFSET>, SpeakingRate::<Impl, OFFSET>, SetSpeakingRate::<Impl, OFFSET>, AudioPitch::<Impl, OFFSET>, SetAudioPitch::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechSynthesizerOptions3Vtbl {
        unsafe extern "system" fn AppendedSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechAppendedSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppendedSilence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppendedSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SpeechAppendedSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppendedSilence(value).into()
        }
        unsafe extern "system" fn PunctuationSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechPunctuationSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PunctuationSilence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPunctuationSilence<Impl: ISpeechSynthesizerOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SpeechPunctuationSilence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPunctuationSilence(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechSynthesizerOptions3>, base.5, AppendedSilence::<Impl, OFFSET>, SetAppendedSilence::<Impl, OFFSET>, PunctuationSilence::<Impl, OFFSET>, SetPunctuationSilence::<Impl, OFFSET>)
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
    pub const fn new<Impl: IVoiceInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceInformationVtbl {
        unsafe extern "system" fn DisplayName<Impl: IVoiceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IVoiceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IVoiceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IVoiceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gender<Impl: IVoiceInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceGender) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceInformation>, base.5, DisplayName::<Impl, OFFSET>, Id::<Impl, OFFSET>, Language::<Impl, OFFSET>, Description::<Impl, OFFSET>, Gender::<Impl, OFFSET>)
    }
}
