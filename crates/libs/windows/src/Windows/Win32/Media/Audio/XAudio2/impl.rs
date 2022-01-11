#[cfg(feature = "Win32_Foundation")]
pub trait IXAPOImpl: Sized {
    fn GetRegistrationProperties();
    fn IsInputFormatSupported();
    fn IsOutputFormatSupported();
    fn Initialize();
    fn Reset();
    fn LockForProcess();
    fn UnlockForProcess();
    fn Process();
    fn CalcInputFrames();
    fn CalcOutputFrames();
}
#[cfg(feature = "Win32_Foundation")]
impl IXAPOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAPOVtbl {
        unsafe extern "system" fn GetRegistrationProperties<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOutputFormatSupported<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, databytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockForProcess<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockForProcess<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Process<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalcInputFrames<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalcOutputFrames<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetRegistrationProperties::<Impl, IMPL_OFFSET>,
            IsInputFormatSupported::<Impl, IMPL_OFFSET>,
            IsOutputFormatSupported::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            LockForProcess::<Impl, IMPL_OFFSET>,
            UnlockForProcess::<Impl, IMPL_OFFSET>,
            Process::<Impl, IMPL_OFFSET>,
            CalcInputFrames::<Impl, IMPL_OFFSET>,
            CalcOutputFrames::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAPO as ::windows::core::Interface>::IID
    }
}
pub trait IXAPOHrtfParametersImpl: Sized {
    fn SetSourcePosition();
    fn SetSourceOrientation();
    fn SetSourceGain();
    fn SetEnvironment();
}
impl IXAPOHrtfParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOHrtfParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAPOHrtfParametersVtbl {
        unsafe extern "system" fn SetSourcePosition<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *const HrtfPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceOrientation<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: *const HrtfOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceGain<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnvironment<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: HrtfEnvironment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetSourcePosition::<Impl, IMPL_OFFSET>, SetSourceOrientation::<Impl, IMPL_OFFSET>, SetSourceGain::<Impl, IMPL_OFFSET>, SetEnvironment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAPOHrtfParameters as ::windows::core::Interface>::IID
    }
}
pub trait IXAPOParametersImpl: Sized {
    fn SetParameters();
    fn GetParameters();
}
impl IXAPOParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAPOParametersVtbl {
        unsafe extern "system" fn SetParameters<Impl: IXAPOParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParameters<Impl: IXAPOParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetParameters::<Impl, IMPL_OFFSET>, GetParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAPOParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2Impl: Sized {
    fn RegisterForCallbacks();
    fn UnregisterForCallbacks();
    fn CreateSourceVoice();
    fn CreateSubmixVoice();
    fn CreateMasteringVoice();
    fn StartEngine();
    fn StopEngine();
    fn CommitChanges();
    fn GetPerformanceData();
    fn SetDebugConfiguration();
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2Vtbl {
        unsafe extern "system" fn RegisterForCallbacks<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterForCallbacks<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSourceVoice<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourcevoice: *mut ::windows::core::RawPtr, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: ::windows::core::RawPtr, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSubmixVoice<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubmixvoice: *mut ::windows::core::RawPtr, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMasteringVoice<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmasteringvoice: *mut ::windows::core::RawPtr, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: super::super::super::Foundation::PWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartEngine<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopEngine<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitChanges<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPerformanceData<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDebugConfiguration<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RegisterForCallbacks::<Impl, IMPL_OFFSET>,
            UnregisterForCallbacks::<Impl, IMPL_OFFSET>,
            CreateSourceVoice::<Impl, IMPL_OFFSET>,
            CreateSubmixVoice::<Impl, IMPL_OFFSET>,
            CreateMasteringVoice::<Impl, IMPL_OFFSET>,
            StartEngine::<Impl, IMPL_OFFSET>,
            StopEngine::<Impl, IMPL_OFFSET>,
            CommitChanges::<Impl, IMPL_OFFSET>,
            GetPerformanceData::<Impl, IMPL_OFFSET>,
            SetDebugConfiguration::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2 as ::windows::core::Interface>::IID
    }
}
pub trait IXAudio2EngineCallbackImpl: Sized {
    fn OnProcessingPassStart();
    fn OnProcessingPassEnd();
    fn OnCriticalError();
}
impl IXAudio2EngineCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2EngineCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2EngineCallbackVtbl {
        unsafe extern "system" fn OnProcessingPassStart<Impl: IXAudio2EngineCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnProcessingPassEnd<Impl: IXAudio2EngineCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCriticalError<Impl: IXAudio2EngineCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(OnProcessingPassStart::<Impl, IMPL_OFFSET>, OnProcessingPassEnd::<Impl, IMPL_OFFSET>, OnCriticalError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2EngineCallback as ::windows::core::Interface>::IID
    }
}
pub trait IXAudio2ExtensionImpl: Sized {
    fn GetProcessingQuantum();
    fn GetProcessor();
}
impl IXAudio2ExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2ExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2ExtensionVtbl {
        unsafe extern "system" fn GetProcessingQuantum<Impl: IXAudio2ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcessor<Impl: IXAudio2ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processor: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProcessingQuantum::<Impl, IMPL_OFFSET>, GetProcessor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2Extension as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2MasteringVoiceImpl: Sized + IXAudio2VoiceImpl {
    fn GetChannelMask();
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2MasteringVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2MasteringVoiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2MasteringVoiceVtbl {
        unsafe extern "system" fn GetChannelMask<Impl: IXAudio2MasteringVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            GetVoiceDetails::<Impl, IMPL_OFFSET>,
            SetOutputVoices::<Impl, IMPL_OFFSET>,
            SetEffectChain::<Impl, IMPL_OFFSET>,
            EnableEffect::<Impl, IMPL_OFFSET>,
            DisableEffect::<Impl, IMPL_OFFSET>,
            GetEffectState::<Impl, IMPL_OFFSET>,
            SetEffectParameters::<Impl, IMPL_OFFSET>,
            GetEffectParameters::<Impl, IMPL_OFFSET>,
            SetFilterParameters::<Impl, IMPL_OFFSET>,
            GetFilterParameters::<Impl, IMPL_OFFSET>,
            SetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            GetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
            GetVolume::<Impl, IMPL_OFFSET>,
            SetChannelVolumes::<Impl, IMPL_OFFSET>,
            GetChannelVolumes::<Impl, IMPL_OFFSET>,
            SetOutputMatrix::<Impl, IMPL_OFFSET>,
            GetOutputMatrix::<Impl, IMPL_OFFSET>,
            DestroyVoice::<Impl, IMPL_OFFSET>,
            GetChannelMask::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2MasteringVoice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2SourceVoiceImpl: Sized + IXAudio2VoiceImpl {
    fn Start();
    fn Stop();
    fn SubmitSourceBuffer();
    fn FlushSourceBuffers();
    fn Discontinuity();
    fn ExitLoop();
    fn GetState();
    fn SetFrequencyRatio();
    fn GetFrequencyRatio();
    fn SetSourceSampleRate();
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2SourceVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2SourceVoiceVtbl {
        unsafe extern "system" fn Start<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubmitSourceBuffer<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushSourceBuffers<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Discontinuity<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExitLoop<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetState<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrequencyRatio<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrequencyRatio<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pratio: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceSampleRate<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsourcesamplerate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            GetVoiceDetails::<Impl, IMPL_OFFSET>,
            SetOutputVoices::<Impl, IMPL_OFFSET>,
            SetEffectChain::<Impl, IMPL_OFFSET>,
            EnableEffect::<Impl, IMPL_OFFSET>,
            DisableEffect::<Impl, IMPL_OFFSET>,
            GetEffectState::<Impl, IMPL_OFFSET>,
            SetEffectParameters::<Impl, IMPL_OFFSET>,
            GetEffectParameters::<Impl, IMPL_OFFSET>,
            SetFilterParameters::<Impl, IMPL_OFFSET>,
            GetFilterParameters::<Impl, IMPL_OFFSET>,
            SetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            GetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
            GetVolume::<Impl, IMPL_OFFSET>,
            SetChannelVolumes::<Impl, IMPL_OFFSET>,
            GetChannelVolumes::<Impl, IMPL_OFFSET>,
            SetOutputMatrix::<Impl, IMPL_OFFSET>,
            GetOutputMatrix::<Impl, IMPL_OFFSET>,
            DestroyVoice::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            SubmitSourceBuffer::<Impl, IMPL_OFFSET>,
            FlushSourceBuffers::<Impl, IMPL_OFFSET>,
            Discontinuity::<Impl, IMPL_OFFSET>,
            ExitLoop::<Impl, IMPL_OFFSET>,
            GetState::<Impl, IMPL_OFFSET>,
            SetFrequencyRatio::<Impl, IMPL_OFFSET>,
            GetFrequencyRatio::<Impl, IMPL_OFFSET>,
            SetSourceSampleRate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2SourceVoice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2SubmixVoiceImpl: Sized + IXAudio2VoiceImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2SubmixVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SubmixVoiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2SubmixVoiceVtbl {
        Self(
            GetVoiceDetails::<Impl, IMPL_OFFSET>,
            SetOutputVoices::<Impl, IMPL_OFFSET>,
            SetEffectChain::<Impl, IMPL_OFFSET>,
            EnableEffect::<Impl, IMPL_OFFSET>,
            DisableEffect::<Impl, IMPL_OFFSET>,
            GetEffectState::<Impl, IMPL_OFFSET>,
            SetEffectParameters::<Impl, IMPL_OFFSET>,
            GetEffectParameters::<Impl, IMPL_OFFSET>,
            SetFilterParameters::<Impl, IMPL_OFFSET>,
            GetFilterParameters::<Impl, IMPL_OFFSET>,
            SetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            GetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
            GetVolume::<Impl, IMPL_OFFSET>,
            SetChannelVolumes::<Impl, IMPL_OFFSET>,
            GetChannelVolumes::<Impl, IMPL_OFFSET>,
            SetOutputMatrix::<Impl, IMPL_OFFSET>,
            GetOutputMatrix::<Impl, IMPL_OFFSET>,
            DestroyVoice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2SubmixVoice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2VoiceImpl: Sized {
    fn GetVoiceDetails();
    fn SetOutputVoices();
    fn SetEffectChain();
    fn EnableEffect();
    fn DisableEffect();
    fn GetEffectState();
    fn SetEffectParameters();
    fn GetEffectParameters();
    fn SetFilterParameters();
    fn GetFilterParameters();
    fn SetOutputFilterParameters();
    fn GetOutputFilterParameters();
    fn SetVolume();
    fn GetVolume();
    fn SetChannelVolumes();
    fn GetChannelVolumes();
    fn SetOutputMatrix();
    fn GetOutputMatrix();
    fn DestroyVoice();
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2VoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2VoiceVtbl {
        unsafe extern "system" fn GetVoiceDetails<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputVoices<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffectChain<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableEffect<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableEffect<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectState<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, penabled: *mut super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEffectParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolume<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolume<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvolume: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChannelVolumes<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *const f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelVolumes<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputMatrix<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputMatrix<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyVoice<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            GetVoiceDetails::<Impl, IMPL_OFFSET>,
            SetOutputVoices::<Impl, IMPL_OFFSET>,
            SetEffectChain::<Impl, IMPL_OFFSET>,
            EnableEffect::<Impl, IMPL_OFFSET>,
            DisableEffect::<Impl, IMPL_OFFSET>,
            GetEffectState::<Impl, IMPL_OFFSET>,
            SetEffectParameters::<Impl, IMPL_OFFSET>,
            GetEffectParameters::<Impl, IMPL_OFFSET>,
            SetFilterParameters::<Impl, IMPL_OFFSET>,
            GetFilterParameters::<Impl, IMPL_OFFSET>,
            SetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            GetOutputFilterParameters::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
            GetVolume::<Impl, IMPL_OFFSET>,
            SetChannelVolumes::<Impl, IMPL_OFFSET>,
            GetChannelVolumes::<Impl, IMPL_OFFSET>,
            SetOutputMatrix::<Impl, IMPL_OFFSET>,
            GetOutputMatrix::<Impl, IMPL_OFFSET>,
            DestroyVoice::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2Voice as ::windows::core::Interface>::IID
    }
}
pub trait IXAudio2VoiceCallbackImpl: Sized {
    fn OnVoiceProcessingPassStart();
    fn OnVoiceProcessingPassEnd();
    fn OnStreamEnd();
    fn OnBufferStart();
    fn OnBufferEnd();
    fn OnLoopEnd();
    fn OnVoiceError();
}
impl IXAudio2VoiceCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXAudio2VoiceCallbackVtbl {
        unsafe extern "system" fn OnVoiceProcessingPassStart<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesrequired: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnVoiceProcessingPassEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStreamEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnBufferStart<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnBufferEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLoopEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnVoiceError<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(OnVoiceProcessingPassStart::<Impl, IMPL_OFFSET>, OnVoiceProcessingPassEnd::<Impl, IMPL_OFFSET>, OnStreamEnd::<Impl, IMPL_OFFSET>, OnBufferStart::<Impl, IMPL_OFFSET>, OnBufferEnd::<Impl, IMPL_OFFSET>, OnLoopEnd::<Impl, IMPL_OFFSET>, OnVoiceError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2VoiceCallback as ::windows::core::Interface>::IID
    }
}
