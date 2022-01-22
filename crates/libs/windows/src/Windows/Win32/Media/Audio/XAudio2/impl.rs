#[cfg(feature = "Win32_Foundation")]
pub trait IXAPO_Impl: Sized {
    fn GetRegistrationProperties(&mut self) -> ::windows::core::Result<*mut XAPO_REGISTRATION_PROPERTIES>;
    fn IsInputFormatSupported(&mut self, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<*mut super::WAVEFORMATEX>;
    fn IsOutputFormatSupported(&mut self, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<*mut super::WAVEFORMATEX>;
    fn Initialize(&mut self, pdata: *const ::core::ffi::c_void, databytesize: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self);
    fn LockForProcess(&mut self, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> ::windows::core::Result<()>;
    fn UnlockForProcess(&mut self);
    fn Process(&mut self, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL);
    fn CalcInputFrames(&mut self, outputframecount: u32) -> u32;
    fn CalcOutputFrames(&mut self, inputframecount: u32) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
impl IXAPO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>() -> IXAPO_Vtbl {
        unsafe extern "system" fn GetRegistrationProperties<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegistrationProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppregistrationproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInputFormatSupported(::core::mem::transmute_copy(&poutputformat), ::core::mem::transmute_copy(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedinputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOutputFormatSupported(::core::mem::transmute_copy(&pinputformat), ::core::mem::transmute_copy(&prequestedoutputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedoutputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, databytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&databytesize)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset()
        }
        unsafe extern "system" fn LockForProcess<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockForProcess(::core::mem::transmute_copy(&inputlockedparametercount), ::core::mem::transmute_copy(&pinputlockedparameters), ::core::mem::transmute_copy(&outputlockedparametercount), ::core::mem::transmute_copy(&poutputlockedparameters)).into()
        }
        unsafe extern "system" fn UnlockForProcess<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockForProcess()
        }
        unsafe extern "system" fn Process<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Process(::core::mem::transmute_copy(&inputprocessparametercount), ::core::mem::transmute_copy(&pinputprocessparameters), ::core::mem::transmute_copy(&outputprocessparametercount), ::core::mem::transmute_copy(&poutputprocessparameters), ::core::mem::transmute_copy(&isenabled))
        }
        unsafe extern "system" fn CalcInputFrames<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CalcInputFrames(::core::mem::transmute_copy(&outputframecount))
        }
        unsafe extern "system" fn CalcOutputFrames<Identity: ::windows::core::IUnknownImpl, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CalcOutputFrames(::core::mem::transmute_copy(&inputframecount))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRegistrationProperties: GetRegistrationProperties::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
            IsOutputFormatSupported: IsOutputFormatSupported::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            LockForProcess: LockForProcess::<Identity, Impl, OFFSET>,
            UnlockForProcess: UnlockForProcess::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
            CalcInputFrames: CalcInputFrames::<Identity, Impl, OFFSET>,
            CalcOutputFrames: CalcOutputFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAPO as ::windows::core::Interface>::IID
    }
}
pub trait IXAPOHrtfParameters_Impl: Sized {
    fn SetSourcePosition(&mut self, position: *const HrtfPosition) -> ::windows::core::Result<()>;
    fn SetSourceOrientation(&mut self, orientation: *const HrtfOrientation) -> ::windows::core::Result<()>;
    fn SetSourceGain(&mut self, gain: f32) -> ::windows::core::Result<()>;
    fn SetEnvironment(&mut self, environment: HrtfEnvironment) -> ::windows::core::Result<()>;
}
impl IXAPOHrtfParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>() -> IXAPOHrtfParameters_Vtbl {
        unsafe extern "system" fn SetSourcePosition<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *const HrtfPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourcePosition(::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetSourceOrientation<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: *const HrtfOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourceOrientation(::core::mem::transmute_copy(&orientation)).into()
        }
        unsafe extern "system" fn SetSourceGain<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourceGain(::core::mem::transmute_copy(&gain)).into()
        }
        unsafe extern "system" fn SetEnvironment<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: HrtfEnvironment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnvironment(::core::mem::transmute_copy(&environment)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSourcePosition: SetSourcePosition::<Identity, Impl, OFFSET>,
            SetSourceOrientation: SetSourceOrientation::<Identity, Impl, OFFSET>,
            SetSourceGain: SetSourceGain::<Identity, Impl, OFFSET>,
            SetEnvironment: SetEnvironment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAPOHrtfParameters as ::windows::core::Interface>::IID
    }
}
pub trait IXAPOParameters_Impl: Sized {
    fn SetParameters(&mut self, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32);
    fn GetParameters(&mut self, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32);
}
impl IXAPOParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOParameters_Impl, const OFFSET: isize>() -> IXAPOParameters_Vtbl {
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&parameterbytesize))
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetParameters(::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&parameterbytesize))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAPOParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2_Impl: Sized {
    fn RegisterForCallbacks(&mut self, pcallback: &::core::option::Option<IXAudio2EngineCallback>) -> ::windows::core::Result<()>;
    fn UnregisterForCallbacks(&mut self, pcallback: &::core::option::Option<IXAudio2EngineCallback>);
    fn CreateSourceVoice(&mut self, ppsourcevoice: *mut ::core::option::Option<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: &::core::option::Option<IXAudio2VoiceCallback>, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::Result<()>;
    fn CreateSubmixVoice(&mut self, ppsubmixvoice: *mut ::core::option::Option<IXAudio2SubmixVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::Result<()>;
    fn CreateMasteringVoice(&mut self, ppmasteringvoice: *mut ::core::option::Option<IXAudio2MasteringVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: super::super::super::Foundation::PWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<()>;
    fn StartEngine(&mut self) -> ::windows::core::Result<()>;
    fn StopEngine(&mut self);
    fn CommitChanges(&mut self, operationset: u32) -> ::windows::core::Result<()>;
    fn GetPerformanceData(&mut self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA);
    fn SetDebugConfiguration(&mut self, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut ::core::ffi::c_void);
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>() -> IXAudio2_Vtbl {
        unsafe extern "system" fn RegisterForCallbacks<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterForCallbacks(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn UnregisterForCallbacks<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterForCallbacks(::core::mem::transmute(&pcallback))
        }
        unsafe extern "system" fn CreateSourceVoice<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourcevoice: *mut ::windows::core::RawPtr, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: ::windows::core::RawPtr, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSourceVoice(::core::mem::transmute_copy(&ppsourcevoice), ::core::mem::transmute_copy(&psourceformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&maxfrequencyratio), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psendlist), ::core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn CreateSubmixVoice<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubmixvoice: *mut ::windows::core::RawPtr, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSubmixVoice(::core::mem::transmute_copy(&ppsubmixvoice), ::core::mem::transmute_copy(&inputchannels), ::core::mem::transmute_copy(&inputsamplerate), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&processingstage), ::core::mem::transmute_copy(&psendlist), ::core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn CreateMasteringVoice<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmasteringvoice: *mut ::windows::core::RawPtr, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: super::super::super::Foundation::PWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateMasteringVoice(::core::mem::transmute_copy(&ppmasteringvoice), ::core::mem::transmute_copy(&inputchannels), ::core::mem::transmute_copy(&inputsamplerate), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&szdeviceid), ::core::mem::transmute_copy(&peffectchain), ::core::mem::transmute_copy(&streamcategory)).into()
        }
        unsafe extern "system" fn StartEngine<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartEngine().into()
        }
        unsafe extern "system" fn StopEngine<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopEngine()
        }
        unsafe extern "system" fn CommitChanges<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitChanges(::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetPerformanceData<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPerformanceData(::core::mem::transmute_copy(&pperfdata))
        }
        unsafe extern "system" fn SetDebugConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDebugConfiguration(::core::mem::transmute_copy(&pdebugconfiguration), ::core::mem::transmute_copy(&preserved))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterForCallbacks: RegisterForCallbacks::<Identity, Impl, OFFSET>,
            UnregisterForCallbacks: UnregisterForCallbacks::<Identity, Impl, OFFSET>,
            CreateSourceVoice: CreateSourceVoice::<Identity, Impl, OFFSET>,
            CreateSubmixVoice: CreateSubmixVoice::<Identity, Impl, OFFSET>,
            CreateMasteringVoice: CreateMasteringVoice::<Identity, Impl, OFFSET>,
            StartEngine: StartEngine::<Identity, Impl, OFFSET>,
            StopEngine: StopEngine::<Identity, Impl, OFFSET>,
            CommitChanges: CommitChanges::<Identity, Impl, OFFSET>,
            GetPerformanceData: GetPerformanceData::<Identity, Impl, OFFSET>,
            SetDebugConfiguration: SetDebugConfiguration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2 as ::windows::core::Interface>::IID
    }
}
pub trait IXAudio2EngineCallback_Impl: Sized {
    fn OnProcessingPassStart(&mut self);
    fn OnProcessingPassEnd(&mut self);
    fn OnCriticalError(&mut self, error: ::windows::core::HRESULT);
}
impl IXAudio2EngineCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2EngineCallback_Impl, const OFFSET: isize>() -> IXAudio2EngineCallback_Vtbl {
        unsafe extern "system" fn OnProcessingPassStart<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2EngineCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProcessingPassStart()
        }
        unsafe extern "system" fn OnProcessingPassEnd<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2EngineCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProcessingPassEnd()
        }
        unsafe extern "system" fn OnCriticalError<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2EngineCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCriticalError(::core::mem::transmute_copy(&error))
        }
        Self {
            OnProcessingPassStart: OnProcessingPassStart::<Identity, Impl, OFFSET>,
            OnProcessingPassEnd: OnProcessingPassEnd::<Identity, Impl, OFFSET>,
            OnCriticalError: OnCriticalError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2EngineCallback as ::windows::core::Interface>::IID
    }
}
pub trait IXAudio2Extension_Impl: Sized {
    fn GetProcessingQuantum(&mut self, quantumnumerator: *mut u32, quantumdenominator: *mut u32);
    fn GetProcessor(&mut self, processor: *mut u32);
}
impl IXAudio2Extension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Extension_Impl, const OFFSET: isize>() -> IXAudio2Extension_Vtbl {
        unsafe extern "system" fn GetProcessingQuantum<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProcessingQuantum(::core::mem::transmute_copy(&quantumnumerator), ::core::mem::transmute_copy(&quantumdenominator))
        }
        unsafe extern "system" fn GetProcessor<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processor: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProcessor(::core::mem::transmute_copy(&processor))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProcessingQuantum: GetProcessingQuantum::<Identity, Impl, OFFSET>,
            GetProcessor: GetProcessor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2Extension as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2MasteringVoice_Impl: Sized + IXAudio2Voice_Impl {
    fn GetChannelMask(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2MasteringVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2MasteringVoice_Impl, const OFFSET: isize>() -> IXAudio2MasteringVoice_Vtbl {
        unsafe extern "system" fn GetChannelMask<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2MasteringVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChannelMask() {
                ::core::result::Result::Ok(ok__) => {
                    *pchannelmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IXAudio2Voice_Vtbl::new::<Identity, Impl, OFFSET>(), GetChannelMask: GetChannelMask::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2MasteringVoice as ::windows::core::Interface>::IID || iid == &<IXAudio2Voice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2SourceVoice_Impl: Sized + IXAudio2Voice_Impl {
    fn Start(&mut self, flags: u32, operationset: u32) -> ::windows::core::Result<()>;
    fn Stop(&mut self, flags: u32, operationset: u32) -> ::windows::core::Result<()>;
    fn SubmitSourceBuffer(&mut self, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> ::windows::core::Result<()>;
    fn FlushSourceBuffers(&mut self) -> ::windows::core::Result<()>;
    fn Discontinuity(&mut self) -> ::windows::core::Result<()>;
    fn ExitLoop(&mut self, operationset: u32) -> ::windows::core::Result<()>;
    fn GetState(&mut self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32);
    fn SetFrequencyRatio(&mut self, ratio: f32, operationset: u32) -> ::windows::core::Result<()>;
    fn GetFrequencyRatio(&mut self, pratio: *mut f32);
    fn SetSourceSampleRate(&mut self, newsourcesamplerate: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2SourceVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>() -> IXAudio2SourceVoice_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn SubmitSourceBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubmitSourceBuffer(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pbufferwma)).into()
        }
        unsafe extern "system" fn FlushSourceBuffers<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushSourceBuffers().into()
        }
        unsafe extern "system" fn Discontinuity<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Discontinuity().into()
        }
        unsafe extern "system" fn ExitLoop<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExitLoop(::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetState(::core::mem::transmute_copy(&pvoicestate), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn SetFrequencyRatio<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFrequencyRatio(::core::mem::transmute_copy(&ratio), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetFrequencyRatio<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pratio: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFrequencyRatio(::core::mem::transmute_copy(&pratio))
        }
        unsafe extern "system" fn SetSourceSampleRate<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsourcesamplerate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourceSampleRate(::core::mem::transmute_copy(&newsourcesamplerate)).into()
        }
        Self {
            base: IXAudio2Voice_Vtbl::new::<Identity, Impl, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            SubmitSourceBuffer: SubmitSourceBuffer::<Identity, Impl, OFFSET>,
            FlushSourceBuffers: FlushSourceBuffers::<Identity, Impl, OFFSET>,
            Discontinuity: Discontinuity::<Identity, Impl, OFFSET>,
            ExitLoop: ExitLoop::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            SetFrequencyRatio: SetFrequencyRatio::<Identity, Impl, OFFSET>,
            GetFrequencyRatio: GetFrequencyRatio::<Identity, Impl, OFFSET>,
            SetSourceSampleRate: SetSourceSampleRate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2SourceVoice as ::windows::core::Interface>::IID || iid == &<IXAudio2Voice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2SubmixVoice_Impl: Sized + IXAudio2Voice_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2SubmixVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SubmixVoice_Impl, const OFFSET: isize>() -> IXAudio2SubmixVoice_Vtbl {
        Self { base: IXAudio2Voice_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2SubmixVoice as ::windows::core::Interface>::IID || iid == &<IXAudio2Voice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2Voice_Impl: Sized {
    fn GetVoiceDetails(&mut self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS);
    fn SetOutputVoices(&mut self, psendlist: *const XAUDIO2_VOICE_SENDS) -> ::windows::core::Result<()>;
    fn SetEffectChain(&mut self, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::Result<()>;
    fn EnableEffect(&mut self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()>;
    fn DisableEffect(&mut self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()>;
    fn GetEffectState(&mut self, effectindex: u32, penabled: *mut super::super::super::Foundation::BOOL);
    fn SetEffectParameters(&mut self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows::core::Result<()>;
    fn GetEffectParameters(&mut self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows::core::Result<()>;
    fn SetFilterParameters(&mut self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()>;
    fn GetFilterParameters(&mut self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetOutputFilterParameters(&mut self, pdestinationvoice: &::core::option::Option<IXAudio2Voice>, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()>;
    fn GetOutputFilterParameters(&mut self, pdestinationvoice: &::core::option::Option<IXAudio2Voice>, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetVolume(&mut self, volume: f32, operationset: u32) -> ::windows::core::Result<()>;
    fn GetVolume(&mut self, pvolume: *mut f32);
    fn SetChannelVolumes(&mut self, channels: u32, pvolumes: *const f32, operationset: u32) -> ::windows::core::Result<()>;
    fn GetChannelVolumes(&mut self, channels: u32, pvolumes: *mut f32);
    fn SetOutputMatrix(&mut self, pdestinationvoice: &::core::option::Option<IXAudio2Voice>, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows::core::Result<()>;
    fn GetOutputMatrix(&mut self, pdestinationvoice: &::core::option::Option<IXAudio2Voice>, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32);
    fn DestroyVoice(&mut self);
}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2Voice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>() -> IXAudio2Voice_Vtbl {
        unsafe extern "system" fn GetVoiceDetails<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVoiceDetails(::core::mem::transmute_copy(&pvoicedetails))
        }
        unsafe extern "system" fn SetOutputVoices<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputVoices(::core::mem::transmute_copy(&psendlist)).into()
        }
        unsafe extern "system" fn SetEffectChain<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffectChain(::core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn EnableEffect<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableEffect(::core::mem::transmute_copy(&effectindex), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn DisableEffect<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableEffect(::core::mem::transmute_copy(&effectindex), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetEffectState<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, penabled: *mut super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectState(::core::mem::transmute_copy(&effectindex), ::core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn SetEffectParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffectParameters(::core::mem::transmute_copy(&effectindex), ::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&parametersbytesize), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetEffectParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectParameters(::core::mem::transmute_copy(&effectindex), ::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&parametersbytesize)).into()
        }
        unsafe extern "system" fn SetFilterParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilterParameters(::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetFilterParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilterParameters(::core::mem::transmute_copy(&pparameters))
        }
        unsafe extern "system" fn SetOutputFilterParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputFilterParameters(::core::mem::transmute(&pdestinationvoice), ::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetOutputFilterParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputFilterParameters(::core::mem::transmute(&pdestinationvoice), ::core::mem::transmute_copy(&pparameters))
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&volume), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetVolume<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvolume: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVolume(::core::mem::transmute_copy(&pvolume))
        }
        unsafe extern "system" fn SetChannelVolumes<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *const f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChannelVolumes(::core::mem::transmute_copy(&channels), ::core::mem::transmute_copy(&pvolumes), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetChannelVolumes<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChannelVolumes(::core::mem::transmute_copy(&channels), ::core::mem::transmute_copy(&pvolumes))
        }
        unsafe extern "system" fn SetOutputMatrix<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputMatrix(::core::mem::transmute(&pdestinationvoice), ::core::mem::transmute_copy(&sourcechannels), ::core::mem::transmute_copy(&destinationchannels), ::core::mem::transmute_copy(&plevelmatrix), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetOutputMatrix<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputMatrix(::core::mem::transmute(&pdestinationvoice), ::core::mem::transmute_copy(&sourcechannels), ::core::mem::transmute_copy(&destinationchannels), ::core::mem::transmute_copy(&plevelmatrix))
        }
        unsafe extern "system" fn DestroyVoice<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Voice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyVoice()
        }
        Self {
            GetVoiceDetails: GetVoiceDetails::<Identity, Impl, OFFSET>,
            SetOutputVoices: SetOutputVoices::<Identity, Impl, OFFSET>,
            SetEffectChain: SetEffectChain::<Identity, Impl, OFFSET>,
            EnableEffect: EnableEffect::<Identity, Impl, OFFSET>,
            DisableEffect: DisableEffect::<Identity, Impl, OFFSET>,
            GetEffectState: GetEffectState::<Identity, Impl, OFFSET>,
            SetEffectParameters: SetEffectParameters::<Identity, Impl, OFFSET>,
            GetEffectParameters: GetEffectParameters::<Identity, Impl, OFFSET>,
            SetFilterParameters: SetFilterParameters::<Identity, Impl, OFFSET>,
            GetFilterParameters: GetFilterParameters::<Identity, Impl, OFFSET>,
            SetOutputFilterParameters: SetOutputFilterParameters::<Identity, Impl, OFFSET>,
            GetOutputFilterParameters: GetOutputFilterParameters::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            GetVolume: GetVolume::<Identity, Impl, OFFSET>,
            SetChannelVolumes: SetChannelVolumes::<Identity, Impl, OFFSET>,
            GetChannelVolumes: GetChannelVolumes::<Identity, Impl, OFFSET>,
            SetOutputMatrix: SetOutputMatrix::<Identity, Impl, OFFSET>,
            GetOutputMatrix: GetOutputMatrix::<Identity, Impl, OFFSET>,
            DestroyVoice: DestroyVoice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2Voice as ::windows::core::Interface>::IID
    }
}
pub trait IXAudio2VoiceCallback_Impl: Sized {
    fn OnVoiceProcessingPassStart(&mut self, bytesrequired: u32);
    fn OnVoiceProcessingPassEnd(&mut self);
    fn OnStreamEnd(&mut self);
    fn OnBufferStart(&mut self, pbuffercontext: *mut ::core::ffi::c_void);
    fn OnBufferEnd(&mut self, pbuffercontext: *mut ::core::ffi::c_void);
    fn OnLoopEnd(&mut self, pbuffercontext: *mut ::core::ffi::c_void);
    fn OnVoiceError(&mut self, pbuffercontext: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT);
}
impl IXAudio2VoiceCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>() -> IXAudio2VoiceCallback_Vtbl {
        unsafe extern "system" fn OnVoiceProcessingPassStart<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesrequired: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnVoiceProcessingPassStart(::core::mem::transmute_copy(&bytesrequired))
        }
        unsafe extern "system" fn OnVoiceProcessingPassEnd<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnVoiceProcessingPassEnd()
        }
        unsafe extern "system" fn OnStreamEnd<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStreamEnd()
        }
        unsafe extern "system" fn OnBufferStart<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBufferStart(::core::mem::transmute_copy(&pbuffercontext))
        }
        unsafe extern "system" fn OnBufferEnd<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBufferEnd(::core::mem::transmute_copy(&pbuffercontext))
        }
        unsafe extern "system" fn OnLoopEnd<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLoopEnd(::core::mem::transmute_copy(&pbuffercontext))
        }
        unsafe extern "system" fn OnVoiceError<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnVoiceError(::core::mem::transmute_copy(&pbuffercontext), ::core::mem::transmute_copy(&error))
        }
        Self {
            OnVoiceProcessingPassStart: OnVoiceProcessingPassStart::<Identity, Impl, OFFSET>,
            OnVoiceProcessingPassEnd: OnVoiceProcessingPassEnd::<Identity, Impl, OFFSET>,
            OnStreamEnd: OnStreamEnd::<Identity, Impl, OFFSET>,
            OnBufferStart: OnBufferStart::<Identity, Impl, OFFSET>,
            OnBufferEnd: OnBufferEnd::<Identity, Impl, OFFSET>,
            OnLoopEnd: OnLoopEnd::<Identity, Impl, OFFSET>,
            OnVoiceError: OnVoiceError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2VoiceCallback as ::windows::core::Interface>::IID
    }
}
