#[cfg(feature = "Win32_Foundation")]
pub trait IXAPO_Impl: Sized {
    fn GetRegistrationProperties(&self) -> ::windows::core::Result<*mut XAPO_REGISTRATION_PROPERTIES>;
    fn IsInputFormatSupported(&self, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<*mut super::WAVEFORMATEX>;
    fn IsOutputFormatSupported(&self, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<*mut super::WAVEFORMATEX>;
    fn Initialize(&self, pdata: *const ::core::ffi::c_void, databytesize: u32) -> ::windows::core::Result<()>;
    fn Reset(&self);
    fn LockForProcess(&self, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> ::windows::core::Result<()>;
    fn UnlockForProcess(&self);
    fn Process(&self, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL);
    fn CalcInputFrames(&self, outputframecount: u32) -> u32;
    fn CalcOutputFrames(&self, inputframecount: u32) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXAPO {}
#[cfg(feature = "Win32_Foundation")]
impl IXAPO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>() -> IXAPO_Vtbl {
        unsafe extern "system" fn GetRegistrationProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegistrationProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregistrationproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInputFormatSupported(::core::mem::transmute_copy(&poutputformat), ::core::mem::transmute_copy(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsupportedinputformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOutputFormatSupported(::core::mem::transmute_copy(&pinputformat), ::core::mem::transmute_copy(&prequestedoutputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsupportedoutputformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, databytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&databytesize)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset()
        }
        unsafe extern "system" fn LockForProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockForProcess(::core::mem::transmute_copy(&inputlockedparametercount), ::core::mem::transmute_copy(&pinputlockedparameters), ::core::mem::transmute_copy(&outputlockedparametercount), ::core::mem::transmute_copy(&poutputlockedparameters)).into()
        }
        unsafe extern "system" fn UnlockForProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockForProcess()
        }
        unsafe extern "system" fn Process<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Process(::core::mem::transmute_copy(&inputprocessparametercount), ::core::mem::transmute_copy(&pinputprocessparameters), ::core::mem::transmute_copy(&outputprocessparametercount), ::core::mem::transmute_copy(&poutputprocessparameters), ::core::mem::transmute_copy(&isenabled))
        }
        unsafe extern "system" fn CalcInputFrames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputframecount: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CalcInputFrames(::core::mem::transmute_copy(&outputframecount))
        }
        unsafe extern "system" fn CalcOutputFrames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputframecount: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CalcOutputFrames(::core::mem::transmute_copy(&inputframecount))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    fn SetSourcePosition(&self, position: *const HrtfPosition) -> ::windows::core::Result<()>;
    fn SetSourceOrientation(&self, orientation: *const HrtfOrientation) -> ::windows::core::Result<()>;
    fn SetSourceGain(&self, gain: f32) -> ::windows::core::Result<()>;
    fn SetEnvironment(&self, environment: HrtfEnvironment) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXAPOHrtfParameters {}
impl IXAPOHrtfParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>() -> IXAPOHrtfParameters_Vtbl {
        unsafe extern "system" fn SetSourcePosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *const HrtfPosition) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourcePosition(::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetSourceOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: *const HrtfOrientation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceOrientation(::core::mem::transmute_copy(&orientation)).into()
        }
        unsafe extern "system" fn SetSourceGain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceGain(::core::mem::transmute_copy(&gain)).into()
        }
        unsafe extern "system" fn SetEnvironment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: HrtfEnvironment) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnvironment(::core::mem::transmute_copy(&environment)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    fn SetParameters(&self, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32);
    fn GetParameters(&self, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32);
}
impl ::windows::core::RuntimeName for IXAPOParameters {}
impl IXAPOParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOParameters_Impl, const OFFSET: isize>() -> IXAPOParameters_Vtbl {
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&parameterbytesize))
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParameters(::core::mem::transmute_copy(&pparameters), ::core::mem::transmute_copy(&parameterbytesize))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    fn RegisterForCallbacks(&self, pcallback: &::core::option::Option<IXAudio2EngineCallback>) -> ::windows::core::Result<()>;
    fn UnregisterForCallbacks(&self, pcallback: &::core::option::Option<IXAudio2EngineCallback>);
    fn CreateSourceVoice(&self, ppsourcevoice: *mut ::core::option::Option<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: &::core::option::Option<IXAudio2VoiceCallback>, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::Result<()>;
    fn CreateSubmixVoice(&self, ppsubmixvoice: *mut ::core::option::Option<IXAudio2SubmixVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::Result<()>;
    fn CreateMasteringVoice(&self, ppmasteringvoice: *mut ::core::option::Option<IXAudio2MasteringVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: &::windows::core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<()>;
    fn StartEngine(&self) -> ::windows::core::Result<()>;
    fn StopEngine(&self);
    fn CommitChanges(&self, operationset: u32) -> ::windows::core::Result<()>;
    fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA);
    fn SetDebugConfiguration(&self, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut ::core::ffi::c_void);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXAudio2 {}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>() -> IXAudio2_Vtbl {
        unsafe extern "system" fn RegisterForCallbacks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterForCallbacks(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn UnregisterForCallbacks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterForCallbacks(::core::mem::transmute(&pcallback))
        }
        unsafe extern "system" fn CreateSourceVoice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourcevoice: *mut *mut ::core::ffi::c_void, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: *mut ::core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSourceVoice(::core::mem::transmute_copy(&ppsourcevoice), ::core::mem::transmute_copy(&psourceformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&maxfrequencyratio), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psendlist), ::core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn CreateSubmixVoice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubmixvoice: *mut *mut ::core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSubmixVoice(::core::mem::transmute_copy(&ppsubmixvoice), ::core::mem::transmute_copy(&inputchannels), ::core::mem::transmute_copy(&inputsamplerate), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&processingstage), ::core::mem::transmute_copy(&psendlist), ::core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn CreateMasteringVoice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmasteringvoice: *mut *mut ::core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: ::windows::core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateMasteringVoice(::core::mem::transmute_copy(&ppmasteringvoice), ::core::mem::transmute_copy(&inputchannels), ::core::mem::transmute_copy(&inputsamplerate), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&szdeviceid), ::core::mem::transmute_copy(&peffectchain), ::core::mem::transmute_copy(&streamcategory)).into()
        }
        unsafe extern "system" fn StartEngine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartEngine().into()
        }
        unsafe extern "system" fn StopEngine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopEngine()
        }
        unsafe extern "system" fn CommitChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitChanges(::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetPerformanceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPerformanceData(::core::mem::transmute_copy(&pperfdata))
        }
        unsafe extern "system" fn SetDebugConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebugConfiguration(::core::mem::transmute_copy(&pdebugconfiguration), ::core::mem::transmute_copy(&preserved))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
pub trait IXAudio2Extension_Impl: Sized {
    fn GetProcessingQuantum(&self, quantumnumerator: *mut u32, quantumdenominator: *mut u32);
    fn GetProcessor(&self, processor: *mut u32);
}
impl ::windows::core::RuntimeName for IXAudio2Extension {}
impl IXAudio2Extension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2Extension_Impl, const OFFSET: isize>() -> IXAudio2Extension_Vtbl {
        unsafe extern "system" fn GetProcessingQuantum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProcessingQuantum(::core::mem::transmute_copy(&quantumnumerator), ::core::mem::transmute_copy(&quantumdenominator))
        }
        unsafe extern "system" fn GetProcessor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processor: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProcessor(::core::mem::transmute_copy(&processor))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    fn GetChannelMask(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXAudio2MasteringVoice {}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2MasteringVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2MasteringVoice_Impl, const OFFSET: isize>() -> IXAudio2MasteringVoice_Vtbl {
        unsafe extern "system" fn GetChannelMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2MasteringVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchannelmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IXAudio2Voice_Vtbl::new::<Identity, Impl, OFFSET>(), GetChannelMask: GetChannelMask::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2MasteringVoice as ::windows::core::Interface>::IID || iid == &<IXAudio2Voice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXAudio2SourceVoice_Impl: Sized + IXAudio2Voice_Impl {
    fn Start(&self, flags: u32, operationset: u32) -> ::windows::core::Result<()>;
    fn Stop(&self, flags: u32, operationset: u32) -> ::windows::core::Result<()>;
    fn SubmitSourceBuffer(&self, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> ::windows::core::Result<()>;
    fn FlushSourceBuffers(&self) -> ::windows::core::Result<()>;
    fn Discontinuity(&self) -> ::windows::core::Result<()>;
    fn ExitLoop(&self, operationset: u32) -> ::windows::core::Result<()>;
    fn GetState(&self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32);
    fn SetFrequencyRatio(&self, ratio: f32, operationset: u32) -> ::windows::core::Result<()>;
    fn GetFrequencyRatio(&self, pratio: *mut f32);
    fn SetSourceSampleRate(&self, newsourcesamplerate: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IXAudio2SourceVoice {}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2SourceVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>() -> IXAudio2SourceVoice_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn SubmitSourceBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubmitSourceBuffer(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pbufferwma)).into()
        }
        unsafe extern "system" fn FlushSourceBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushSourceBuffers().into()
        }
        unsafe extern "system" fn Discontinuity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Discontinuity().into()
        }
        unsafe extern "system" fn ExitLoop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExitLoop(::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetState(::core::mem::transmute_copy(&pvoicestate), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn SetFrequencyRatio<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrequencyRatio(::core::mem::transmute_copy(&ratio), ::core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetFrequencyRatio<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pratio: *mut f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrequencyRatio(::core::mem::transmute_copy(&pratio))
        }
        unsafe extern "system" fn SetSourceSampleRate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SourceVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsourcesamplerate: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceSampleRate(::core::mem::transmute_copy(&newsourcesamplerate)).into()
        }
        Self {
            base__: IXAudio2Voice_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IXAudio2SubmixVoice {}
#[cfg(feature = "Win32_Foundation")]
impl IXAudio2SubmixVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2SubmixVoice_Impl, const OFFSET: isize>() -> IXAudio2SubmixVoice_Vtbl {
        Self { base__: IXAudio2Voice_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXAudio2SubmixVoice as ::windows::core::Interface>::IID || iid == &<IXAudio2Voice as ::windows::core::Interface>::IID
    }
}
