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
impl ::windows::core::RuntimeName for IXAPO {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAPO";
}
impl IXAPOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOImpl, const OFFSET: isize>() -> IXAPOVtbl {
        unsafe extern "system" fn GetRegistrationProperties<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegistrationProperties(::core::mem::transmute_copy(&ppregistrationproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputFormatSupported(&*(&poutputformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&prequestedinputformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsupportedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOutputFormatSupported(&*(&pinputformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&prequestedoutputformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsupportedoutputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, databytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), databytesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn LockForProcess<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockForProcess(inputlockedparametercount, &*(&pinputlockedparameters as *const <XAPO_LOCKFORPROCESS_PARAMETERS as ::windows::core::Abi>::Abi as *const <XAPO_LOCKFORPROCESS_PARAMETERS as ::windows::core::DefaultType>::DefaultType), outputlockedparametercount, &*(&poutputlockedparameters as *const <XAPO_LOCKFORPROCESS_PARAMETERS as ::windows::core::Abi>::Abi as *const <XAPO_LOCKFORPROCESS_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockForProcess<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockForProcess().into()
        }
        unsafe extern "system" fn Process<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Process(
                    inputprocessparametercount,
                    &*(&pinputprocessparameters as *const <XAPO_PROCESS_BUFFER_PARAMETERS as ::windows::core::Abi>::Abi as *const <XAPO_PROCESS_BUFFER_PARAMETERS as ::windows::core::DefaultType>::DefaultType),
                    outputprocessparametercount,
                    &*(&poutputprocessparameters as *const <XAPO_PROCESS_BUFFER_PARAMETERS as ::windows::core::Abi>::Abi as *const <XAPO_PROCESS_BUFFER_PARAMETERS as ::windows::core::DefaultType>::DefaultType),
                    &*(&isenabled as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CalcInputFrames<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcInputFrames(outputframecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalcOutputFrames<Impl: IXAPOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcOutputFrames(inputframecount) {
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
            ::windows::core::GetRuntimeClassName::<IXAPO>,
            ::windows::core::GetTrustLevel,
            GetRegistrationProperties::<Impl, OFFSET>,
            IsInputFormatSupported::<Impl, OFFSET>,
            IsOutputFormatSupported::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            LockForProcess::<Impl, OFFSET>,
            UnlockForProcess::<Impl, OFFSET>,
            Process::<Impl, OFFSET>,
            CalcInputFrames::<Impl, OFFSET>,
            CalcOutputFrames::<Impl, OFFSET>,
        )
    }
}
pub trait IXAPOHrtfParametersImpl: Sized {
    fn SetSourcePosition();
    fn SetSourceOrientation();
    fn SetSourceGain();
    fn SetEnvironment();
}
impl ::windows::core::RuntimeName for IXAPOHrtfParameters {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAPOHrtfParameters";
}
impl IXAPOHrtfParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>() -> IXAPOHrtfParametersVtbl {
        unsafe extern "system" fn SetSourcePosition<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *const HrtfPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourcePosition(&*(&position as *const <HrtfPosition as ::windows::core::Abi>::Abi as *const <HrtfPosition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceOrientation<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: *const HrtfOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceOrientation(&*(&orientation as *const <HrtfOrientation as ::windows::core::Abi>::Abi as *const <HrtfOrientation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceGain<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceGain(gain) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironment<Impl: IXAPOHrtfParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: HrtfEnvironment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnvironment(environment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXAPOHrtfParameters>, ::windows::core::GetTrustLevel, SetSourcePosition::<Impl, OFFSET>, SetSourceOrientation::<Impl, OFFSET>, SetSourceGain::<Impl, OFFSET>, SetEnvironment::<Impl, OFFSET>)
    }
}
pub trait IXAPOParametersImpl: Sized {
    fn SetParameters();
    fn GetParameters();
}
impl ::windows::core::RuntimeName for IXAPOParameters {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAPOParameters";
}
impl IXAPOParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAPOParametersImpl, const OFFSET: isize>() -> IXAPOParametersVtbl {
        unsafe extern "system" fn SetParameters<Impl: IXAPOParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(&*(&pparameters as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), parameterbytesize).into()
        }
        unsafe extern "system" fn GetParameters<Impl: IXAPOParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParameters(::core::mem::transmute_copy(&pparameters), parameterbytesize).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXAPOParameters>, ::windows::core::GetTrustLevel, SetParameters::<Impl, OFFSET>, GetParameters::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXAudio2 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2";
}
impl IXAudio2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2Impl, const OFFSET: isize>() -> IXAudio2Vtbl {
        unsafe extern "system" fn RegisterForCallbacks<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterForCallbacks(&*(&pcallback as *const <IXAudio2EngineCallback as ::windows::core::Abi>::Abi as *const <IXAudio2EngineCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterForCallbacks<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterForCallbacks(&*(&pcallback as *const <IXAudio2EngineCallback as ::windows::core::Abi>::Abi as *const <IXAudio2EngineCallback as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateSourceVoice<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourcevoice: *mut ::windows::core::RawPtr, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: ::windows::core::RawPtr, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSourceVoice(
                ::core::mem::transmute_copy(&ppsourcevoice),
                &*(&psourceformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType),
                flags,
                maxfrequencyratio,
                &*(&pcallback as *const <IXAudio2VoiceCallback as ::windows::core::Abi>::Abi as *const <IXAudio2VoiceCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&psendlist as *const <XAUDIO2_VOICE_SENDS as ::windows::core::Abi>::Abi as *const <XAUDIO2_VOICE_SENDS as ::windows::core::DefaultType>::DefaultType),
                &*(&peffectchain as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::Abi>::Abi as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubmixVoice<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubmixvoice: *mut ::windows::core::RawPtr, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubmixVoice(::core::mem::transmute_copy(&ppsubmixvoice), inputchannels, inputsamplerate, flags, processingstage, &*(&psendlist as *const <XAUDIO2_VOICE_SENDS as ::windows::core::Abi>::Abi as *const <XAUDIO2_VOICE_SENDS as ::windows::core::DefaultType>::DefaultType), &*(&peffectchain as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::Abi>::Abi as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMasteringVoice<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmasteringvoice: *mut ::windows::core::RawPtr, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: super::super::super::Foundation::PWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMasteringVoice(::core::mem::transmute_copy(&ppmasteringvoice), inputchannels, inputsamplerate, flags, &*(&szdeviceid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&peffectchain as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::Abi>::Abi as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::DefaultType>::DefaultType), streamcategory) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartEngine<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartEngine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopEngine<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopEngine().into()
        }
        unsafe extern "system" fn CommitChanges<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitChanges(operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPerformanceData<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPerformanceData(::core::mem::transmute_copy(&pperfdata)).into()
        }
        unsafe extern "system" fn SetDebugConfiguration<Impl: IXAudio2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDebugConfiguration(&*(&pdebugconfiguration as *const <XAUDIO2_DEBUG_CONFIGURATION as ::windows::core::Abi>::Abi as *const <XAUDIO2_DEBUG_CONFIGURATION as ::windows::core::DefaultType>::DefaultType), &*(&preserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXAudio2>,
            ::windows::core::GetTrustLevel,
            RegisterForCallbacks::<Impl, OFFSET>,
            UnregisterForCallbacks::<Impl, OFFSET>,
            CreateSourceVoice::<Impl, OFFSET>,
            CreateSubmixVoice::<Impl, OFFSET>,
            CreateMasteringVoice::<Impl, OFFSET>,
            StartEngine::<Impl, OFFSET>,
            StopEngine::<Impl, OFFSET>,
            CommitChanges::<Impl, OFFSET>,
            GetPerformanceData::<Impl, OFFSET>,
            SetDebugConfiguration::<Impl, OFFSET>,
        )
    }
}
pub trait IXAudio2EngineCallbackImpl: Sized {
    fn OnProcessingPassStart();
    fn OnProcessingPassEnd();
    fn OnCriticalError();
}
impl ::windows::core::RuntimeName for IXAudio2EngineCallback {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2EngineCallback";
}
impl IXAudio2EngineCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2EngineCallbackImpl, const OFFSET: isize>() -> IXAudio2EngineCallbackVtbl {
        unsafe extern "system" fn OnProcessingPassStart<Impl: IXAudio2EngineCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProcessingPassStart().into()
        }
        unsafe extern "system" fn OnProcessingPassEnd<Impl: IXAudio2EngineCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProcessingPassEnd().into()
        }
        unsafe extern "system" fn OnCriticalError<Impl: IXAudio2EngineCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCriticalError(error).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXAudio2EngineCallback>, ::windows::core::GetTrustLevel, OnProcessingPassStart::<Impl, OFFSET>, OnProcessingPassEnd::<Impl, OFFSET>, OnCriticalError::<Impl, OFFSET>)
    }
}
pub trait IXAudio2ExtensionImpl: Sized {
    fn GetProcessingQuantum();
    fn GetProcessor();
}
impl ::windows::core::RuntimeName for IXAudio2Extension {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2Extension";
}
impl IXAudio2ExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2ExtensionImpl, const OFFSET: isize>() -> IXAudio2ExtensionVtbl {
        unsafe extern "system" fn GetProcessingQuantum<Impl: IXAudio2ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProcessingQuantum(::core::mem::transmute_copy(&quantumnumerator), quantumdenominator).into()
        }
        unsafe extern "system" fn GetProcessor<Impl: IXAudio2ExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processor: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProcessor(processor).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXAudio2Extension>, ::windows::core::GetTrustLevel, GetProcessingQuantum::<Impl, OFFSET>, GetProcessor::<Impl, OFFSET>)
    }
}
pub trait IXAudio2MasteringVoiceImpl: Sized + IXAudio2VoiceImpl {
    fn GetChannelMask();
}
impl ::windows::core::RuntimeName for IXAudio2MasteringVoice {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2MasteringVoice";
}
impl IXAudio2MasteringVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2MasteringVoiceImpl, const OFFSET: isize>() -> IXAudio2MasteringVoiceVtbl {
        unsafe extern "system" fn GetChannelMask<Impl: IXAudio2MasteringVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelMask(::core::mem::transmute_copy(&pchannelmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXAudio2MasteringVoice>, ::windows::core::GetTrustLevel, GetChannelMask::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IXAudio2SourceVoice {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2SourceVoice";
}
impl IXAudio2SourceVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>() -> IXAudio2SourceVoiceVtbl {
        unsafe extern "system" fn Start<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(flags, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop(flags, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitSourceBuffer<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitSourceBuffer(&*(&pbuffer as *const <XAUDIO2_BUFFER as ::windows::core::Abi>::Abi as *const <XAUDIO2_BUFFER as ::windows::core::DefaultType>::DefaultType), &*(&pbufferwma as *const <XAUDIO2_BUFFER_WMA as ::windows::core::Abi>::Abi as *const <XAUDIO2_BUFFER_WMA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushSourceBuffers<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushSourceBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discontinuity<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Discontinuity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitLoop<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitLoop(operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetState(::core::mem::transmute_copy(&pvoicestate), flags).into()
        }
        unsafe extern "system" fn SetFrequencyRatio<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratio: f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFrequencyRatio(ratio, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrequencyRatio<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pratio: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFrequencyRatio(::core::mem::transmute_copy(&pratio)).into()
        }
        unsafe extern "system" fn SetSourceSampleRate<Impl: IXAudio2SourceVoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsourcesamplerate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceSampleRate(newsourcesamplerate) {
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
            ::windows::core::GetRuntimeClassName::<IXAudio2SourceVoice>,
            ::windows::core::GetTrustLevel,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            SubmitSourceBuffer::<Impl, OFFSET>,
            FlushSourceBuffers::<Impl, OFFSET>,
            Discontinuity::<Impl, OFFSET>,
            ExitLoop::<Impl, OFFSET>,
            GetState::<Impl, OFFSET>,
            SetFrequencyRatio::<Impl, OFFSET>,
            GetFrequencyRatio::<Impl, OFFSET>,
            SetSourceSampleRate::<Impl, OFFSET>,
        )
    }
}
pub trait IXAudio2SubmixVoiceImpl: Sized + IXAudio2VoiceImpl {}
impl ::windows::core::RuntimeName for IXAudio2SubmixVoice {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2SubmixVoice";
}
impl IXAudio2SubmixVoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2SubmixVoiceImpl, const OFFSET: isize>() -> IXAudio2SubmixVoiceVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXAudio2SubmixVoice>, ::windows::core::GetTrustLevel)
    }
}
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
impl ::windows::core::RuntimeName for IXAudio2Voice {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2Voice";
}
impl IXAudio2VoiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceImpl, const OFFSET: isize>() -> IXAudio2VoiceVtbl {
        unsafe extern "system" fn GetVoiceDetails<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVoiceDetails(::core::mem::transmute_copy(&pvoicedetails)).into()
        }
        unsafe extern "system" fn SetOutputVoices<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputVoices(&*(&psendlist as *const <XAUDIO2_VOICE_SENDS as ::windows::core::Abi>::Abi as *const <XAUDIO2_VOICE_SENDS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffectChain<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffectChain(&*(&peffectchain as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::Abi>::Abi as *const <XAUDIO2_EFFECT_CHAIN as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableEffect<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableEffect(effectindex, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableEffect<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableEffect(effectindex, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectState<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, penabled: *mut super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectState(effectindex, ::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEffectParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffectParameters(effectindex, &*(&pparameters as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), parametersbytesize, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectParameters(effectindex, ::core::mem::transmute_copy(&pparameters), parametersbytesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFilterParameters(&*(&pparameters as *const <XAUDIO2_FILTER_PARAMETERS as ::windows::core::Abi>::Abi as *const <XAUDIO2_FILTER_PARAMETERS as ::windows::core::DefaultType>::DefaultType), operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterParameters(::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn SetOutputFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputFilterParameters(&*(&pdestinationvoice as *const <IXAudio2Voice as ::windows::core::Abi>::Abi as *const <IXAudio2Voice as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <XAUDIO2_FILTER_PARAMETERS as ::windows::core::Abi>::Abi as *const <XAUDIO2_FILTER_PARAMETERS as ::windows::core::DefaultType>::DefaultType), operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFilterParameters<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputFilterParameters(&*(&pdestinationvoice as *const <IXAudio2Voice as ::windows::core::Abi>::Abi as *const <IXAudio2Voice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn SetVolume<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolume(volume, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolume<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvolume: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVolume(::core::mem::transmute_copy(&pvolume)).into()
        }
        unsafe extern "system" fn SetChannelVolumes<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *const f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelVolumes(channels, pvolumes, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolumes<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelVolumes(channels, ::core::mem::transmute_copy(&pvolumes)).into()
        }
        unsafe extern "system" fn SetOutputMatrix<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputMatrix(&*(&pdestinationvoice as *const <IXAudio2Voice as ::windows::core::Abi>::Abi as *const <IXAudio2Voice as ::windows::core::DefaultType>::DefaultType), sourcechannels, destinationchannels, plevelmatrix, operationset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputMatrix<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationvoice: ::windows::core::RawPtr, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputMatrix(&*(&pdestinationvoice as *const <IXAudio2Voice as ::windows::core::Abi>::Abi as *const <IXAudio2Voice as ::windows::core::DefaultType>::DefaultType), sourcechannels, destinationchannels, ::core::mem::transmute_copy(&plevelmatrix)).into()
        }
        unsafe extern "system" fn DestroyVoice<Impl: IXAudio2VoiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyVoice().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXAudio2Voice>,
            ::windows::core::GetTrustLevel,
            GetVoiceDetails::<Impl, OFFSET>,
            SetOutputVoices::<Impl, OFFSET>,
            SetEffectChain::<Impl, OFFSET>,
            EnableEffect::<Impl, OFFSET>,
            DisableEffect::<Impl, OFFSET>,
            GetEffectState::<Impl, OFFSET>,
            SetEffectParameters::<Impl, OFFSET>,
            GetEffectParameters::<Impl, OFFSET>,
            SetFilterParameters::<Impl, OFFSET>,
            GetFilterParameters::<Impl, OFFSET>,
            SetOutputFilterParameters::<Impl, OFFSET>,
            GetOutputFilterParameters::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            GetVolume::<Impl, OFFSET>,
            SetChannelVolumes::<Impl, OFFSET>,
            GetChannelVolumes::<Impl, OFFSET>,
            SetOutputMatrix::<Impl, OFFSET>,
            GetOutputMatrix::<Impl, OFFSET>,
            DestroyVoice::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IXAudio2VoiceCallback {
    const NAME: &'static str = "Windows.Win32.Media.Audio.XAudio2.IXAudio2VoiceCallback";
}
impl IXAudio2VoiceCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>() -> IXAudio2VoiceCallbackVtbl {
        unsafe extern "system" fn OnVoiceProcessingPassStart<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesrequired: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVoiceProcessingPassStart(bytesrequired).into()
        }
        unsafe extern "system" fn OnVoiceProcessingPassEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVoiceProcessingPassEnd().into()
        }
        unsafe extern "system" fn OnStreamEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStreamEnd().into()
        }
        unsafe extern "system" fn OnBufferStart<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBufferStart(&*(&pbuffercontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnBufferEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBufferEnd(&*(&pbuffercontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnLoopEnd<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLoopEnd(&*(&pbuffercontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnVoiceError<Impl: IXAudio2VoiceCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void, error: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVoiceError(&*(&pbuffercontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), error).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXAudio2VoiceCallback>,
            ::windows::core::GetTrustLevel,
            OnVoiceProcessingPassStart::<Impl, OFFSET>,
            OnVoiceProcessingPassEnd::<Impl, OFFSET>,
            OnStreamEnd::<Impl, OFFSET>,
            OnBufferStart::<Impl, OFFSET>,
            OnBufferEnd::<Impl, OFFSET>,
            OnLoopEnd::<Impl, OFFSET>,
            OnVoiceError::<Impl, OFFSET>,
        )
    }
}
